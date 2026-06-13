use ark_snark::CircuitSpecificSetupSNARK;
use rand::rngs::OsRng;
use ark_bn254::Bn254;
use ark_groth16::Groth16;
use ark_groth16::data_structures::{VerifyingKey,ProvingKey};
use circuits::circuit;
use ark_serialize::CanonicalSerialize;
use anyhow::Result;

fn generate_verifier_contract(vk:VerifyingKey<Bn254>) -> Result<String> {
    let path= String::new();

    Ok(path)
}

fn main() {
    let mut rng = OsRng;
    let circuit = circuit::ZkMixer::default();
    let (pk, vk) = Groth16::<Bn254>::setup(circuit, &mut rng).unwrap();

    let mut pk_compressed_bytes = Vec::new();
    pk.serialize_compressed(&mut pk_compressed_bytes).unwrap();

    let mut vk_compressed_bytes = Vec::new();
    vk.serialize_compressed(&mut vk_compressed_bytes).unwrap();


    std::fs::create_dir_all("keys").unwrap();
    std::fs::write("keys/proving_key.bin", &pk_compressed_bytes).unwrap();
    std::fs::write("keys/verification_key.bin", &vk_compressed_bytes).unwrap();
    println!("pk: {} bytes", pk_compressed_bytes.len());
    println!("vk: {} bytes", vk_compressed_bytes.len());


    match generate_verifier_contract(vk) {
        Ok(path)=>println!("Verifier contract saved: {}",path),
        Err(_)=>eprintln!("Error while genertaing the verifier contract")
    }

}
