use num_bigint::{BigUint};
use rand::prelude::*;

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

fn divide_small_primes(n: &BigUint) -> bool {
    let small_primes = erastothenes_sieve(20000);
    for m in small_primes {
        if divides(n, m) {
            return false;
        }
    }
    return true;
}

fn mr_decomp(candidate: &BigUint) -> (u32, BigUint) {
    let mut i = 0u32;
    let r: u32 = loop {
        if divides(&(candidate - 1u32), u64::pow(2, i)) {
            i += 1;
        } else {
            i -= 1;
            break i;
        }
    };
    let d = (candidate - 1u32)/u64::pow(2, r);
    return (r, d);
}


fn miller_rabin(candidate: &BigUint, rounds: usize) -> bool {
    let (r, d) = mr_decomp(candidate);
    for _k in 0..rounds {
        let mut a = gen_large_number(candidate.bits() as usize, false);
        while a < BigUint::from(2u32) || a > candidate - 2u32 {
            a = gen_large_number(candidate.bits() as usize, false);
        }
        let mut x = a.modpow(&d, candidate);
        if x == BigUint::from(1u32) || x == candidate - 1u32 {
            continue;
        }
        let mut failed = false;
        for _i in 1..r {
            x = x.modpow(&BigUint::from(2u32), candidate);
            if x == candidate - 1u32 {
                failed = true;
                break;
            }
        }
        if failed {
            continue;
        }
        return false;
    }
    return true;
}

fn gen_large_number(size: usize, force_size: bool) -> BigUint {
    let mut rng = thread_rng();
    let mut w: BigUint;
    {
        let mut v = Vec::<u32>::new();
        let et = match size%32 { 0 => 0, _ => 1};
        for _i in 0..(size/32 + et) {
            v.push(rng.gen());
        }
        w = BigUint::new(v);
    }
    if force_size {
        w.set_bit((size - 1) as u64, true);
        w.set_bit(0u64, true);  
    }
    return w;
}

pub fn gen_prime(size: usize) -> BigUint {
    let res = loop {
        let candidate = gen_large_number(size, true);
        if !divide_small_primes(&candidate) {
            continue;
        }
        if miller_rabin(&candidate, 8) {
            break candidate;
        }
    };
    return res;
}