//! Read, display and eval 2D array

mod simple_input;
use simple_input::Input;

fn main() {
    let garden = read_array();
    show_array(&garden);
    let size = area(&garden);
    if let Some(min_val) = min(&garden) {
        let max_val = max(&garden).unwrap();
        for val in min_val..=max_val {
            let count = count_value(&garden, val);
            println!(
                "val: {}, count: {} ratio: {}%",
                val,
                count,
                count * 100 / size,
            );
        }
    } else {
        println!("Array is empty");
    }
}

fn read_array() -> Vec<Vec<i32>> {
    let mut inp = Input::new();
    let n_rows = inp.read_int();
    let n_cols = inp.read_int();
    let mut array = Vec::new();
    for _r in 0..n_rows {
        let mut row_cells: Vec<i32> = Vec::new();
        for _c in 0..n_cols {
            let val = inp.read_int();
            row_cells.push(val);
        }
        array.push(row_cells);
    }
    array
}

fn show_array(garden: &[Vec<i32>]) {
    let (n_rows, n_cols) = dim(garden);
    // table head
    print!("  |");
    for c in 0..n_cols {
        print!(" {:2}", c);
    }
    println!();
    print!("--+");
    for _c in 0..n_cols {
        print!("---");
    }
    println!();
    // Cells
    for r in 0..n_rows {
        print!("{:2}|", r);
        for c in 0..n_cols {
            print!(" {:2}", garden[r][c]);
        }
        println!();
    }
}

fn area(garden: &[Vec<i32>]) -> usize {
    let (n_rows, n_cols) = dim(garden);
    n_rows * n_cols
}

fn dim(garden: &[Vec<i32>]) -> (usize, usize) {
    let n_rows = garden.len();
    let n_cols = if n_rows > 0 { garden[0].len() } else { 0 };
    (n_rows, n_cols)
}

fn min(garden: &[Vec<i32>]) -> Option<i32> {
    let (n_rows, n_cols) = dim(garden);
    if n_rows == 0 || n_cols == 0 {
        return None;
    }
    let mut min_val = garden[0][0];
    for row in garden {
        for &cell in row {
            if cell < min_val {
                min_val = cell;
            }
        }
    }
    Some(min_val)
}

fn max(garden: &[Vec<i32>]) -> Option<i32> {
    let (n_rows, n_cols) = dim(garden);
    if n_rows == 0 || n_cols == 0 {
        return None;
    }
    let mut max_val = garden[0][0];
    for row in garden {
        for &cell in row {
            if cell > max_val {
                max_val = cell;
            }
        }
    }
    Some(max_val)
}

fn count_value(garden: &[Vec<i32>], target: i32) -> usize {
    let mut count = 0;
    for row in garden {
        for &cell in row {
            if cell == target {
                count += 1;
            }
        }
    }
    count
}
