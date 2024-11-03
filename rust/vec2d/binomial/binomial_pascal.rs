//! binomial coefficient direct formula

mod simple_args;

fn main() {
    let args = simple_args::int_args();
    if args.len() != 2 {
        simple_args::usage("n k");
        return;
    }
    let n = args[0];
    let k = args[1];
    let pascal = create_pascal(n);
    show_pascal(&pascal);
    let res = binomial(&pascal, n, k);
    println!("binomial({}, {}) = {}", n, k, res);
}

fn create_pascal(n_max: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if n_max < 0 {
        return res;
    }
    for n in 0..=n_max as usize {
        let mut row = Vec::new();
        row.push(1); // binomial(n, 0)
        for k in 1..n {
            let val = res[n - 1][k - 1] + res[n - 1][k];
            row.push(val);
        }
        if n > 0 {
            row.push(1); // binomial(n, n)
        }
        res.push(row);
    }
    res
}

fn show_pascal(pascal: &[Vec<i32>]) {
    for row in pascal {
        for cell in row {
            print!(" {:3}", cell);
        }
        println!();
    }
}

fn binomial(pascal: &[Vec<i32>], n: i32, k: i32) -> i32 {
    // debug
    //print!(" ({},{})", n, k);
    if n < 0 || k < 0 || k > n {
        0
    } else {
        pascal[n as usize][k as usize]
    }
}
