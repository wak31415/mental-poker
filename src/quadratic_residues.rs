use num_bigint::{BigUint};

// a is assumed to be co-prime
// quadratic residue check

pub fn is_quadratic_residue(a: &BigUint, p: &BigUint) -> bool {
    a.modpow((p-1)/2, p) == 1
}