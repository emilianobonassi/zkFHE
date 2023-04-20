// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

risc0_zkvm::guest::entry!(main);

use cupcake::traits::*;
use risc0_zkvm::guest::env;

pub fn main() {
    println!("guest - init context");
    let fv = cupcake::default();

    println!("guest - read encrypted data from host");
    let enc_au8: Vec<u8> = env::read();
    let enc_bu8: Vec<u8> = env::read();

    println!("guest - deserialize data");
    let mut enc_a = fv.from_bytes(&enc_au8);
    let enc_b = fv.from_bytes(&enc_bu8);

    println!("guest - perform computation on encrypted data");
    fv.add_inplace(&mut enc_a, &enc_b);

    println!("guest - sending result back to host");
    let enc_sumu8 = enc_a.to_bytes();
    env::commit(&enc_sumu8);
}
