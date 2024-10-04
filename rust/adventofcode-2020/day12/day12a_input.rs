use std::io;

/// consts for direction
const EAST: i32 = 0;
const NORTH: i32 = 1;
const WEST: i32 = 2;
const SOUTH: i32 = 3;

/// State of Ferry
#[derive(Debug)]
struct State {
    x: i32,
    y: i32,
    dir: i32,
}

impl State {
    fn new(x: i32, y: i32, dir: i32) -> Self {
        State { x, y, dir }
    }
}

/// Instructions
#[derive(Debug)]
enum Op {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    R(i32),
    L(i32),
    F(i32),
}

impl Op {
    fn new(op_text: char, val: i32) -> Self {
        match op_text {
            'N' => Op::N(val),
            'S' => Op::S(val),
            'E' => Op::E(val),
            'W' => Op::W(val),
            'R' => Op::R(val),
            'L' => Op::L(val),
            'F' => Op::F(val),
            _ => panic!("unknown op {}", op_text),
        }
    }
}

fn main() {
    let mut state = State::new(0, 0, EAST);
    println!("{state:?}");
    for line in io::stdin().lines() {
        let op = parse_line(&line.unwrap());
        println!("{op:?}");
        // TODO handle op
    }
}

fn parse_line(line: &str) -> Op {
    let mut it = line.chars();
    let op_text = it.next().unwrap();
    let val_text: String = it.collect();
    let val: i32 = val_text.parse().unwrap();
    Op::new(op_text, val)
}
