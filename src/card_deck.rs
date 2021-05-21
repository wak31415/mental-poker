use super::prime_gen;
use rand::thread_rng;
use rand::seq::SliceRandom;
use num_bigint::{BigUint};
use std::thread;

#[derive(Debug)]
pub struct Card {
    p: BigUint,
    q: BigUint,
    N: BigUint
}

fn gen_card(prime_size: usize, id: usize) -> Card {
    // generate p_i, q_i s.t. p_i congruent q_i = 3 mod 4
    let mut p_i: BigUint;
    let mut q_i: BigUint;
    loop {
        p_i = prime_gen::gen_prime(prime_size);
        q_i = prime_gen::gen_prime(prime_size);
        if &p_i % 4u16 == BigUint::from(3u32) && &q_i % 4u16 == BigUint::from(3u32) { break; }
    }
    let N = &p_i * &q_i;
    let c = Card { p: p_i.clone(), q: q_i.clone(), N: N.clone() };
    println!("Generated card {}.", id);
    c
}

pub fn gen_card_deck(prime_size: usize, deck_size: usize) -> Vec::<Box<Card>> {
    let mut cards: Vec::<Box<Card>> = Vec::new();
    let mut children = Vec::new();

    // let num_threads : usize = 8;

    // let cards_per_thread : usize = deck_size / num_threads;
    
    for n in 0..deck_size {
        let child = thread::spawn(move || { gen_card(prime_size, n) });
        children.push(child);
    }
    for child in children {
        let c = child.join().unwrap();
        cards.push(Box::new(c));
    }
    cards.shuffle(&mut thread_rng());
    cards
}