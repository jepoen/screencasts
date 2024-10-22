//! Advent of Code 2020/13/1

use std::io;

#[derive(Debug)]
struct Bus {
    nr: i64,
    _offset: i64,
    miss: i64,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let busses = parse_line(&buffer);
    println!("{:?}", busses);
    let t = search_t(&busses);
    println!("{t}");
}

fn parse_line(buffer: &str) -> Vec<Bus> {
    let mut busses = Vec::new();
    let mut offset = 0;
    for part in buffer.trim_end().split(",") {
        if part != "x" {
            let val = part.parse().unwrap();
            busses.push(Bus {
                nr: val,
                _offset: offset,
                miss: get_miss(val, offset),
            });
        }
        offset += 1;
    }
    busses
}

fn get_miss(bus: i64, mut offset: i64) -> i64 {
    while offset > 0 {
        offset -= bus;
    }
    -offset
}

fn search_t(busses: &[Bus]) -> i64 {
    let ll = busses.len();
    let mut delta = busses[0].nr;
    let mut t = 0;
    for idx in 1..ll {
        while t % busses[idx].nr != busses[idx].miss {
            t += delta;
        }
        println!("idx {idx} nr {} delta {delta} t {t}", busses[idx].nr);
        delta *= busses[idx].nr;
    }
    t
}
