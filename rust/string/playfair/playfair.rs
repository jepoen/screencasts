//! playfair cipher
//! Usage: playfair enc|dec|prep key message

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} enc|dec|prep key message", args[0]);
        return;
    }
    let key = &args[2].to_ascii_uppercase();
    if let Err(err) = check_key(&key) {
        println!("Key error: {}", err);
        return;
    }
    let playfair_square = new_playfair_square(key);
    show_square(&playfair_square);
    let msg = &args[3];
    let res = match args[1].as_str() {
        "prep" => prepare_msg(msg),
        "enc" => en_decrypt(&playfair_square, &prepare_msg(msg)),
        "dec" => clean_result(&en_decrypt(&playfair_square, msg)),
        _ => {
            println!("Unknown action: {}", args[1]);
            return;
        }
    };
    println!("{}", res);
}

fn check_key(key: &str) -> Result<(), String> {
    let mut letters = Vec::new();
    for c in key.chars() {
        match c {
            'J' => return Err(format!("invalid char {}", c)),
            'A'..='Z' => {
                if letters.contains(&c) {
                    return Err(format!("duplicate char {}", c));
                }
                letters.push(c);
            }
            _ => return Err(format!("invalid char {}", c)),
        }
    }
    Ok(())
}

#[test]
fn test_check_key() {
    assert!(check_key("LINUX").is_ok());
    assert!(check_key("LJNUX").is_err());
    assert!(check_key("äöü# X").is_err());
    assert!(check_key("LINUXI").is_err());
}

fn prepare_msg(msg: &str) -> String {
    let mut letters = String::new();
    for c in msg.to_ascii_uppercase().chars() {
        match c {
            'ä' | 'Ä' | 'æ' | 'Æ' => letters.push_str("AE"),
            'ö' | 'Ö' | 'œ' | 'Œ' => letters.push_str("OE"),
            'ü' | 'Ü' => letters.push_str("UE"),
            'ß' => letters.push_str("SZ"),
            'J' | 'Y' => letters.push('I'),
            'A'..='Z' => letters.push(c),
            _ => (),
        }
    }
    let mut res = String::new();
    let mut prev_char = '_';
    for c in letters.chars() {
        if c == prev_char {
            res.push('Y');
        }
        res.push(c);
        prev_char = c;
    }
    if res.len() % 2 != 0 {
        res.push('Y');
    }
    res
}

#[test]
fn test_prepare_msg() {
    assert_eq!(prepare_msg("baby sitter"), "BABISITYTERY");
    assert_eq!(prepare_msg("IJY"), "IYIYIY");
}

fn new_playfair_square(key: &str) -> Vec<Vec<char>> {
    let mut letters: Vec<char> = key.chars().collect();
    for c in 'A'..='Z' {
        if c == 'J' {
            continue;
        }
        if !letters.contains(&c) {
            letters.push(c);
        }
    }
    assert_eq!(letters.len(), 25);
    let mut square = Vec::new();
    for r in 0..5 {
        let mut row = Vec::new();
        for c in 0..5 {
            row.push(letters[r * 5 + c]);
        }
        square.push(row);
    }
    square
}

fn show_square(square: &[Vec<char>]) {
    println!("Playfair square:");
    for row in square {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn en_decrypt(square: &[Vec<char>], msg: &str) -> String {
    let mut res = String::new();
    let mut it = msg.chars();
    while let Some(c0) = it.next() {
        let c1 = it.next().unwrap();
        let (row0, col0) = square_pos(square, c0).unwrap();
        let (row1, col1) = square_pos(square, c1).unwrap();
        res.push(square[row1][col0]);
        res.push(square[row0][col1]);
    }
    res
}

fn square_pos(square: &[Vec<char>], ch: char) -> Option<(usize, usize)> {
    for r in 0..5 {
        for c in 0..5 {
            if square[r][c] == ch {
                return Some((r, c));
            }
        }
    }
    None
}

fn clean_result(msg: &str) -> String {
    /*
    let mut res = String::new();
    for c in msg.chars() {
        if c != 'Y' {
            res.push(c)
        }
    }
    res
    */
    // shorter with iterator
    msg.chars().filter(|c| *c != 'Y').collect()
}

#[test]
fn test_clean_result() {
    assert_eq!(clean_result("YYY"), "");
    assert_eq!(clean_result("AYBYCY"), "ABC");
}
