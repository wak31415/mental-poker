use num_bigint::BigUint;
use super::prime_gen;
use super::gcd;
use super::jacobi;


pub fn rand_astar(n: &BigUint) -> BigUint {
    loop {
        let candidate = prime_gen::gen_large_number(n.bits() as usize, false);
        if &candidate >= n {
            continue;
        }
        if gcd::compute(n, &candidate) != BigUint::from(1u32) {
            continue;
        }
        if jacobi::compute(&candidate, n) == -1 {
            continue;
        }
        return candidate;
    }
}

pub fn rand_zstar(n: &BigUint) -> BigUint {
    loop {
        let candidate = prime_gen::gen_large_number(n.bits() as usize, false);
        if &candidate >= n {
            continue;
        }
        if gcd::compute(n, &candidate) != BigUint::from(1u32) {
            continue;
        }
        return candidate;
    }
}