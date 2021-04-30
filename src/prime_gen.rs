use num_bigint::BigUint;

fn erastothenes_sieve(roof: usize) -> Vec::<u64> {
    let mut sieve = Vec::<u64>::new();
    for i in 0..roof {
        sieve.push(i as u64);
    }
    sieve[1] = 0;
    for i in 2..roof {
        if sieve[i] == 0 {
            continue;
        }
        let mut k = 2;
        while i*k < roof {
            sieve[i*k] = 0;
            k += 1;
        }
    }
    let mut p: usize = 0;
    let mut deleted: bool;
    while p < sieve.len() {
        deleted = false;
        if sieve[p] == 0 {
            sieve.remove(p);
            deleted = true;
        }
        if !deleted {
            p += 1;
        }
    }
    return sieve;
}

fn divides(b: &BigUint, d: u64) -> bool {
    let r = b/d;
    if &(r*d) == b {
        return true;
    }
    return false;
}

pub fn divide_small_primes(n: &BigUint) -> bool {
    let small_primes = erastothenes_sieve(20000);
    for m in small_primes {
        if divides(n, m) {
            return false;
        }
    }
    return true;
}