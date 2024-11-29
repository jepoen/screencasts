//! Advent of Code 2020, day 18, part 1

use std::io;

#[derive(Debug)]
enum Token {
    Num(u64),
    Op(char),
    Lparen,
    Rparen,
    End,
}

struct Lex {
    chars: Vec<char>,
    idx_lah: usize,
}

impl Lex {
    fn new(line: &str) -> Lex {
        let chars: Vec<_> = line.trim().chars().collect();
        Lex { chars, idx_lah: 0 }
    }

    fn lah(&mut self) -> Token {
        if self.idx_lah >= self.chars.len() {
            Token::End
        } else {
            let c = self.chars[self.idx_lah];
            Self::to_token(c)
        }
    }

    fn consume(&mut self) -> Token {
        let res = self.lah();
        self.idx_lah += 1;
        while self.idx_lah < self.chars.len() && self.chars[self.idx_lah] == ' ' {
            self.idx_lah += 1;
        }
        res
    }

    fn to_token(c: char) -> Token {
        match c {
            '+' | '*' => Token::Op(c),
            '0'..='9' => Token::Num(c.to_digit(10).unwrap() as u64),
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            _ => panic!("Unknown token {}", c),
        }
    }
}

fn main() {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let mut lex = Lex::new(&line.unwrap());
        //test_lex(&mut lex);
        let res = expr(&mut lex);
        sum += res;
        println!("{}", res);
    }
    println!("Sum of results: {}", sum);
}

#[allow(dead_code)]
fn test_lex(lex: &mut Lex) {
    loop {
        let lah = lex.lah();
        print!("<{:?}>", lah);
        if let Token::End = lah {
            break;
        }
        lex.consume();
    }
    println!();
}

fn expr(lex: &mut Lex) -> u64 {
    let mut lhs = term(lex);
    while let Token::Op(op) = lex.lah() {
        lex.consume();
        let rhs = term(lex);
        match op {
            '*' => lhs *= rhs,
            '+' => lhs += rhs,
            _ => panic!("unknown op {}", op),
        }
    }
    lhs
}

fn term(lex: &mut Lex) -> u64 {
    match lex.lah() {
        Token::Num(val) => {
            lex.consume();
            val
        }
        Token::Lparen => {
            lex.consume();
            let res = expr(lex);
            if let Token::Rparen = lex.lah() {
                lex.consume();
                res
            } else {
                panic!("expected '(', got {:?}", lex.lah())
            }
        }
        _ => panic!("expected num '(', got {:?}", lex.lah()),
    }
}
