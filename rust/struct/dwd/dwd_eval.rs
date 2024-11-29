//! Evaluation of weather data (DWD)

use std::io;

// Struct
#[derive(Debug)]
struct Datum {
    year: u32,
    temp: Option<f64>,
    rain: Option<f64>,
    sun: Option<f64>,
}

fn get_temp(datum: &Datum) -> Option<f64> {
    datum.temp
}

fn get_rain(datum: &Datum) -> Option<f64> {
    datum.rain
}

fn get_sun(datum: &Datum) -> Option<f64> {
    datum.sun
}

fn main() {
    let mut data = Vec::new();
    /*let mut first_line_processed = false;
    for line in io::stdin().lines() {
        if !first_line_processed {
            first_line_processed = true
        } else {
            let datum = parse_datum(&line.unwrap());
            data.push(datum);
        }
    }
    */
    for line in io::stdin().lines().skip(1) {
        let datum = parse_datum(&line.unwrap());
        data.push(datum);
    }
    //println!("{:?}", data);
    show_data(&data);
    print!("Mean temp:");
    print_option(&mean_val(&data, get_temp));
    println!();
    print!("Mean rain:");
    print_option(&mean_val(&data, get_rain));
    println!();
    print!("Mean sun: ");
    print_option(&mean_val(&data, get_sun));
    println!();
    print!("Mean sun 1881-1900:");
    print_option(&mean_val(&data[..20], get_sun));
    println!();
}

fn parse_datum(line: &str) -> Datum {
    let parts: Vec<_> = line.split(',').collect();
    assert_eq!(parts.len(), 4);
    let year: u32 = parts[0].parse().unwrap();
    let temp = try_parse(parts[1]);
    let rain = try_parse(parts[2]);
    let sun = try_parse(parts[3]);
    //Datum{year: year, temp: temp, rain: rain, sun: sun}
    Datum {
        year,
        temp,
        rain,
        sun,
    }
}

fn try_parse(s: &str) -> Option<f64> {
    if let Ok(val) = s.parse() {
        Some(val)
    } else {
        None
    }
}

fn show_data(data: &[Datum]) {
    for datum in data {
        print!("{:4}", datum.year);
        print_option(&datum.temp);
        print_option(&datum.rain);
        print_option(&datum.sun);
        println!();
    }
}

fn print_option(opt_val: &Option<f64>) {
    match opt_val {
        Some(val) => print!(" {:7.1}", val),
        None => print!(" {:^7}", "---"),
    }
}

fn mean_val(data: &[Datum], get_field: fn(&Datum) -> Option<f64>) -> Option<f64> {
    let mut sum = 0.0;
    let mut count = 0;
    for datum in data {
        if let Some(val) = get_field(datum) {
            sum += val;
            count += 1;
        }
    }
    if count > 0 {
        Some(sum / count as f64)
    } else {
        None
    }
}
