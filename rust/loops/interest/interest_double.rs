/// interest: duration for doubling balance
mod simple_args;

// TODO: comment
const BALANCE_BASE: i32 = 100_000;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 1 {
        simple_args::usage("rate_perct");
        return;
    }
    let rate = args[0];
    let balance = BALANCE_BASE;
    println!("balance: {} rate: {}%", balance, rate);
    let duration = interest_double(balance, rate);
    println!("duration : {}", duration);
}

fn interest_double(balance_0: i32, rate: i32) -> i32 {
    assert!(balance_0 > 0, "balance_0 must be positive");
    assert!(rate > 0, "rate must be positive");
    // init
    let mut year = 0;
    let mut balance = balance_0;
    // test
    while balance < 2 * balance_0 {
        balance = balance + balance * rate / 100;
        println!("y {} b {}", year, balance);
        // reinit
        year += 1;
    }
    year
}
