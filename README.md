# zk-mixer

A ground-up reimplementation of Tornado Cash built purely for learning. The goal is to understand how privacy protocols work under the hood (ZK circuits, Merkle inclusion proofs, Groth16), and how they compose into a working mixer by building each piece in Rust from scratch using arkworks.

> **Academic use only.** Not audited, not production-ready. Built to learn ZK, nothing more.

## Architecture

```
zk-mixer
├── crates/             # rust crates for all the zk parts
│   ├── circuits/       # handle the ZK circuit and primitives (Poseidon, Merkle gadget, R1CS constraints)
│   ├── prover/         # proof generation + wasm-bindgen bindings
│   ├── setup/          # groth16 setup generation (keys, Verifier.sol)
├── contracts/          # protocol contracts (ZkMixer, MerkleTree, PoseidonHash and Verifier)
├── frontend/           # minimalist front to use the protocol
├── relayer/            # axum service that signs and broadcasts withdraws 
├── docker-compose.yml
└── deploy-e2e.sh
```


**Flow.** Deposit: the browser generates a secret note (secret + nullifier), sends the commitment on-chain. The commitment is the Poseidon Hash of the nullifier and the secret. Withdraw: the browser generates the proof using the wasm (private and public inputs needed). Then the proof is sent to the relayer service in order to break completly the link between the deposit address and the withdraw address. In order to withdraw, gas is needed to broadcast the tx, so without the relayer, the user should fund his withdraw address and creates a link with his existing addresses.

## Quick start

```bash
cp .env.example .env
./deploy-e2e.sh
```

## Stack

- **ZK / circuits**: Rust, [arkworks](https://arkworks.rs/) (Groth16 over BN254, Poseidon hash), compiled to WebAssembly via [wasm-pack](https://github.com/rustwasm/wasm-pack) / wasm-bindgen.
- **Contracts**: Solidity, [Foundry](https://book.getfoundry.sh/) (forge), deployed on Sepolia.
- **Frontend**: React + Vite + TypeScript, [viem](https://viem.sh/) for chain interaction.
- **Relayer**: Rust, [Axum](https://github.com/tokio-rs/axum) + [Tokio](https://tokio.rs/), [Alloy](https://alloy.rs/) for signing/broadcasting.
- **Infra**: Docker / Docker Compose.

## Status & limitations

This project exists to **understand how a ZK mixer works end to end** (circuit, proving, verifier, contracts, relayer, frontend). It is intentionally simplified and **must not be used in production**.

## Next improvements:

- **Relayer fees**: the relayer currently pays gas for free; it should deduct a fee from the withdrawn amount (the fee would also become a public input of the proof).
- **An indexer** : the frontend rebuilds the Merkle tree by scanning `Deposit` events through Etherscan on every withdraw; this belongs in a dedicated indexer/backend.

## References

- [Arkworks — Rust ZK toolkit](https://arkworks.rs/)
- [R1CS and QAP](https://risencrypto.github.io/R1CSQAP/)
- [Groth16 Explained](https://risencrypto.github.io/Groth16/)
- [Zero-Knowledge Proofs: Arkworks Usage Guide](https://www.byont.io/blog/zero-knowledge-proof-arkworks-usage-guide)
- [How Tornado Cash Works](https://rareskills.io/post/how-does-tornado-cash-work)
- [Tornado Cash Manual](https://zk.bearblog.dev/tornado-cash-manual/)
- [Arkworks R1CS Tutorial](https://github.com/arkworks-rs/r1cs-tutorial/)
- [Wasm Rust](https://wasm-bindgen.github.io/wasm-pack/book/introduction.html)
- [Viem](https://viem.sh/)
- [Axum Rust](https://www.rustfinity.com/blog/axum-rust-tutorial)
- [Tokio Rust](https://tokio.rs/)
- [Alloy Rust](https://alloy.rs/)