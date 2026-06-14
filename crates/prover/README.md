# prover

WASM bindings for proof generation: `generate_note` (deposit) and `prove` (withdraw).

## Build the WASM package

From the workspace root (`crates/`):

```bash
# 1. Generate the proving key
cargo run -p setup

# 2. Compile to WebAssembly. Output -> crates/prover/pkg/
wasm-pack build prover --target web
```


### One-time tooling setup

```bash
rustup target add wasm32-unknown-unknown
curl https://wasm-bindgen.github.io/wasm-pack/installer/init.sh -sSf | sh
```


### Links

- [WASM Docs](https://wasm-bindgen.github.io/wasm-pack/)
