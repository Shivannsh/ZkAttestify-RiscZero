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

import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {VerifyAttestation} from "../contracts/verify_attestation.sol";
import {Elf} from "./Elf.sol"; // auto-generated contract after running `cargo build`.

// contract EvenNumberTest is RiscZeroCheats, Test {
//     EvenNumber public evenNumber;

//     function setUp() public {
//         IRiscZeroVerifier verifier = deployRiscZeroVerifier();
//         evenNumber = new EvenNumber(verifier);
//         assertEq(evenNumber.get(), 0);
//     }

//     function test_SetEven() public {
//         uint256 number = 12345678;
//         (bytes memory journal, bytes memory seal) = prove(Elf.IS_EVEN_PATH, abi.encode(number));

//         evenNumber.set(abi.decode(journal, (uint256)), seal);
//         assertEq(evenNumber.get(), number);
//     }

//     function test_SetZero() public {
//         uint256 number = 0;
//         (bytes memory journal, bytes memory seal) = prove(Elf.IS_EVEN_PATH, abi.encode(number));

//         evenNumber.set(abi.decode(journal, (uint256)), seal);
//         assertEq(evenNumber.get(), number);
//     }
// }

    struct Attest{
        uint16 version;
        bytes32 schema;
        address receipent;
        uint64 time;
        uint64  expiration_time;
        bool revocable;
        bytes32 ref_uid;
        bytes data;
        bytes32 salt;
    }

     struct Signature {
        bytes32 r;
        bytes32 s;
        uint8 v;
    }

contract VerifyAttestationTest is RiscZeroCheats, Test{

    VerifierAttestation public verifierAttestation;

    function  setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        verifierAttestation = new VerifyAttestation(verifier);
    }
    function test_VerifyAttestation( ) public 
    {
        address signer_address =    address(0x7242Ccc30D68fca5cd8aEfc0ffbb545A1804439F);

        uint64 threshold_age = 18 * 365 * 24 * 60 * 60;
        uint64 currentTimestamp = 1725111844;
        // Populate the Attest struct using the provided message
        Attest memory message = Attest({
            version: 2,
            schema: 0xe102b6f4e9491f87a8ca24a7bb9ccab0bdbc57cc2d58dacc38295c349f17542e,  // Convert schema string to bytes32
            recipient: address(0x0000000000000000000000000000000000000000),  // recipient as address
            time: 1724970125,  // Convert string to uint64
            expiration_time: 0,  // Convert string to uint64
            revocable: true,  // Revocability
            ref_uid: 0x0000000000000000000000000000000000000000000000000000000000000000,  // refUID as bytes32
            data: hex"000000000000000000000000000000000000000000000000000000003e4be428",  // data as bytes
            salt: 0x6d1f5bd7a78a1da090c4178ec4cb9d963d87231b1133398b9b84c7a0d239b2f7  // salt as bytes32
        });
        bytes32 domain_seperator = 0xb0d90c6a70c303bb1c0f0c525fce9473dd6de970950af010b0f48ecff37baf73;

        Signature memory signature = Signature({
            r: 0x7b4cfc17d9af9a8e56581298b34840e073d4075feafee920c533e03a9a6dae2f,
            s: 0x2842955b866c043a45b46829b3ab94bd780b05d2f24f2e5f571ebd260e8c7856,
            v: 28
        });

        ()
        
    }
    
}