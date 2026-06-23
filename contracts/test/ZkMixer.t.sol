// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {ZkMixer, IVerifier} from "../src/ZkMixer.sol";
import {Verifier} from "../src/Verifier.sol";


contract ZkMixerTest is Test {
    ZkMixer mixer;
    Verifier verifier;

    address depositor = makeAddr("depositor");
    address recipient = address(uint160(0xdeadbeef));
    address relayer = address(uint160(0xcafe));

    // proof and args generated with prover/src/bin/test_proof.rs
    // to generate the proof, the pk should match with the right verifier contract
    bytes32 constant COMMITMENT = bytes32(0x2ce75eae2352c153497f9989e3a6014f9d0e39e1ecc5b6b9ed460c8d9a84cd25);
    bytes32 constant NULLIFIER_HASH = bytes32(0x0456891b8e256eff9b350bfdd44a21566acfa2639e86c51344d498c408f057fd);
    bytes32 constant ROOT = bytes32(0x05f44b26f90c7ab4fb51ca59defc723139484ddd14d361ca8a41d208347936db);
    bytes constant PROOF = hex"1ec566a8cd15c89f0a3c6b30a3f9f3fd72ee62ad9a67404bd5201a4110571e262ccce5f0aa268afca619da818ce5ff0480b47eed11bb254ef9a077a3ecb1fe350e71109b3dd2c7baccdce1c65813bf154e8403b40f41c17ae40a9084b32237ee27d39d8892138968cfc7fd0af7b580ecc201577dbc2d6b01ec4416d9dc7aef370d1c64dd7888b0fc29d8ebf4f79df1a691efaa5199f7347ebca5d02bc4d557ab0cbdd8c9d76d98c4b787b741e8d25c3c07e580b88984b681e882b1daab8fc4e2052a2755c287bf288ccd6347219bc0406d2f6d9e6742469b177239836c502b5b16b1f5e9be3ada9745e5e645efd6efe71d224fd6511c459e3ee19353c9290f88";

    function setUp() public {
        verifier = new Verifier();
        mixer = new ZkMixer(IVerifier(address(verifier)));
        vm.deal(depositor, 1 ether);
    }

    function test_deposit_and_withdraw() public {
        vm.prank(depositor);
        mixer.deposit{value: 0.01 ether}(COMMITMENT);

        assertEq(address(mixer).balance, 0.01 ether);

        uint256 balanceBefore = recipient.balance;
        vm.prank(relayer);
        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, recipient);
        assertEq(recipient.balance - balanceBefore, 0.01 ether);
    }

    function test_double_withdraw_reverts() public {
        vm.prank(depositor);
        mixer.deposit{value: 0.01 ether}(COMMITMENT);

        vm.prank(relayer);
        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, recipient);

        vm.expectRevert("nullifier hash already used");
        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, recipient);
    }


    function test_invalid_proof() public {
        vm.prank(depositor);
        mixer.deposit{value: 0.01 ether}(COMMITMENT);

        // wrong nullifier hash
        vm.prank(relayer);
        vm.expectRevert("invalid proof");
        mixer.withdraw(PROOF, ROOT, bytes32(0x0456891b8e256eff9b350bfdd44a21566acfa2639e86c51344d498c408f057fe), recipient);


        // wrong recipient address
        vm.prank(relayer);
        vm.expectRevert("invalid proof");
        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, address(0xD15749a7Ae26cbAb8F56831e029471D4cC756082));
    }
}
