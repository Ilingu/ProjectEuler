use prime_factorization::Factorization;

fn main() {
    println!("{}", first_triangle_number_with_n_divisor(500));
}

fn first_triangle_number_with_n_divisor(divisor_num: u32) -> usize {
    let mut nth = 1;
    loop {
        let nth_triangle = triangle_num(nth);
        if number_of_divisor(nth_triangle as u128) >= divisor_num {
            break nth_triangle;
        }
        nth += 1
    }
}

// return the nth triangle number
fn triangle_num(nth: usize) -> usize {
    nth * (nth + 1) / 2
}

// return the exact number of divisor/factor of a number
fn number_of_divisor(n: u128) -> u32 {
    let factor_repr = Factorization::run(n).prime_factor_repr();
    factor_repr
        .iter()
        .map(|(_, exposant)| exposant + 1)
        .product()
}
