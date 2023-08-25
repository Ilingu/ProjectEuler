pub fn lexicographic_permutations(digits: &[u8], n: usize) -> String {
    let perms = digits
        .to_vec()
        .permute()
        .into_iter()
        .map(|digits| {
            digits
                .into_iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>();
    perms[n - 1].to_owned()
}

pub trait Permutation<T: IntoIterator> {
    fn permute(&self) -> Vec<T>;
}

impl Permutation<Vec<u8>> for Vec<u8> {
    fn permute(&self) -> Vec<Vec<u8>> {
        let mut permutations = vec![];
        for (i, &digit) in self.iter().enumerate() {
            let child = self
                .iter()
                .enumerate()
                .filter(|(j, _)| j != &i)
                .map(|(_, d)| *d)
                .collect::<Vec<_>>();
            if child.len() == 1 {
                permutations.push(vec![digit, child[0]]);
                continue;
            }

            let mut child_perms = child.permute();
            child_perms
                .iter_mut()
                .for_each(|perm| perm.insert(0, digit));
            permutations.append(&mut child_perms);
        }
        permutations
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn bin() {
        lexicographic_permutations(&[0, 1, 2], 1);
    }

    #[test]
    fn permute_test() {
        let test: Vec<u8> = [0, 1, 2].to_vec();
        itertools::assert_equal(
            test.permute(),
            test.into_iter().permutations(3).collect::<Vec<_>>(),
        );

        let test: Vec<u8> = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
        itertools::assert_equal(
            test.permute(),
            test.into_iter().permutations(10).collect::<Vec<_>>(),
        );
    }

    #[test]
    fn problem_24_test() {
        // fake digits test
        const FAKE_DIGITS: [u8; 3] = [0, 1, 2];
        const FAKE_TESTS: [&str; 6] = ["012", "021", "102", "120", "201", "210"];

        for (n, expect) in FAKE_TESTS.into_iter().enumerate() {
            assert_eq!(&lexicographic_permutations(&FAKE_DIGITS, n + 1), expect);
        }

        // real digit test
        const DIGITS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        const TESTS: [(usize, &str); 4] = [
            (700_000, "1938246570"),
            (900_000, "2536987410"),
            (900_001, "2537014689"),
            (1_000_000, "2783915460"),
        ];

        for (n, expect) in TESTS {
            assert_eq!(&lexicographic_permutations(&DIGITS, n), expect);
        }
    }
}
