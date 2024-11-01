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
    println!("OK: {}", check_garden(&garden));
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

fn dim(garden: &[Vec<Cell>]) -> (usize, usize) {
    let n_rows = garden.len();
    let n_cols = if n_rows > 0 { garden[0].len() } else { 0 };
    (n_rows, n_cols)
}

fn check_garden(garden: &[Vec<Cell>]) -> bool {
    // search start pos.
    let (r0, c0) = find_start(garden);
    // TODO error handling: no wall found
    println!("Start: {}, {}", r0, c0);
    let (n_rows, n_cols) = dim(garden);
    let mut visited = vec![vec![false; n_cols]; n_rows];
    // set walking start position
    let (mut r, mut c) = (r0, c0);
    while !visited[r][c] {
        // debug
        print!(" ({},{})", r, c);
        visited[r][c] = true;
        if let Some((r_new, c_new)) = get_next(garden, &visited, r, c) {
            (r, c) = (r_new, c_new);
        } else {
            break;
        }
    }
    // path complete
    // check if closed: Manhattan-Distance (r,c) - (r0, c0) == 1
    let dist = r0.abs_diff(r) + c0.abs_diff(c);
    println!("Manhattan-Distance: {}", dist);
    // return value
    dist == 1 && all_visited(garden, &visited)
}

fn find_start(garden: &[Vec<Cell>]) -> (usize, usize) {
    let (n_rows, n_cols) = dim(garden);
    for r in 0..n_rows {
        for c in 0..n_cols {
            if let Cell::Wall = garden[r][c] {
                return (r, c);
            }
        }
    }
    // TODO: use Option
    (n_rows, n_cols)
}

fn get_next(
    garden: &[Vec<Cell>],
    visited: &[Vec<bool>],
    r: usize,
    c: usize,
) -> Option<(usize, usize)> {
    let (n_rows, n_cols) = dim(garden);
    // loop over 4 directions
    for dir in 0..4 {
        if let Some((r_new, c_new)) = neighbor(n_rows, n_cols, r, c, dir) {
            if let Cell::Wall = garden[r_new][c_new] {
                if !visited[r_new][c_new] {
                    return Some((r_new, c_new));
                }
            }
        }
    }
    None
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

fn all_visited(garden: &[Vec<Cell>], visited: &[Vec<bool>]) -> bool {
    let (n_rows, n_cols) = dim(garden);
    for r in 0..n_rows {
        for c in 0..n_cols {
            if let Cell::Wall = garden[r][c] {
                if !visited[r][c] {
                    println!("not visited: {}, {}", r, c);
                    return false;
                }
            }
        }
    }
    true
}
