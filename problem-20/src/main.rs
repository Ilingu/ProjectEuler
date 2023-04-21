use std::ops::Mul;

use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

fn main() {
    let sum_100_factorial: u32 = factorial_big(100)
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum();
    println!("{sum_100_factorial}")
}

fn factorial_big(n: u8) -> BigUint {
    let mut product: BigUint = One::one();
    for k in 1..=n {
        product = product.mul(k.to_biguint().unwrap());
    }
    product
}
