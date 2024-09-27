/// Advent of Code 2020 Day 10 Part 1
use std::io;

fn main() {
    let mut adapters = Vec::new();
    adapters.push(0);
    for line in io::stdin().lines() {
        let val: u32 = line.unwrap().parse().unwrap();
        print!(" {}", val);
        adapters.push(val);
    }
    println!();
    // missing: charger
    let val_max = adapters.iter().max().unwrap();
    println!("max {val_max}");
    adapters.push(val_max + 3);
    adapters.sort();
    println!("{:?}", adapters);
    let (diff1, diff3) = count_diffs(&adapters);
    println!("d1: {} d3: {} prod: {}", diff1, diff3, diff1 * diff3,);
}

fn count_diffs(adapters: &[u32]) -> (u32, u32) {
    let (mut count_d1, mut count_d3) = (0, 0);
    for idx in 1..adapters.len() {
        match adapters[idx] - adapters[idx - 1] {
            1 => count_d1 += 1,
            2 => (),
            3 => count_d3 += 1,
            _ => panic!("should not happen!"),
        }
    }
    (count_d1, count_d3)
}
