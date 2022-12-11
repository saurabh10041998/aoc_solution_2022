use std::collections::HashSet;

fn check_signal_strength(cycle_counter: i32, stops: &HashSet<i32>, current_val: i32,  sum: &mut i64) {
    if stops.contains(&cycle_counter) {
        *sum += cycle_counter as i64 * current_val as i64;
    }
}

fn draw_cycles(cycle_counter: i32, current_val: i32) { 
    let current_pos = cycle_counter % 40;
    if current_pos < current_val || current_pos > current_val + 2 {
        print!(".")
    }else {
        print!("#");
    }

    if current_pos == 0 { 
        println!();
    }
}

fn main() {
    let mut stops = HashSet::new();
    stops.insert(20);
    stops.insert(60);
    stops.insert(100);
    stops.insert(140);
    stops.insert(180);
    stops.insert(220);

    //let data = include_str!("./example.txt");
    let data = include_str!("./input.txt");

    let mut cycle_counter  = 0;
    let mut sum = 0;
    let mut current_val = 1;
    for line in data.lines() {
        let parts =  line.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        if parts[0] == "noop" { 
            cycle_counter += 1;
            check_signal_strength(cycle_counter, &stops, current_val, &mut sum);
            draw_cycles(cycle_counter, current_val);
        }else {
            cycle_counter += 1;
            check_signal_strength(cycle_counter, &stops, current_val, &mut sum);
            draw_cycles(cycle_counter, current_val);
            cycle_counter += 1;
            check_signal_strength(cycle_counter, &stops, current_val, &mut sum);
            draw_cycles(cycle_counter, current_val);
            current_val += parts[1].parse::<i32>().unwrap();
        }
    }

    println!("Part1 - {}", sum);
    
}
