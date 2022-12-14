use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Solver {
    grid: HashMap<(i32, i32), i32>,
    target: (i32, i32),
    queue: VecDeque<(i32, i32, i32)>,
}

impl Solver {
    fn new() -> Self {
        Solver {
            grid: HashMap::new(),
            queue: VecDeque::new(),
            target: (0, 0),
        }
    }

    fn create_grid(&mut self, data: &str) {
        for (r, line) in data.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch == 'E' {
                    self.target = (r as i32, c as i32);
                    self.grid.insert((r as i32, c as i32), 25);
                } else if ch == 'S' {
                    self.grid.insert((r as i32, c as i32), 0);
                    self.queue.push_back((r as i32, c as i32, 0));
                } else {
                    self.grid
                        .insert((r as i32, c as i32), ((ch as u8) - ('a' as u8)) as i32);
                }
            }
        }
    }
    fn create_grid_for_part2(&mut self, data: &str) {
        for (r, line) in data.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch == 'E' {
                    self.target = (r as i32, c as i32);
                    self.grid.insert((r as i32, c as i32), 25);
                } else if ch == 'S' {
                    self.grid.insert((r as i32, c as i32), 0);
                    self.queue.push_back((r as i32, c as i32, 0));
                } else if ch == 'a' {
                    self.grid.insert((r as i32, c as i32), 0);
                    self.queue.push_back((r as i32, c as i32, 0));
                } else {
                    self.grid
                        .insert((r as i32, c as i32), ((ch as u8) - ('a' as u8)) as i32);
                }
            }
        }
    }

    fn get_elevation(&self, i: i32, j: i32) -> i32 {
        match self.grid.get(&(i, j)) {
            Some(v) => *v,
            None => 30,
        }
    }

    fn solve(&mut self) -> i32 {
        let mut visited = HashSet::new();
        while let Some((r, c, d)) = self.queue.pop_front() {
            if self.target == (r, c) {
                return d;
            }
            if visited.contains(&(r, c)) {
                continue;
            }
            visited.insert((r, c));
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if self.get_elevation(r + dr, c + dc) <= self.get_elevation(r, c) + 1 {
                    self.queue.push_back((r + dr, c + dc, d + 1));
                }
            }
        }
        unreachable!();
    }
}

fn main() {
    //let data = include_str!("./example.txt");
    let data = include_str!("./input.txt");
    let mut solver = Solver::new();
    solver.create_grid(data);
    println!("Part - 1: {}", solver.solve());
    // part-2
    let mut solver2 = Solver::new();
    solver2.create_grid_for_part2(data);
    println!("Part - 2: {}", solver2.solve());
}
