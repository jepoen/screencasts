use std::io;

fn main() {
    let mut buffer = String::new();
    let mut occupied_seats = vec![false; 1024];
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let (row, col) = parse_line(&buffer);
        let id = (8 * row + col) as usize;
        occupied_seats[id] = true;
        println!("row {} col {} id {}", row, col, id);
        buffer.clear();
    }
    println!("{:?}", occupied_seats);
    let free_id = get_free_id(&occupied_seats);
    println!("our place has id {} row {} col {}",
        free_id, free_id / 8, free_id % 8);
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

fn get_free_id(occupied_seats: &[bool]) -> usize {
    for id in 1..1023 {
        if occupied_seats[id-1] && occupied_seats[id+1] &&
                !occupied_seats[id] {
            return id
        }
    }
    99999
}