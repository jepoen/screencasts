//! Process: Modules

use std::io;

mod process2;
use process2::{Duration, Process};

fn main() {
    let mut processes = read_input();
    show_all_processes(&processes);
    processes.sort();
    println!("ordered:");
    show_all_processes(&processes);
    processes.sort_by(Process::cmp_duration);
    println!("ordered by duration:");
    show_all_processes(&processes);
    let total_duration = sum_process_duration(&processes);
    println!("total duration {}", total_duration);
}

fn read_input() -> Vec<Process> {
    let mut processes = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        match Process::parse_line(&line) {
            Ok(process) => processes.push(process),
            Err(err) => println!("Error: {} line: <{}>", err, line),
        }
    }
    processes
}

fn show_all_processes(processes: &[Process]) {
    for process in processes {
        println!("{}", process);
        //process.show();
    }
}

fn sum_process_duration(processes: &[Process]) -> Duration {
    let mut sum = Duration::new(0, 0);
    for p in processes {
        sum = sum + *p.duration();
    }
    sum
}
