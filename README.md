# zkFHE

Verifiable and confidential computation based on ZKP and FHE, powered by [risc0 zkVM](https://www.risczero.com/).

A PoC to demonstrate an approach for private computation on a public environment w/o give up data availability (since data can stay encrypted).

FHE provides confidentiality, ZKP provides the proof that the algorithm run following the rules.

## How it works

It's based on a tweaked version of [Cupcake](https://github.com/emilianobonassi/Cupcake), a library for the (additive version of) Fan-Vercauteren homomorphic encryption scheme. Tweaked so it can run in the risc0 zkVM (i.e. single thread, custom getrandom).

It runs the following protocol

1. (Host) Create public and secret key
1. (Host) Encrypt two numbers with the secret key
1. (Host) Pass the public key and the two encrypted numbers to the guest vm
1. (Guest) Fetch the encrypted data and init the FHE framework
1. (Guest) Perform the (encrypted) sum on the encrypted data using the public key
1. (Guest) Send back the encrypted sum to the host
1. (Host) Decrypt the sum with secret key

TL;DR host has the guarantee the guest run the algorithm (ZKP) and that the guest did not access to the unecrypted data (FHE).
## How to run

First, [install Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html) if you don't already have it, then

```
cargo run --release
```

If you see 107 (19+88) at the end of the run, it worked.

PS: proof generation takes a lot (2hrs)