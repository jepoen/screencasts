// eval_dwd4.rs: moving average
mod dwd;

const Y_START: usize = 1881;

fn main() {
    let temp_list = dwd::read_temp();
    //println!("{:?}", temp_list);
    //show_temp_list(&temp_list, Y_START);
    let avg_10 = moving_avg(&temp_list, 10);
    show_avg(&avg_10, Y_START, 10);
}

fn moving_avg(values: &[f64], win_size: usize) -> Vec<f64> {
    values
        .windows(win_size) // window of size win_size
        .map(|win| win.iter().sum::<f64>() / win_size as f64) // avg of window
        .collect() // result vector
}

fn show_avg(values: &[f64], y_start: usize, win_size: usize) {
    for (idx, val) in values.iter().enumerate() {
        println!(
            "{}-{} {:5.1}",
            idx + y_start,
            idx + win_size - 1 + y_start,
            val
        );
    }
}
#[allow(dead_code)]
fn show_temp_list(temp_list: &[f64], y_start: usize) {
    // for val in collection creates collection.into_iter()
    for (idx, temp) in temp_list.iter().enumerate() {
        println!("{}\t{:.1}", idx + y_start, temp);
    }
}
