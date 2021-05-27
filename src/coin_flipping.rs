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

use super::crypto_system::{Plaintext, Ciphertext, PrivateKey, PublicKey};

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
pub fn generate_keys(size: usize) -> (PrivateKey, PublicKey) {
    // I really hate constants - Mihails
    let P = prime_gen::gen_prime(size);
    let Q = prime_gen::gen_prime(size);

    let N = &P * &Q;

    let mut y = astarzstar::rand_astar(&N);
    while check_quadratic_residuoity(&y, &N, &P, &Q) {
        y = astarzstar::rand_astar(&N);
    }

    let privkey = PrivateKey { p: P, q: Q };
    let pubkey = PublicKey { N: N, y: y };

    // publicize N, y
    // keep private P, Q
    (privkey, pubkey)
}

pub fn guess_whether_quadratic_residue(pubkey: &PublicKey, q: &BigUint) -> bool {
    let jacobi = jacobi::compute(q, &pubkey.N);
    if jacobi != 1 {
        return false;
    }
    if &pubkey.y == q {
        return false;
    }

    let mut rng = thread_rng();
    let flip : bool = rng.gen();

    return flip;
}

/* The sender, in the paper called "B", has the factorization
 * and thus knows if A succeeded or not. Thus, contrary to usual
 * notation, sender is B and receiver is A. */

// The sender generates the big numbers to send.
pub fn values_sender(n: usize, N: &BigUint) -> Ciphertext {
    let mut b: Ciphertext = Ciphertext::with_capacity(n);
    for _i in 0..n {
        b.push(astarzstar::rand_astar(N));
    }
    return b;
}

/* The receiver receives the big numbers and
 * sends guesses */
pub fn values_receiver(pubkey: &PublicKey, b: &Ciphertext) -> Plaintext {
    let mut a: Plaintext = Plaintext::with_capacity(b.len());
    for i in 0..b.len() {
        a.push(guess_whether_quadratic_residue(&pubkey, &b[i]));
    }
    return a;
}

/* The sender receives the guesses and knows
 * whether the receiver succeeded their coin throws
 * from the original numbers that the sender also has */
pub fn values_checker(b: &Ciphertext, a: &Plaintext, privkey: &PrivateKey) -> Plaintext {
    assert_eq!(b.len(), a.len());
    let mut m: Plaintext = Plaintext::with_capacity(a.len());
    for i in 0..a.len() {
        let a_guessed = a[i];
        let b_knows = quadratic_residues::is_n(&b[i], &privkey.p, &privkey.q);
        if a_guessed == b_knows {
            m.push(true);       
        } else {
            m.push(false);
        }
    }
    return m;
}