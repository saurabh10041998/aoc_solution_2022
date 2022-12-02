use std::collections::HashMap;

fn main() {
    let data = include_str!("./input.txt");
    let win_map = HashMap::from([
        ("X", "C"),
        ("Y", "A"),
        ("Z", "B"),
    ]);

    let win_map_inv = HashMap::from([
        ("C", "X"),
        ("A", "Y"),
        ("B", "Z"),
    ]);

    let draw_map = HashMap::from([
        ("X", "A"),
        ("Y", "B"),
        ("Z", "C"),
    ]);

    let draw_map_inv = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z"),
    ]);

    let _lose_map = HashMap::from([
        ("X", "B"),
        ("Y", "C"),
        ("Z", "A"),
    ]);

    let lose_map_inv = HashMap::from([
        ("B", "X"),
        ("C", "Y"),
        ("A", "Z"),
    ]);


   
    // Part 1...
    let mut total_score = 0;
    for line in data.lines() {
        let mov = line.split_whitespace().collect::<Vec<&str>>(); 
        let (x, y) = (mov[0], mov[1]);
        if x == *win_map.get(&y).unwrap() {
            total_score += 6;
        }else if x == *draw_map.get(&y).unwrap() {
            total_score += 3;
        }
        match y { 
            "X" => total_score += 1,
            "Y" => total_score += 2,
            "Z" => total_score += 3,
            _ => panic!("Not valid move !!")
        }
    }

    // Part 2..
    let mut total_score_2 = 0;
    for line in data.lines() {
        let mov = line.split_whitespace().collect::<Vec<&str>>(); 
        let (x, y) = (mov[0], mov[1]);
        match y { 
            "X" => {
                // loose
                total_score_2 += 0;
                let choice = *lose_map_inv.get(&x).unwrap();
                match choice {
                    "X" => total_score_2 += 1,
                    "Y" => total_score_2 += 2,
                    "Z" => total_score_2 += 3,
                    _ => panic!("Not valid move !!")
                }
            },
            "Y" => {
                // draw
                total_score_2 += 3;
                let choice = *draw_map_inv.get(&x).unwrap();
                match choice {
                    "X" => total_score_2 += 1,
                    "Y" => total_score_2 += 2,
                    "Z" => total_score_2 += 3,
                    _ => panic!("Not valid move !!")
                }

            },
            "Z" => {
                // win
                total_score_2 += 6;
                let choice = *win_map_inv.get(&x).unwrap();
                match choice {
                    "X" => total_score_2 += 1,
                    "Y" => total_score_2 += 2,
                    "Z" => total_score_2 += 3,
                    _ => panic!("Not valid move !!")
                }
                
            },
            _ => panic!("Invalid choice"),
        }
    }

    println!("part1: {:?}", total_score);
    println!("part2: {:?}", total_score_2);
}
