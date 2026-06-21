// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Verifier} from "../src/Verifier.sol";
import {ZkMixer, IVerifier} from "../src/ZkMixer.sol";

contract DeploySepolia is Script {
    function run() external {
        vm.startBroadcast();

        Verifier verifier = new Verifier();
        ZkMixer mixer = new ZkMixer(IVerifier(address(verifier)));

        vm.stopBroadcast();

        console.log("Verifier :", address(verifier));
        console.log("ZkMixer :", address(mixer));
    }
}
