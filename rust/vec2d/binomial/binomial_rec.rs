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
    // debug
    //print!(" ({},{})", n, k);
    if n < 0 || k < 0 || k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        binomial(n - 1, k - 1) + binomial(n - 1, k)
    }
}
