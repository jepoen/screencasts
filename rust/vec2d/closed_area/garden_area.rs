//! Read, display and eval 2D array

mod simple_input;
use simple_input::Input;

enum Cell {
    Free,
    Wall,
    Water,
    Unknown,
}

fn main() {
    let mut garden = read_array();
    show_garden(&garden);
    flood(&mut garden);
    println!("Flooded:");
    show_garden(&garden);
    println!("Garden area: {}", free_area(&garden));
}

fn read_array() -> Vec<Vec<Cell>> {
    let mut inp = Input::new();
    let n_rows = inp.read_int();
    let n_cols = inp.read_int();
    let mut array = Vec::new();
    for _r in 0..n_rows {
        let mut row_cells: Vec<Cell> = Vec::new();
        for _c in 0..n_cols {
            let val = inp.read_int();
            row_cells.push(int_to_cell(val));
        }
        array.push(row_cells);
    }
    array
}

fn int_to_cell(val: i32) -> Cell {
    match val {
        0 => Cell::Free,
        1 => Cell::Wall,
        _ => Cell::Unknown,
    }
}

fn show_garden(garden: &[Vec<Cell>]) {
    // Cells
    for row in garden {
        for cell in row {
            match cell {
                Cell::Free => print!("·"),
                Cell::Wall => print!("⋄"), // \u{25a0}
                Cell::Water => print!("≈"),
                Cell::Unknown => print!("?"),
            }
        }
        println!();
    }
}

fn flood(garden: &mut [Vec<Cell>]) {
    // TODO search start cell, multiple searches necessary
    if let Cell::Free = garden[0][0] {
        // cont.
    } else {
        return;
    }
    garden[0][0] = Cell::Water;
    let (n_rows, n_cols) = dim(garden);
    let mut changed = true;
    while changed {
        changed = false;
        for r in 0..n_rows {
            for c in 0..n_cols {
                if let Cell::Water = garden[r][c] {
                    if flood_neighbors(garden, r, c) {
                        changed = true;
                    }
                }
            }
        }
    }
}

fn flood_neighbors(garden: &mut [Vec<Cell>], r: usize, c: usize) -> bool {
    let (n_rows, n_cols) = dim(garden);
    let mut changed = false;
    for dir in 0..4 {
        if let Some((r_neighbor, c_neighbor)) = neighbor(n_rows, n_cols, r, c, dir) {
            if let Cell::Free = garden[r_neighbor][c_neighbor] {
                garden[r_neighbor][c_neighbor] = Cell::Water;
                changed = true;
            }
        }
    }
    changed
}

fn free_area(garden: &[Vec<Cell>]) -> usize {
    let mut count = 0;
    for row in garden {
        for cell in row {
            if let Cell::Free = cell {
                count += 1;
            }
        }
    }
    count
}

fn dim(garden: &[Vec<Cell>]) -> (usize, usize) {
    let n_rows = garden.len();
    let n_cols = if n_rows > 0 { garden[0].len() } else { 0 };
    (n_rows, n_cols)
}

fn neighbor(n_rows: usize, n_cols: usize, r: usize, c: usize, dir: i32) -> Option<(usize, usize)> {
    match dir {
        0 => {
            if r > 0 {
                Some((r - 1, c))
            } else {
                None
            }
        } // N
        1 => {
            if c < n_cols - 1 {
                Some((r, c + 1))
            } else {
                None
            }
        } // E
        2 => {
            if r < n_rows - 1 {
                Some((r + 1, c))
            } else {
                None
            }
        } // S
        3 => {
            if c > 0 {
                Some((r, c - 1))
            } else {
                None
            }
        } // W
        _ => None,
    }
}
