use num_bigint::BigUint;

fn main() {
    let max_prime = 20000;
    let mut sieve = Vec::<Option::<BigUint>>::new();
    for i in 0..max_prime {
        sieve.push(Some(BigUint::new(vec![0 as u32])));
        sieve[i] = match &sieve[i] {
            Some(n) => Some(n+(i as u32)),
            None => None
        }
    }
    sieve[0] = None;
    sieve[1] = None;
    for i in 2..max_prime {
        if sieve[i] == None {
            continue;
        }
        let mut k = 2;
        while i*k < max_prime {
            sieve[i*k] = None;
            k += 1;
        }
    }
    let mut p: usize = 0;
    let mut deleted: bool;
    while p < sieve.len() {
        deleted = false;
        if sieve[p] == None {
            sieve.remove(p);
            deleted = true;
        }
        p += 1;
        if deleted {
            p -= 1;
        }
    }
    for bi in sieve {
        match bi {
            None => println!("None"),
            Some(n) => println!("{}",n)
        }
    }
}
