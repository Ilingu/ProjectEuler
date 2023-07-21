use std::collections::HashMap;

fn compute_max_path(triangle_str: &str) -> usize {
    let mut triangle = HashMap::new();

    for (lid, line) in triangle_str.lines().enumerate() {
        for (nid, num) in line.split_whitespace().enumerate() {
            triangle.insert((nid, lid), num.parse::<usize>().unwrap());
        }
    }

    let mut max = 0;
    let mut queue = vec![((0_usize, 0_usize), triangle[&(0, 0)])];
    'outer: while !queue.is_empty() {
        let ((x, y), num) = queue.remove(0);
        max = max.max(num);

        let mut children = Vec::with_capacity(2);
        for next_pos in [(x, y + 1), (x + 1, y + 1)] {
            let c = match triangle.get(&next_pos) {
                Some(d) => d,
                None => continue 'outer,
            };
            children.push((next_pos, *c + num));
        }
        queue.append(&mut children);
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::compute_max_path;

    const INPUT: &str = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

    const BASE_INPUT: &str = "3
7 4
2 4 6
8 5 9 3";

    #[test]
    fn base_input_test() {
        assert_eq!(compute_max_path(BASE_INPUT), 23);
    }

    #[test]
    fn real_input_test() {
        assert_eq!(compute_max_path(INPUT), 1074);
    }
}
