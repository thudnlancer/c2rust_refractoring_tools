use ndarray::{array, Array2};
use ndarray_linalg::Scalar;

fn main() {
    // Initialize matrices
    let a = array![
        [0.11, 0.12, 0.13],
        [0.21, 0.22, 0.23]
    ];
    let b = array![
        [1011.0, 1012.0],
        [1021.0, 1022.0],
        [1031.0, 1032.0]
    ];
    let mut c = Array2::zeros((2, 2));

    // Compute C = A B
    c = a.dot(&b);

    // Print the result
    println!("[ {}, {}", c[[0, 0]], c[[0, 1]]);
    println!("  {}, {} ]", c[[1, 0]], c[[1, 1]]);
}