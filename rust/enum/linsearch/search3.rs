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
    for idx in 0..search_count {
        // search pos
        println!("search {}", search_values[idx]);
        // 1st variant
        match index_of(&numbers, size, search_values[idx]) {
            Some(pos) => println!("pos {} val {}", pos, numbers[pos]),
            None => println!("val not found"),
        }
        // 2nd variant
        if let Some(pos) = index_of(&numbers, size, search_values[idx]) {
            println!("*pos {} val {}", pos, numbers[pos]);
        } else {
            println!("*val not found");
        }
    }
    /* saubere Auswertung
    if let Some(pos) = max_arg(&numbers, size) {
        println!("Max value at {} = {}", pos, numbers[pos]);
    }
    */
    // Wir sind sicher, dass das Feld nie leer ist
    //let pos = max_arg(&numbers, size).unwrap();
    // bessere Fehlermeldung
    let pos = max_arg(&numbers, size).expect("empty array is not allowed");
    println!("Max value at {} = {}", pos, numbers[pos]);
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

fn index_of(numbers: &[i32], size: usize, search: i32) -> Option<usize> {
    for idx in 0..size {
        if numbers[idx] == search {
            return Some(idx);
        }
    }
    None
}

fn max_arg(numbers: &[i32], size: usize) -> Option<usize> {
    if size == 0 {
        return None;
    }
    let mut max_pos = 0;
    for pos in 1..size {
        if numbers[pos] > numbers[max_pos] {
            max_pos = pos;
        }
    }
    Some(max_pos)
}
