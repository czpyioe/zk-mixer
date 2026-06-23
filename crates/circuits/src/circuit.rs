use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_bn254::Fr;
use ark_r1cs_std::{alloc::AllocVar, fields::fp::FpVar, boolean::Boolean, eq::EqGadget};
use ark_ff::Zero;

use crate::gadgets::poseidon;
use crate::gadgets::merkle;



pub const TREE_DEPTH: usize = 20;


#[derive(Clone,Debug)]
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
    pub relayer_address: Option<Fr>,
}


impl Default for ZkMixer {
    fn default() -> Self {
        ZkMixer { 
            secret: Some(Fr::zero()), 
            nullifier: Some(Fr::zero()), 
            merkle_siblings: Some(vec![Fr::zero(); TREE_DEPTH]), 
            merkle_siblings_sides: Some(vec![false; TREE_DEPTH]), 
            merkle_root: Some(Fr::zero()),
            nullifier_hash: Some(Fr::zero()),
            recipient_address: Some(Fr::zero()),
            relayer_address: Some(Fr::zero())
        }
    }
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
        let _relayer_address = FpVar::new_input(cs.clone(), || self.relayer_address.ok_or(SynthesisError::AssignmentMissing))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::ConstraintSystem;
    use crate::primitives::hash::{compute_commitment, compute_nullifier_hash};
    use crate::primitives::merkle_tree::{MerkleTree, Side};

    fn build_valid_circuit(secret: Fr, nullifier: Fr) -> ZkMixer {
        let commitment = compute_commitment(secret, nullifier);
        let nullifier_hash = compute_nullifier_hash(nullifier);

        let mut tree = MerkleTree::new(4);
        let idx = tree.insert(commitment);
        let proof = tree.generate_proof(idx);

        ZkMixer {
            secret: Some(secret),
            nullifier: Some(nullifier),
            merkle_siblings: Some(proof.siblings.iter().map(|(sib, _)| *sib).collect()),
            merkle_siblings_sides: Some(proof.siblings.iter().map(|(_, side)| *side == Side::Right).collect()),
            merkle_root: Some(tree.root()),
            nullifier_hash: Some(nullifier_hash),
            recipient_address: Some(Fr::from(0xabc123u64)),
            relayer_address: Some(Fr::from(0xdef456u64)),
        }
    }

    #[test]
    fn test_circuit_satisfiable() {
        let circuit = build_valid_circuit(Fr::from(1), Fr::from(2));
        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_wrong_nullifier_hash() {
        let mut circuit = build_valid_circuit(Fr::from(1), Fr::from(2));
        // wrong nullifier hash
        circuit.nullifier_hash = Some(Fr::from(0));

        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_wrong_secret() {
        let mut circuit = build_valid_circuit(Fr::from(1), Fr::from(2));
        // wrong secret: leaf computed in circuit won't match commitment in tree
        circuit.secret = Some(Fr::from(99));

        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_wrong_nullifier() {
        let mut circuit = build_valid_circuit(Fr::from(1), Fr::from(2));
        // wrong nullifier: breaks both constraint 1 (nullifier_hash) and the leaf
        circuit.nullifier = Some(Fr::from(99));

        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_wrong_merkle_root() {
        let mut circuit = build_valid_circuit(Fr::from(1), Fr::from(2));
        circuit.merkle_root = Some(Fr::from(0));

        let cs = ConstraintSystem::<Fr>::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }
}
