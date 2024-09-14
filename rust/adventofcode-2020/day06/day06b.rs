use std::io;
use std::collections::HashSet;

enum LineType {
	Member(HashSet<char>),
	Skip,
}

fn main() {
	let mut buffer = String::new();
	let mut group_answers: Vec<HashSet<char>> = Vec::new();
	let mut count = 0;
	while io::stdin().read_line(&mut buffer).unwrap() > 0 {
		match parse_line(&buffer) {
			LineType::Skip => {
				// Gruppenwechel
				println!("{:?}", group_answers);
				let group_all = group_intersect(&group_answers);
				println!("Intersection {:?}", group_all);
				count += group_all.len();
				group_answers.clear();
			},
			LineType::Member(answers) => {
				group_answers.push(answers);
			},
		}
		buffer.clear();
	}
	// Gruppenwechsel letzte Gruppe
	println!("{:?}", group_answers);
	let group_all = group_intersect(&group_answers);
	println!("Intersection {:?}", group_all);
	count += group_all.len();
	println!("Count {count}");
}

fn parse_line(buffer: &str) -> LineType {
	let line = buffer.trim_end();
	if line == "" {
		//println!("---");
		return LineType::Skip;
	}
	let mut answers = HashSet::new();
	for c in line.chars() {
		answers.insert(c);
	}
	//println!("{:?}", answers);
	LineType::Member(answers)
}

fn group_intersect(group: &[HashSet<char>]) -> HashSet<char> {
	let mut res: HashSet<char> = group[0].iter().map(|c| *c).collect();
	// Todo: Durchschnitt iterativ mit Zeilen 1 .. n-1
	for member in group.iter().skip(1) {
		res = res.intersection(member).map(|c| *c).collect();
	}
	res
}
