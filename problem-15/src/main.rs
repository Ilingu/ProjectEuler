use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

fn main() {
    println!("{}", binomial_coeff(40, 20));
}

fn binomial_coeff(n: usize, k: usize) -> BigUint {
    arrangement(n, n - k + 1) / arrangement(k, 1)
}

fn arrangement(n: usize, k: usize) -> BigUint {
    (k..=n).fold(One::one(), |acc, x| acc * x.to_biguint().unwrap())
}
