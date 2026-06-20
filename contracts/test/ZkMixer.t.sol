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

    // proof and args generated with prover/src/bin/test_proof.rs
    // to generate the proof, the pk should match with the right verifier contract
    bytes32 constant COMMITMENT = bytes32(0x1d404212570adf68e1534aeb1f1f534b8861e6ef11ef5cd47e1c69f156ea2266);
    bytes32 constant NULLIFIER_HASH = bytes32(0x2f4da06090662cc4ef91c891c465c2aea1bb9c4fac105a285b5ad85ecae33851);
    bytes32 constant ROOT = bytes32(0x01f6a4ba0600cf838e92dfbdb296175331e50ba2f647aecc8a87cb6aac6caf14);
    bytes constant PROOF = hex"00b9a555c8ef7448fe01aec5f5eb186e2edbb246e0e2f5c0d8efedcf3bdaf84c1b93ebcef4f2db53b77fe74081bfd59d05158b4a8995dd8c399eace1824bf9f92358b55e825c31a9d665f029b1870cb42adccf437a4c446c278aeb7003bec89e216920a2794235e4c39ab6ceb06b8d3d3f947afce57d8fcc7d031d2285c7774b0097af7bc50cce1c965ff2e31d3d538cdb6c88eefa0916a178c6a44fe5ceb634008415ad35b6ea6778ff9a94f6e1235b968bb6527499076d66bee4174543f2e5301ef391df8e744f7b95eff03ec2d96e75aa2560d0bf4c9d9f150d414fc1c843010412822ee05a771c6a0fd1554ca13b87cca8c0e35d591f67a42b3ca587e951";

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
        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, recipient);
        assertEq(recipient.balance - balanceBefore, 0.01 ether);
    }

    function test_double_withdraw_reverts() public {
        vm.prank(depositor);
        mixer.deposit{value: 0.01 ether}(COMMITMENT);

        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, recipient);

        vm.expectRevert("nullifier hash already used");
        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, recipient);    
    }


    function test_invalid_proof() public {
        vm.prank(depositor);
        mixer.deposit{value: 0.01 ether}(COMMITMENT);

        // wrong nullifier hash
        vm.expectRevert("invalid proof");
        mixer.withdraw(PROOF, ROOT, bytes32(0x2f4da06090662cc4ef91c891c465c2aea1bb9c4fac105a285b5ad85ecae33852), recipient);


        // wrong recipient address
        vm.expectRevert("invalid proof");
        mixer.withdraw(PROOF, ROOT, NULLIFIER_HASH, address(0xD15749a7Ae26cbAb8F56831e029471D4cC756082));
    }
}
