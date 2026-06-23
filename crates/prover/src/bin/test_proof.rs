use rand::rngs::OsRng;
use ark_bn254::Fr;
use circuits::circuit::TREE_DEPTH;
use circuits::primitives::merkle_tree::{MerkleTree, Side};
use prover::keys;
use prover::types::{SecretNote, ProofInputs, fq_to_be_bytes};
use prover::prove::generate_proof;

fn main() {
    let pk = keys::load_proving_key();
    let mut rng = OsRng;

    let note = SecretNote::generate(&mut rng);
    let commitment = note.commitment();

    let mut tree = MerkleTree::new(TREE_DEPTH);
    let idx = tree.insert(commitment);
    let merkle_proof = tree.generate_proof(idx);
    let root = tree.root();

    let siblings: Vec<Fr> = merkle_proof.siblings.iter().map(|(sib, _)| *sib).collect();
    let sides: Vec<bool> = merkle_proof.siblings.iter().map(|(_, side)| *side == Side::Right).collect();

    let recipient_addr = Fr::from(0xdeadbeef_u64);
    let relayer_addr = Fr::from(0xcafe_u64);
    let inputs = ProofInputs {
        secret_note: note,
        root,
        merkle_siblings: siblings,
        merkle_siblings_side: sides,
        recepient_address: recipient_addr,
        relayer_address: relayer_addr,
    };
    let (proof, pub_inputs) = generate_proof(inputs, &pk).unwrap();

    println!("proof = 0x{}", hex::encode(proof.to_solidity_call_data()));
    println!("root = 0x{}", hex::encode(fq_to_be_bytes(pub_inputs.merkle_root)));
    println!("nullifierHash = 0x{}", hex::encode(fq_to_be_bytes(pub_inputs.nullifier_hash)));
    println!("commitment = 0x{}", hex::encode(fq_to_be_bytes(commitment)));
    println!("recipient = 0x{:040x}", 0xdeadbeef_u64);
    println!("relayer = 0x{:040x}", 0xcafe_u64);
}
