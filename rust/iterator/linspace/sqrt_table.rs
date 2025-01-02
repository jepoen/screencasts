mod linspace;
use linspace::LinSpace;

fn main() {
    let it = LinSpace::new(0.0, 20.0, 50).unwrap();
    /*
    for val in it {
        println!("√{} = {}", val, val.sqrt());
    }
    */
    /*
    let table: Vec<_> = it.map(|v| (v, v.sqrt())).collect();
    //println!("{:?}", table);
    for (v, sq_val) in table {
        println!("{} {}", v, sq_val);
    }
    */
    for (v, sq_val) in it.map(|v| (v, v.sqrt())) {
        println!("{} {}", v, sq_val);
    }
}
