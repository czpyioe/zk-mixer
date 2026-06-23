// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {MerkleTree} from "./MerkleTree.sol";
import {ReentrancyGuard} from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

interface IVerifier {
    function verifyProof(bytes memory _proof, uint256[4] memory _input) external view returns (bool);
}


contract ZkMixer is MerkleTree, ReentrancyGuard {
    uint256 public constant DENOMINATION = 0.01 ether;

    mapping(bytes32 => bool) nullifierHashes;
    mapping(bytes32 => bool) commitments;

    IVerifier public immutable verifier;

    event Deposit(bytes32 indexed commitment, uint32 leafIndex, uint256 timestamp);
    event Withdraw(address to, bytes32 nullifierHash);


    constructor(IVerifier _verifier) {
        verifier = _verifier;
    }

    function deposit(bytes32 commitment) external payable nonReentrant {
        require(!commitments[commitment], "commitment already submitted");
        require(msg.value == DENOMINATION, "msg.value different from the contract denomination");

        commitments[commitment] = true;

        uint32 leafIndex = _insert(commitment);

        emit Deposit(commitment, leafIndex, block.timestamp);
    }

    function withdraw(
        bytes calldata proof,
        bytes32 root,
        bytes32 nullifierHash,
        address recipientAddress
    ) external nonReentrant {
        require(recipientAddress != address(0), "invalid recipient");
        require(!nullifierHashes[nullifierHash], "nullifier hash already used");
        require(IsOrWasRoot(root), "unknown root");
        require(
            verifier.verifyProof(
                proof,
                [
                    uint256(root),
                    uint256(nullifierHash),
                    uint256(uint160(recipientAddress)),
                    uint256(uint160(msg.sender))
                ]
            ),
            "invalid proof"
        );

        nullifierHashes[nullifierHash] = true;

        (bool sent, ) = recipientAddress.call{value: DENOMINATION}("");
        require(sent, "payment failed");
        emit Withdraw(recipientAddress, nullifierHash);
    }
}
