//! Advent of Code 2020/13/1

use std::io;

#[derive(Debug)]
struct Bus {
    nr: i64,
    _offset: i64,
    miss: i64,
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let busses = parse_line(&buffer);
    println!("{:?}", busses);
    let t = search_t(&busses);
    println!("{t}");
}

fn parse_line(buffer: &str) -> Vec<Bus> {
    let mut busses = Vec::new();
    let mut offset = 0;
    for part in buffer.trim_end().split(",") {
        if part != "x" {
            let val = part.parse().unwrap();
            busses.push(Bus {
                nr: val,
                _offset: offset,
                miss: get_miss(val, offset),
            });
        }
        offset += 1;
    }
    busses
}

fn get_miss(bus: i64, mut offset: i64) -> i64 {
    while offset > 0 {
        offset -= bus;
    }
    -offset
}

fn search_t(busses: &[Bus]) -> i64 {
    let mut ppp = 1;
    for bus in busses {
        ppp *= bus.nr;
    }
    let mut sum = 0;
    for bus in busses {
        let p = bus.nr;
        let q = ppp / p;
        let (_, _d, r) = ext_euclid(p, q);
        let summand = bus.miss * r * q;
        sum += summand;
    }
    while sum < 0 {
        sum += ppp;
    }
    sum % ppp
}

/// return (lhs, x, y) for lhs = a*x + b*y
fn ext_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut lhs_old, mut lhs_new);
    let (mut x_old, mut x_new);
    let (mut y_old, mut y_new);
    if a > b {
        (lhs_old, lhs_new) = (a, b);
        (x_old, y_old) = (1, 0);
        (x_new, y_new) = (0, 1);
    } else {
        (lhs_old, lhs_new) = (b, a);
        (x_old, y_old) = (0, 1);
        (x_new, y_new) = (1, 0);
    }
    while lhs_new > 1 {
        let quot = lhs_old / lhs_new;
        (lhs_old, lhs_new) = (lhs_new, lhs_old - quot * lhs_new);
        (x_old, x_new) = (x_new, x_old - quot * x_new);
        (y_old, y_new) = (y_new, y_old - quot * y_new);
    }
    (lhs_new, x_new, y_new)
}

#[test]
fn test_ext_euclid() {
    assert_eq!(ext_euclid(3, 5), (1, 2, -1));
    assert_eq!(ext_euclid(5, 3), (1, -1, 2));
    assert_eq!(ext_euclid(19, 7), (1, 3, -8));
}
