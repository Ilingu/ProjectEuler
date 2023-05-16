use std::collections::HashSet;

fn main() {
    println!("{}", sieve_of_eratosthenes(2_000_000).iter().sum::<usize>())
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut list: HashSet<usize> = HashSet::from_iter(2..=n);
    let mut p = 2;

    loop {
        for i in 2..=(n / p) {
            list.remove(&(p * i));
        }

        let next_p = list.iter().filter(|&&x| x > p).min();
        match next_p {
            Some(&np) => {
                if np > (n as f64).sqrt() as usize {
                    break list.iter().map(|x| *x).collect();
                }
                p = np // I'm rich now, lol
            }
            None => break list.iter().map(|x| *x).collect(),
        }
    }
}
