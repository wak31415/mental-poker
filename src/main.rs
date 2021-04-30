mod prime_gen;

fn main() {
    let p = prime_gen::gen_prime(512);
    println!("{}", p);
    println!("{}", p.bits());
}
