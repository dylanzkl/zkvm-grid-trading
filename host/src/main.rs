use std::env;
use std::time::Instant;
use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    GRID_TRADING_ELF, GRID_TRADING_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use grid_core::{Account, calculate_hash, GridRequest};
use rand::random;
use rand_core::OsRng;

const DEFAULT_TRANSACTION_AMOUNT: i32 = 1000;

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    env_logger::init();

    let mut transaction_amount = DEFAULT_TRANSACTION_AMOUNT;
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        transaction_amount = args[1].parse().unwrap();
    }

    let pre_state = Account {
        usd: 10,
        eth: 0,
    };
    let pre_hash = calculate_hash(&pre_state);

    let mut post_state = pre_state;
    let ts = forge_transactions(&mut post_state, transaction_amount);
    let post_hash = calculate_hash(&post_state);

    let grid_request = GridRequest {
        acc: pre_state,
        ts,
    };
    let signing_key = SigningKey::random(&mut OsRng);
    let message = b"This is a message that will be signed and verified within the zkvm";
    let signature: Signature = signing_key.sign(message);

    let receipt = prove_transactions_with_ecdsa_verification(grid_request, signing_key.verifying_key(), message, &signature);

    // For example:
    let (committed_pre_hash, committed_post_hash): (u64, u64) = receipt.journal.decode().unwrap();
    assert_eq!(pre_hash, committed_pre_hash);
    assert_eq!(post_hash, committed_post_hash);

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(GRID_TRADING_ID).unwrap();
}

fn prove_transactions_with_ecdsa_verification(
    grid_request: GridRequest,
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> Receipt {
    let signature_input = (verifying_key.to_encoded_point(true), message, signature);


    let env = ExecutorEnv::builder().write(&grid_request).unwrap()
        .write(&signature_input).unwrap().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let start = Instant::now();
    let receipt = prover.prove_elf(env, GRID_TRADING_ELF).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed in prove is: {:?}", duration);
    receipt
}

fn forge_transactions(acc: &mut Account, amount: i32) -> Vec<[i32; 2]> {
    let mut ts: Vec<[i32; 2]> = Vec::new();
    let mut idx = 0;
    while idx < amount {
        let price: u32 = random::<u32>() % 2 + 1;
        // if eth price is 1, buy 1 eth
        if price == 1 && acc.usd >= 1 {
            acc.usd -= 1;
            acc.eth += 1;
            ts.push([-1, 1]);
            idx += 1;
        } else if price == 2 && acc.eth >= 1 {
            acc.usd += 2;
            acc.eth -= 1;
            ts.push([2, -1]);
            idx += 1;
        }
    }
    return ts
}


