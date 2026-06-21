#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [ -f .env ]; then
  source .env
fi

: "${ETHERSCAN_API_KEY:?ETHERSCAN_API_KEY is not set}"
: "${SEPOLIA_RPC_URL:?SEPOLIA_RPC_URL is not set}"

BROADCAST="broadcast/DeploySepolia.s.sol/11155111/run-latest.json"

# 1. Deploy
echo "==> Deploying on Sepolia..."
forge script script/DeploySepolia.s.sol \
  --account deployer \
  --broadcast \
  --rpc-url sepolia

# 2. Parse addresses from broadcast
eval "$(python3 - <<'EOF'
import json
d = json.load(open("broadcast/DeploySepolia.s.sol/11155111/run-latest.json"))
for tx in d["transactions"]:
    if tx["transactionType"] == "CREATE":
        print(f'{tx["contractName"].upper()}_ADDR={tx["contractAddress"]}')
for lib in d.get("libraries", []):
    name = lib.split(":")[1].upper()
    addr = lib.split(":")[2]
    print(f"{name}_ADDR={addr}")
EOF
)"

echo ""
echo "==> Deployed:"
echo "    Poseidon  ${POSEIDON_ADDR:-not deployed}"
echo "    Verifier  $VERIFIER_ADDR"
echo "    ZkMixer   $ZKMIXER_ADDR"

# 3. Wait for Etherscan to index
echo ""
echo "==> Waiting 30s for Etherscan to index..."
sleep 30

# 4. Verify each contract separately
echo ""
echo "==> Verifying contracts..."

if [ -n "${POSEIDON_ADDR:-}" ]; then
  echo "--> Poseidon library ($POSEIDON_ADDR)"
  forge verify-contract "$POSEIDON_ADDR" \
    src/PoseidonT3.sol:Poseidon \
    --chain sepolia \
    --etherscan-api-key "$ETHERSCAN_API_KEY" \
    --retries 5 --delay 15
fi

echo "--> Verifier ($VERIFIER_ADDR)"
forge verify-contract "$VERIFIER_ADDR" \
  src/Verifier.sol:Verifier \
  --chain sepolia \
  --etherscan-api-key "$ETHERSCAN_API_KEY" \
  --retries 5 --delay 15

echo "--> ZkMixer ($ZKMIXER_ADDR)"
forge verify-contract "$ZKMIXER_ADDR" \
  src/ZkMixer.sol:ZkMixer \
  --constructor-args "$(cast abi-encode "constructor(address)" "$VERIFIER_ADDR")" \
  ${POSEIDON_ADDR:+--libraries "src/PoseidonT3.sol:Poseidon:$POSEIDON_ADDR"} \
  --chain sepolia \
  --etherscan-api-key "$ETHERSCAN_API_KEY" \
  --retries 5 --delay 15

echo ""
echo "==> All contracts deployed and verified."
