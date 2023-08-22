// SPDX-License-Identifier: MIT
pragma solidity ^0.8.18;

import {Test} from "forge-std/Test.sol";
import {SignedContextV1} from "rain.interpreter/interface/IInterpreterCallerV2.sol";
import {ECDSA} from "openzeppelin-contracts/contracts/utils/cryptography/ECDSA.sol";

contract SignContext is Test {
    function signContext(uint256 privateKey, uint256[] memory context) public pure returns (SignedContextV1 memory) {
        SignedContextV1 memory signedContext;

        // Store the signer's address in the struct
        signedContext.signer = vm.addr(privateKey);
        signedContext.context = context; // copy the context data into the struct

        // Create a digest of the context data
        bytes32 contextHash = keccak256(abi.encodePacked(context));
        bytes32 digest = ECDSA.toEthSignedMessageHash(contextHash);

        // Create the signature using the cheatcode 'sign'
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, digest);
        signedContext.signature = abi.encodePacked(r, s, v);

        return signedContext;
    }
}
