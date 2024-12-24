// eval_energy.rs
use std::io;

mod energy_mix;
use energy_mix::EnergyMix;

fn main() {
    let mix = read_input();
    mix.show();
    mix.mermaid(80.0);
    if let Some(ratio) = mix.change_relative("Windkraft", 2019, 2023) {
        println!("Zuwachs Wind: {:.1}%", ratio * 100.0);
    }
    println!();
    println!("# Ausbau Photovolaik");
    println!();
    for year in 2019..2030 {
        if let Some(ratio) = mix.ratio("Photovoltaik", year) {
            println!("- Anteil im Jahr {}: {:.1}%", year, ratio * 100.0);
        }
    }
    println!();
    println!("# Gesamtenergie");
    println!();
    for year in 2019..2030 {
        if let Some(sum) = mix.total_energy(year) {
            println!("- Jahr {}: {:.1} Mrd. kWh", year, sum);
        }
    }
}

fn read_input() -> EnergyMix {
    let mut is_head = true;
    let mut years = Vec::new();
    let mut kinds = Vec::new();
    let mut data = Vec::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let chars: Vec<_> = line.chars().collect();
        if chars.len() == 0 || chars[0] == '#' {
            continue;
        }
        if is_head {
            years = parse_head(&line);
            is_head = false;
        } else {
            let (kind, values) = parse_data(&line);
            kinds.push(kind);
            data.push(values);
        }
    }
    EnergyMix::new(kinds, years, data)
}

fn parse_head(line: &str) -> Vec<u32> {
    let parts: Vec<_> = line.split(",").skip(1).step_by(2).collect();
    let mut res = Vec::new();
    for part in parts {
        let year_parts: Vec<_> = part.split("_").collect();
        res.push(year_parts[0].parse().unwrap());
    }
    res
}

fn parse_data(line: &str) -> (String, Vec<f64>) {
    let mut it = line.split(",");
    let kind = it.next().unwrap().to_string();
    let mut data = Vec::new();
    for part in it.step_by(2) {
        let val = part.parse().unwrap();
        data.push(val);
    }
    (kind, data)
}
