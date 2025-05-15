use num_complex::Complex32;

pub fn cblas_cdotc_sub(
    n: usize,
    x: &[Complex32],
    inc_x: usize,
    y: &[Complex32],
    inc_y: usize,
) -> Result<Complex32, &'static str> {
    if x.len() < (n - 1) * inc_x + 1 {
        return Err("X vector length too small");
    }
    if y.len() < (n - 1) * inc_y + 1 {
        return Err("Y vector length too small");
    }
    if inc_x == 0 || inc_y == 0 {
        return Err("Increment cannot be zero");
    }

    let mut result = Complex32::new(0.0, 0.0);
    for i in 0..n {
        let x_index = i * inc_x;
        let y_index = i * inc_y;
        result += x[x_index] * y[y_index].conj();
    }

    Ok(result)
}