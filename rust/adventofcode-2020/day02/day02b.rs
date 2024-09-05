use std::io;

#[derive(Debug)]
struct Entry {
    number1: u32,
    number2: u32,
    letter: char,
    text: String,
}

fn parse_line(buffer: &str) -> Result<Entry, String> {
    #[derive(Debug)]
    enum State {
        Start,
        N1,
        Dash,
        N2,
        Bl1,
        Letter,
        Colon,
        Bl2,
        Text,
    }
    let mut state = State::Start;
    let mut number1 = 0;
    let mut number2 = 0;
    let mut letter = '?';
    let mut text = String::new();
    for c in buffer.trim_end().chars() {
        match state {
            State::Start => {
                if c >= '0' && c <= '9' {
                    state = State::N1;
                    number1 = c.to_digit(10).unwrap();
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::N1 => {
                if c >= '0' && c <= '9' {
                    number1 = 10 * number1 + c.to_digit(10).unwrap();
                } else if c == '-' {
                    state = State::Dash
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::Dash => {
                if c >= '0' && c <= '9' {
                    state = State::N2;
                    number2 = c.to_digit(10).unwrap();
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::N2 => {
                if c >= '0' && c <= '9' {
                    number2 = 10 * number2 + c.to_digit(10).unwrap();
                } else if c == ' ' {
                    state = State::Bl1;
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::Bl1 => {
                if c >= 'a' && c <= 'z' {
                    letter = c;
                    state = State::Letter;
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::Letter => {
                if c == ':' {
                    state = State::Colon;
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::Colon => {
                if c == ' ' {
                    state = State::Bl2;
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::Bl2 => {
                if c >= 'a' && c <= 'z' {
                    text.push(c);
                    state = State::Text;
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
            State::Text => {
                if c >= 'a' && c <= 'z' {
                    text.push(c);
                } else {
                    return Err(format!("wrong char {}", c));
                }
            }
        }
    }
    if let State::Text = state {
        //println!("{} {} {} {}", number1, number2, letter, text);
        Ok(Entry {
            number1: number1,
            number2: number2,
            letter: letter,
            text: text,
        })
    } else {
        println!("State: {:?}", state);
        Err(format!("wrong state {:?}", state))
    }
}

fn is_valid(entry: &Entry) -> bool {
    let mut count_letter = 0;
    let chars: Vec<char> = entry.text.chars().collect();
    let pos1 = (entry.number1 - 1) as usize;
    let pos2 = (entry.number2 - 1) as usize;
    if chars[pos1] == entry.letter {
        count_letter += 1;
    }
    if chars[pos2] == entry.letter {
        count_letter += 1;
    }
    count_letter == 1
}

fn main() {
    let mut buffer = String::new();
    let mut count_valid = 0;
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let entry = parse_line(&buffer).unwrap();
        if is_valid(&entry) {
            println!("{:?}", entry);
            count_valid += 1;
        }
        buffer.clear();
    }
    println!("valid lines: {}", count_valid);
}
