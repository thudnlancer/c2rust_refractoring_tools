use num_complex::Complex64;
use std::error::Error;
use std::result::Result;

pub fn cblas_zaxpy(
    n: usize,
    alpha: &Complex64,
    x: &[Complex64],
    inc_x: usize,
    y: &mut [Complex64],
    inc_y: usize,
) -> Result<(), Box<dyn Error>> {
    if x.len() < (n - 1) * inc_x + 1 {
        return Err("X vector length too small".into());
    }
    if y.len() < (n - 1) * inc_y + 1 {
        return Err("Y vector length too small".into());
    }

    for i in 0..n {
        let x_index = i * inc_x;
        let y_index = i * inc_y;
        y[y_index] += alpha * x[x_index];
    }

    Ok(())
}