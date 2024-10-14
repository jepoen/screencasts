use std::io;

pub struct Input {
    parts: Vec<String>,
    pos: usize,
}

impl Input {
    pub fn read_line(&mut self) -> String {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line
    }

    pub fn new() -> Input {
        return Input {
            parts: Vec::new(),
            pos: 0,
        };
    }

    fn read_buffer(&mut self) {
        if self.pos < self.parts.len() {
            return;
        }
        self.parts.clear();
        while self.parts.len() == 0 {
            for s in self.read_line().trim().split(" ") {
                if s != "" {
                    self.parts.push(String::from(s));
                }
            }
        }
        self.pos = 0;
    }

    pub fn read_string(&mut self) -> String {
        self.read_buffer();
        let res = self.parts[self.pos].clone();
        self.pos += 1;
        res
    }

    #[allow(dead_code)]
    pub fn read_int(&mut self) -> i32 {
        let s = self.read_string();
        let res: i32 = s.parse().expect(&format!("Token {} is not i32", s));
        res
    }

    #[allow(dead_code)]
    pub fn read_float(&mut self) -> f64 {
        let s = self.read_string();
        let res: f64 = s.parse().expect(&format!("Token {} is not f64", s));
        res
    }

    #[allow(dead_code)]
    pub fn read_bool(&mut self) -> bool {
        let s = self.read_string();
        let res: bool = s.parse().expect(&format!("Token {} is not bool", s));
        res
    }
}

// Verwendung:
// mod simple_input;
// use simple_input::Input;
//
// fn main() {
//   let mut inp = Input::new();
//   let number = inp.read_int();
//   ...
// }
