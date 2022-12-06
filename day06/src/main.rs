use std::collections::HashSet;
use std::collections::VecDeque;

fn solve(data: &str, num_of_uniq: usize) -> usize {
    let mut idx = 0;
    for line in data.lines() {
        let mut data = VecDeque::new();
        for (i, c) in line.chars().enumerate() {
            if data.len() == num_of_uniq {
                let hs = data.iter().collect::<HashSet<&char>>();
                if hs.len() == num_of_uniq {
                    idx = i;
                    break;
                }
                data.pop_front().unwrap();
                data.push_back(c);
            } else {
                data.push_back(c);
            }
        }
    }
    idx
}

fn main() {
    let data = include_str!("./input.txt");
    let idx = solve(data, 4);
    let idx2 = solve(data, 14);

    println!("Part 1: {}", idx);
    println!("Part 2: {}", idx2);
}