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
    println!("All values:");
    println!("{:?}", values);
    let (sorted, gulag) = stalin_sort(values);
    println!("Sorted:");
    //show_vec(sorted);
    println!("{:?}", sorted);
    println!("Gulag:");
    //show_vec(gulag);
    println!("{:?}", gulag);
    // Error
    //println!("{:?}", values);
}

fn stalin_sort(values: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut sorted = Vec::new();
    let mut gulag = Vec::new();
    if values.len() == 0 {
        return (sorted, gulag);
    }
    let mut last_sorted = values[0];
    sorted.push(values[0]);
    for idx in 1..values.len() {
        if values[idx] < last_sorted {
            gulag.push(values[idx]);
        } else {
            sorted.push(values[idx]);
            last_sorted = values[idx];
        }
    }
    (sorted, gulag)
}

fn _show_vec(values: Vec<i32>) {
    println!("size: {}", values.len());
    for idx in 0..values.len() {
        print!(" {:3}: {:3};", idx, values[idx]);
    }
    println!();
}
