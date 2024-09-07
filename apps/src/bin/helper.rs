use alloy_sol_types::abi::Token;
// src/helpers.rs
// use ethers_core::abi::Token;
// use ethers_core::types::transaction::eip712::EIP712Domain;
// use ethers_core::types::{H256};
use alloy_primitives::{B256, Address, U256,keccak256};
use alloy_sol_types::Eip712Domain;


pub fn domain_separator(domain: &EIP712Domain, type_hash: B256) -> B256 {
    // let encoded = ethers_core::abi::encode(&[
    //     Token::FixedBytes(type_hash.as_bytes().to_vec()),
    //     Token::FixedBytes(keccak256(domain.name.as_ref().unwrap().as_bytes()).to_vec()),
    //     Token::FixedBytes(keccak256(domain.version.as_ref().unwrap().as_bytes()).to_vec()),
    //     Token::Uint(domain.chain_id.unwrap()),
    //     Token::Address(domain.verifying_contract.unwrap()),
    // ]);

    let encoded = alloy_core::dyn_abi::abi::encode(&[
        type_hash.as_bytes().to_vec(),
        keccak256(domain.name.as_ref().unwrap().as_bytes()).to_vec(),
        keccak256(domain.version.as_ref().unwrap().as_bytes()).to_vec(),
        domain.chain_id.unwrap().to_string().as_bytes().to_vec(),
        domain.verifying_contract.unwrap().to_string().as_bytes().to_vec(),

    ]);
    keccak256(&encoded).into()
} 
fn main(){}