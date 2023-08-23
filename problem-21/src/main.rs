use std::collections::HashSet;

use problem_21::is_amicable_numbers;

fn amicable_num_sum(majorant: usize) -> usize {
    let mut amicable = HashSet::new();
    for a in 1..=majorant {
        for b in 1..=majorant {
            if is_amicable_numbers(a, b) {
                amicable.insert(a);
                amicable.insert(b);
            }
        }
    }

    amicable.into_iter().sum::<usize>()
}

fn main() {
    let amicable_sum = amicable_num_sum(10_000);
    println!("{amicable_sum}")
}

#[cfg(test)]
mod ptests {
    use super::amicable_num_sum;

    #[test]
    fn amicable_num_sum_test() {
        assert_eq!(amicable_num_sum(1000), 504);
        assert_eq!(amicable_num_sum(2000), 2898);
        assert_eq!(amicable_num_sum(5000), 8442);
        // assert_eq!(amicable_num_sum(10000), 31626);
    }
}
