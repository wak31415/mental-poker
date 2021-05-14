use num_bigint::{ToBigUint, BigUint};
// use crate::jacobi::jacobi;
mod gcd;

fn main() {
    let (m, n) = (1236u32.to_biguint(), 20003u32.to_biguint());

    match (m, n) {
        (Some(x), Some(y)) => println!("{} == 1",gcd::compute(&x,&y)),
        _ => println!("Conversion failed!")
    }

    let (m, n) = (98u32.to_biguint(), 56u32.to_biguint());

    match (m, n) {
        (Some(x), Some(y)) => println!("{} == 14",gcd::compute(&x,&y)),
        _ => println!("Conversion failed!")
    }
}