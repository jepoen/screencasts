use std::io;

fn main() {
    let mut values: Vec<i32> = Vec::new();
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let val = parse_line(&buffer);
        println!("{}", val);
        values.push(val);
        buffer.clear();
    }
    // Auswertung
    analyze(&values);
    let pairs = find_pairs(&values, 2020);
    println!("{:?}", pairs);
    println!("Result: {}", pairs[0].0 * pairs[0].1);
}

fn parse_line(line: &str) -> i32 {
    let val = line.trim_end().parse().unwrap();
    val
}

fn analyze(values: &[i32]) {
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();
    let count = values.len();
    println!("count {} min {} max {}", count, min, max);
}

fn find_pairs(values: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    let val_count = values.len();
    for i in 0..val_count {
        for k in i + 1..val_count {
            if values[i] + values[k] == target {
                res.push((values[i], values[k]));
            }
        }
    }
    res
}
