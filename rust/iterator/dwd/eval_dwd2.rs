// eval_dwd2.rs: avg, hottest, coldest year
mod dwd;

const Y_START: usize = 1881;

fn main() {
    let temp_list = dwd::read_temp();
    //show_temp_list(&temp_list, Y_START);
    println!("average: {:.1} °C", avg_all(&temp_list));
    let (idx_hottest, temp) = argmax(&temp_list);
    println!("hottest: {}: {:.1} °C", idx_hottest + Y_START, temp);
    let (idx_coldest, temp) = argmin(&temp_list);
    println!("coldest: {}: {:.1} °C", idx_coldest + Y_START, temp);
}

fn avg_all(temp_list: &[f64]) -> f64 {
    temp_list.iter().sum::<f64>() / temp_list.len() as f64
}

fn argmax(values: &[f64]) -> (usize, f64) {
    values
        .iter()
        .map(|ref_temp| *ref_temp) // derefence temp
        .enumerate() // numbering entries
        .max_by(|(_, temp1), (_, temp2)| temp1.partial_cmp(temp2).unwrap()) // compare temps
        .unwrap() // unpack Some(val)
}

fn argmin(values: &[f64]) -> (usize, f64) {
    values
        .iter()
        .map(|ref_temp| *ref_temp) // derefence temp
        .enumerate() // numbering entries
        .min_by(|(_, temp1), (_, temp2)| temp1.partial_cmp(temp2).unwrap()) // compare temps
        .unwrap() // unpack Some(val)
}

#[allow(dead_code)]
fn show_temp_list(temp_list: &[f64], y_start: usize) {
    // for val in collection creates collection.into_iter()
    for (idx, temp) in temp_list.iter().enumerate() {
        println!("{}\t{:.1}", idx + y_start, temp);
    }
}
