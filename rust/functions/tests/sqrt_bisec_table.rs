/// Tests: Wurzeltabelle ganze Zahlen

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
	let x = try_sqrt(a, 0, a + 1); // x² ≤ a < (x+1)²
	let err_0 = a - sqr(x);
	let err_1 = sqr(x+1) - a;
	if err_0 < err_1 {
		x
	} else {
		x + 1
	}
}

// Sinnvollerer Test für Austausch Algorithmus Wurzelberechnung
#[test]
fn test_sqrt() {
	assert_eq!(sqrt(10), 3);
	assert_eq!(sqrt(15), 4);
	// Randfälle
	assert_eq!(sqrt(-42), -1);
	assert_eq!(sqrt(0), 0);
	assert_eq!(sqrt(1), 1);
}

fn sqr(x: i32) -> i32 {
	x * x
}

// 1. Gruppe von Tests
#[test]
fn test_sqr1() {
	assert_eq!(sqr(3), 9);
	assert_eq!(sqr(0), 0);
}

// 2. Gruppe von Tests
#[test]
fn test_sqr2() {
	assert_eq!(sqr(1), 1);
	assert_eq!(sqr(-3), 9);
}

// TODO: Test der Garantien
// garantiert: x_l² ≤ a < x_r² entspricht x_l² - a ≤ 0 < x_r² - a
fn try_sqrt(a: i32, x_l: i32, x_r: i32) -> i32 {
	assert!(sqr(x_l) <= a, "{}² ≤ {} failed", x_l, a);
	assert!(sqr(x_r) > a, "{}² > {} failed", x_r, a);
	// automatisch: a ≥ 0, x_l < x_r
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

// Test: linker, rechter Rand, Abbruchbedingung, x_l < x_r
#[test]
fn test_try_sqrt() {
	// smoke tests
	assert_eq!(try_sqrt(10, 0, 10), 3);
	assert_eq!(try_sqrt(15, 0, 5), 3);
	// x_l == x_r, x_l² ≤ a < x_r² verletzt
	//assert_eq!(try_sqrt(10, 3, 3), 3); → Absturz
	//assert_ne!(try_sqrt(10, 1, 3), 3); → ungünstiger Test
	// Randfälle
	assert_eq!(try_sqrt(0, 0, 10), 0);
	assert_eq!(try_sqrt(1, 0, 2), 1);
	assert_eq!(try_sqrt(10, 3, 4), 3);
	assert_eq!(try_sqrt(15, 3, 4), 3);
}

fn sqrt_table(a: i32, arg_max: i32) {
	if a > arg_max {
		return;
	}
	let x = sqrt(a);
	println!("{} {}", a, x);
	sqrt_table(a + 1, arg_max);
}
