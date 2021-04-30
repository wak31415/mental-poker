use num_bigint::{ToBigUint, BigUint};
// use crate::jacobi::jacobi;
mod jacobi;

fn main() {
    let (m, n) = (1236u32.to_biguint(), 20003u32.to_biguint());

    match (m, n) {
        (Some(x), Some(y)) => println!("{}",jacobi::compute(&x,&y)),
        _ => println!("Conversion failed!")
    }
}