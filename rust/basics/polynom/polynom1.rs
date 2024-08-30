mod simple_args;
mod simple_input;

use simple_input::Input;

fn main() {
    let args = simple_args::int_args();
    println!("Anzahl Argumente: {}", args.len());
    let a_2 = args[0];
    let a_1 = args[1];
    let a_0 = args[2];
    println!("Polynom: {} x² + {} x + {}", a_2, a_1, a_0);
    let mut inp = Input::new();
    println!("x=");
    let x = inp.read_int();
    println!("x = {}", x);
    // x^2 bedeutet nicht x²
    //let y = a_2 * x^2 + a_1 * x + a_0;
    let y = a_2 * x * x + a_1 * x + a_0;
    println!("y = f({}) = {}", x, y);
    println!("x=");
    let x2 = inp.read_int();
    println!("x = {}", x2);
    let y2 = a_2 * x2 * x2 + a_1 * x2 + a_0;
    println!("y = f({}) = {}", x2, y2);
}
