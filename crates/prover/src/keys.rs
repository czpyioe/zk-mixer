use ark_bn254::Bn254;
use ark_groth16::data_structures::ProvingKey;
use ark_serialize::CanonicalDeserialize;

// needed for wasm (file system doesn't work)
static PROVING_KEY_BYTES: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../keys/proving_key.bin"));

pub fn load_proving_key() -> ProvingKey<Bn254> {
    ProvingKey::deserialize_compressed(PROVING_KEY_BYTES)
        .expect("embedded proving key is malformed; regenerate it with the setup binary")
}
