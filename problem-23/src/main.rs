use std::collections::HashSet;

use problem_23::is_abundant;

fn main() {
    let abundant = (12..=28_123)
        .filter(|&n| is_abundant(n))
        .collect::<Vec<_>>();

    let mut sum_of_2_abundants = HashSet::new();
    for a1 in &abundant {
        for a2 in &abundant {
            if a1 + a2 > 28_123 {
                break;
            }
            sum_of_2_abundants.insert(a1 + a2);
        }
    }

    let result = (0..=28_123)
        .filter(|n| !sum_of_2_abundants.contains(n))
        .sum::<usize>();
    println!("{result:?}");
}
