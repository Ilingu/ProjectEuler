use std::collections::{hash_set::IntoIter, HashSet};

pub fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = set!(1, n);
    for d in 2..=((n as f64).sqrt().floor() as usize) {
        if n % d == 0 {
            divisors.insert(d);
            divisors.insert(n / d);
        }
    }
    divisors.into_iter().sorted()
}

pub fn is_abundant(n: usize) -> bool {
    divisors(n).into_iter().filter(|&x| x != n).sum::<usize>() > n
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
