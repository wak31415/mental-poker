use super::astarzstar;
use super::quadratic_residues::is_n;
use super::card_deck;
use num_bigint::{BigUint};

pub type Plaintext = Vec<bool>;
pub type Ciphertext = Vec<BigUint>;

pub struct PrivateKey {
    pub p: BigUint,
    pub q: BigUint
}

pub struct PublicKey {
    pub N: BigUint,
    pub y: BigUint
}

pub fn num_to_plaintext(num: u8) -> Plaintext {
    let mut pt : Plaintext = Vec::new();
    let mut tmp = num;
    for _i in 0..6 {
        pt.push((tmp&1) != 0);
        tmp = tmp >> 1;
    }
    pt

}

pub fn encrypt(message: &Plaintext, pubkey: &PublicKey) -> Ciphertext {
    let mut res: Ciphertext = Vec::<BigUint>::with_capacity(message.len());
    for i in 0..message.len() {
        let xi = astarzstar::rand_zstar(&pubkey.N);
        if message[i] {
            res.push((&pubkey.y*&xi*&xi) % &pubkey.N);
        } else {
            res.push(xi.modpow(&BigUint::from(2u32), &pubkey.N));
        }
    }
    res
}


pub fn encrypt_card(message: &Box<card_deck::Card>, pubkey: &PublicKey) -> card_deck::EncrCard {
    let encr_card = card_deck::EncrCard { N: pubkey.N.clone(), ciphertext: encrypt(&message.plaintext, &pubkey) };
    encr_card
}


pub fn decrypt(cipher: &Ciphertext, privkey: &PrivateKey) -> Plaintext {
    let mut res = Plaintext::with_capacity(cipher.len());
    for i in 0..cipher.len() {
        if is_n(&cipher[i], &privkey.p, &privkey.q) {
            res.push(false);
        } else {
            res.push(true);
        }
    }
    res
}

pub fn decrypt_card(cipher: &Box<card_deck::EncrCard>, privkey: &PrivateKey) -> card_deck::Card {
    let decr_card = card_deck::Card { p: BigUint::from(0u8), q: BigUint::from(0u8), N: BigUint::from(0u8), plaintext: decrypt(&cipher.ciphertext, &privkey) };
    decr_card
}