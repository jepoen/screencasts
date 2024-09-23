use std::io;
use std::collections::HashSet;

#[derive(Debug)]
enum Op {
	Nop(i32),
	Acc(i32),
	Jmp(i32),
}

fn main() {
	let mut program: Vec<Op> = Vec::new();
	for line in io::stdin().lines() {
		program.push(parse_line(&line.unwrap()));
	}
	println!("{:?}", program);
	for idx in 0 .. program.len() {
		switch_op(&mut program, idx);
		if let Some(accu) = process(&program) {
			println!("idx: {} accu: {}", idx, accu);
			break;
		}
		switch_op(&mut program, idx);
	}
}

fn parse_line(line: &str) -> Op {
	let mut it = line.split_whitespace();
	let op_str = it.next().unwrap();
	let val: i32 = it.next().unwrap().parse().unwrap();
	match op_str {
		"nop" => Op::Nop(val),
		"acc" => Op::Acc(val),
		"jmp" => Op::Jmp(val),
		_ => panic!("unknown op"),
	}
}

fn process(program: &[Op]) -> Option<i32> {
	// CPU
	let mut ac: i32 = 0;
	let mut pc: usize = 0;
	let mut visited: HashSet<usize> = HashSet::new();
	// Alternative: bool-Vektor der Größe program.len()
	loop {
		if pc == program.len() {
			return Some(ac);
		}
		// Debug
		println!("pc: {} ac: {}, op: {:?}", pc, ac, program[pc]);
		if visited.contains(&pc) {
			return None;
		}
		visited.insert(pc);
		match program[pc] {
			Op::Nop(_) => pc += 1,
			Op::Acc(val) => {
				ac += val;
				pc += 1;
			},
			Op::Jmp(val) => {
				if val >= 0 {
					pc += val as usize;
				} else {
					// TODO: -val > pc? ok: panic
					pc -= (-val) as usize;
				}
			},
		}
	}
}

fn switch_op(program: &mut [Op], idx: usize) {
	match program[idx] {
		Op::Nop(val) => program[idx] = Op::Jmp(val),
		Op::Jmp(val) => program[idx] = Op::Nop(val),
		_ => (),
	}
}
