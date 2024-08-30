mod simple_args;

fn poly3(x: i32, a_3: i32, a_2: i32, a_1: i32, a_0: i32) -> i32 {
    // Berechnung nach Horner-Schema
    let mut y = 0;
    y += a_3;
    y *= x;
    y += a_2;
    y *= x;
    y += a_1;
    y *= x;
    y += a_0;
    //return y;
    y
}

fn usage() {
    println!("Usage: polynom3 x a₃ a₂ a₁ a₀");
}

fn show_result(x: i32, y: i32) {
    println!("y = f({}) = {}", x, y);
}

fn main() {
    let args = simple_args::int_args();
    if args.len() != 5 {
        usage();
        return;
    }
    let x = args[0];
    let a_3 = args[1];
    let a_2 = args[2];
    let a_1 = args[3];
    let a_0 = args[4];
    println!(
        "Polynom: {} x³ + {} x² + {} x + {} [x = {}]",
        a_3, a_2, a_1, a_0, x
    );
    let y = poly3(x, a_3, a_2, a_1, a_0);
    show_result(x, y);
    let y_0 = poly3(0, a_3, a_2, a_1, a_0);
    show_result(0, y_0);
}
