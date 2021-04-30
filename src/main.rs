use num_bigint::BigUint;

mod prime_gen;

fn main() {
    let n = BigUint::parse_bytes(b"1001692", 10);
    match n {
        None => return,
        Some(m) => println!("{}", prime_gen::divide_small_primes(&m))
    }
}
