//! Read, display and eval 2D array

mod simple_input;
use simple_input::Input;

enum Cell {
    Free,
    Wall,
    Unknown,
}

fn main() {
    let garden = read_array();
    show_garden(&garden);
    let size = area(&garden);
    let count = count_free(&garden);
    println!(
        "Area: {}, free: {} ratio: {}%",
        size,
        count,
        count * 100 / size,
    );
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
                Cell::Unknown => print!("?"),
            }
        }
        println!();
    }
}

fn area(garden: &[Vec<Cell>]) -> usize {
    let (n_rows, n_cols) = dim(garden);
    n_rows * n_cols
}

fn dim(garden: &[Vec<Cell>]) -> (usize, usize) {
    let n_rows = garden.len();
    let n_cols = if n_rows > 0 { garden[0].len() } else { 0 };
    (n_rows, n_cols)
}

fn count_free(garden: &[Vec<Cell>]) -> usize {
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
