pub struct Range {
    start: i64,
    end: i64,
}

impl From<&str> for Range {
    fn from(range: &str) -> Self {
        let r: Vec<&str> = range.split("-").collect::<Vec<&str>>();
        let r = r
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        assert_eq!(r.len(), 2);
        Range {
            start: r[0],
            end: r[1],
        }
    }
}

fn main() {
    let data = include_str!("./input.txt");

    let (mut ans1, mut all, mut non_overlap): (i64, i64, i64) = (0, 0, 0);
    // Part 1
    for line in data.lines() {
        let ranges = line.split(",").collect::<Vec<&str>>();
        let r1: Range = ranges[0].into();
        let r2: Range = ranges[1].into();

        if r1.start <= r2.start && r1.end >= r2.end {
            // r1 contains r2
            ans1 += 1;
        } else if r1.start >= r2.start && r1.end <= r2.end {
            // r2 contains r1
            ans1 += 1;
        }

        if r1.end < r2.start {
            non_overlap += 1;
        } else if r1.start > r2.end {
            non_overlap += 1;
        }
        all += 1;
    }

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", all - non_overlap);
}
