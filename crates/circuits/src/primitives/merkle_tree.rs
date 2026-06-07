use ark_bn254::Fr;
use ark_ff::Zero;
use crate::primitives::hash;

pub struct MerkleTree{
    depth: usize, // 20 in tornado cash
    nodes : Vec<Vec<Fr>>,
    next_index: usize
}

enum Side{
    Left,
    Right
}

struct MerkleProof {
    pub siblings: Vec<(Fr,Side)>, // (hash, side)
}


impl MerkleTree{
    pub fn new(depth: usize) -> Self {
        let mut zero_values = Vec::new();
        zero_values.push(hash::hash_left_right(Fr::zero(), Fr::zero()));
        for i in 1..=depth {
            let z = zero_values[i - 1];
            zero_values.push(hash::hash_left_right(z, z));
        }


        let nodes = (0..=depth)
            .map(|level| vec![zero_values[level]; 1 << (depth - level)])
            .collect();

        MerkleTree { depth, nodes, next_index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_merkle_tree() {
        let tree = MerkleTree::new(4);
        let expected_leaf = hash::hash_left_right(Fr::zero(), Fr::zero());
        assert_eq!(tree.nodes[0].len(), 1 << 4);
        assert_eq!(tree.nodes[4].len(), 1);
        assert_eq!(tree.nodes[0][0], expected_leaf);
        assert_eq!(tree.next_index, 0);
    }
}