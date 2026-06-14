use rand::Rng;
use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ff::{PrimeField, BigInteger, UniformRand};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use circuits::primitives::hash;

pub struct SecretNote {
    pub secret: Fr,
    pub nullifier: Fr
}


pub struct PublicInputs {
    pub merkle_root: Fr,
    pub nullifier_hash: Fr,
    pub recipient_address: Fr,
}


impl PublicInputs {
    pub fn to_vec(&self) -> Vec<Fr> {
        vec![
            self.merkle_root,
            self.nullifier_hash,
            self.recipient_address,
        ]
    }
}


impl SecretNote {
    pub fn generate(rng: &mut impl Rng) -> Self {
        SecretNote {
            secret: Fr::rand(rng),
            nullifier: Fr::rand(rng),
        }
    }

    pub fn commitment(&self) -> Fr {
        hash::compute_commitment(self.secret, self.nullifier)
    }

    pub fn nullifier_hash(&self) -> Fr {
        hash::compute_nullifier_hash(self.nullifier)
    }

    /// Serialize the note as 64 bytes: secret (32) || nullifier (32).
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];

        let mut secret_bytes = Vec::new();
        self.secret.serialize_compressed(&mut secret_bytes).unwrap();
        bytes[0..32].copy_from_slice(&secret_bytes);

        let mut nullifier_bytes = Vec::new();
        self.nullifier.serialize_compressed(&mut nullifier_bytes).unwrap();
        bytes[32..64].copy_from_slice(&nullifier_bytes);

        bytes
    }

    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        let secret = Fr::deserialize_compressed(&bytes[0..32]).unwrap();
        let nullifier = Fr::deserialize_compressed(&bytes[32..64]).unwrap();

        SecretNote { secret, nullifier }
    }
}


pub struct ProofInputs {
    pub secret_note: SecretNote,
    pub root: Fr,
    pub merkle_siblings: Vec<Fr>,
    pub merkle_siblings_side: Vec<bool>,
    pub recepient_address: Fr
}


pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine
}


impl Proof {
    /// ABI-encode the proof as `uint256[8]` for the Solidity verifier:
    /// [A.x, A.y, B.x.c1, B.x.c0, B.y.c1, B.y.c0, C.x, C.y].
    ///
    /// From Verifier.sol
    /// Proof memory _proof;
    /// _proof.A = Pairing.G1Point(p[0], p[1]);
    /// _proof.B = Pairing.G2Point([p[2], p[3]], [p[4], p[5]]);
    /// _proof.C = Pairing.G1Point(p[6], p[7]);
    pub fn to_solidity_call_data(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(8 * 32);

        data.extend_from_slice(&fq_to_be_bytes(self.a.x));
        data.extend_from_slice(&fq_to_be_bytes(self.a.y));

        data.extend_from_slice(&fq_to_be_bytes(self.b.x.c1));
        data.extend_from_slice(&fq_to_be_bytes(self.b.x.c0));
        data.extend_from_slice(&fq_to_be_bytes(self.b.y.c1));
        data.extend_from_slice(&fq_to_be_bytes(self.b.y.c0));

        data.extend_from_slice(&fq_to_be_bytes(self.c.x));
        data.extend_from_slice(&fq_to_be_bytes(self.c.y));

        data
    }
}

/// Convert a field element into its 32-byte big-endian (uint256) representation.
pub fn fq_to_be_bytes<F: PrimeField>(value: F) -> [u8; 32] {
    let bytes = value.into_bigint().to_bytes_be();
    let mut out = [0u8; 32];
    out[32 - bytes.len()..].copy_from_slice(&bytes);
    out
}


pub fn hex_to_fr(s: &str) -> Fr {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).expect("invalid hex");
    // from_be_bytes_mod_order also accepts the 20-byte recipient address.
    Fr::from_be_bytes_mod_order(&bytes)
}
