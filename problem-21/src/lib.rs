use std::collections::{hash_set::IntoIter, HashSet};

pub fn is_amicable_numbers(a: usize, b: usize) -> bool {
    a != b && d(a) == b && d(b) == a
}

fn d(n: usize) -> usize {
    proper_divisors(n).into_iter().sum::<usize>()
}

fn proper_divisors(n: usize) -> Vec<usize> {
    let mut divisors = set!(1);
    for d in 2..=((n as f64).sqrt().floor() as usize) {
        if n % d == 0 {
            divisors.insert(d);
            divisors.insert(n / d);
        }
    }
    divisors.remove(&n);
    divisors.into_iter().sorted()
}

trait IterSorted<T> {
    fn sorted(self) -> Vec<T>;
}

impl<T: std::cmp::Ord> IterSorted<T> for IntoIter<T> {
    fn sorted(self) -> Vec<T> {
        let mut v = self.collect::<Vec<_>>();
        v.sort();
        v
    }
}

#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

#[cfg(test)]
mod libtests {
    use crate::is_amicable_numbers;

    #[test]
    fn is_amicable_numbers_test() {
        assert!(is_amicable_numbers(220, 284));
        assert!(is_amicable_numbers(284, 220));
    }
}
