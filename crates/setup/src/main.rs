use ark_snark::CircuitSpecificSetupSNARK;
use rand::rngs::OsRng;
use ark_bn254::Bn254;
use ark_groth16::Groth16;
use ark_groth16::data_structures::VerifyingKey;
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



#[cfg(test)]
mod tests {
    use super::*;
    use ark_snark::SNARK;
    use ark_bn254::Fr;
    use ark_groth16::data_structures::ProvingKey;
    use ark_serialize::CanonicalDeserialize;
    use circuits::primitives::hash::{compute_commitment, compute_nullifier_hash};
    use circuits::primitives::merkle_tree::{MerkleTree, Side};
    use circuits::circuit::{ZkMixer, TREE_DEPTH};

    fn load_keys() -> (ProvingKey<Bn254>, VerifyingKey<Bn254>) {
        let pk_bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../keys/proving_key.bin"))
            .expect("keys/proving_key.bin not found. run the setup binary first");
        let vk_bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../keys/verification_key.bin"))
            .expect("keys/verification_key.bin not found. run the setup binary first");
        let pk = ProvingKey::deserialize_compressed(pk_bytes.as_slice()).unwrap();
        let vk = VerifyingKey::deserialize_compressed(vk_bytes.as_slice()).unwrap();
        (pk, vk)
    }

    fn valid_witness(secret: Fr, nullifier: Fr) -> ZkMixer {
        let commitment = compute_commitment(secret, nullifier);
        let nullifier_hash = compute_nullifier_hash(nullifier);
        let mut tree = MerkleTree::new(TREE_DEPTH);
        let idx = tree.insert(commitment);
        let proof = tree.generate_proof(idx);
        ZkMixer {
            secret: Some(secret),
            nullifier: Some(nullifier),
            merkle_siblings: Some(proof.siblings.iter().map(|(sib, _)| *sib).collect()),
            merkle_siblings_sides: Some(proof.siblings.iter().map(|(_, side)| *side == Side::Right).collect()),
            merkle_root: Some(tree.root()),
            nullifier_hash: Some(nullifier_hash),
            recipient_address: Some(Fr::from(0xabc123)),
        }
    }

    #[test]
    fn test_keys_load_from_disk() {
        let (_pk, _vk) = load_keys();
    }

    #[test]
    fn test_prove_verify_valid_witness() {
        let mut rng = OsRng;
        let (pk, vk) = load_keys();

        let witness = valid_witness(Fr::from(1), Fr::from(2));
        let root = witness.merkle_root.unwrap();
        let nullifier_hash = witness.nullifier_hash.unwrap();
        let recipient = witness.recipient_address.unwrap();

        let proof = Groth16::<Bn254>::prove(&pk, witness, &mut rng).unwrap();
        let public_inputs = vec![root, nullifier_hash, recipient];

        assert!(Groth16::<Bn254>::verify(&vk, &public_inputs, &proof).unwrap());
    }

    #[test]
    fn test_verify_fails_wrong_public_inputs() {
        let mut rng = OsRng;
        let (pk, vk) = load_keys();

        let witness = valid_witness(Fr::from(1), Fr::from(2));
        let proof = Groth16::<Bn254>::prove(&pk, witness, &mut rng).unwrap();

        let fake_inputs = vec![Fr::from(999), Fr::from(888), Fr::from(777)];
        assert!(!Groth16::<Bn254>::verify(&vk, &fake_inputs, &proof).unwrap());
    }

    #[test]
    fn test_verify_fails_wrong_witness() {
        let (pk, _vk) = load_keys();

        let mut witness = valid_witness(Fr::from(1), Fr::from(2));
        witness.secret = Some(Fr::from(3));

        let result = std::panic::catch_unwind(|| {
            Groth16::<Bn254>::prove(&pk, witness, &mut OsRng).unwrap()
        });

        assert!(result.is_err(), "prove() panicked while trying to create the false proof");
    }
}

