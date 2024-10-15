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
    if !is_sorted(&numbers, size) {
        println!("not sorted");
        return;
    }
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
    let idx_left = 0;
    let idx_right = size;
    let idx = binsearch_rec(numbers, idx_left, idx_right, search);
    if idx == size {
        // all values smaller than search value
        MAX_NUMBER_COUNT
    } else if numbers[idx] == search {
        idx
    } else {
        MAX_NUMBER_COUNT
    }
}

// Condition: numbers[idx_left-1] < search ≤ numbers[idx_right]
fn binsearch_rec(numbers: &[i32], idx_left: usize, idx_right: usize, search: i32) -> usize {
    if idx_left == idx_right {
        idx_right
    } else {
        let idx_m = (idx_left + idx_right) / 2;
        if numbers[idx_m] < search {
            //idx_left = idx_m + 1
            binsearch_rec(numbers, idx_m + 1, idx_right, search)
        } else {
            // numbers[idx_m] >= search
            // idx_right = idx_m
            binsearch_rec(numbers, idx_left, idx_m, search)
        }
    }
}

fn is_sorted(numbers: &[i32], size: usize) -> bool {
    for idx in 1..size {
        if numbers[idx - 1] > numbers[idx] {
            return false;
        }
    }
    true
}
