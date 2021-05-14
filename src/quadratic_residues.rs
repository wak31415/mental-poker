/* quadratic_residues.rs
 * Part of the Mental Poker project
 * Course: MAA313-2020
 * Members: William Koch,
 *          Olavi Äikäs,
 *          Mihails Valtusovs
 * See git history for contributions
 */

use num_bigint::{BigUint};

// a is assumed to be co-prime
// quadratic residue check
pub fn is_p(a: &BigUint, p: &BigUint) -> bool {
    if a.modpow(&((p-1u32)/2u32), p) == BigUint::from(1u32) {
        return true;
    }
    return false;
}

pub fn is_n(a: &BigUint, p1: &BigUint, p2: &BigUint) -> bool {
    return is_p(a, p1) && is_p(a, p2);
}