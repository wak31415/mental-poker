use super::prime_gen;
use rand::thread_rng;
use rand::seq::SliceRandom;
use num_bigint::{BigUint};

#[derive(Debug)]
pub struct Card {
    p: BigUint,
    q: BigUint,
    N: BigUint
}

pub fn gen_card_deck(prime_size: usize, deck_size: usize) -> Vec::<Box<Card>> {
    let mut cards: Vec::<Box<Card>> = Vec::new();

    let _3 = BigUint::from(3u32);
    for i in 0..deck_size {
        println!("Generating card {}", i);
        // generate p_i, q_i s.t. p_i congruent q_i = 3 mod 4
        let mut p_i: BigUint;
        let mut q_i: BigUint;
        loop {
            p_i = prime_gen::gen_prime(prime_size);
            q_i = prime_gen::gen_prime(prime_size);
            if &p_i % 4u16 == _3 && &q_i % 4u16 == _3 { break; }
        }
        let N = &p_i * &q_i;
        let c = Card { p: p_i.clone(), q: q_i.clone(), N: N.clone() };
        cards.push(Box::new(c));
    }
    cards.shuffle(&mut thread_rng());
    cards
}