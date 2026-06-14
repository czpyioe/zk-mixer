pub mod prove;
pub mod types;
pub mod keys;

use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use wasm_bindgen::prelude::*;

// return a JSON: { proof, publicInputs }
#[wasm_bindgen]
pub fn prove(
    secret_hex: &str,
    nullifier_hex: &str,
    root_hex: &str,
    siblings_json: &str, // JSON array of hex strings, leaf -> root
    path_indices_json: &str, // JSON array of bools, true = sibling on the right
    recipient: &str
) -> Result<JsValue, JsValue> {
    let secret = hex_to_fr(secret_hex);
    let nullifier = hex_to_fr(nullifier_hex);
    let secret_note = types::SecretNote {
        secret: secret,
        nullifier: nullifier
    };

    let root = hex_to_fr(root_hex);
    let recepient_address = hex_to_fr(recipient);

    let sibling_hexes: Vec<String> = serde_json::from_str(siblings_json)
        .map_err(|e| JsValue::from_str(&format!("invalid siblings_json: {e}")))?;
    let merkle_siblings: Vec<Fr> = sibling_hexes.iter().map(|s| hex_to_fr(s)).collect();
    let merkle_siblings_side: Vec<bool> = serde_json::from_str(path_indices_json)
        .map_err(|e| JsValue::from_str(&format!("invalid path_indices_json: {e}")))?;

    let inputs = types::ProofInputs{
        secret_note: secret_note,
        root: root,
        merkle_siblings: merkle_siblings,
        merkle_siblings_side: merkle_siblings_side,
        recepient_address: recepient_address
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


fn hex_to_fr(s: &str) -> Fr {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).expect("invalid hex");
    // from_be_bytes_mod_order also accepts the 20-byte recipient address.
    Fr::from_be_bytes_mod_order(&bytes)
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
