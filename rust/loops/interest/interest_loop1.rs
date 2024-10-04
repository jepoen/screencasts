/// interest rate with while loop
mod simple_args;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 3 {
        simple_args::usage("balance rate_perct duration_years");
        return;
    }
    let balance = args[0];
    let rate = args[1];
    let duration = args[2];
    println!(
        "balance: {} rate: {}% duration: {} years",
        balance, rate, duration
    );
    let balance_result = interest(balance, rate, duration);
    println!("new balance: {}", balance_result);
}

fn interest(mut balance: i32, rate: i32, duration: i32) -> i32 {
    // init
    let mut year = 1;
    // test
    while year <= duration {
        balance = balance + balance * rate / 100;
        println!("y {} b {}", year, balance);
        // reinit
        year += 1;
    }
    balance
}
