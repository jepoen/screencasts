/// Power of integer values, fast power algorithm
mod simple_args;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 3 {
        simple_args::usage("base exponent module");
        return;
    }
    let a = args[0];
    let b = args[1];
    let m = args[2];
    let (pw, count) = power(a, b, m);
    println!("{}^{} = {}, {} operations", a, b, pw, count);
}

fn power(mut a: i32, mut b: i32, m: i32) -> (i32, i32) {
    if a < 0 || b < 0 || m < 2 {
        return (-1, 0);
    }
    let mut prod = 1;
    let mut count = 0;
    while b > 0 {
        if b % 2 == 1 {
            prod = prod * a % m;
            count += 2;
        }
        a = a * a % m;
        b /= 2;
        count += 3;
    }
    (prod, count)
}
