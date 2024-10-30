//! Stalin-Sort In-place
//! Vec-Objekte als veränderliche Referenzen

mod simple_input;
use simple_input::Input;

fn main() {
    //let mut values = read_values();
    // Variante: Vec anlegen und Eingabe soll diesen ändern
    let mut values = Vec::new();
    // Startwert 1 am Anfang ergänzt
    values.push(1);
    read_values_inplace(&mut values);
    println!("All values:");
    show_vec(&values);
    stalin_sort(&mut values);
    println!("Sorted:");
    show_vec(&values);
}

/* idiomatisch korrekt
fn read_values() -> Vec<i32> {
    let mut inp = Input::new();
    let mut values = Vec::new();
    let mut val = inp.read_int();
    while val >= 0 {
        // process val
        values.push(val);
        val = inp.read_int();
    }
    values
}
*/

fn read_values_inplace(values: &mut Vec<i32>) {
    let mut inp = Input::new();
    let mut val = inp.read_int();
    while val >= 0 {
        // process val
        values.push(val);
        val = inp.read_int();
    }
}

fn stalin_sort(values: &mut Vec<i32>) {
    // Debug
    let mut gulag = Vec::new();
    if values.len() == 0 {
        return;
    }
    let mut last_val = values[0];
    let mut pos = 1;
    while pos < values.len() {
        if values[pos] < last_val {
            let _val = values.remove(pos);
            // Debug
            gulag.push(_val);
        } else {
            last_val = values[pos];
            pos += 1;
        }
    }
    // Debug
    println!("Gulag:");
    show_vec(&gulag);
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
