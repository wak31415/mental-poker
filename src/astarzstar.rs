mod jacobi;
mod prime_gen;
mod gcd;
use num_bigint::BigUint;

pub fn rand_astar(n: &BigUInt) -> BigUInt {
    loop {
        candidate = gen_large_number(n.size(), false);
        if candidate >= n {
            continue;
        }
        if gcd::compute(n, &candidate) != 1 {
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
        candidate = gen_large_number(n.size(), false);
        if candidate >= n {
            continue;
        }
        if gcd::compute(n, &candidate) != 1 {
            continue;
        }
        return candidate;
    }
}