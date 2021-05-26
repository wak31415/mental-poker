mod prime_gen;
mod astarzstar;
mod crypto_system;
mod gcd;
mod jacobi;
mod quadratic_residues;
mod card_deck;

fn main() {
    let cards = card_deck::gen_card_deck(32, 8);
    println!("{:?}", cards);

    
    let p1 = prime_gen::gen_prime(512);
    let mut p2 = prime_gen::gen_prime(512);
    while &p1 == &p2 {
        p2 = prime_gen::gen_prime(512);
    }
    
    println!("Generated primes");
    //p1 and p2 are unequal primes (private key)
    let privkey = crypto_system::PrivateKey{p: p1, q: p2};
    let n = &privkey.p*&privkey.q; //n is the public key together with some non-residue y to be computed
    let mut y = astarzstar::rand_astar(&n); //astar (vec<bigUInt>) is the coprimes of n with jacobi symbol 1
    while quadratic_residues::is_n(&y, &privkey.p, &privkey.q) {
        //We regenerate y until its not a quadratic residue
        y = astarzstar::rand_astar(&n);
    }
    //Now (n, y) is the public key
    let message: Vec<bool> = vec![true, true, false, false, true];
    let pubkey = crypto_system::PublicKey{N: n, y: y};
    
    let cipher = crypto_system::encrypt(&message, &pubkey);
    for i in 0..5 {
        println!("{}", cipher[i]);
    }
    let decrypted_message = crypto_system::decrypt(&cipher, privkey);
    for i in 0..5 {
        println!("{}", decrypted_message[i]);
    }
    // println!("{:?}", crypto_system::encrypt_card(cards[0], &pubkey));
}
