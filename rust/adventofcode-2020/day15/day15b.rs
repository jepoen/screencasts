use std::collections::HashMap;
/// Advent of Code 2020 day 15
use std::io;

const DURATION: usize = 30000000;

fn main() {
    let start_vals = read_input();
    println!("{:?}", start_vals);
    let mut prev_pos: HashMap<usize, usize> = HashMap::new();
    for pos in 0..start_vals.len() - 1 {
        prev_pos.insert(start_vals[pos], pos);
    }
    let mut pos = start_vals.len() - 1;
    let mut prev_val = start_vals[pos];
    while pos < DURATION - 1 {
        let new_val = if prev_pos.contains_key(&prev_val) {
            pos - prev_pos.get(&prev_val).unwrap()
        } else {
            0
        };
        prev_pos.insert(prev_val, pos);
        pos += 1;
        prev_val = new_val;
        //println!("pos {} value {}", pos, prev_val);
    }
    println!("result {}", prev_val);
}

fn read_input() -> Vec<usize> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    // let res: Vec<usize> =
    buffer
        .trim()
        .split(",")
        .map(|e| e.parse().unwrap())
        .collect()
}
