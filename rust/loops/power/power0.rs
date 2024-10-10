/// Compute power of integer values
mod simple_args;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 2 {
        simple_args::usage("base exponent");
        return;
    }
    let a = args[0];
    let b = args[1];
    let (pw, count) = power(a, b);
    println!("{}^{} = {}, {} operations", a, b, pw, count);
}

fn power(a: i32, b: i32) -> (i128, i32) {
    if a < 0 || b < 0 {
        return (-1, 0);
    }
    let mut prod = 1_i128;
    let mut count = 0;
    //for _ in 1..=b {
    for _ in 0..b {
        prod *= a as i128;
        count += 1;
    }
    (prod, count)
}
