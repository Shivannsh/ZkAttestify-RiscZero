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
    let (signer_address, signature, digest, threshold_age, current_timestamp, data): (
        H160,
        Signature,
        H256,
        u64,
        u64,
        Vec<u8>,
    ) = env::read();

    let recovery_message = RecoveryMessage::Hash(digest);
    let recovered_address = signature.recover(recovery_message).unwrap();

    // Age calculation in seconds
    let current_age = decode_date_of_birth(&data) as u64;
    let age_in_seconds = current_timestamp - current_age;

    if signer_address != recovered_address {
        panic!("Invalid signature");
    } else {
        if age_in_seconds < threshold_age {
            panic!("Age is below threshold");
        } else {
            env::commit::<(H160,  u64 ,u64)>(&(
                signer_address,
                threshold_age,
                current_timestamp,
            ));
        }
    } 
}
