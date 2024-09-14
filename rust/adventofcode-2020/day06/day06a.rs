use std::io;
use std::collections::HashSet;

enum LineType {
	Member,
	Skip,
}

fn main() {
	let mut buffer = String::new();
	let mut answers = HashSet::new();
	let mut count = 0;
	while io::stdin().read_line(&mut buffer).unwrap() > 0 {
		if let LineType::Skip = parse_line(&buffer, &mut answers) {
			// Gruppenwechel
			println!("{:?}", answers);
			count += answers.len();
			answers.clear();
		}
		buffer.clear();
	}
	// Gruppenwechsel letzte Gruppe
	println!("{:?}", answers);
	count += answers.len();
	println!("Count {count}");
}

fn parse_line(buffer: &str, answers: &mut HashSet<char>) -> LineType {
	let line = buffer.trim_end();
	if line == "" {
		//println!("---");
		return LineType::Skip;
	}
	for c in line.chars() {
		answers.insert(c);
	}
	//println!("{:?}", answers);
	LineType::Member
}
