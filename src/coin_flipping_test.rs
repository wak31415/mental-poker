mod prime_gen;
mod astarzstar;
mod crypto_system;
mod gcd;
mod jacobi;
mod quadratic_residues;
mod coin_flipping;

fn main() {
    let (N, y, P, Q) = coin_flipping::generate_keys(10);
    let q = astarzstar::rand_astar(&N);

    let b_answers = coin_flipping::guess_whether_quadratic_residue(&N, &y, &q);

    let mut s_b : String = String::from("");
    if b_answers { s_b = String::from("is") } else { s_b = String::from("is not") }

    let a_knows = quadratic_residues::is_n(&q, &P, &Q);

    let mut s_a : String = String::from("");
    if a_knows { s_a = String::from("is") } else { s_a = String::from("is not") }

    let mut s_w : String = String::from("");
    if b_answers == a_knows { s_w = String::from("won") } else { s_w = String::from("lost") }

    println!("One run of coin-flipping-in-a-well: ");
    println!("A generated N = {}, y = {}, P = {}, Q = {}; ", N, y, P, Q);
    println!("A generated q = {}; ", q);
    println!("B guessed that q {} a q.r. mod N; ", s_b);
    println!("A knows that q {} a q.r. mod N; ", s_a);
    println!("B {} this coin flip.", s_w);
}