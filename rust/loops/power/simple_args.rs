use std::env;

#[allow(dead_code)]
pub fn int_args() -> Vec<i32> {
    let mut res = Vec::<i32>::new();
    for arg in env::args().skip(1) {
        let val = arg.parse::<i32>().expect(&format!("not a number {}", arg));
        res.push(val);
    }
    res
}

#[allow(dead_code)]
pub fn float_args() -> Vec<f64> {
    let mut res = Vec::<f64>::new();
    for arg in env::args().skip(1) {
        let val = arg.parse::<f64>().expect(&format!("not a number {}", arg));
        res.push(val);
    }
    res
}

#[allow(dead_code)]
pub fn usage(text: &str) {
    println!("Usage: {} {}", env::args().next().unwrap(), text);
}

// Verwendung:
// mod simple_args;
//
// let args = simple_args::int_args();
// if args.len() != GEWUENSCHTE_ZAHL {
//   usage("argumente");
//   return;
// }
// println!("1. Argument: {}", args[0]);
