use ark_bn254::Fr;
use ark_crypto_primitives::sponge::{
    poseidon::{find_poseidon_ark_and_mds, PoseidonConfig}
};

use ark_crypto_primitives::crh::{poseidon::CRH, CRHScheme};

pub fn poseidon_config() -> PoseidonConfig<Fr> {
    let (ark, mds) = find_poseidon_ark_and_mds::<Fr>(
        254,
        2,
        8,
        57,
        0,
    );
    PoseidonConfig {
        full_rounds: 8,
        partial_rounds: 57,
        alpha: 5,
        ark,
        mds,
        rate: 2,
        capacity: 1,
    }
}

pub fn hash_left_right(left: Fr, right: Fr) -> Fr {
    let cfg = poseidon_config();
    CRH::<Fr>::evaluate(&cfg, vec![left, right]).unwrap()
}


pub fn compute_nullifier_hash(nullifier: Fr) -> Fr {
    let cfg = poseidon_config();
    let crh_nullifier = CRH::<Fr>::evaluate(&cfg, vec![nullifier]).unwrap();
    crh_nullifier
}


pub fn compute_commitment(secret: Fr, nullifier: Fr) -> Fr {
    let cfg = poseidon_config();
    let crh_commitment = CRH::<Fr>::evaluate(&cfg, vec![secret,nullifier]).unwrap();
    crh_commitment
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_deterministic() {
        let s = Fr::from(42);
        let n = Fr::from(99);
        assert_eq!(compute_commitment(s, n), compute_commitment(s, n));
    }

    #[test]
    fn test_hash_left_right_deterministic() {
        let l = Fr::from(42);
        let r = Fr::from(99);
        assert_eq!(hash_left_right(l, r), hash_left_right(l, r));
    }

    #[test]
    fn test_nullifier_hash_deterministic() {
        let n = Fr::from(99);
        assert_eq!(compute_nullifier_hash(n), compute_nullifier_hash(n));
    }


    #[test]
    fn test_commitment_is_not_symmetric() {
        let s = Fr::from(1);
        let n = Fr::from(2);
        assert_ne!(compute_commitment(s, n), compute_commitment(n, s));
    }

    #[test]
    fn test_different_nullifiers_produce_different_hashes() {
        let n1 = Fr::from(1);
        let n2 = Fr::from(2);
        assert_ne!(compute_nullifier_hash(n1), compute_nullifier_hash(n2));
    }

    #[test]
    fn test_commitment_sensitive_to_both_inputs() {
        let n = Fr::from(99);
        let c1 = compute_commitment(Fr::from(1), n);
        let c2 = compute_commitment(Fr::from(2), n);
        assert_ne!(c1, c2);
    }
}
