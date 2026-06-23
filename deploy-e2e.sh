#!/usr/bin/env bash
#
# End-to-end deploy: trusted setup -> Verifier -> wasm -> deploy -> .env -> containers.
#
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

for cmd in cargo wasm-pack forge cast python3 docker; do
  command -v "$cmd" >/dev/null 2>&1 || { echo "ERROR: '$cmd' not found in PATH"; exit 1; }
done
docker compose version >/dev/null 2>&1 || { echo "ERROR: 'docker compose' (v2) not available"; exit 1; }
[ -f .env ] || { echo "ERROR: .env not found at repo root (copy .env.example)"; exit 1; }

set -a; source .env; set +a

BROADCAST="contracts/broadcast/DeploySepolia.s.sol/11155111/run-latest.json"

update_env() {
  python3 - "$1" "$2" <<'EOF'
import sys
key, val = sys.argv[1], sys.argv[2]
path = ".env"
lines = open(path).read().splitlines()
found = False
for i, l in enumerate(lines):
    if l.startswith(key + "="):
        lines[i] = f"{key}={val}"
        found = True
if not found:
    lines.append(f"{key}={val}")
open(path, "w").write("\n".join(lines) + "\n")
EOF
}

echo "==> [1/6] Trusted setup (proving/verification keys + Verifier.sol)..."
( cd crates && cargo run -p setup --release )

echo "==> [2/6] Copying generated Verifier.sol into contracts/src..."
cp crates/setup/src/contract/generated/Verifier.sol contracts/src/Verifier.sol

echo "==> [3/6] Rebuilding the wasm prover into the frontend (proving key changed)..."
wasm-pack build crates/prover --target web --out-dir "$ROOT/frontend/src/wasm"

echo "==> [4/6] Deploying contracts to Sepolia..."
./contracts/deploy-sepolia.sh

echo "==> [5/6] Updating .env (contract address + deploy block)..."
read -r ZKMIXER_ADDR DEPLOY_BLOCK < <(python3 - "$BROADCAST" <<'EOF'
import json, sys
d = json.load(open(sys.argv[1]))
addr = next((tx["contractAddress"] for tx in d["transactions"]
             if tx.get("transactionType") == "CREATE" and tx.get("contractName") == "ZkMixer"), None)
block = next((int(r["blockNumber"], 16) for r in d.get("receipts", [])
             if addr and r.get("contractAddress", "").lower() == addr.lower()), None)
if not addr or block is None:
    sys.exit("could not parse ZkMixer address/block from broadcast")
print(addr, block)
EOF
)

update_env VITE_CONTRACT_ADDRESS    "$ZKMIXER_ADDR"
update_env RELAYER_CONTRACT_ADDRESS "$ZKMIXER_ADDR"
update_env VITE_DEPLOY_BLOCK        "$DEPLOY_BLOCK"
echo "    ZkMixer = $ZKMIXER_ADDR"
echo "    block   = $DEPLOY_BLOCK"

echo "==> [6/6] Building and starting containers..."
set -a; source .env; set +a
docker compose up -d --build

echo ""
echo "==> Done."
echo "    Frontend : http://localhost:8080"
echo "    Relayer  : http://localhost:3000"
