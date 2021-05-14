mod gen_prime
mod quadratic_residues

// Returns A_N, N, y, P, Q
pub fn generate_keys() -> (Vec::<BigUint> A_N, BigUint, BigUint, BigUint, BigUint) {
    let P = prime_gen::gen_prime(512);
    let Q = prime_gen::gen_prime(512);

    let N = P * Q;

    // TODO: Generate A_N
    // A_N = 

    /*
    let mut y = sample_from_A_N()
    while !is_quad_res(y, p) {
        y = sample_from_A_N
    }

    */
}