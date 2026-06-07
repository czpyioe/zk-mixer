# zk-mixer

A ground-up reimplementation of Tornado Cash built purely for learning. The goal is to understand how privacy protocols work under the hood (ZK circuits, Merkle inclusion proofs, Groth16), and how they compose into a working mixer by building each piece in Rust from scratch using arkworks.

> **Academic use only.** Not audited, not production-ready. Built to learn ZK, nothing more.

## Architecture

```
zk-mixer/
│
├── docker-compose.yml 
├── .env.example
│
├── crates/                        
│   ├── circuit/                
│   ├── prover/       
│   └── setup/       
│
├── contracts/                    
│
├── frontend/
```


## References

- [Arkworks — Rust ZK toolkit](https://arkworks.rs/)
- [R1CS and QAP](https://risencrypto.github.io/R1CSQAP/)
- [Groth16 Explained](https://risencrypto.github.io/Groth16/)
- [Zero-Knowledge Proofs: Arkworks Usage Guide](https://www.byont.io/blog/zero-knowledge-proof-arkworks-usage-guide)
- [How Tornado Cash Works](https://rareskills.io/post/how-does-tornado-cash-work)
- [Tornado Cash Manual](https://zk.bearblog.dev/tornado-cash-manual/)
- [Arkworks R1CS Tutorial](https://github.com/arkworks-rs/r1cs-tutorial/)