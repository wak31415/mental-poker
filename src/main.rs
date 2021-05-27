mod prime_gen;
mod astarzstar;
mod crypto_system;
mod gcd;
mod jacobi;
mod quadratic_residues;
mod card_deck;
mod coin_flipping;

use crypto_system::{Ciphertext, Plaintext};

fn main() {
    let (privkey, pubkey) = coin_flipping::generate_keys(10);
    let q = astarzstar::rand_astar(&pubkey.N);

    let b_answers = coin_flipping::guess_whether_quadratic_residue(&pubkey, &q);

    let mut s_b : String = String::from("");
    if b_answers { s_b = String::from("is") } else { s_b = String::from("is not") }

    let a_knows = quadratic_residues::is_n(&q, &privkey.p, &privkey.q);

    let mut s_a : String = String::from("");
    if a_knows { s_a = String::from("is") } else { s_a = String::from("is not") }

    let mut s_w : String = String::from("");
    if b_answers == a_knows { s_w = String::from("won") } else { s_w = String::from("lost") }

    println!("One run of coin-flipping-in-a-well: ");
    println!("A generated N = {}, y = {}, P = {}, Q = {}; ", pubkey.N, pubkey.y, privkey.p, privkey.q);
    println!("A generated q = {}; ", q);
    println!("B guessed that q {} a q.r. mod N; ", s_b);
    println!("A knows that q {} a q.r. mod N; ", s_a);
    println!("B {} this coin flip.", s_w);

    println!("...");
    println!("Now we try the aggregate coin-flipping.");

    let b: Ciphertext = coin_flipping::values_sender(10usize, &pubkey.N);
    println!("B generated 10 values: {:?}", b);
    
    let b_q: Vec<bool> = b.iter().map(|x| quadratic_residues::is_n(&(x.clone()), &privkey.p, &privkey.q)).collect::<Vec<_>>();
    println!("Their quadratic residuoity is as follows: {:?}", b_q);

    let a: Plaintext = coin_flipping::values_receiver(&pubkey, &b);
    println!("A did 10 guesses: {:?}", a);

    let c: Ciphertext = crypto_system::encrypt(&a, &pubkey);
    println!("A encrypted their guesses to send them to B: {:?}", c);
    
    let a_d: Plaintext = crypto_system::decrypt(&c, &privkey);
    print!("B decrypted A's guesses: {:?}. They ", a_d);
    if a_d != a {print!("do not ");}
    println!("match what A sent.");

    let m: Plaintext = coin_flipping::values_checker(&b, &a, &privkey);
    println!("Here are the results of the coin flips: {:?}", m);

    println!("...");
    println!("Now let's check that aggregate coin-flipping is not biased.");
    println!("Generating 10000 values...");
    let b: Ciphertext = coin_flipping::values_sender(10000usize, &pubkey.N);    
    println!("Generated 10000 values...");
    let a: Plaintext = coin_flipping::values_receiver(&pubkey, &b);
    println!("Generated 10000 guesses...");
    let m: Plaintext = coin_flipping::values_checker(&b, &a, &privkey);
    println!("Checked 10000 guesses...");
    let n: usize = m.iter().map(|&x| if x {1usize} else {0usize}).sum();
    let f = (n as f64) / (10000f64);
    println!("Frequency of true over 10000 flips: {}", f); 
    
}
