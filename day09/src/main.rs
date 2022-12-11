use std::ops::Sub;

#[derive(Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

pub struct Solver {
    gridsize: i32,
    grid: Vec<char>,
    tails: Vec<Position>,
    total_tails: usize,
}

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    }
    y - x
}

impl Solver {
    fn new(gridsize: i32, initial_pos: (i32, i32), total_tails: usize) -> Self {
        let mut _s = Solver {
            gridsize,
            grid: Vec::new(),
            tails: Vec::new(),
            total_tails,
        };
        let grid_size = gridsize as usize;
        _s.grid.resize(grid_size * grid_size, '.');
        _s.tails
            .resize_with(total_tails, || Position::new(initial_pos.0, initial_pos.1));
        _s
    }

    fn get_move(&mut self, dir: &str, amount: i32) {
        for _i in 0..amount {
            self.movement(dir);
        }
    }

    fn movement(&mut self, dir: &str) {
        match dir {
            "U" => {
                self.tails[0].y -= 1;
            }
            "D" => {
                self.tails[0].y += 1;
            }
            "L" => {
                self.tails[0].x -= 1;
            }
            "R" => {
                self.tails[0].x += 1;
            }
            _ => {
                panic!("Invalid direction");
            }
        };

        for i in 0..self.total_tails - 1 {
            let move_x = abs(self.tails[i].x, self.tails[i + 1].x) > 1;
            let move_y = abs(self.tails[i].y, self.tails[i + 1].y) > 1;
            let move_x_y = abs(self.tails[i].x, self.tails[i + 1].x)
                + abs(self.tails[i].y, self.tails[i + 1].y)
                > 2;

            if move_x_y {
                if self.tails[i].x > self.tails[i + 1].x {
                    self.tails[i + 1].x += 1;
                } else {
                    self.tails[i + 1].x -= 1;
                }

                // Change y
                if self.tails[i].y > self.tails[i + 1].y {
                    self.tails[i + 1].y += 1;
                } else {
                    self.tails[i + 1].y -= 1;
                }
            } else if move_x {
                if self.tails[i].x > self.tails[i + 1].x {
                    self.tails[i + 1].x += 1;
                } else {
                    self.tails[i + 1].x -= 1;
                }
            } else if move_y {
                if self.tails[i].y > self.tails[i + 1].y {
                    self.tails[i + 1].y += 1;
                } else {
                    self.tails[i + 1].y -= 1;
                }
            }
        }

        let last_tail = self.total_tails - 1;
        self.grid[(self.tails[last_tail].y * self.gridsize + self.tails[last_tail].x) as usize] =
            '#';
    }

    #[allow(dead_code)]
    // FOR DEBUG purpose only
    fn print_grid(&self) {
        for y in 0..self.gridsize {
            for x in 0..self.gridsize {
                print!("{} ", self.grid[(y * self.gridsize + x) as usize]);
            }
            println!();
        }
    }

    fn count_ans(&self) -> i32 {
        let mut count = 0;
        for y in 0..self.gridsize {
            for x in 0..self.gridsize {
                if self.grid[(y * self.gridsize + x) as usize] == '#' {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    //let data = include_str!("./example.txt");
    //let data = include_str!("./example2.txt");
    let data = include_str!("./input.txt");

    // ****************** Part 1 *********************************
    let mut solver = Solver::new(1000, (500, 500), 2);
    for line in data.lines() {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        let dir = parts[0];
        let amount = parts[1].parse::<i32>().unwrap();
        solver.get_move(dir, amount);
    }
    //solver.print_grid();
    println!("Part-1: {}", solver.count_ans());

    // ***************** Part 2 ***********************************
    let mut solver2 = Solver::new(1000, (500, 500), 10);
    for line in data.lines() {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        let dir = parts[0];
        let amount = parts[1].parse::<i32>().unwrap();
        solver2.get_move(dir, amount);
    }

    // For Debug
    // solver2.print_grid();
    println!("Part-2: {}", solver2.count_ans());
}
