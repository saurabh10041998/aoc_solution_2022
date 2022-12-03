use std::collections::HashSet;

fn get_priority(c: char) -> i64 {
    match c {
        'a'..='z' => (c as u8 - 0x61 + 1) as i64,
        'A'..='Z' => (c as u8 - 0x41 + 27) as i64,
        _ => {
            panic!("Invalid char {}", c);
        }
    }
}

fn find_intersection(hs1: HashSet<char>, hs2: HashSet<char>, hs3: HashSet<char>) -> HashSet<char> {
    let mut hs = HashSet::new();
    for c in hs1.intersection(&hs2) {
        hs.insert(*c);
    }
    let mut hs2 = HashSet::new();
    for c in hs.intersection(&hs3) {
        hs2.insert(*c);
    }
    hs2
}

fn main() {
    let data = include_str!("./input.txt");

    // Part 1;
    let mut total_priority = 0;
    for line in data.lines() {
        let n = line.len();
        let p1 = &line[0..n / 2];
        let p2 = &line[n / 2..];
        assert_eq!(p1.len(), n / 2);
        assert_eq!(p2.len(), n / 2);
        let hs1 = p1.chars().collect::<HashSet<char>>();
        let hs2 = p2.chars().collect::<HashSet<char>>();

        let hs = hs1.intersection(&hs2).collect::<HashSet<_>>();
        total_priority += get_priority(*hs.into_iter().next().unwrap());
    }

    // Part 2
    let mut ans2 = 0;
    let mut window: Vec<&str> = vec![];
    for (i, l) in data.lines().enumerate() {
        if i % 3 == 0 {
            if window.len() == 3 {
                let hs1 = window[0].chars().collect::<HashSet<char>>();
                let hs2 = window[1].chars().collect::<HashSet<char>>();
                let hs3 = window[2].chars().collect::<HashSet<char>>();
                let hs = find_intersection(hs1, hs2, hs3);
                ans2 += get_priority(*hs.iter().next().unwrap());
            }
            window.clear();
        }
        window.push(l);
    }

    assert_eq!(window.len(), 3);
    let hs1 = window[0].chars().collect::<HashSet<char>>();
    let hs2 = window[1].chars().collect::<HashSet<char>>();
    let hs3 = window[2].chars().collect::<HashSet<char>>();
    let hs = find_intersection(hs1, hs2, hs3);
    ans2 += get_priority(*hs.iter().next().unwrap());

    println!("Part1: {}", total_priority);
    println!("Part2: {}", ans2);
}
