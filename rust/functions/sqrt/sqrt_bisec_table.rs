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
	//println!("√{} ≈ {}", arg_max, sqrt(arg_max));
}

fn sqrt(a: i32) -> i32 {
	if a < 0 {
		return -1;
	}
	/* alternativ
	if a < 2 {
		return a;
	}
	* */
	let x = try_sqrt(a, 0, a + 1); // x² ≤ a < (x+1)²
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

// garantiert: x_l² ≤ a < x_r² entspricht x_l² - a ≤ 0 < x_r² - a
fn try_sqrt(a: i32, x_l: i32, x_r: i32) -> i32 {
	if x_r - x_l == 1 {
		return x_l;
	}
	let x_m = (x_l + x_r)/2;
	//println!("{} {} {}", x_l, x_r, x_m);
	let f_x_m = sqr(x_m) - a;
	if f_x_m > 0 {
		// x_r ersetzen
		try_sqrt(a, x_l, x_m)
	} else {
		// x_l ersetzen
		try_sqrt(a, x_m, x_r)
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
