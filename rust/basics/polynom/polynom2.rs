mod simple_args;

fn main() {
    let args = simple_args::int_args();
    println!("Anzahl Argumente: {}", args.len());
    let x = args[0];
    let a_3 = args[1];
    let a_2 = args[2];
    let a_1 = args[3];
    let a_0 = args[4];
    println!("Polynom: {} x³ + {} x² + {} x + {} [x = {}]",
     a_3, a_2, a_1, a_0, x
    );
    // Berechnung nach Horner-Schema
    let mut y = 0;
    y += a_3;
    y *= x;
    y += a_2;
    y *= x;
    y += a_1;
    y *= x;
    y += a_0;
    println!("y = f({}) = {}", x, y);
}
