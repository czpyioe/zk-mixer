use ark_bn254::Bn254;
use ark_groth16::ProvingKey;
use circuits::circuit::ZkMixer;
use ark_groth16::Groth16;
use rand::rngs::OsRng;
use ark_snark::SNARK;
use anyhow::Result;


use crate::types;


pub fn generate_proof(inputs: types::ProofInputs, pk: &ProvingKey<Bn254>) -> Result<(types::Proof, types::PublicInputs)> {
    let mut rng = OsRng;

    let witness = ZkMixer { 
        secret: Some(inputs.secret_note.secret), 
        nullifier: Some(inputs.secret_note.nullifier), 
        merkle_siblings: Some(inputs.merkle_siblings), 
        merkle_siblings_sides: Some(inputs.merkle_siblings_side), 
        merkle_root: Some(inputs.root), 
        nullifier_hash: Some(inputs.secret_note.nullifier_hash()), 
        recipient_address: Some(inputs.recepient_address)
    };

    let proof = Groth16::<Bn254>::prove(&pk, witness, &mut rng)?;

    let proof = types::Proof { 
        a: proof.a, 
        b: proof.b, 
        c: proof.c 
    };

    let public_inputs = types::PublicInputs{
        merkle_root: inputs.root,
        nullifier_hash: inputs.secret_note.nullifier_hash(),
        recipient_address: inputs.recepient_address
    };

    Ok((proof,public_inputs))
}