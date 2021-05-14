use num_bigint::{BigUint};
use rand::prelude::*

type message = Vec<Bool>;
type ciphertext = Vec<BigUint>;

pub fn encrypt(message: &message, n: &BigUint, y: &BigUint) {
    let mut res: ciphertext = Vec<BigUint>::with_capacity(message.len());
    let mut rng = rand::thread_rng();
    for i in 0..message.len() {
        let xi = rand_zn(n);
        if message[i] {
            res[i] = y*xi*xi % n;
        } else {
            res[i] = xi.modpow(&BigUint::from(2u32), n);
        }
    }
}
