use num_bigint::BigUint;
use num_bigint::BigInt;
use num_bigint::Sign;
use super::prime_gen::mr_decomp;
use std::cmp::min;
use std::convert::TryInto;

fn pow(n: &BigUint, exp: BigUint) -> BigUint {
    n.pow(exp.try_into().expect("exponent too large for pow()"))
}

fn inverse_in_field(a: &BigUint, n: &BigUint) -> BigUint {
    let mut t: BigInt = BigInt::from(0u32);
    let mut newt: BigInt = BigInt::from(1u32);
    let mut r: BigInt = BigInt::from_biguint(Sign::Plus, n.clone());
    let mut newr: BigInt = BigInt::from_biguint(Sign::Plus, a.clone());
    while &newr != &BigInt::from(0u32) {
        let quotient = &r / &newr;
        let old_t = t;
        t = newt.clone();
        newt =  &old_t - &quotient*&newt;
        let old_r = r;
        r = newr.clone();
        newr = &old_r - &quotient*&newr;
    }
    if &r > &BigInt::from(1u32) {
        return BigUint::from(0u32);
    }
    if &t < &BigInt::from(0u32) {
        t = &t + &BigInt::from_biguint(Sign::Plus, n.clone());
    }
    match t.to_biguint() {
        Some(ut) => ut,
        None => BigUint::from(0u32)
    }
}

pub fn least_root(a: &BigUint, p: &BigUint) -> BigUint {
    let mut g: BigUint = BigUint::from(2u32);
    let mut ma = a.clone();
    while g.modpow(&((p - 1u32)/2u32), p) == BigUint::from(0u32) {
        g += 1u32;
    }
    let (k, d) = mr_decomp(p);
    let n = (&d - 1u32)/2u32;
    let mut l: BigUint = BigUint::from(1u32);
    loop {
        let mut j: BigUint = BigUint::from(0u32);
        while &ma.modpow(&(&(BigUint::from(2u32).modpow(&j, p))*&d), p) != &BigUint::from(1u32) {
            j += 1u32;
        }
        if &j == &BigUint::from(0u32) {
            let d = ma.modpow(&(n + 1u32), p);
            let linv = inverse_in_field(&l, p);
            return min((&d*&linv) % p, (&d*&linv) % p);
        }
        ma = &ma*(&g.modpow(&BigUint::from(2u32).modpow(&(&k - &j), p), p));
        println!("{}", ma);
        l = &l*(&g.modpow(&BigUint::from(2u32).modpow(&(&k - &j - 1u32), p), p));
    }
}