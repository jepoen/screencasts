use std::io;

fn main() {
    let mut buffer = String::new();
    let mut id_min = 1024;
    let mut id_max = 0;
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let (row, col) = parse_line(&buffer);
        let id = 8 * row + col;
        if id > id_max {
            id_max = id;
        }
        if id < id_min {
            id_min = id;
        }
        println!("row {} col {} id {}", row, col, id);
        buffer.clear();
    }
    println!("min: {} max: {}", id_min, id_max);
}

fn parse_line(buffer: &str) -> (u32, u32) {
    (get_num(&buffer[..7], 'F', 'B'), get_num(&buffer[7..10], 'L', 'R'))
}

fn get_num(s: &str, zero: char, one: char) -> u32 {
    let mut accu = 0;
    for c in s.chars() {
        accu = 2 * accu + if c == zero {
            0
        } else if c == one {
            1
        } else {
            99999
        };
    }
    accu
}