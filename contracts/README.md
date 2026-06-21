## Contracts

### Architecture


**Deployment order (managed by Foundry):**

1. `Poseidon` : external library deployed automatically by Foundry before the script runs (its functions are `public`). Its address is statically linked into ZkMixer's bytecode.
2. `Verifier` : ZK proof verification contract (groth16). Deployed first in the script.
3. `ZkMixer` : main contract. Inherits from `MerkleTree` (no separate deployment, Merkle functions are compiled directly into ZkMixer). Receives the Verifier address in its constructor.

> `MerkleTree` is **not** deployed separately. It is Solidity inheritance: its functions (`_insert`, `IsOrWasRoot`, …) are part of `ZkMixer`'s bytecode.


### Test

```bash
forge test
```


### Deploy

**Anvil (local)**

```bash
anvil  # in a separate terminal
forge script script/DeployAnvil.s.sol --rpc-url anvil --broadcast
```

**Ethereum Sepolia**

Create a deployer account once:
```bash
cast wallet import deployer --interactive
```

Deploy and verify in one command:
```bash
./deploy-sepolia.sh
```

The script deploys all contracts, waits for Etherscan to index the block, then verifies each contract individually. Deployed addresses are printed at the end and saved to `broadcast/DeploySepolia.s.sol/11155111/run-latest.json`.


### References

- [Poseidon hash contract](https://github.com/geovgy/poseidon-solidity/blob/main/PoseidonT3.sol)
