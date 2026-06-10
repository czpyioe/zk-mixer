use ark_r1cs_std::fields::fp::FpVar;
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};
use ark_bn254::Fr;
use ark_crypto_primitives::crh::poseidon::constraints::{CRHGadget, CRHParametersVar};
use ark_crypto_primitives::crh::CRHSchemeGadget;
use ark_r1cs_std::alloc::AllocVar;
use crate::primitives::hash;

pub fn poseidon_gadget(
    cs: ConstraintSystemRef<Fr>,
    inputs: &[FpVar<Fr>],
) -> Result<FpVar<Fr>, SynthesisError> {
    let params = hash::poseidon_config();
    let params_var = CRHParametersVar::<Fr>::new_witness(cs, || Ok(params)).unwrap();
    CRHGadget::<Fr>::evaluate(&params_var, inputs)
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_r1cs_std::R1CSVar;

    #[test]
    fn test_native_vs_gadget_same_output() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let s: Fr = Fr::from(42);
        let n: Fr = Fr::from(99);
        let s_var = FpVar::new_witness(cs.clone(), || Ok(s)).unwrap();
        let n_var = FpVar::new_witness(cs.clone(), || Ok(n)).unwrap();

        let native_result = hash::compute_commitment(s, n);
        let gadget_result = poseidon_gadget(cs, &[s_var, n_var]).unwrap();

        assert_eq!(native_result, gadget_result.value().unwrap());
    }
}
