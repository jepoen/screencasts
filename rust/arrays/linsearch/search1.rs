/// Search values in array
mod simple_input;

use simple_input::Input;

const MAX_NUMBER_COUNT: usize = 30;

fn main() {
    let mut inp = Input::new();
    let mut val = inp.read_int();
    // define array
    // let a = [0, 0, 0, 0, ...];
    let mut numbers = [0; MAX_NUMBER_COUNT];
    let mut size = 0;
    while val >= 0 {
        //print!(" {}", val);
        numbers[size] = val;
        size += 1;
        val = inp.read_int();
    }
    println!("\nsize: {}", size);
    //Debug
    //println!("{:?}", numbers);
    show_numbers(numbers, size);
    // Start search
    val = inp.read_int();
    while val >= 0 {
        println!("search {}", val);
        // search pos
        let idx = index_of(numbers, size, val);
        if idx < MAX_NUMBER_COUNT {
            println!("pos {} val {}", idx, numbers[idx]);
        } else {
            println!("val {} not found", val);
        }
        val = inp.read_int();
    }
}

fn show_numbers(numbers: [i32; MAX_NUMBER_COUNT], size: usize) {
    print!("Idx: ");
    for idx in 0..size {
        print!("{:3}", idx);
    }
    println!();
    print!("Val: ");
    for idx in 0..size {
        print!("{:3}", numbers[idx]);
    }
    println!();
}

/// result: pos if found, MAX_NUMBER_COUNT if not in array
fn index_of(numbers: [i32; MAX_NUMBER_COUNT], size: usize, search: i32) -> usize {
    for idx in 0..size {
        if numbers[idx] == search {
            return idx;
        }
    }
    MAX_NUMBER_COUNT
}
