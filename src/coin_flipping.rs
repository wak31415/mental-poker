/* coin_flipping.rs
 * Part of the Mental Poker project
 * Course: MAA313-2020
 * Members: William Koch,
 *          Mihails Valtusovs,
 *          Olavi Äikäs
 * See git history for contributions
 */

// mod prime_gen;
use super::prime_gen;
mod quadratic_residues;
mod astarzstar;
mod jacobi;
mod gcd;
use rand::prelude::*;

fn check_quadratic_residuoity(y: &BigUint, N: &BigUint, P: &BigUint, Q: &BigUint) -> bool {
    if gcd::compute(y, N) != 1 {
        return false;
    }   
    if jacobi::compute(y, N) == -1 {
        return false;
    }
    let qr_p : bool = quadratic_residues::is_quadratic_residue(y, P);
    let qr_q : bool = quadratic_residues::is_quadratic_residue(y, Q);

    return (qr_p && qr_q);
}

// Returns A_N, N, y, P, Q
pub fn generate_keys(size: usize) -> (BigUint, BigUint, BigUint , BigUint) {
    // I really hate constants - Mihails
    let P = prime_gen::gen_prime(size);
    let Q = prime_gen::gen_prime(size);

    let N = P * Q;

    let mut y = astarzstar::rand_astar(&N);
    while !check_quadratic_residuoity(&y, &N, &P, &Q) {
        y = astarzstar::rand_astar(&N);
    }

    // publicize N, y
    // keep private P, Q
    return (N, y, P, Q);
}

pub fn guess_whether_quadratic_residue(N: &BigUint, y: &BigUint, q: &BigUint) -> bool {
    let jacobi = jacobi::compute(q, N);

    if (jacobi == -1) {
        return false;
    }

    let mut rng = thread_rng();
    let flip : bool = rng.gen();

    return flip;
}