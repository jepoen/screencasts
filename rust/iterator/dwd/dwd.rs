// mod dwd
use std::io;

pub fn read_temp() -> Vec<f64> {
    io::stdin()
        .lines()
        .map(|res_line| res_line.unwrap()) // unwrap Result
        .map(|year_line| {
            year_line
                .split_whitespace() // split year-temp
                .skip(1) // skip year
                .next() // second column = Some(temp)
                .unwrap() // temp as &str
                .to_string()
        }) // split temp
        .map(|str_year| str_year.parse().unwrap()) // convert to number
        .collect()
}

/*
fn main() {
    let temp_list = read_temp();
    println!("{:?}", temp_list);
}
*/
