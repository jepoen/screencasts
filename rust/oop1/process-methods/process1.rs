pub struct Duration {
    /* old
    hours: u32,
    mins: u32,
    * new */
    mins: u32,
}

impl Duration {
    // Constructor
    fn new(hours: u32, mins: u32) -> Duration {
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
        println!(
            "{:30} {:3}:{:02} {}",
            self.name,
            self.duration.hours(),
            self.duration.mins(),
            self.priority,
        );
    }
}
