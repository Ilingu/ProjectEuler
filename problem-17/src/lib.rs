use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref DIGIT_TO_WORD: HashMap<char, &'static str> = {
        let mut dtw = HashMap::new();
        dtw.insert('0', "zero");
        dtw.insert('1', "one");
        dtw.insert('2', "two");
        dtw.insert('3', "three");
        dtw.insert('4', "four");
        dtw.insert('5', "five");
        dtw.insert('6', "six");
        dtw.insert('7', "seven");
        dtw.insert('8', "eight");
        dtw.insert('9', "nine");
        dtw
    };
    static ref TEN_TO_NINETEEN_TO_WORD: HashMap<&'static str, &'static str> = {
        let mut dtw = HashMap::new();
        dtw.insert("10", "ten");
        dtw.insert("11", "eleven");
        dtw.insert("12", "twelve");
        dtw.insert("13", "thirteen");
        dtw.insert("14", "fourteen");
        dtw.insert("15", "fifteen");
        dtw.insert("16", "sixteen");
        dtw.insert("17", "seventeen");
        dtw.insert("18", "eighteen");
        dtw.insert("19", "nineteen");
        dtw
    };
    static ref TEN_TO_WORD: HashMap<char, &'static str> = {
        let mut dtw = HashMap::new();
        dtw.insert('1', "ten");
        dtw.insert('2', "twenty");
        dtw.insert('3', "thirty");
        dtw.insert('4', "forty");
        dtw.insert('5', "fifty");
        dtw.insert('6', "sixty");
        dtw.insert('7', "seventy");
        dtw.insert('8', "eighty");
        dtw.insert('9', "ninety");
        dtw
    };
}

pub fn number_to_word(n: usize) -> String {
    let mut number_words = vec![];
    let number_digits = n.to_string().chars().rev().collect::<Vec<_>>();
    for (i, &digit) in number_digits.iter().enumerate() {
        if digit == '0' {
            continue;
        }

        match i {
            0 => {
                let ten_place = number_digits.get(i + 1);
                if ten_place.is_some() && ten_place.unwrap() == &'1' {
                    continue;
                }

                let word = DIGIT_TO_WORD[&digit];
                number_words.insert(0, word);
            } // x
            1 => {
                if digit == '1' {
                    let tenth = format!("{digit}{}", number_digits[i - 1]);
                    number_words.insert(0, TEN_TO_NINETEEN_TO_WORD[tenth.as_str()]);
                    continue;
                }
                number_words.insert(0, TEN_TO_WORD[&digit]);
            } // x0
            2 => {
                let word = DIGIT_TO_WORD[&digit];
                if !number_words.is_empty() {
                    number_words.insert(0, "and");
                }
                number_words.insert(0, "hundred");
                number_words.insert(0, word);
            } // x00
            3 => {
                let word = DIGIT_TO_WORD[&digit];
                number_words.insert(0, "thousand");
                number_words.insert(0, word);
            } // x000
            _ => unreachable!("number shouldn't be >10000"),
        }
    }
    number_words.join("")
}

#[cfg(test)]
mod tests {
    use crate::number_to_word;

    #[test]
    fn number_to_word_test() {
        let tests = vec![(342, 23), (115, 20), (1000, 11), (100, 10), (10, 3)];
        for (input, expected) in tests {
            let resp = number_to_word(input);
            println!("{resp}");
            assert_eq!(resp.len(), expected)
        }
    }
}
