use std::io;

fn main() {
	let mut buffer = String::new();
	let mut board = Vec::new();
	while io::stdin().read_line(&mut buffer).unwrap() > 0 {
		board.push(parse_line(&buffer));
		buffer.clear();
	}
	show_board(&board);
	// Order in direction pair: (down, right) == (d_row, d_col)
	let directions = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
	let mut prod: u64 = 1;
	for dir in directions {
		let (d_row, d_col) = dir;
		let n_trees = count_trees(&board, d_row, d_col);
		println!("Number of Trees {} in direction {:?}", n_trees, dir);
		prod *= n_trees;
	}
	println!("Result: {}", prod);
}

fn parse_line(buffer: &str) -> Vec<bool> {
	let mut res = Vec::new();
	for c in buffer.chars() {
		match c {
			'.' => {res.push(false)},
			'#' => {res.push(true)},
			_ => (),
		}
	}
	res
}

fn show_board(board: &Vec<Vec<bool>>) {
	println!("rows: {} cols: {}", board.len(), board[0].len());
	for row in board {
		for entry in row {
			if *entry {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!();
	}
}

fn count_trees(board: &Vec<Vec<bool>>, d_row: usize, d_col: usize) -> u64 {
	let (mut row, mut col) = (0, 0);
	let n_rows = board.len();
	let mut count = 0;
	loop {
		row += d_row;
		col += d_col;
		if row >= n_rows {
			return count;
		}
		if is_tree(board, row, col) {
			count += 1
		}
	}
}

fn is_tree(board: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
	let n_rows = board.len();
	let n_cols = board[0].len();
	board[row % n_rows][col % n_cols]
}
