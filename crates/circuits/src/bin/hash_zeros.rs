use circuits::primitives::hash;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField, Zero};

fn main() {
    let mut previous = Fr::zero();
    for i in 0..=20 {
        let current = hash::hash_left_right(previous, previous);
        println!("i=={}: 0x{}", i, to_hex(current));
        previous = current;
    }
}

fn to_hex(fr: Fr) -> String {
    fr.into_bigint().to_bytes_be().iter().map(|b| format!("{:02x}", b)).collect()
}
