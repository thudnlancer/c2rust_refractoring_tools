pub fn cblas_daxpy(
    n: usize,
    alpha: f64,
    x: &[f64],
    inc_x: usize,
    y: &mut [f64],
    inc_y: usize,
) -> Result<(), &'static str> {
    if n == 0 {
        return Ok(());
    }

    if x.len() < 1 + (n - 1) * inc_x {
        return Err("x vector length too small");
    }

    if y.len() < 1 + (n - 1) * inc_y {
        return Err("y vector length too small");
    }

    if inc_x == 0 || inc_y == 0 {
        return Err("increments cannot be zero");
    }

    for i in 0..n {
        let x_idx = i * inc_x;
        let y_idx = i * inc_y;
        y[y_idx] += alpha * x[x_idx];
    }

    Ok(())
}