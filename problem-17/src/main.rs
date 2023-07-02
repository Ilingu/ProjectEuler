use problem_17::number_to_word;

fn main() {
    let result = (1..=1000).map(|n| number_to_word(n).len()).sum::<usize>();
    println!("{result}");
}
