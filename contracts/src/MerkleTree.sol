// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Poseidon} from "./PoseidonT3.sol";


contract MerkleTree {
    mapping(bytes32=>bool) roots;
    uint8 constant DEPTH = 21;

    uint32 nextIndex = 0;
    bytes32[DEPTH] currentPath;

    constructor() {
        for(uint8 i = 0; i < DEPTH; i++) {
            currentPath[i] = zeros(i);
        }
    }


    function _insert(bytes32 commitment) internal returns (uint32) {
        require(nextIndex != uint32(2)**DEPTH, "tree full");
        uint32 leafIndex = nextIndex;
        uint32 currentIndex = leafIndex;
        nextIndex ++;
        bytes32 currentHash = commitment;
        bytes32 left;
        bytes32 right;
        for (uint8 i = 0; i < DEPTH; i++) {
            // if current index is on the left
            if (currentIndex%2==0){
                left = currentHash;
                right = zeros(i);
                currentPath[i] = currentHash;
            }
            // if current index is on the right
            else {
                left = currentPath[i];
                right = currentHash;
            }
            currentHash = bytes32(Poseidon.hashLeftRight(uint256(left), uint256(right)));

            currentIndex /= 2;
        }

        roots[currentHash] = true;

        return leafIndex;
    }


    function zeros(uint256 i) internal pure returns (bytes32) {
        require(i < DEPTH);
        if (i == 0) return bytes32(0x0000000000000000000000000000000000000000000000000000000000000000);
        else if (i == 1) return bytes32(0x13a545a13f1d91dddb87f46679dfaec0900ce24791a924bee7fa4d69a9569d85);
        else if (i == 2) return bytes32(0x17a29aee69939bae962185bd5b5bc318b21252767d57638118a4a37d48cd9f5e);
        else if (i == 3) return bytes32(0x235456bddddeed41c0aecd3ec4c5030746e0641a3593c307fa51a52b1c49f24b);
        else if (i == 4) return bytes32(0x09625a0c3fff19b78077e137bb8980aa7a59ce36b133fa10de6628f301cc0ee2);
        else if (i == 5) return bytes32(0x19170aff688ab7b212862b253004984d8326cc8197577a531b9ad0cff6919f7e);
        else if (i == 6) return bytes32(0x1dfb3fa1d82c446ee3e252abf5871c73e4c6c74060541bcbb879f206784275d1);
        else if (i == 7) return bytes32(0x071398d6f643a5ebe52d509434c483042741aa2d40ef16cca4d7f2e1f6d95f4c);
        else if (i == 8) return bytes32(0x18b9968fda945c7dd3145d2e49b106125489e2ecc7e1d2defc99ad4ff4878852);
        else if (i == 9) return bytes32(0x11278523bfba90390b17701a32d0463e440df178a769ebee9e4027e361b7ebf8);
        else if (i == 10) return bytes32(0x1a2b8167da30b05e02f78b57846cd77df8a8c0e3e41477cc1baff8479b0b185e);
        else if (i == 11) return bytes32(0x1adcc7591a5be843dd3a8b9f7c708a31977121d7c9b10a565ac36e4d6ea5f09c);
        else if (i == 12) return bytes32(0x2b2a8e8d1d654a2accb00c5e2c9f1e30546f2b93a03b1b9f9d3c7048ef108f99);
        else if (i == 13) return bytes32(0x0150e0810128cd79a66b1ceffb0dfba81cede1b9c3abc80256fc0356e1f5d92b);
        else if (i == 14) return bytes32(0x2e161a5ff7b9dac73fdc1e5f40c1e29412f49f8a3c9d54eba9d0ed99ee789f45);
        else if (i == 15) return bytes32(0x23e15b836bc49772eb5b20a252a8f0b5862a09a6109381fb2352b977e920d8b8);
        else if (i == 16) return bytes32(0x2e59e2b3d5920b70bb0716c272d9947bccfb66148d36e9b758eec970658fadfa);
        else if (i == 17) return bytes32(0x1dc295ddb285aa0b61bd42438fbe98c271371c9e8e10f5e5d368bf0faa0a0e55);
        else if (i == 18) return bytes32(0x2fd372a756b96d3a01a6e4851f7146194bf133eb40003b623fb8772b7e15c9fb);
        else if (i == 19) return bytes32(0x1c118d46c460daf599f7e9baa998784a4bbaf68c660cefc485f22d6991fd1fa8);
        else if (i == 20) return bytes32(0x0ea614eb8a8052d533b9a351d661ba06f77bcce13b8fedbfc7ef04a8fa2b3f6f);
        else revert();
    }

    function getNextIndex() public view returns (uint256) {
        return nextIndex;
    }

    function wasRoot(bytes32 root) public view returns (bool){
        return roots[root];
    }
}