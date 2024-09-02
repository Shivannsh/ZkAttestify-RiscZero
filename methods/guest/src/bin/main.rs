use ethers_core::types::Address;
use ethers_core::types::{RecoveryMessage, Signature, H160, H256};
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use ethers_core::abi::{decode, ParamType};
use ethers_core::utils::keccak256;

#[derive(Debug, Serialize, Deserialize)]
struct Attest {
    version: u16,
    schema: H256,
    recipient: Address,
    time: u64,
    expiration_time: u64,
    revocable: bool,
    ref_uid: H256,
    data: Vec<u8>,
    salt: H256,
}

#[derive(Debug, Serialize, Deserialize)]
struct DateOfBirth {
    unix_timestamp: u128,
}

fn hash_message(domain_separator: &H256, message: &Attest) -> H256 {
    let message_typehash: H256 = keccak256(
        b"Attest(uint16 version,bytes32 schema,address recipient,uint64 time,uint64 expirationTime,bool revocable,bytes32 refUID,bytes data,bytes32 salt)"
    ).into();

    let encoded_message = ethers_core::abi::encode(&[
        ethers_core::abi::Token::FixedBytes(message_typehash.as_bytes().to_vec()),
        ethers_core::abi::Token::Uint(ethers_core::types::U256::from(message.version)),
        ethers_core::abi::Token::FixedBytes(message.schema.as_bytes().to_vec()),
        ethers_core::abi::Token::Address(message.recipient),
        ethers_core::abi::Token::Uint(ethers_core::types::U256::from(message.time)),
        ethers_core::abi::Token::Uint(ethers_core::types::U256::from(message.expiration_time)),
        ethers_core::abi::Token::Bool(message.revocable),
        ethers_core::abi::Token::FixedBytes(message.ref_uid.as_bytes().to_vec()),
        ethers_core::abi::Token::FixedBytes(keccak256(&message.data).to_vec()),
        ethers_core::abi::Token::FixedBytes(message.salt.as_bytes().to_vec()),
    ]);

    let hashed_message = keccak256(&encoded_message);

    let mut combined = Vec::new();
    combined.extend_from_slice(&[0x19, 0x01]);
    combined.extend_from_slice(domain_separator.as_bytes());
    combined.extend_from_slice(&hashed_message);

    keccak256(&combined).into()
}

pub fn decode_date_of_birth(data:&Vec<u8>)-> u64{
    let param_types = vec![
        ParamType::Uint(256),
    ];

    // Decode the data
    let decoded: Vec<ethers_core::abi::Token> =
        decode(&param_types, data).expect("Failed to decode data");

    let dob = decoded[0].clone().into_uint().expect("Failed to parse dob");
    return dob.as_u64();
}

fn main() {
    let (signer_address, signature, threshold_age, current_timestamp, attest, domain_separator): (
        H160,
        Signature,
        u64,
        u64,
        Attest,
        H256,
    ) = env::read();

     // Verify that the data is related to the digest
     let calculated_digest = hash_message(&domain_separator, &attest);

    let recovery_message = RecoveryMessage::Hash(calculated_digest);
    let recovered_address = signature.recover(recovery_message).unwrap();

    // Age calculation in seconds
    let current_age = decode_date_of_birth(&attest.data) as u64;
    let age_in_seconds = current_timestamp - current_age;

    if signer_address != recovered_address {
        panic!("Invalid signature");
    } else {
        if age_in_seconds < threshold_age {
            panic!("Age is below threshold");
        } else {
            env::commit::<(H160, u64, u64, u64, H160, H256)>(&(
                signer_address,
                threshold_age,
                current_timestamp,
                attest.time,
                attest.recipient, // address of the recipient
                domain_separator,
            ));
        }
    } 
}
