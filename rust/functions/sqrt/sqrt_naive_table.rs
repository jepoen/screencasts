/// Wurzeltabelle ganze Zahlen

mod simple_args;

fn main() {
	let args = simple_args::int_args();
	if args.len() != 1 {
		simple_args::usage("max_argument");
		return;
	}
	let arg_max = args[0];
	sqrt_table(0, arg_max);
}

fn sqrt(a: i32) -> i32 {
	if a < 0 {
		return -1;
	}
	let x = try_sqrt(a, 0); // x² ≤ a < (x+1)²
	let err_0 = a - sqr(x);
	let err_1 = sqr(x+1) - a;
	if err_0 < err_1 {
		x
	} else {
		x + 1
	}
}

fn sqr(x: i32) -> i32 {
	x * x
}

// garantiert: x² ≤ a
fn try_sqrt(a: i32, x: i32) -> i32 {
	if sqr(x + 1) > a {
		x
	} else {
		try_sqrt(a, x + 1)
	}
}

fn sqrt_table(a: i32, arg_max: i32) {
	if a > arg_max {
		return;
	}
	let x = sqrt(a);
	println!("{} {}", a, x);
	sqrt_table(a + 1, arg_max);
}
