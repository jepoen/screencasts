use std::collections::HashMap;
use std::io; // for memory

#[derive(Debug)]
enum Entry {
    Mask(String),
    Mem(u64, u64), // (addr, val)
}

fn main() {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: (u64, u64, u64) = (0, 0, 0); // m_0, m_1, m_X
    for line in io::stdin().lines() {
        let entry = parse_line(&line.unwrap().trim());
        println!("{:?}", entry);
        match entry {
            Entry::Mask(mask_str) => {
                mask = parse_mask(&mask_str);
            }
            Entry::Mem(addr, val) => {
                let val_new = apply_mask(mask, val);
                memory.insert(addr, val_new);
            }
        }
    }
    println!("{:?}", memory);
    println!("{}", sum_mem(&memory));
}

fn parse_line(line: &str) -> Entry {
    let parts: Vec<_> = line.split(" = ").collect();
    if parts[0] == "mask" {
        Entry::Mask(parts[1].to_string())
    } else {
        let addr = parts[0].trim_matches(|c| c < '0' || c > '9');
        Entry::Mem(addr.parse().unwrap(), parts[1].parse().unwrap())
    }
}

fn parse_mask(mask_str: &str) -> (u64, u64, u64) {
    let mut m_0 = 0;
    let mut m_1 = 0;
    let mut m_x = 0;
    for c in mask_str.chars() {
        m_0 *= 2;
        m_1 *= 2;
        m_x *= 2;
        match c {
            '0' => m_0 += 1,
            '1' => m_1 += 1,
            'X' => m_x += 1,
            _ => panic!("unknown char {}", c),
        }
    }
    (m_0, m_1, m_x)
}

#[test]
fn test_parse_mask() {
    assert_eq!(
        parse_mask("00X11X001"),
        (0b110000110, 0b000110001, 0b001001000)
    );
}

fn apply_mask(mask: (u64, u64, u64), val: u64) -> u64 {
    let (m_0, m_1, _m_x) = mask;
    // apply m_0
    let val_0 = val & !m_0;
    // apply m_1
    let val_1 = val_0 | m_1;
    // m_x unnecessary
    val_1
}

fn sum_mem(memory: &HashMap<u64, u64>) -> u64 {
    /*
    let mut sum = 0;
    for (_k, val) in memory {
        sum += *val;
    }
    sum
    */
    memory.iter().map(|(_addr, val)| val).sum()
}
