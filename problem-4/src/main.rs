fn main() {
    let mut max = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            if is_palindromic(i * j) {
                max = max.max(i * j);
            }
        }
    }
    println!("{max}");
}

fn is_palindromic(n: usize) -> bool {
    n.to_string() == n.to_string().chars().rev().collect::<String>()
}
