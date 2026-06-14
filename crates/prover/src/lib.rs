pub mod prove;
pub mod types;
pub mod keys;

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{Bn254, Fr};
    use ark_groth16::Groth16;
    use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
    use rand::rngs::OsRng;

    use circuits::circuit::{ZkMixer, TREE_DEPTH};
    use circuits::primitives::merkle_tree::{MerkleTree, Side};

    use types::{ProofInputs, SecretNote};

    #[test]
    fn test_note_bytes_roundtrip() {
        let note = SecretNote::generate(&mut OsRng);
        let restored = SecretNote::from_bytes(&note.to_bytes());

        assert_eq!(note.secret, restored.secret);
        assert_eq!(note.nullifier, restored.nullifier);
    }

    #[test]
    fn test_generate_proof_verifies() {
        let mut rng = OsRng;

        let (pk, vk) = Groth16::<Bn254>::setup(ZkMixer::default(), &mut rng).unwrap();

        let note = SecretNote::generate(&mut rng);
        let mut tree = MerkleTree::new(TREE_DEPTH);
        let idx = tree.insert(note.commitment());
        let merkle_proof = tree.generate_proof(idx);

        let inputs = ProofInputs {
            merkle_siblings: merkle_proof.siblings.iter().map(|(sib, _)| *sib).collect(),
            merkle_siblings_side: merkle_proof
                .siblings
                .iter()
                .map(|(_, side)| *side == Side::Right)
                .collect(),
            root: tree.root(),
            recepient_address: Fr::from(0xabc123),
            secret_note: note,
        };

        let (proof, public_inputs) = prove::generate_proof(inputs, &pk).unwrap();

        let ark_proof = ark_groth16::Proof {
            a: proof.a,
            b: proof.b,
            c: proof.c,
        };
        assert!(Groth16::<Bn254>::verify(&vk, &public_inputs.to_vec(), &ark_proof).unwrap());

    }
}
