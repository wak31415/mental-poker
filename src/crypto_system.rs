use super::astarzstar;
use super::quadratic_residues::is_n;
use num_bigint::{BigUint};

pub type Message = Vec<bool>;
pub type Ciphertext = Vec<BigUint>;

pub fn encrypt(message: &Message, n: &BigUint, y: &BigUint) -> Ciphertext {
    let mut res: Ciphertext = Vec::<BigUint>::with_capacity(message.len());
    for i in 0..message.len() {
        let xi = astarzstar::rand_zstar(n);
        if message[i] {
            res.push((y*&xi*&xi) % n);
        } else {
            res.push(xi.modpow(&BigUint::from(2u32), n));
        }
    }
    return res;
}

pub fn decrypt(cipher: &Ciphertext, p1: &BigUint, p2: &BigUint) -> Message {
    let mut res = Message::with_capacity(cipher.len());
    for i in 0..cipher.len() {
        if is_n(&cipher[i], p1, p2) {
            res.push(false);
        } else {
            res.push(true);
        }
    }
    return res;
}
