//! zero of polynomial of degree 3

mod simple_args;

fn main() {
    let args = simple_args::float_args();
    if args.len() != 4 {
        simple_args::usage("a₃ a₂ a₁ a₀");
        return;
    }
    // a₀ ... a₃
    let coeff = vec![args[3], args[2], args[1], args[0]];
    let x_zero = bisec_poly(&coeff);
    let y = poly(x_zero, &coeff);
    println!("p({}) = {}", x_zero, y);
}

fn poly(x: f64, coeff: &[f64]) -> f64 {
    let mut res = 0.0;
    for i in 0..coeff.len() {
        res = res * x + coeff[coeff.len() - 1 - i];
    }
    res
}

const EPS: f64 = 1e-6;

fn bisec_poly(coeff: &[f64]) -> f64 {
    let (mut x_left, mut x_right) = find_args_alternate(&coeff);
    println!(
        "x_left = {}, p(x_left) = {}, x_right = {} p(x_right) = {}",
        x_left,
        poly(x_left, coeff),
        x_right,
        poly(x_right, coeff)
    );
    let sign_l = sign(poly(x_left, coeff));
    let sign_r = sign(poly(x_right, coeff));
    assert_eq!(sign_l * sign_r, -1);
    while x_right - x_left > EPS {
        let x_m = (x_left + x_right) / 2.0;
        let y_m = poly(x_m, coeff);
        println!("p({}) = {}", x_m, y_m);
        if y_m == 0.0 {
            return x_m;
        }
        if sign(y_m) * sign_l == 1 {
            // same signum as x_left
            x_left = x_m;
        } else {
            x_right = x_m;
        }
    }
    (x_left + x_right) / 2.0
}

fn find_args_alternate(coeff: &[f64]) -> (f64, f64) {
    let (mut x_left, mut x_right) = (-1.0, 1.0);
    let (mut y_left, mut y_right) = (poly(x_left, coeff), poly(x_right, coeff));
    while sign(y_left) * sign(y_right) == 1 {
        x_left *= 2.0;
        x_right *= 2.0;
        (y_left, y_right) = (poly(x_left, coeff), poly(x_right, coeff));
    }
    (x_left, x_right)
}

fn sign(x: f64) -> i32 {
    if x < 0.0 {
        -1
    } else if x > 0.0 {
        1
    } else {
        // x == 0
        0
    }
}
