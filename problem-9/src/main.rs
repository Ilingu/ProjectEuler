use std::cmp::Ordering;

fn main() {
    {
        let (a, b, c) = find_special_pythagorean_triplet(1000).expect("No solution for this sum");

        println!("Answer for the problem 9:");
        println!("a:{a}, b:{b}, c:{c}");
        println!("Product: {}", a * b * c)
    }

    let res = (0..=1000)
        .filter_map(find_special_pythagorean_triplet)
        .collect::<Vec<_>>();
    println!("Here all the Special Pythagorean Triplet whose sum are under 1000:");
    println!("{res:?}")
}

fn find_special_pythagorean_triplet(n: usize) -> Option<(usize, usize, usize)> {
    let (mut a, mut b, mut c) = (0, 0, 0);

    loop {
        match (a + b + c).cmp(&n) {
            Ordering::Less => {
                b += 1;
                c = if hypt(a, b).fract() == 0.0 {
                    hypt(a, b) as usize
                } else {
                    continue;
                }
            }
            Ordering::Equal if is_pythagorean_triplet(a, b, c) => {
                return Some((a, b, c));
            }
            _ => {
                if a >= n {
                    return None;
                }

                a += 1;
                b = 0;
                c = 0;
            }
        }
    }
}

fn is_pythagorean_triplet(a: usize, b: usize, c: usize) -> bool {
    a < b && b < c && a * a + b * b == c * c
}

fn hypt(a: usize, b: usize) -> f64 {
    ((a * a + b * b) as f64).sqrt()
}
