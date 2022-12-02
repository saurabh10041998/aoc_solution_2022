// for speed run: cargo run --release
fn main() {
    let data = include_str!("./input.txt");
    let mut calories = vec![];
    let mut sum = 0;
    for s in data.lines() {
        if s.len() == 0 {
            calories.push(sum);
            sum = 0;
        }else{
            let x = s.parse::<i64>().expect("A number");
            sum += x;
        }
    }
    calories.sort_by(|a,b| b.cmp(a));
    println!("Part 1: {:?}", calories[0]);
    println!("Part 2: {}", calories[0] + calories[1] + calories[2]);
}
