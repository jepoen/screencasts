/// Advent of Code 2020 Day 10 Part 2
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
    let variants = count_variants(&adapters);
    println!("variants {}", variants);
}

fn count_variants(adapters: &[u32]) -> u64 {
    let mut variants: Vec<u64> = vec![0; adapters.len()];
    variants[0] = 1;
    for idx in 1..adapters.len() {
        let mut m = idx;
        while m > 0 && adapters[idx] - adapters[m - 1] <= 3 {
            variants[idx] += variants[m - 1];
            m -= 1;
        }
    }
    println!("{:?}", variants);
    variants[variants.len() - 1]
}
