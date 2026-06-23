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
    bytes32 constant COMMITMENT = bytes32(0x1056d89c1f84a2c4d119d8a6d7e4ac3d5b3b62dab4aac68e64e875e48ab50d8c);
    bytes32 constant NULLIFIER_HASH = bytes32(0x11b13934f870d5c975f3c44de022ae34a707f0efb2130830826c44fd33b4513b);
    bytes32 constant ROOT = bytes32(0x24a18a1789489a9b9da4fb59db9f13d96c2a0406e54ba33f837edccbd382a1b6);
    bytes constant PROOF = hex"216f4c7f1a6b49f9e82360ff0240629989f4655dc23360ee0fc76f70d8823c58055bc423a50eaf33f4b4e769e6ef66b995705ea0e3a30077238d384c3b98d6c31bdfd5640024f1c5cec240888834a4a4b134f0b71fe6305e45b15c50ea21a80e152f558025944f0e5ffcd532da58acb16545b6b2bec7c8f0b50dd284034641d7087ab70d9b20583bbf2925870e3fbcb7ff1f387b61de3f0f67c5fbc079c1162500d0d0a57871cdac6d55772afbf9dd01e694650404bce4ba134188b0083ee88f0feb278450f72cfa2e8d112350549c5037f91235e5b07ec866d561d5f9c3795a0c887f90a254a9128e4ebc5c47f286f13e72957c8c725c749bd66b48ba6a93fd";

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
