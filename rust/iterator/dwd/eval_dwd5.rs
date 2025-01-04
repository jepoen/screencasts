// eval_dwd5.rs: average of segment/chunk
mod dwd;

const Y_START: usize = 1881;

fn main() {
    let temp_list = dwd::read_temp();
    //println!("{:?}", temp_list);
    //show_temp_list(&temp_list, Y_START);
    let avg_seg = segment_avg(&temp_list, 10);
    show_avg_seg(&avg_seg, Y_START, Y_START + temp_list.len(), 10);
}

fn segment_avg(values: &[f64], seg_size: usize) -> Vec<f64> {
    values
        .chunks(seg_size) // segments of size usize
        .map(|seg| seg.iter().sum::<f64>() / seg.len() as f64) // avg of segment
        .collect() // result vector
}

fn show_avg_seg(values: &[f64], y_start: usize, y_end: usize, seg_size: usize) {
    for (idx, val) in values.iter().enumerate() {
        println!(
            "{}-{} {:5.1}",
            idx * seg_size + y_start,
            {
                let seg_end = idx * seg_size + seg_size - 1 + y_start;
                if seg_end < y_end {
                    seg_end
                } else {
                    y_end - 1
                }
            },
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
