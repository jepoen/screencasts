//! Longest increasing Subsequence

mod simple_input;
use simple_input::Input;

fn main() {
    let values = read_values();
    println!("All values: {:?}, Length: {}", values, values.len());
    let subseq = longest_incr(&values);
    println!("Incr. subsequence: {:?} Length: {}", subseq, subseq.len());
}

fn read_values() -> Vec<i32> {
    let mut inp = Input::new();
    let mut values = Vec::new();
    let mut val = inp.read_int();
    while val >= 0 {
        values.push(val);
        val = inp.read_int();
    }
    values
}

fn longest_incr(values: &[i32]) -> Vec<i32> {
    if values.len() == 0 {
        return Vec::new();
    }
    let mut inc_sub_len: Vec<usize> = vec![0; values.len()];
    let mut pred: Vec<Option<usize>> = vec![None; values.len()];
    inc_sub_len[0] = 1;
    // Loop over all subsequences
    for k in 1..values.len() {
        let mut longest = 0;
        let mut idx_longest = None;
        for idx in 0..k {
            if values[idx] <= values[k] && inc_sub_len[idx] > longest {
                longest = inc_sub_len[idx];
                idx_longest = Some(idx);
            }
        }
        inc_sub_len[k] = longest + 1;
        pred[k] = idx_longest;
    }
    let mut idx_max = argmax(&inc_sub_len);
    let mut res = Vec::new();
    res.push(values[idx_max]);
    while let Some(idx) = pred[idx_max] {
        res.push(values[idx]);
        idx_max = idx;
    }
    res.reverse();
    res
}

fn argmax(values: &[usize]) -> usize {
    let mut idx_max = 0;
    for idx in 1..values.len() {
        if values[idx] > values[idx_max] {
            idx_max = idx;
        }
    }
    idx_max
}
