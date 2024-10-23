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
    show_vec(&values);
    let (sorted, gulag) = stalin_sort(&values);
    println!("Sorted:");
    show_vec(&sorted);
    println!("Gulag:");
    show_vec(&gulag);
    println!("Values 2-4:");
    show_vec(&values[1..4]);
}

fn stalin_sort(values: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut sorted = Vec::new();
    let mut gulag = Vec::new();
    let mut it = values.into_iter();
    let mut last_sorted;
    if let Some(&val) = it.next() {
        last_sorted = val;
        sorted.push(val);
        //print!("<{}>", val);
    } else {
        return (sorted, gulag);
    }
    // for val in it {
    // alternativ:
    while let Some(&val) = it.next() {
        //print!(" {}", val);
        if val < last_sorted {
            gulag.push(val);
        } else {
            sorted.push(val);
            last_sorted = val;
        }
    }
    println!();
    (sorted, gulag)
}

fn show_vec(values: &[i32]) {
    print!("Idx:");
    for idx in 0..values.len() {
        print!(" {:3}", idx);
    }
    println!();
    print!("Val:");
    for val in values {
        print!(" {:3}", val);
    }
    println!();
}
