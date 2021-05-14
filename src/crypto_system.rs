use super::astarzstar;
use num_bigint::{BigUint};

type Message = Vec<bool>;
type Ciphertext = Vec<BigUint>;

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
