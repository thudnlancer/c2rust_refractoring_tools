use ndarray::{Array1, ArrayView1};
use num_complex::Complex64;

pub fn cblas_zcopy(
    n: usize,
    x: ArrayView1<Complex64>,
    inc_x: usize,
    y: &mut Array1<Complex64>,
    inc_y: usize,
) -> Result<(), String> {
    if x.len() < n * inc_x {
        return Err("X array length is too small".to_string());
    }
    if y.len() < n * inc_y {
        return Err("Y array length is too small".to_string());
    }
    if inc_x == 0 || inc_y == 0 {
        return Err("Increments cannot be zero".to_string());
    }

    for i in 0..n {
        let x_idx = i * inc_x;
        let y_idx = i * inc_y;
        y[y_idx] = x[x_idx];
    }

    Ok(())
}