//! Stalin-Sort, Input in Vec

mod simple_input;
use simple_input::Input;

fn main() {
    let mut inp = Input::new();
    let mut values = Vec::new();
    let mut val = inp.read_int();
    while val >= 0 {
        // process val
        values.push(val);
        val = inp.read_int();
    }
    show_vec(values);
    // Error
    //println!("{:?}", values);
}

fn show_vec(values: Vec<i32>) {
    println!("size: {}", values.len());
    for idx in 0..values.len() {
        println!("{:3} {:3}", idx, values[idx]);
    }
}
