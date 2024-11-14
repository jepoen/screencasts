//! Matrix-Vector multiplication

mod simple_input;
use simple_input::Input;

fn main() {
    let mut inp = Input::new();
    let matrix = read_matrix(&mut inp);
    let m_size = matrix.len();
    let vector = read_vector(&mut inp, m_size);
    println!("Matrix:");
    show_matrix(&matrix);
    println!("Vector:");
    show_vector(&vector);
    let res = mat_vec_mul(&matrix, &vector);
    println!("Result Matrix · Vector:");
    show_vector(&res);
    // eval result
    let v_expect = read_vector(&mut inp, m_size);
    // test vec_diff()
    //let v_diff = vec_diff(&res, &vector);
    let v_diff = vec_diff(&res, &v_expect);
    println!("Error:");
    show_vector(&v_diff);
}

fn read_matrix(inp: &mut Input) -> Vec<Vec<f64>> {
    let size = inp.read_int();
    let mut matrix: Vec<Vec<f64>> = Vec::new();
    for _row in 0..size {
        let mut mat_row: Vec<f64> = Vec::new();
        for _col in 0..size {
            let val = inp.read_float();
            mat_row.push(val);
        }
        matrix.push(mat_row);
    }
    matrix
}

fn read_vector(inp: &mut Input, size: usize) -> Vec<f64> {
    let mut vector: Vec<f64> = Vec::new();
    for _ in 0..size {
        let val = inp.read_float();
        vector.push(val);
    }
    vector
}

fn show_matrix(matrix: &[Vec<f64>]) {
    for row in matrix {
        for value in row {
            print!("{:8.2}", value);
        }
        println!();
    }
}

fn show_vector(vector: &[f64]) {
    for value in vector {
        print!("{:8.2}", value);
    }
    println!();
}

fn mat_vec_mul(matrix: &[Vec<f64>], vector: &[f64]) -> Vec<f64> {
    let mut res: Vec<f64> = Vec::new();
    let size = matrix.len();
    for row in 0..size {
        // row = i
        let mut sum = 0.0;
        for col in 0..size {
            // col = k
            sum += matrix[row][col] * vector[col];
        }
        res.push(sum); // res[i]
    }
    res
}

/* ori
fn vec_diff(v1: &[f64], v2: &[f64]) -> Vec<f64> {
    // alternative: for (e1, e2) in v1.iter().zip(v2)
    let size = v1.len();
    let mut v_diff: Vec<f64> = Vec::new();
    for idx in 0 .. size {
        v_diff.push(v2[idx] - v1[idx]);
    }
    v_diff
}
*/

// idiomatic solution with iterator
fn vec_diff(v1: &[f64], v2: &[f64]) -> Vec<f64> {
    let mut v_diff: Vec<f64> = Vec::new();
    for (e1, e2) in v1.iter().zip(v2) {
        //println!("{e1} {e2}");
        v_diff.push(e2 - e1);
    }
    v_diff
}
