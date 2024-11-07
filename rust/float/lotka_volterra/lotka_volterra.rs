//! Lotka-Volterra-Equations

mod simple_args;

const EPOCHS: u32 = 1000;
const T_STEP: f64 = 0.1;
fn main() {
    let args = simple_args::float_args();
    if args.len() != 6 {
        simple_args::usage("prey predator prey_grow prey_death predator_grow predator_death");
        return;
    }
    let x0 = args[0];
    let y0 = args[1];
    let a = args[2];
    let b = args[3];
    let d = args[4]; // (swapped)
    let c = args[5];
    let (mut x, mut y) = (x0, y0);
    let mut t = 0.0;
    for _ in 0..EPOCHS {
        t += T_STEP;
        (x, y) = lotka_volterra(x, y, a, b, c, d, T_STEP);
        println!("{} {} {}", t, x, y);
    }
}

fn lotka_volterra(mut x: f64, mut y: f64, a: f64, b: f64, c: f64, d: f64, t: f64) -> (f64, f64) {
    const N_SPLIT: u32 = 1_000;
    let dt = t / (N_SPLIT as f64);
    for _ in 0..N_SPLIT {
        let dx = a * x - b * x * y;
        let dy = -c * y + d * x * y;
        (x, y) = (x + dt * dx, y + dt * dy);
    }
    (x, y)
}
