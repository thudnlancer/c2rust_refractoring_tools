pub fn cblas_scopy(
    n: usize,
    x: &[f32],
    inc_x: usize,
    y: &mut [f32],
    inc_y: usize,
) -> Result<(), &'static str> {
    if n == 0 {
        return Ok(());
    }

    if x.len() < 1 + (n - 1) * inc_x {
        return Err("x array too short");
    }

    if y.len() < 1 + (n - 1) * inc_y {
        return Err("y array too short");
    }

    if inc_x == 1 && inc_y == 1 {
        y[..n].copy_from_slice(&x[..n]);
    } else {
        for i in 0..n {
            y[i * inc_y] = x[i * inc_x];
        }
    }

    Ok(())
}