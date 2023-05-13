// resolve with newton method : x/ln(x) - b = 0; the solution x is the just above the right answer
// from here use the eratosthenes algorithm and return the nth (or go from x to 0 in brute force?)

use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{}", nth_prime(10_001));
    let duration = start.elapsed();
    println!("Time elapsed in calculating this prime is: {:?}", duration); // >100s (which is bad? idk)
}

fn nth_prime(n: usize) -> usize {
    let f_equation = |x: f64| (x / x.ln()) - n as f64; // defined on ]1;+inf[
    let derivative = |x: f64| (x.ln() - 1.0) / (x.ln()).powi(2); // defined on ]1;+inf[
    let solution = newton_raphson(3.0, 1.0, f_equation, derivative).round() as usize;

    let primes = sieve_of_eratosthenes(solution);
    primes[n - 1]
}

fn newton_raphson(
    x0: f64,
    approx: f64,
    f: impl Fn(f64) -> f64,
    f_derivative: impl Fn(f64) -> f64,
) -> f64 {
    let (mut root, mut last_root) = (x0, x0);
    loop {
        root -= f(root) / f_derivative(root);
        if (root - last_root).abs() <= approx {
            break root;
        }
        last_root = root
    }
}

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let (mut list, mut marked) = ((2..=n).collect::<Vec<_>>(), vec![]);
    let mut p = 2;

    loop {
        for i in 2..=(n / p) {
            if !marked.contains(&(p * i)) {
                list.remove(list.iter().position(|&x| x == p * i).unwrap());
                marked.push(p * i);
            }
        }

        let next_p = list.iter().find(|&&x| x > p);
        match next_p {
            Some(&np) => p = np, // I'm rich now, lol
            None => break list,
        }
    }
}
