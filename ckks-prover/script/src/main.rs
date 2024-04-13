//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{utils, ProverClient, PublicValues, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {

    utils::setup_logger();
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 8;
    stdin.write(&n);
    let client = ProverClient::new();

    let mut proof = client.prove(ELF, stdin).expect("proving failed");

    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!")
}
