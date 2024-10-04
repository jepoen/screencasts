use std::io;

/// State of Ferry
#[derive(Debug)]
struct State {
    x: i32,
    y: i32,
    wx: i32,
    wy: i32,
}

impl State {
    fn new(x: i32, y: i32, wx: i32, wy: i32) -> Self {
        State { x, y, wx, wy }
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
    let mut state = State::new(0, 0, 10, 1);
    println!("{state:?}");
    for line in io::stdin().lines() {
        let op = parse_line(&line.unwrap());
        println!("{op:?}");
        state = handle_op(state, op);
        println!("{state:?}");
    }
    // Manhattan distance from (0, 0)
    let dist = state.x.abs() + state.y.abs();
    println!("dist {dist}");
}

fn parse_line(line: &str) -> Op {
    let mut it = line.chars();
    let op_text = it.next().unwrap();
    let val_text: String = it.collect();
    let val: i32 = val_text.parse().unwrap();
    Op::new(op_text, val)
}

fn handle_op(state: State, op: Op) -> State {
    match op {
        Op::F(val) => forward(state, val),
        Op::N(val) => move_wp_north(state, val),
        Op::S(val) => move_wp_south(state, val),
        Op::E(val) => move_wp_east(state, val),
        Op::W(val) => move_wp_west(state, val),
        Op::L(val) => rot_left(state, val),
        Op::R(val) => rot_right(state, val),
    }
}

fn rot_left(state: State, angle: i32) -> State {
    match angle {
        0 | 360 => state,
        90 => State::new(state.x, state.y, -state.wy, state.wx),
        180 => State::new(state.x, state.y, -state.wx, -state.wy),
        270 => State::new(state.x, state.y, state.wy, -state.wx),
        _ => panic!("invalid angle {}", angle),
    }
}

fn rot_right(state: State, val: i32) -> State {
    rot_left(state, 360 - val)
}

fn forward(state: State, val: i32) -> State {
    State::new(
        state.x + val * state.wx,
        state.y + val * state.wy,
        state.wx,
        state.wy,
    )
}

fn move_wp_north(state: State, val: i32) -> State {
    State::new(state.x, state.y, state.wx, state.wy + val)
}
fn move_wp_south(state: State, val: i32) -> State {
    State::new(state.x, state.y, state.wx, state.wy - val)
}
fn move_wp_east(state: State, val: i32) -> State {
    State::new(state.x, state.y, state.wx + val, state.wy)
}
fn move_wp_west(state: State, val: i32) -> State {
    State::new(state.x, state.y, state.wx - val, state.wy)
}
