use std::io;

fn main() {
    let mut values: Vec<i32> = Vec::new();
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let val = parse_line(&buffer);
        //println!("{}", val);
        values.push(val);
        buffer.clear();
    }
    // Auswertung
    analyze(&values);
    let triples = find_triples(&values, 2020);
    println!("{:?}", triples);
    println!("Result: {}", triples[0].0 * triples[0].1 * triples[0].2);
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

fn find_triples(values: &[i32], target: i32) -> Vec<(i32, i32, i32)> {
    let mut res = Vec::new();
    let val_count = values.len();
    for i in 0..val_count {
        let tgt_remain = target - values[i];
        for j in i + 1..val_count {
            for k in j + 1..val_count {
                if values[j] + values[k] == tgt_remain {
                    res.push((values[i], values[j], values[k]));
                }
            }
        }
    }
    res
}
