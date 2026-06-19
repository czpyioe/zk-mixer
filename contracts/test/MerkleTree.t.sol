// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {MerkleTree} from "../src/MerkleTree.sol";
import {Poseidon} from "../src/PoseidonT3.sol";


// needed for internal functions
contract MerkleTreeExternal is MerkleTree {
    function insert(bytes32 commitment) public returns (uint32) {
        return _insert(commitment);
    }

    function getZero(uint256 i) public pure returns (bytes32) {
        return zeros(i);
    }
}

contract MerkleTreeTest is Test {
    MerkleTreeExternal tree;

    function setUp() public {
        tree = new MerkleTreeExternal();
    }

    function test_insert_returns_correct_leaf_index() public {
        uint32 leafIndex = tree.insert(bytes32(uint256(42)));
        assertEq(leafIndex, 0);
        assertEq(tree.getNextIndex(), 1);
    }

    function test_two_inserts_increment_index() public {
        assertEq(tree.insert(bytes32(uint256(1))), 0);
        assertEq(tree.insert(bytes32(uint256(2))), 1);
        assertEq(tree.getNextIndex(), 2);
    }


    function test_root_stored_after_first_insert() public {
        bytes32 commitment = bytes32(uint256(42));
        tree.insert(commitment);

        bytes32 expected = commitment;
        for (uint8 i = 0; i < 21; i++) {
            expected = bytes32(Poseidon.hashLeftRight(uint256(expected), uint256(tree.getZero(i))));
        }

        assertTrue(tree.IsOrWasRoot(expected), "first root should be stored");
    }

    function test_root_stored_after_two_inserts() public {
        bytes32 c1 = bytes32(uint256(1));
        bytes32 c2 = bytes32(uint256(2));

        tree.insert(c1);
        tree.insert(c2);

        bytes32 expected = bytes32(Poseidon.hashLeftRight(uint256(c1), uint256(c2)));
        for (uint8 i = 1; i < 21; i++) {
            expected = bytes32(Poseidon.hashLeftRight(uint256(expected), uint256(tree.getZero(i))));
        }

        assertTrue(tree.IsOrWasRoot(expected), "second root should be stored");
    }

    function test_previous_root_still_valid_after_new_insert() public {
        bytes32 c1 = bytes32(uint256(1));
        bytes32 c2 = bytes32(uint256(2));

        tree.insert(c1);

        bytes32 root1 = c1;
        for (uint8 i = 0; i < 21; i++) {
            root1 = bytes32(Poseidon.hashLeftRight(uint256(root1), uint256(tree.getZero(i))));
        }

        tree.insert(c2);

        assertTrue(tree.IsOrWasRoot(root1), "old root should be saved");
    }
}
