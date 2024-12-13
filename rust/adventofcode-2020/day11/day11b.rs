//! day11b.rs: Advent of Code 2020, day 11 part 2
use std::io;

enum Cell {
    // alternativ: Seat{occupied: bool, update: bool}
    Empty { update: bool },
    Occupied { update: bool },
    Floor,
}

fn main() {
    let mut places = Vec::new();
    for line in io::stdin().lines() {
        places.push(parse_line(&line.unwrap()));
    }
    show_places(&places);
    while next_step(&mut places) {
        println!("next step");
        show_places(&places);
    }
    println!("occupied: {}", count_occupied_places(&places));
}

fn parse_line(line: &str) -> Vec<Cell> {
    let mut row = Vec::new();
    for c in line.trim().chars() {
        match c {
            'L' => row.push(Cell::Empty { update: false }),
            '#' => row.push(Cell::Occupied { update: false }),
            '.' => row.push(Cell::Floor),
            _ => panic!("unknown {}", c),
        }
    }
    row
}

fn show_places(places: &[Vec<Cell>]) {
    for row in places {
        for cell in row {
            let c = match cell {
                Cell::Empty { update } if *update => 'l',
                Cell::Empty { .. } => 'L',
                Cell::Occupied { update } if *update => '*',
                Cell::Occupied { .. } => '#',
                Cell::Floor => '.',
            };
            print!("{c}");
        }
        println!();
    }
}

fn next_step(places: &mut [Vec<Cell>]) -> bool {
    let n_row = places.len();
    let n_col = places[0].len();
    let mut changed = false;
    // 1st step, seach for changes
    for r in 0..n_row {
        for c in 0..n_col {
            match places[r][c] {
                Cell::Empty { .. } => {
                    if count_occupied_neighbors(places, r, c) == 0 {
                        places[r][c] = Cell::Empty { update: true };
                        changed = true;
                    }
                }
                Cell::Occupied { .. } => {
                    if count_occupied_neighbors(places, r, c) >= 5 {
                        places[r][c] = Cell::Occupied { update: true };
                        changed = true;
                    }
                }
                Cell::Floor => (),
            }
        }
    }
    println!();
    show_places(places);
    println!();
    // 2nd step, apply changes
    for r in 0..n_row {
        for c in 0..n_col {
            match places[r][c] {
                Cell::Empty { update } if update => {
                    places[r][c] = Cell::Occupied { update: false };
                }
                Cell::Occupied { update } if update => {
                    places[r][c] = Cell::Empty { update: false };
                }
                _ => (),
            }
        }
    }
    changed
}

fn count_occupied_neighbors(places: &[Vec<Cell>], row: usize, col: usize) -> u32 {
    // dir_occupied(dr, dc) dr in -1 .. 1, dc in -1 .. 1, Ausschluss 0, 0
    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr != 0 || dc != 0 {
                count += if dir_occupied(places, row, col, dr, dc) {
                    1
                } else {
                    0
                }
            }
        }
    }
    count
}

fn dir_occupied(places: &[Vec<Cell>], row: usize, col: usize, dr: isize, dc: isize) -> bool {
    let n_row = places.len() as isize;
    let n_col = places[0].len() as isize;

    let mut r = row as isize;
    let mut c = col as isize;
    r += dr;
    c += dc;
    while r >= 0 && c >= 0 && r < n_row && c < n_col {
        match places[r as usize][c as usize] {
            Cell::Occupied { .. } => return true,
            Cell::Empty { .. } => return false,
            Cell::Floor => (),
        }
        r += dr;
        c += dc;
    }
    false
}

fn count_occupied_places(places: &[Vec<Cell>]) -> u32 {
    let mut count = 0;
    for row in places {
        for cell in row {
            if let Cell::Occupied { .. } = cell {
                count += 1;
            }
        }
    }
    count
}
