// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`

use std::fs;
use cupcake::traits::*;
use methods::{METHOD_NAME_ELF, METHOD_NAME_ID};
use risc0_zkvm::{
    prove::default_hal,
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv, SessionReceipt,
};

fn main() {
    println!("host - init fv context");
    let fv = cupcake::default();

    println!("host - init fv keys");
    let (pk, sk) = fv.generate_keypair();

    println!("guest - encrypt data");
    let a = vec![19; fv.n];
    let b = vec![88; fv.n];

    let enc_a = fv.encrypt(&a, &pk);
    let enc_b = fv.encrypt(&b, &pk);

    let enc_au8 = enc_a.to_bytes();
    let enc_bu8 = enc_b.to_bytes();

    println!("host - init executor");
    let env = ExecutorEnv::builder()
        .segment_limit_po2(20)
        .add_input(&to_vec(&enc_au8).unwrap())
        .add_input(&to_vec(&enc_bu8).unwrap())
        .build();

    // Make the Executor.
    println!("host - make executor");
    let mut exec = Executor::from_elf(env, METHOD_NAME_ELF).unwrap();

    // Run the executor to produce a session.
    println!("host - run executor");
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let (hal, eval) = default_hal();
    println!("host - prove session and build receipt");
    let receipt = session.prove(hal.as_ref(), &eval).unwrap();

    println!("host - get result from host");
    let enc_sumu8: Vec<u8> = from_slice(&receipt.journal).expect("Failed to get result");

    println!("host - check results");
    let enc_sum = fv.from_bytes(&enc_sumu8);
    let sum: Vec<u8> = fv.decrypt(&enc_sum, &sk);
    println!("{}", sum[0]);

    // Save receipt to disk for further usages
    fs::write("./receipt.bin", bincode::serialize(&receipt).unwrap());

    // Prove hash and verify it
    println!("host - verify execution");
    receipt.verify(METHOD_NAME_ID).expect("Proven code should verify");
}

