mod prime_gen;
mod astarzstar;
mod crypto_system;
mod gcd;
mod jacobi;
mod quadratic_residues;

fn main() {
    let p1 = prime_gen::gen_prime(512);
    let mut p2 = prime_gen::gen_prime(512);
    while &p1 == &p2 {
        p2 = prime_gen::gen_prime(512);
    }
    println!("Generated primes");
    //p1 and p2 are unequal primes (private key)
    let n = &p1*&p2; //n is the public key together with some non-residue y to be computed
    let mut y = astarzstar::rand_astar(&n); //astar (vec<bigUInt>) is the coprimes of n with jacobi symbol 1
    while quadratic_residues::is_quadratic_residue_n(&y, &p1, &p2) {
        //We regenerate y until its not a quadratic residue
        y = astarzstar::rand_astar(&n);
    }
    //Now (n, y) is the public key
    let message: Vec<bool> = vec![true, true, false, false, true, false, false, true, false, true];
    let cipher = crypto_system::encrypt(&message, &n, &y);
    for i in 0..message.len() {
        println!("{}", cipher[i]);
    }
    let decrypted_message = crypto_system::decrypt(&cipher, &p1, &p2);
    for i in 0..decrypted_message.len() {
        println!("{}", decrypted_message[i]);
    }
}
