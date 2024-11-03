//! binomial coefficient direct formula

mod simple_args;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 2 {
        simple_args::usage("n k");
        return;
    }
    let n = args[0];
    let k = args[1];
    let res = binomial(n, k);
    println!("binomial({}, {}) = {}", n, k, res);
}

fn binomial(n: i32, k: i32) -> i32 {
    if n < 0 || k < 0 || k > n {
        0
    } else {
        factorial(n) / (factorial(k) * factorial(n - k))
    }
}

fn factorial(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    let mut prod = 1;
    for i in 1..=n {
        prod *= i;
    }
    prod
}
