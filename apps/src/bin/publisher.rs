mod helper;
mod structs;

use alloy_primitives::{Address, B256,Signature};
use alloy_sol_types::{sol, SolInterface, SolValue};
use anyhow::{Context, Result};
use clap::Parser;
use ethers::prelude::*;
// use ethers_core::types::Signature;
// use ethers_core::types::{Address, B256};
use helper::domain_separator; // Alias for clarity
use methods::VERIFYATTESTATION_ELF;
use risc0_ethereum_contracts::groth16;
use risc0_zkvm::guest::env;
use std::fs;
use structs::{Attest, InputData}; // Alias for clarity

use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, Receipt, VerifierContext};

// `IAddress` interface automatically generated via the alloy `sol!` macro.
sol! {
    interface IAddress {
        function verifyAttestation(
            address signers_address,
            uint64 threshold_age,
            uint64 current_timestamp,
            uint64 attest_time,
            address receipent_address,
            bytes32 domain_seperator, 
            bytes calldata seal);
    }
}

/// Wrapper of a `SignerMiddleware` client to send transactions to the given
/// contract's `Address`.
pub struct TxSender {
    chain_id: u64,
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    contract: Address,
}

impl TxSender {
    /// Creates a new `TxSender`.
    pub fn new(chain_id: u64, rpc_url: &str, private_key: &str, contract: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        let contract = contract.parse::<Address>()?;

        Ok(TxSender {
            chain_id,
            client,
            contract,
        })
    }

    /// Send a transaction with the given calldata.
    pub async fn send(&self, calldata: Vec<u8>) -> Result<Option<TransactionReceipt>> {
        let tx = TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(self.contract)
            .from(self.client.address())
            .data(calldata);

        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;

        log::info!("Transaction receipt: {:?}", &tx);

        Ok(tx)
    }
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: String,

    /// Ethereum Node endpoint.
    #[clap(long)]
    rpc_url: String,

    /// Application's contract address on Ethereum
    #[clap(long)]
    contract: String,
}

fn main() -> Result<()> {
    env_logger::init();
    // Parse CLI Arguments: The application starts by parsing command-line arguments provided by the user.
    let args = Args::parse();

    // Create a new transaction sender using the parsed arguments.
    let tx_sender = TxSender::new(
        args.chain_id,
        &args.rpc_url,
        &args.eth_wallet_private_key,
        &args.contract,
    )?;

    // Read and parse the JSON file
    let json_str = fs::read_to_string(
        "/Users/shivanshgupta/Desktop/ZkAttestify-onChain/ZkAttestify-RiscZero/apps/src/bin/input.json",
    )?;
    let input_data: InputData = serde_json::from_str(&json_str)?;

    // Extract data from the parsed JSON
    let domain = alloy_sol_types::Eip712Domain {
        name: Some(input_data.sig.domain.name),
        version: Some(input_data.sig.domain.version),
        chain_id: Some(ethers_core::types::U256::from_dec_str(
            &input_data.sig.domain.chain_id,
        )?),
        verifying_contract: Some(input_data.sig.domain.verifying_contract.parse()?),
        salt: None,
    };

    let signer_address: Address = input_data.signer.parse()?;
    let message = Attest {
        // Use the helper's Attest
        version: input_data.sig.message.version,
        schema: input_data.sig.message.schema.parse()?,
        recipient: input_data.sig.message.recipient.parse()?,
        time: input_data.sig.message.time.parse()?,
        expiration_time: input_data.sig.message.expiration_time.parse()?,
        revocable: input_data.sig.message.revocable,
        ref_uid: input_data.sig.message.ref_uid.parse()?,
        data: alloy_primitives::hex::decode(&input_data.sig.message.data[2..])?,
        salt: input_data.sig.message.salt.parse()?,
    };

    // Calculate the current timestamp and the threshold age
    // let current_timestamp = chrono::Utc::now().timestamp() as u64;
    let current_timestamp = chrono::Utc::now().timestamp() as u64;
    let threshold_age: u64 = 18 * 365 * 24 * 60 * 60; // 18 years in seconds

    // Calculate the domain separator and the message hash
    let domain_separator = domain_separator(
        &domain,
        alloy_primitives::keccak256(
            b"EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
        )
        .into(),
    );

    // Parse the signature
    let signature = Signature::new(
        input_data.sig.signature.r.parse()?,
        input_data.sig.signature.s.parse()?,
        input_data.sig.signature.v,
    );

    let input: (&Address, &Signature, &u64, &u64, &Attest, B256) = (
        &signer_address,
        &signature,
        &threshold_age,
        &current_timestamp,
        &message,
        domain_separator,
    );
    let env: ExecutorEnv<'_> = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            VERIFYATTESTATION_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;

    let seal: Vec<u8> = groth16::encode(receipt.inner.groth16()?.seal.clone())?;

    // Extract the journal from the receipt.
    let journal = receipt.journal.bytes.clone();

    let signer_address_bytes: [u8; 20] = signer_address.into();
    let recipient_address_bytes: [u8; 20] = message.recipient.into(); 
    let domain_separator_bytes: [u8; 32] = domain_separator.into();
   
    // let attest_time = input_data.sig.message.time.parse::<u64>()?;

    let calldata = IAddress::IAddressCalls::verifyAttestation(IAddress::verifyAttestationCall {
        signers_address: signer_address_bytes.into(),
        threshold_age,
        current_timestamp,
        attest_time: message.time,
        receipent_address: recipient_address_bytes.into(),
        domain_seperator: domain_separator_bytes.into(),
        seal: seal.into(),
    })
    .abi_encode();

    // Initialize the async runtime environment to handle the transaction sending.
    let runtime = tokio::runtime::Runtime::new()?;

    // Send transaction: Finally, the TxSender component sends the transaction to the Ethereum blockchain,
    // effectively calling the set function of the EvenNumber contract with the verified number and proof.
    runtime.block_on(tx_sender.send(calldata))?;

    Ok(())
}
