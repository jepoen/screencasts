mod linspace;
use linspace::LinSpace;

fn main() {
    let mut it = LinSpace::new(0.0, 5.0, 2).unwrap();
    println!("{:?}", it.next()); // 0.0
    println!("{:?}", it.next()); // 2.5
    println!("{:?}", it.next()); // 5.0
    println!("{:?}", it.next()); // None
                                 // Verwenden in Schleife
    println!("while loop");
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    while let Some(val) = it.next() {
        print!(" {:.2}", val);
    }
    println!();
    println!("for loop");
    for val in LinSpace::new(0.0, 10.0, 15).unwrap() {
        print!(" {:.2}", val);
    }
    println!();
    // Consumer
    println!("Werte einsammeln");
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    let v: Vec<_> = it.collect();
    println!("{:?}", v);
    println!("Summe");
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    println!("{}", it.sum::<f64>());
    println!("Anzahl");
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    println!("{}", it.count());
    println!("Bedingung");
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    println!("alle Werte > 5? {}", it.all(|v| v > 5.0));
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    println!("existieren Werte > 5? {}", it.any(|v| v > 5.0));
    // Adapter
    println!("Filter");
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    let v5: Vec<_> = it.filter(|v| *v > 5.0).collect();
    println!("Werte > 5: {:?}", v5);
    println!("Skip und Take");
    it = LinSpace::new(0.0, 10.0, 15).unwrap();
    // überspringe 1. Wert, dann nimm jeden 2. Wert und brich nach 4 Werten ab
    let v_st: Vec<_> = it.skip(1).step_by(2).take(4).collect();
    println!("ohne 1. Werte, dann 4x jeden 2. Wert: {:?}", v_st);
    // map und enumerate
    it = LinSpace::new(0.0, 10.0, 10).unwrap();
    println!("Wurzeln");
    for val in it.map(|v| v.sqrt()).enumerate() {
        print!(" {:?}", val);
    }
    it = LinSpace::new(10.0, 20.0, 10).unwrap();
    println!();
    println!("Wurzeln 2");
    for val in it.map(|v| v.sqrt()).zip(10..15) {
        print!(" {:?}", val);
    }
    println!();
}
