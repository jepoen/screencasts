use std::io;

fn main() {
    /*
    let mut buffer = String::new();
    // skip table header
    io::stdin().read_line(&mut buffer);
    buffer.clear();
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        println!("{}", buffer.trim_end());
        buffer.clear();
    }
    */
    let mut entries = Vec::new();
    for line in io::stdin().lines().skip(1) {
        //println!("{}", line.unwrap());
        if let Some(entry) = parse_csv_line(&line.unwrap()) {
            println!("{:?}", entry);
            entries.push(entry);
        }
    }
    show_table(&entries);
}

fn parse_csv_line(line: &str) -> Option<(String, i32, f64)> {
    // skip comments
    if &line[0..1] == "#" {
        return None;
    }
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let text = parts[0].trim_matches('"').to_string();
    let size;
    let time;
    if let Ok(val) = parts[1].parse() {
        size = val;
    } else {
        return None;
    }
    if let Ok(val) = parts[2].parse() {
        time = val;
    } else {
        return None;
    }
    Some((text, size, time))
}

fn show_table(entries: &Vec<(String, i32, f64)>) {
    println!("{:20} {:>6} {:>8}", "Algorithm", "Size", "Time");
    let mut old_size = 0;
    for (text, size, time) in entries {
        if old_size > 0 && *size != old_size {
            println!();
        }
        old_size = *size;
        println!("{:20} {:6} {:8.1}", text, size, time);
    }
}
