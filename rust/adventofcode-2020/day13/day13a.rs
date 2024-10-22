//! Advent of Code 2020/13/1

use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let timestamp: u32 = buffer.trim_end().parse().unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let busses = parse_line(&buffer);
    println!("{} {:?}", timestamp, busses);
    let (wait, bus_nr) = earliest(&busses, timestamp);
    println!("{wait} {bus_nr}");
    println!("solution {}", wait * bus_nr);
}

fn parse_line(buffer: &str) -> Vec<u32> {
    let mut busses = Vec::new();
    for part in buffer.trim_end().split(",") {
        if part != "x" {
            let val = part.parse().unwrap();
            busses.push(val);
        }
    }
    busses
}

fn earliest(busses: &[u32], timestamp: u32) -> (u32, u32) {
    let mut it = busses.iter();
    let mut bus_min = *it.next().unwrap();
    let missing = timestamp % bus_min;
    let mut wait_min = bus_min - missing;
    while let Some(bus) = it.next() {
        let missing = timestamp % *bus;
        let wait = *bus - missing;
        if wait < wait_min {
            bus_min = *bus;
            wait_min = wait;
        }
    }
    (wait_min, bus_min)
}
