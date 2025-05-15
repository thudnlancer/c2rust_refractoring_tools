pub fn cblas_dcopy(n: usize, x: &[f64], inc_x: usize, y: &mut [f64], inc_y: usize) -> Result<(), &'static str> {
    if x.len() < (n - 1) * inc_x + 1 {
        return Err("x vector too short");
    }
    if y.len() < (n - 1) * inc_y + 1 {
        return Err("y vector too short");
    }
    if inc_x == 0 || inc_y == 0 {
        return Err("increment cannot be zero");
    }

    for i in 0..n {
        let x_idx = i * inc_x;
        let y_idx = i * inc_y;
        y[y_idx] = x[x_idx];
    }

    Ok(())
}