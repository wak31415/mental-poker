/* coin_flipping.rs
 * Part of the Mental Poker project
 * Course: MAA313-2020
 * Members: William Koch,
 *          Mihails Valtusovs,
 *          Olavi Äikäs
 * See git history for contributions
 */
#![allow(non_snake_case)]

// mod prime_gen;
use super::prime_gen;
use super::quadratic_residues;
use super::astarzstar;
use super::jacobi;
use super::gcd;
use rand::prelude::*;
use num_bigint::BigUint;
use num_traits::One;

fn check_quadratic_residuoity(y: &BigUint, N: &BigUint, P: &BigUint, Q: &BigUint) -> bool {
    if !(gcd::compute(y, N).is_one()) {
        return false;
    }   
    if jacobi::compute(y, N) != 1 {
        return false;
    }
    return quadratic_residues::is_n(y, P, Q);
}

// Returns N, y, P, Q
pub fn generate_keys(size: usize) -> (BigUint, BigUint, BigUint , BigUint) {
    // I really hate constants - Mihails
    let P = prime_gen::gen_prime(size);
    let Q = prime_gen::gen_prime(size);

    let N = &P * &Q;

    let mut y = astarzstar::rand_astar(&N);
    while check_quadratic_residuoity(&y, &N, &P, &Q) {
        y = astarzstar::rand_astar(&N);
    }

    // publicize N, y
    // keep private P, Q
    return (N, y, P, Q);
}

pub fn guess_whether_quadratic_residue(N: &BigUint, y: &BigUint, q: &BigUint) -> bool {
    let jacobi = jacobi::compute(q, N);
    if jacobi != 1 {
        return false;
    }
    if y == q {
        return false;
    }

    let mut rng = thread_rng();
    let flip : bool = rng.gen();

    return flip;
}