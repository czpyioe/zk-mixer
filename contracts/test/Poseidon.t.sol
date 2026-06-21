// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Poseidon} from "../src/PoseidonT3.sol";


contract PoseidonTest is Test {
    function test_matches_arkworks_hashLeftRight() public pure {
        assertEq(
            Poseidon.hashLeftRight(1, 2),
            7142104613055408817911962100316808866448378443474503659992478482890339429929
        );
    }

    function test_matches_arkworks_hashSingle() public pure {
        assertEq(
            Poseidon.hashSingle(99),
            16274034975375178320209483099676314254138021896889886198911688894008665278747
        );
    }
}
