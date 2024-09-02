// src/helpers.rs
// Ensure the correct path to the structs module

use ethers_core::abi::Token;
use ethers_core::types::transaction::eip712::EIP712Domain;
use ethers_core::types::{H256, U256};
use ethers_core::utils::keccak256;
use ethers_core::types::{Address};


pub struct Attest {
    pub version: u16,
    pub schema: H256,
    pub recipient: Address,
    pub time: u64,
    pub expiration_time: u64,
    pub revocable: bool,
    pub ref_uid: H256,
    pub data: Vec<u8>,
    pub salt: H256,
}


pub fn domain_separator(domain: &EIP712Domain, type_hash: H256) -> H256 {
    let encoded = ethers_core::abi::encode(&[
        Token::FixedBytes(type_hash.as_bytes().to_vec()),
        Token::FixedBytes(keccak256(domain.name.as_ref().unwrap().as_bytes()).to_vec()),
        Token::FixedBytes(keccak256(domain.version.as_ref().unwrap().as_bytes()).to_vec()),
        Token::Uint(domain.chain_id.unwrap()),
        Token::Address(domain.verifying_contract.unwrap()),
    ]);
    keccak256(&encoded).into()
}

pub fn hash_message(domain_separator: &H256, message: &Attest) -> H256 {
    let message_typehash: H256 = keccak256(
        b"Attest(uint16 version,bytes32 schema,address recipient,uint64 time,uint64 expirationTime,bool revocable,bytes32 refUID,bytes data,bytes32 salt)").into();

    let encoded_message = ethers_core::abi::encode(&[
        Token::FixedBytes(message_typehash.as_bytes().to_vec()),
        Token::Uint(U256::from(message.version)),
        Token::FixedBytes(message.schema.as_bytes().to_vec()),
        Token::Address(message.recipient),
        Token::Uint(U256::from(message.time)),
        Token::Uint(U256::from(message.expiration_time)),
        Token::Bool(message.revocable),
        Token::FixedBytes(message.ref_uid.as_bytes().to_vec()),
        Token::FixedBytes(keccak256(&message.data).to_vec()),
        Token::FixedBytes(message.salt.as_bytes().to_vec()),
    ]);

    let hashed_message = keccak256(&encoded_message);

    let mut combined = Vec::new();
    combined.extend_from_slice(&[0x19, 0x01]);
    combined.extend_from_slice(domain_separator.as_bytes());
    combined.extend_from_slice(&hashed_message);

    keccak256(&combined).into()
}

fn main() {
    // This is a placeholder for the main function
}