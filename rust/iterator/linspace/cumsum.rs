mod linspace;
use linspace::LinSpace;

fn main() {
    let it = LinSpace::new(0.0, 20.0, 15).unwrap();
    println!("CumSum");
    for (val, cs) in it.scan(0.0, |state, v| {
        *state += v;
        Some((v, *state))
    }) {
        println!("{:.2} {:.2}", val, cs);
    }
}
