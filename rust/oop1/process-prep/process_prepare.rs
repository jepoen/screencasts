//! Process: Data structures, read data

use std::io;

struct Duration {
    hours: u32,
    mins: u32,
}

struct Process {
    name: String,
    duration: Duration,
    priority: u32,
}

fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|_| format!("not a number: {}", s))
}

fn parse_line(line: &str) -> Result<Process, String> {
    let parts: Vec<_> = line.trim().split(",").collect();
    if parts.len() != 3 {
        return Err(format!("wrong # of cols, expected 3 got {}", parts.len()));
    }
    let time_parts: Vec<_> = parts[1].split(":").collect();
    let hours;
    let mins;
    if time_parts.len() == 1 {
        hours = 0;
        mins = parse_u32(time_parts[0])?;
    } else if time_parts.len() == 2 {
        hours = parse_u32(time_parts[0])?;
        mins = parse_u32(time_parts[1])?;
    } else {
        return Err(format!("Not valid duration: {}", parts[1]));
    }
    let priority = parse_u32(parts[2])?;
    Ok(Process {
        name: parts[0].to_string(),
        duration: Duration { hours, mins },
        priority,
    })
}

fn show_process(process: &Process) {
    println!(
        "{:30} {:3}:{:02} {}",
        process.name, process.duration.hours, process.duration.mins, process.priority,
    );
}

fn main() {
    let processes = read_input();
    show_all_processes(&processes);
}

fn read_input() -> Vec<Process> {
    let mut processes = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        match parse_line(&line) {
            Ok(process) => processes.push(process),
            Err(err) => println!("Error: {} line: <{}>", err, line),
        }
    }
    processes
}

fn show_all_processes(processes: &[Process]) {
    for process in processes {
        show_process(process);
    }
}
