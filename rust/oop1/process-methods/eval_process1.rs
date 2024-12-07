//! Process: Modules

use std::io;

mod process1;
use process1::Process;

fn main() {
    let processes = read_input();
    show_all_processes(&processes);
    let p = &processes[0];
    println!(
        "{} {} {}:{:02}h",
        p.name(),
        p.priority(),
        p.duration().hours(),
        p.duration().mins()
    );
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
        process.show();
    }
}
