// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {MerkleTree} from "./MerkleTree.sol";

interface IVerifier {
    function verifyProof(bytes memory _proof, uint256[6] memory _input) external returns (bool);
}


contract ZkMixer is MerkleTree{
    uint256 public constant DENOMINATION = 0.01 ether;

    mapping(bytes32 => bool) nullifier_hash;
    mapping(bytes32 => bool) commitments;

    IVerifier public immutable verifer;


    event Deposit(bytes32 indexed commitment, uint32 leafIndex, uint256 timestamp);


    function deposit(bytes32 commitment) external payable {
        require(!commitments[commitment],"commitment already submitted");
        require(msg.value== DENOMINATION, "msg.value different from the contract denomination");

        commitments[commitment]=true;

        uint32 leafIndex = _insert(commitment);

        emit Deposit(commitment, leafIndex, block.timestamp);
    }
}
