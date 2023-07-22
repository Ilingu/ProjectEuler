use std::fs;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let raw_names = fs::read_to_string("./src/names.txt").unwrap();
    let mut names = raw_names
        .split(',')
        .map(|n| n.trim_matches('"').to_lowercase())
        .collect::<Vec<_>>();
    names.sort();
    let result = names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            (i + 1)
                * name
                    .chars()
                    .map(|c| ALPHABET.chars().position(|c_alpha| c_alpha == c).unwrap() + 1)
                    .sum::<usize>()
        })
        .sum::<usize>();
    println!("{result}")
}
