mod prime_gen;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let p1 = prime_gen::gen_prime(512);
    let mut p2 = prime_gen::gen_prime(512);
    while p1 == p2 {
        p2 = prime_gen::gen_prime(512)
    }
    //p1 and p2 are unequal primes (private key)
    let n = p1*p2; //n is the public key together with some non-residue y to be computed
    let mut y = astar(n).choose(rng); //astar (vec<bigUInt>) is the coprimes of n with jacobi symbol 1
    while is_residue(n, y) {
        //We regenerate y until its not a quadratic residue
        y = astar(n).choose(rng);
    }
    //Now (n, y) is the public key
    let message = "Hello world".to_string();
    let cipher = encrypt(&n, &y, &message);
    

}
