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
                let (_m_0, m_1, m_x) = mask;
                let addr_1 = apply_m1(m_1, addr);
                let addrs = generate_addrs(addr_1, m_x);
                for ad in addrs {
                    memory.insert(ad, val);
                }
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

fn apply_m1(m_1: u64, addr: u64) -> u64 {
    // apply m_1
    addr | m_1
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

const MASK_LEN: u64 = 36;
fn generate_addrs(base: u64, m_x: u64) -> Vec<u64> {
    let mut addrs = Vec::new();
    gen_addrs_rec(base, m_x, MASK_LEN, 0, &mut addrs);
    // debug
    for ad in &addrs {
        print!(" {:b}", ad);
    }
    println!();
    addrs
}

fn gen_addrs_rec(base: u64, m_x: u64, length: u64, pos: u64, addrs: &mut Vec<u64>) {
    if pos == length {
        addrs.push(base);
        return;
    }
    let m_pos = 1 << pos;
    // if floating bit
    if m_pos & m_x > 0 {
        // bit at pos is 0
        gen_addrs_rec(base & !m_pos, m_x, length, pos + 1, addrs);
        // bit at pos is 1
        gen_addrs_rec(base | m_pos, m_x, length, pos + 1, addrs);
    } else {
        gen_addrs_rec(base, m_x, length, pos + 1, addrs);
    }
}
