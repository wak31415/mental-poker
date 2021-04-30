/* jacobi.rs
 * Part of the Mental Poker project
 * Course: MAA313-2020
 * Members: Mihails Valtusovs,
 *          William Koch
 *          Olavi Äikäs
 * See git history for contributions
 */

use num_bigint::{ToBigUint, BigUint};
// use num_bigint::BigUint;
use num_traits::{Zero, One};
// use partial_application::partial!;

// Based on http://math.fau.edu/richman/jacobi.htm
pub fn compute(m: &BigUint, n: &BigUint) -> i32 {
    // println!("compute {}, {}", m, n);
    let z: BigUint = Zero::zero();
    let o: BigUint = One::one();
    assert_eq!(n % 2u8, o);
    let maybe_seven = 7u8.to_biguint();
    let s: BigUint = match maybe_seven {
        Some(x) => x,
        _ => panic!("cannot convert 7 to BigUint")
    };
    let maybe_three = 3u8.to_biguint();
    let t: BigUint = match maybe_three {
        Some(x) => x,
        _ => panic!("cannot convert 3 to BigUint")
    };
         if m == &z     { return 0 }
    else if m == &o     { return 1 }
    else if m > n      { compute(&(m % n), n) }
    else if m % 2u8 == z { 
        if n % 8u8 == o || n % 8u8 == s { compute(&(m / 2u8), n) }
        else { -compute(&(m / 2u8), n) }
    } 
    else {
        if m % 4u8 == t && n % 4u8 == t {
            -compute(n, m)
        } else { 
            compute(n, m)
        }
    } 
}