pub mod prove;
pub mod types;
pub mod keys;

use ark_bn254::Fr;
use wasm_bindgen::prelude::*;
use rand::rngs::OsRng;

use circuits::circuit::TREE_DEPTH;
use circuits::primitives::merkle_tree::{MerkleTree, Side};

/// build the Merkle tree from every on-chain commitment
/// returns { proof, publicInputs } as JSON
#[wasm_bindgen]
pub fn prove(
    secret_hex: &str,
    nullifier_hex: &str,
    leaves_json: &str, // JSON array of every commitment (hex), in insertion order
    recipient: &str,
) -> Result<JsValue, JsValue> {
    let secret_note = types::SecretNote {
        secret: types::hex_to_fr(secret_hex),
        nullifier: types::hex_to_fr(nullifier_hex),
    };
    let recipient_address = types::hex_to_fr(recipient);

    // rebuild the tree from the deposit commitments
    let leaf_hexes: Vec<String> = serde_json::from_str(leaves_json)
        .map_err(|e| JsValue::from_str(&format!("invalid leaves_json: {e}")))?;
    let leaves: Vec<Fr> = leaf_hexes.iter().map(|s| types::hex_to_fr(s)).collect();
    let mut tree = MerkleTree::new(TREE_DEPTH);
    for leaf in &leaves {
        tree.insert(*leaf);
    }

    // search commitment among the leaves
    let commitment = secret_note.commitment();
    let index = leaves
        .iter()
        .position(|leaf| *leaf == commitment)
        .ok_or_else(|| JsValue::from_str("commitment not found in leaves"))?;

    let merkle_proof = tree.generate_proof(index);
    let inputs = types::ProofInputs {
        secret_note,
        root: tree.root(),
        merkle_siblings: merkle_proof.siblings.iter().map(|(sib, _)| *sib).collect(),
        merkle_siblings_side: merkle_proof
            .siblings
            .iter()
            .map(|(_, side)| *side == Side::Right)
            .collect(),
        recepient_address: recipient_address,
    };

    let pk = keys::load_proving_key();
    let (proof, public_inputs) = prove::generate_proof(inputs, &pk)
        .map_err(|e| JsValue::from_str(&format!("proving failed: {e}")))?;

    let json = serde_json::json!({
        "proof": format!("0x{}", hex::encode(proof.to_solidity_call_data())),
        "publicInputs": {
            "root": format!("0x{}", hex::encode(types::fq_to_be_bytes(public_inputs.merkle_root))),
            "nullifier_hash": format!("0x{}", hex::encode(types::fq_to_be_bytes(public_inputs.nullifier_hash))),
            "recipient_address": format!("0x{}", hex::encode(types::fq_to_be_bytes(public_inputs.recipient_address)))
        },
    });
    Ok(JsValue::from_str(&json.to_string()))
}

#[wasm_bindgen]
pub fn generate_note() -> Result<JsValue, JsValue> {
    let note = types::SecretNote::generate(&mut OsRng);

    let json = serde_json::json!({
        "secret": format!("0x{}", hex::encode(types::fq_to_be_bytes(note.secret))),
        "nullifier": format!("0x{}", hex::encode(types::fq_to_be_bytes(note.nullifier))),
        "commitment": format!("0x{}", hex::encode(types::fq_to_be_bytes(note.commitment()))),
    });
    Ok(JsValue::from_str(&json.to_string()))
}



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
