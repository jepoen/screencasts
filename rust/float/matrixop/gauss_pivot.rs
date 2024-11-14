//! Matrix-Vector multiplication

mod simple_input;
use simple_input::Input;

fn main() {
    let mut inp = Input::new();
    let matrix = read_matrix(&mut inp);
    let m_size = matrix.len();
    let rhs = read_vector(&mut inp, m_size);
    println!("Matrix:");
    show_matrix(&matrix);
    println!("Rhs:");
    show_vector(&rhs);
    let x = solve_sle(&matrix, &rhs);
    println!("Result:");
    show_vector(&x);
    // eval result → need copy of orig. matrix and rhs
    let prod = mat_vec_mul(&matrix, &x);
    let err = vec_diff(&prod, &rhs);
    println!("Error:");
    show_vector(&err);
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

fn vec_diff(v1: &[f64], v2: &[f64]) -> Vec<f64> {
    // alternative: for (e1, e2) in v1.iter().zip(v2)
    let size = v1.len();
    let mut v_diff: Vec<f64> = Vec::new();
    for idx in 0..size {
        v_diff.push(v2[idx] - v1[idx]);
    }
    v_diff
}

fn solve_sle(matrix_ori: &Vec<Vec<f64>>, rhs_ori: &Vec<f64>) -> Vec<f64> {
    // copy matrix_ori, rhs_ori
    let mut m = matrix_ori.clone();
    let mut rhs = rhs_ori.clone();
    let size = m.len();
    // forward pass
    for row in 0..size {
        // search pivot row: |m[r_next][row]| → max
        let r_pivot = get_pivot(&m, row);
        if r_pivot != row {
            // not possible -- moves single element to new owner:
            //(m[r_pivot], m[row]) = (m[row], m[r_pivot]);
            // same problem:
            //let tmp = m[r_pivot];
            //m[r_pivot] = m[row];
            //m[row] = tmp;
            m.swap(row, r_pivot);
            rhs.swap(row, r_pivot);
        }
        let m_rr = m[row][row];
        for col in row..size {
            m[row][col] /= m_rr;
        }
        rhs[row] /= m_rr;
        // following rows: subtract factor*row
        for r_next in row + 1..size {
            let q = m[r_next][row];
            for col in row..size {
                m[r_next][col] -= q * m[row][col];
            }
            rhs[r_next] -= q * rhs[row];
        }
        println!("row: {}", row);
        show_matrix(&m);
    }
    // backward pass
    let mut x = vec![0.0; size];
    let mut row = size;
    while row > 0 {
        row -= 1;
        let mut sum = 0.0;
        for col in row + 1..size {
            sum += m[row][col] * x[col];
        }
        x[row] = rhs[row] - sum;
    }
    x
}

fn get_pivot(m: &[Vec<f64>], row: usize) -> usize {
    let mut r_pivot = row;
    for r_next in row + 1..m.len() {
        if m[r_next][row].abs() > m[r_pivot][row].abs() {
            r_pivot = r_next;
        }
    }
    r_pivot
}
