use std::{fs, ops::Add, str::FromStr};

use num_bigint::BigUint;

fn main() {
    let nums = fs::read_to_string("./src/input.txt").unwrap();

    let mut sum = BigUint::new(vec![0]);
    for num in nums.lines() {
        sum = sum.add(BigUint::from_str(num).unwrap());
    }

    println!("{}", &sum.to_string().as_str()[..10]);
}
