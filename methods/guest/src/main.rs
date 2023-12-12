#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
//#![no_std]  // std support is experimental


use risc0_zkvm::guest::env;
use grid_core::{Account, calculate_hash, GridRequest};
use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // read the input
    let request: GridRequest = env::read();
    let (encoded_verifying_key, message, signature): (EncodedPoint, Vec<u8>, Signature) = env::read();
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    // Verify the signature, panicking if verification fails.
    verifying_key
        .verify(&message, &signature)
        .expect("ECDSA signature verification failed");

    let pre_state = request.acc;
    let pre_hash = calculate_hash(&pre_state);

    let mut post_state = pre_state;
    for t in request.ts {
        if !is_valid(post_state, t) {
            panic!("transaction invalid");
        }
        post_state.usd += t[0];
        post_state.eth += t[1];
    }
    let post_hash = calculate_hash(&post_state);

    // write public output to the journal
    env::commit(&(pre_hash, post_hash));
}

fn is_valid(acc: Account, t: [i32; 2]) -> bool {
    // use 1 usd buy 1 eth
    if (t[0] == -1 && t[1] == 1 && acc.usd + t[0] >= 0)
        // sell 1 eth for 2 usd
        || (t[0] == 2 && t[1] == -1 && acc.eth + t[1] >= 0) {
        return true
    }
    return false
}