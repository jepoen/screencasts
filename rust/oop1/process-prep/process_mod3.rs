//! Process: Modules

use std::io;

mod process3;
use process3::{parse_line, show_process, Process};

fn main() {
    let processes = read_input();
    show_all_processes(&processes);
    let p = &processes[0];
    println!(
        "{} {} {} {:02}",
        p.name, p.priority, p.duration.hours, p.duration.mins
    );
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
