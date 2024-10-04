/// interest rate with recursion
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

fn interest(balance: i32, rate: i32, duration: i32) -> i32 {
    if duration <= 0 {
        return balance;
    }
    let balance_new = balance + balance * rate / 100;
    interest(balance_new, rate, duration - 1)
}
