mod simple_input;
use simple_input::Input;

fn main() {
    let data = read_points();
    //println!("{:?}", data);
    let (a, b) = linreg(&data);
    println!("#Model: y = {}x + {}", a, b);
    let mse = linreg_mse(&data, a, b);
    println!("#MSE: {:.03} RMSE: {:.03}", mse, mse.sqrt());
}

fn read_points() -> Vec<(f64, f64)> {
    let mut inp = Input::new();
    let mut points = Vec::new();
    let mut x = inp.read_float();
    let mut y = inp.read_float();
    while x >= 0.0 {
        points.push((x, y));
        x = inp.read_float();
        y = inp.read_float();
    }
    points
}

fn linreg(points: &[(f64, f64)]) -> (f64, f64) {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_xx = 0.0;
    let mut sum_xy = 0.0;
    let mut count = 0;
    let (mut a, mut b) = (0.0, 0.0);
    for &(x, y) in points {
        count += 1;
        sum_x += x;
        sum_y += y;
        sum_xx += sqr(x);
        sum_xy += x * y;
        let n_fl = count as f64;
        a = (n_fl * sum_xy - sum_x * sum_y) / (n_fl * sum_xx - sum_x * sum_x);
        b = (sum_y - a * sum_x) / n_fl;
        println!("{} {} {}", count, a, b);
    }
    (a, b)
}

fn linreg_mse(points: &[(f64, f64)], a: f64, b: f64) -> f64 {
    let mut mse = 0.0;
    for &(x, y) in points {
        let y_model = a * x + b;
        mse += sqr(y_model - y);
    }
    let n_fl = points.len() as f64;
    mse / n_fl
}

fn sqr(x: f64) -> f64 {
    x * x
}
