use num_bigint::{BigUint};

// a is assumed to be co-prime
// quadratic residue check

pub fn is_quadratic_residue(a: &BigUint, p: &BigUint) -> bool {
    if a.modpow(&((p-1u32)/2u32), p) == BigUint::from(1u32) {
        return true;
    }
    return false;
}

pub fn is_quadratic_residue_n(a: &BigUint, p1: &BigUint, p2: &BigUint) -> bool {
    return is_quadratic_residue(a, p1) && is_quadratic_residue(a, p2);
}