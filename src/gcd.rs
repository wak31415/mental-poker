/* gcd.rs
 * Part of the Mental Poker project
 * Course: MAA313-2020
 * Members: Mihails Valtusovs,
 *          William Koch
 *          Olavi Äikäs
 * See git history for contributions
 */

// use num_bigint::{ToBigUint, BigUint};
use num_bigint::BigUint;
use num_traits::Zero;
// use partial_application::partial!;

pub fn compute(a: &BigUint, b: &BigUint) -> BigUint {
    return euclidean(a, b);
}

pub fn euclidean(a: &BigUint, b: &BigUint) -> BigUint {
    // println!("euclidean {} {}", a, b);
    if a > b {
        if b.is_zero() {
            let a : BigUint = a.clone();
            return a;
        }
        return euclidean(b, &(a%b));
    } else {
        if a.is_zero() {
            let b : BigUint = b.clone();
            return b;
        }
        return euclidean(a, &(b%a));
    }
}