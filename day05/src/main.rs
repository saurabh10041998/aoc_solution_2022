use std::collections::VecDeque;

#[derive(Debug)]
pub struct Move {
    qty: i32,
    from: i32,
    to: i32,
}

fn create_stacks(matrix_str: String) -> Vec<VecDeque<char>> {
    let mut ans: Vec<VecDeque<char>> = vec![];
    let mut lines: Vec<Vec<u8>> = vec![];

    for line in matrix_str.lines() {
        let mut line_vec = vec![0x0];
        line_vec.extend_from_slice(line.as_bytes());
        line_vec.push(0x20);

        lines.push(line_vec);
    }
    lines.pop();

    for j in 1..10 {
        let mut cols = VecDeque::new();
        for i in 0..lines.len() {
            let (_c1, c2, _c3, _c4) = (
                lines[i][4 * j - 3],
                lines[i][4 * j - 2],
                lines[i][4 * j - 1],
                lines[i][4 * j],
            );
            if c2 != 0x20 {
                cols.push_back(c2 as char);
            }
        }
        ans.push(cols);
    }
    ans
}

impl From<&str> for Move {
    fn from(move_: &str) -> Self {
        let parts = move_.split_ascii_whitespace().collect::<Vec<&str>>();
        Move {
            qty: parts[1].parse::<i32>().unwrap(),
            from: parts[3].parse::<i32>().unwrap(),
            to: parts[5].parse::<i32>().unwrap(),
        }
    }
}

fn main() {
    let data = include_str!("./input.txt");
    let mut matrix_string = String::new();
    for line in data.lines() {
        if !line.contains("move") {
            // matrix collection
            if line.len() != 0 {
                matrix_string.push_str(line);
                matrix_string.push('\n');
            }
        } else {
            break;
        }
    }
    let mut stacks = create_stacks(matrix_string);

    let mut stacks1 = stacks.clone();

    // Part 1
    for line in data.lines() {
        if line.contains("move") {
            let m: Move = line.into();
            let (mut qty, mut from, mut to) = (m.qty, m.from, m.to);
            from -= 1;
            to -= 1;
            while qty > 0 {
                let x = stacks[from as usize].pop_front().unwrap();
                stacks[to as usize].push_front(x);
                qty -= 1;
            }
        }
    }
    let mut ans = String::new();
    for c in stacks.iter() {
        if c.len() > 0 {
            ans += &c.front().unwrap().to_string();
        }
    }

    // Part2
    for line in data.lines() {
        if line.contains("move") {
            let m: Move = line.into();
            let (mut qty, mut from, mut to) = (m.qty, m.from, m.to);
            from -= 1;
            to -= 1;
            let mut tmp = VecDeque::new();
            while qty > 0 {
                let x = stacks1[from as usize].pop_front().unwrap();
                tmp.push_back(x);
                qty -= 1;
            }

            while tmp.len() > 0 {
                stacks1[to as usize].push_front(tmp.pop_back().unwrap());
            }
        }
    }

    let mut ans2 = String::new();
    for c in stacks1 {
        if c.len() > 0 {
            ans2 += &c.front().unwrap().to_string();
        }
    }

    println!("Part 1: {}", ans);
    println!("Part 2: {}", ans2);
}
