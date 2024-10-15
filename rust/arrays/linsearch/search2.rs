/// Search values in array
mod simple_input;

use simple_input::Input;

const MAX_NUMBER_COUNT: usize = 500_000;
const MAX_SEARCH_VALUES: usize = 100;

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
    show_numbers(&numbers, size);
    // Start search
    let mut search_values = [0; MAX_SEARCH_VALUES];
    let mut search_count = 0;
    val = inp.read_int();
    while val >= 0 {
        search_values[search_count] = val;
        search_count += 1;
        val = inp.read_int();
    }
    println!("\nSearch values:");
    show_numbers(&search_values, search_count);
    //println!("{:?}", search_values);
    for _ in 0..10_000 {
        for idx in 0..search_count {
            // search pos
            println!("search {}", search_values[idx]);
            let pos = index_of(&numbers, size, search_values[idx]);
            if pos < MAX_NUMBER_COUNT {
                println!("pos {} val {}", pos, numbers[pos]);
            } else {
                println!("val {} not found", search_values[idx]);
            }
        }
    }
}

fn show_numbers(numbers: &[i32], size: usize) {
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
fn index_of(numbers: &[i32], size: usize, search: i32) -> usize {
    for idx in 0..size {
        if numbers[idx] == search {
            return idx;
        }
    }
    MAX_NUMBER_COUNT
}
