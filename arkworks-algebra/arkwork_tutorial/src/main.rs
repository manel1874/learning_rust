use ark_ff::Field;
// We'll use a field associated with the BLS12-381 pairing-friendly
// group for this example.
use ark_bls12_381::Fq2 as F;
// `ark-std` is a utility crate that enables `arkworks` libraries
// to easily support `std` and `no_std` workloads, and also re-exports
// useful crates that should be common across the entire ecosystem, such as `rand`.
use ark_std::{One, UniformRand};

fn main() {

    let mut rng = ark_std::rand::thread_rng();
    // Let's sample uniformly random field elements:
    let a = F::rand(&mut rng);
    let b = F::rand(&mut rng);

    // println!("{}", rng);
}
