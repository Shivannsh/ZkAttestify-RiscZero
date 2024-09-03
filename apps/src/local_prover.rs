use alloy_primitives::FixedBytes;
use anyhow::Result;
use risc0_groth16::docker::stark_to_snark;
use risc0_zkvm::{
    get_prover_server, recursion::identity_p254, sha::Digestible, ExecutorEnv, ExecutorImpl,
    ProverOpts, VerifierContext,
};

use apps::bin::structs::Inputs;

/// An implementation of a Prover that runs on local machine.
pub struct LocalProver {}

impl LocalProver {
    pub fn prove(elf: &[u8] , input: &Inputs) -> Result<(Vec<u8>, Vec<u8>)> {
        log::info!("Start local proving");
        let env = ExecutorEnv::builder()
            .write_slice(input)
            // .unwrap()
            .build()
            .unwrap();

        log::info!("Create execution session");
        let mut exec = ExecutorImpl::from_elf(env, elf).unwrap();
        let session = exec.run().unwrap();

        log::info!("Generate STARK proof");
        let opts = ProverOpts::default();
        let ctx = VerifierContext::default();
        let prover = get_prover_server(&opts).unwrap();
        let receipt = prover.prove_session(&ctx, &session).unwrap();

        let claim = receipt.get_claim().unwrap();
        let composite_receipt = receipt.inner.composite().unwrap();
        let succinct_receipt = prover.compress(composite_receipt).unwrap();
        let journal: Vec<u8> = session.journal.unwrap().bytes;

        let ident_receipt = identity_p254(&succinct_receipt).unwrap();
        let seal_bytes = ident_receipt.get_seal_bytes();

        log::info!("Start translate STARK to SNARK");
        let seal = stark_to_snark(&seal_bytes).unwrap().to_vec();
        log::info!(
            "Transform finish, proof size decrease from {:} bytes to {:} bytes, snark proof {:?}",
            seal_bytes.len(),
            seal.len(),
            hex::encode(&seal)
        );

        Ok((journal,seal))
    }
}
