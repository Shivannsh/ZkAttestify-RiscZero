// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.
import {console} from "forge-std/console.sol";
/// @title A starter application using RISC Zero.
/// @notice This basic application holds a number, guaranteed to be even.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
///      or difficult to implement function to a RISC Zero guest running on Bonsai.

contract VerifyAttestation {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public  verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    ///         The image ID is similar to the address of a smart contract.
    ///         It uniquely represents the logic of that guest program,
    ///         ensuring that only proofs generated from a pre-defined guest program
    ///         (in this case, checking if a number is even) are considered valid.
    bytes32 public  imageId; // Update to immutable

    /// @notice A number that is guaranteed, by the RISC Zero zkVM, to be even.
    ///         It can be set by calling the `set` function.


    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }


    function setVerifier(address _verifier,bytes32 _imageId) public  {
        verifier = IRiscZeroVerifier(_verifier);
        imageId = _imageId;
    }


    /// @notice Verify the attestation.
    function verifyAttestation(address signers_address,uint64 threshold_age,uint64 current_timestamp,uint64 attest_time,address receipent,bytes32 domain_seperator ,bytes calldata seal) view public returns (bytes memory) {

        console.log("signers_address: %s", signers_address);
        console.log("Lorem ipsum dolor sit amet, consectetur adipiscing elit dhnfdigfubghs hdskjfhdsgksbfhg hsdfkhgsdhgfhjsdfb sdkhfghkfsdghsfg hdskjgskhgfhshkfg ksdhfsdkgsfkhg sdjkhfjsdfgksf hjkdhjhsfgksfbh hsdjfkhsfkgsfkhgbkhsf khsdkfjhsdfgjgfshksfhksdfhk kbsdkhsfhkjgfjk jkhfdkjsfghjksfghhksjf");
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = abi.encode(signers_address,threshold_age,current_timestamp,attest_time,receipent,domain_seperator); // Updated parameter types

        verifier.verify(seal, imageId, sha256(journal));

        return journal;
    }   

}
