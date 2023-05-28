fn main() {
    let (mut best_start_num, mut best_len) = (1, 1);
    for n in 1..=1_000_000 {
        let len = collatz_sequence_len(n);
        if len > best_len {
            (best_start_num, best_len) = (n, len)
        }
    }
    println!("collatz_sequence_len({best_start_num}) = {best_len}")
}

fn collatz_sequence_len(staring_number: usize) -> usize {
    let mut n = staring_number;
    let mut len = 1;
    while n > 1 {
        n = match n % 2 == 0 {
            true => n / 2,
            false => 3 * n + 1,
        };
        len += 1;
    }
    len
}
