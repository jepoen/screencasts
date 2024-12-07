#![allow(dead_code)]

use std::cmp::Ordering;
use std::fmt;
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Duration {
    /* old
    hours: u32,
    mins: u32,
    * new */
    mins: u32,
}

impl Duration {
    // Constructor
    pub fn new(hours: u32, mins: u32) -> Duration {
        let total_minutes = hours * 60 + mins;
        Duration {
            /* old *
            hours: total_minutes / 60,
            mins: total_minutes % 60,
            * new */
            mins: total_minutes,
        }
    }
    // Getter
    pub fn hours(&self) -> u32 {
        //self.hours // old
        self.mins / 60
    }
    pub fn mins(&self) -> u32 {
        //self.mins // old
        self.mins % 60
    }
}
impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.mins == other.mins
    }
}

impl Eq for Duration {}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.mins.cmp(&other.mins)
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Duration::new(0, self.mins + rhs.mins)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:02}", self.hours(), self.mins())
    }
}

pub struct Process {
    name: String,
    duration: Duration,
    priority: u32,
}

fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|_| format!("not a number: {}", s))
}

impl Process {
    // Constructor
    pub fn new(name: String, duration: Duration, priority: u32) -> Process {
        Process {
            name,
            duration,
            priority,
        }
    }

    pub fn parse_line(line: &str) -> Result<Process, String> {
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
        Ok(Process::new(
            parts[0].to_string(),
            Duration::new(hours, mins),
            priority,
        ))
    }
    // Getter
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn duration(&self) -> &Duration {
        &self.duration
    }
    pub fn priority(&self) -> u32 {
        self.priority
    }

    // other method
    pub fn show(&self) {
        println!("{}", self);
    }

    pub fn cmp_duration(a: &Self, b: &Self) -> Ordering {
        a.duration.cmp(&b.duration)
    }
}
impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Process {}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:30} {:>6}h {}",
            self.name, self.duration, self.priority,
        )
    }
}
