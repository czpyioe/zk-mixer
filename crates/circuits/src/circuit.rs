use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_bn254::Fr;
use ark_r1cs_std::{alloc::AllocVar, fields::fp::FpVar, boolean::Boolean, eq::EqGadget};
use crate::gadgets::poseidon;
use crate::gadgets::merkle;

#[derive(Clone)]
pub struct ZkMixer {
    // private vars (witness)
    pub secret: Option<Fr>,
    pub nullifier: Option<Fr>,
    pub merkle_siblings: Option<Vec<Fr>>,
    pub merkle_siblings_sides: Option<Vec<bool>>,

    // public vars (inputs)
    pub merkle_root: Option<Fr>,
    pub nullifier_hash: Option<Fr>,
    pub recipient_address: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for ZkMixer {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // witnesses (private)
        let s = FpVar::new_witness(cs.clone(), || self.secret.ok_or(SynthesisError::AssignmentMissing))?;
        let n = FpVar::new_witness(cs.clone(), || self.nullifier.ok_or(SynthesisError::AssignmentMissing))?;

        let merkle_siblings: Vec<FpVar<Fr>> = self.merkle_siblings
            .ok_or(SynthesisError::AssignmentMissing)?
            .iter()
            .map(|sib| FpVar::new_witness(cs.clone(), || Ok(*sib)).unwrap())
            .collect();

        let merkle_siblings_sides: Vec<Boolean<Fr>> = self.merkle_siblings_sides
            .ok_or(SynthesisError::AssignmentMissing)?
            .iter()
            .map(|side| Boolean::new_witness(cs.clone(), || Ok(*side)).unwrap())
            .collect();

        // inputs (public)
        let merkle_root = FpVar::new_input(cs.clone(), || self.merkle_root.ok_or(SynthesisError::AssignmentMissing))?;
        let nullifier_hash = FpVar::new_input(cs.clone(), || self.nullifier_hash.ok_or(SynthesisError::AssignmentMissing))?;
        let _recipient_address = FpVar::new_input(cs.clone(), || self.recipient_address.ok_or(SynthesisError::AssignmentMissing))?;

        // constraint 1: nullifier_hash == Poseidon(nullifier)
        let computed_nullifier_hash = poseidon::poseidon_gadget(cs.clone(), &[n.clone()])?;
        computed_nullifier_hash.enforce_equal(&nullifier_hash)?;

        // constraint 2: commitment == Poseidon(secret, nullifier)
        let leaf = poseidon::poseidon_gadget(cs.clone(), &[s, n])?;

        // constraint 3: commitment is in the merkle tree
        merkle::verify_merkle_proof_gadget(cs, leaf, &merkle_siblings, &merkle_siblings_sides, merkle_root)?;

        Ok(())
    }
}
