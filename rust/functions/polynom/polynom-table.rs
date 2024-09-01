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
    println!("Usage: polynom3 x_max a₃ a₂ a₁ a₀");
    println!("Computes table of function values from x=0 up to x=x_max");
}

fn show_result(x: i32, y: i32) {
    println!("{} {}", x, y);
}

fn polynom_table(x_max: i32, a_3: i32, a_2: i32, a_1: i32, a_0: i32) {
    // Abbruchbedingung
    if x_max < 0 {
        return;
    }
    polynom_table(x_max - 1, a_3, a_2, a_1, a_0);
    let y = poly3(x_max, a_3, a_2, a_1, a_0);
    show_result(x_max, y);
}

fn main() {
    let args = simple_args::int_args();
    if args.len() != 5 {
        usage();
        return;
    }
    let x_max = args[0];
    let a_3 = args[1];
    let a_2 = args[2];
    let a_1 = args[3];
    let a_0 = args[4];
    println!(
        "#Polynom: {} x³ + {} x² + {} x + {} [x = 0...{}]",
        a_3, a_2, a_1, a_0, x_max
    );
    println!("#x y");
    polynom_table(x_max, a_3, a_2, a_1, a_0);
}
