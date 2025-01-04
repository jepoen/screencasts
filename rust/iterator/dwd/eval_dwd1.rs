// eval_dwd1.rs: input
mod dwd;

const Y_START: usize = 1881;

fn main() {
    let temp_list = dwd::read_temp();
    //println!("{:?}", temp_list);
    show_temp_list(&temp_list, Y_START);
}

fn show_temp_list(temp_list: &[f64], y_start: usize) {
    // for val in collection creates collection.into_iter()
    for (idx, temp) in temp_list.iter().enumerate() {
        println!("{}\t{:.1}", idx + y_start, temp);
    }
}
