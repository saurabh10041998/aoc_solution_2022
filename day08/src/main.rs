pub struct Cacher<T>
where
    T: Fn(i32, i32) -> bool,
{
    calculation: T,
}

impl<T> Cacher<T>
where
    T: Fn(i32, i32) -> bool,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation }
    }
    fn value(&self, args1: i32, args2: i32) -> bool {
        (self.calculation)(args1, args2)
    }
}

// TODO : Find the optimal way to achieve this

pub struct Cacher2<T>
where
    T: Fn(i32, i32) -> i64,
{
    calculation: T,
}

impl<T> Cacher2<T>
where
    T: Fn(i32, i32) -> i64,
{
    fn new(calculation: T) -> Cacher2<T> {
        Cacher2 { calculation }
    }

    fn value(&self, args1: i32, args2: i32) -> i64 {
        (self.calculation)(args1, args2)
    }
}

fn main() {
    let data = include_str!("./input.txt");
    //let data = include_str!("./example.txt");
    let mut grid = vec![];

    for line in data.lines() {
        let d = line
            .trim()
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        grid.push(d);
    }

    let num_rows = grid.len() as i32;
    let num_cols = grid[0].len() as i32;

    // ******************* Part 1 closure *************************
    let is_visible = Cacher::new(|r, c| {
        if r == 0 || r == num_rows - 1 || c == 0 || c == num_cols - 1 {
            return true;
        }
        // look left
        let mut flag = true;
        for cc in 0..c {
            if grid[r as usize][cc as usize] >= grid[r as usize][c as usize] {
                flag = false;
                break;
            }
        }
        if flag {
            return true;
        }
        flag = true;
        // look right
        for cc in c + 1..num_cols {
            if grid[r as usize][cc as usize] >= grid[r as usize][c as usize] {
                flag = false;
                break;
            }
        }
        if flag {
            return true;
        }
        flag = true;
        // look up
        for rr in 0..r {
            if grid[rr as usize][c as usize] >= grid[r as usize][c as usize] {
                flag = false;
                break;
            }
        }
        if flag {
            return true;
        }
        flag = true;
        // look down
        for rr in r + 1..num_rows {
            if grid[rr as usize][c as usize] >= grid[r as usize][c as usize] {
                flag = false;
                break;
            }
        }
        flag
    });

    // *******************************************************************
    // ****************** Part 2 clousre *********************************
    let scenic_score = Cacher2::new(|r, c| {
        if r == 0 || r == num_rows - 1 || c == 0 || c == num_cols - 1 {
            return 0;
        }
        // count left
        let mut left = 0;
        let mut cc = c - 1;
        while cc >= 0 {
            if grid[r as usize][cc as usize] >= grid[r as usize][c as usize] {
                break;
            }
            left += 1;
            cc -= 1;
        }
        if cc != -1 {
            left += 1;
        }
        // count right
        let mut right = 0;
        let mut cc = c + 1;
        while cc < num_cols {
            if grid[r as usize][cc as usize] >= grid[r as usize][c as usize] {
                break;
            }
            right += 1;
            cc += 1;
        }

        if cc != num_cols {
            right += 1;
        }

        // count up
        let mut up = 0;
        let mut rr = r - 1;
        while rr >= 0 {
            if grid[rr as usize][c as usize] >= grid[r as usize][c as usize] {
                break;
            }
            up += 1;
            rr -= 1;
        }

        if rr != -1 {
            up += 1;
        }

        // count down
        let mut down = 0;
        let mut rr = r + 1;
        while rr < num_rows {
            if grid[rr as usize][c as usize] >= grid[r as usize][c as usize] {
                break;
            }
            down += 1;
            rr += 1;
        }
        if rr != num_rows {
            down += 1;
        }

        left as i64 * right as i64 * up as i64 * down as i64
    });

    // *******************************************************************
    let mut ans1 = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if is_visible.value(i, j) {
                ans1 += 1;
            }
        }
    }

    // Part 2
    let mut ans2: i64 = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            ans2 = i64::max(ans2, scenic_score.value(i, j));
        }
    }

    println!("Part1 : {:?}", ans1);
    println!("Part2 : {}", ans2);
}
