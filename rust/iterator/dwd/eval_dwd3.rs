// eval_dwd3.rs: pairwise difference
mod dwd;

const Y_START: usize = 1881;

fn main() {
    let temp_list = dwd::read_temp();
    //println!("{:?}", temp_list);
    //show_temp_list(&temp_list, Y_START);
    let diffs = pair_diffs(&temp_list);
    show_diffs(&diffs, Y_START);
}

fn pair_diffs(values: &[f64]) -> Vec<f64> {
    values
        .windows(2) // slices of size 2
        .map(|pair| pair[1] - pair[0]) // map values to difference
        .collect() // result vector
}

fn show_diffs(diffs: &[f64], y_start: usize) {
    for (idx, diff) in diffs.iter().enumerate() {
        println!("{}-{} {:5.1}", idx + y_start, idx + 1 + y_start, diff);
    }
}
#[allow(dead_code)]
fn show_temp_list(temp_list: &[f64], y_start: usize) {
    // for val in collection creates collection.into_iter()
    for (idx, temp) in temp_list.iter().enumerate() {
        println!("{}\t{:.1}", idx + y_start, temp);
    }
}
