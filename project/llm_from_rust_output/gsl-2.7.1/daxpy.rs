pub fn cblas_daxpy(
    n: i32,
    alpha: f64,
    x: &[f64],
    inc_x: i32,
    y: &mut [f64],
    inc_y: i32,
) {
    if alpha == 0.0 || n <= 0 {
        return;
    }

    if inc_x == 1 && inc_y == 1 {
        let chunks = n as usize / 4;
        let remainder = n as usize % 4;

        for i in 0..remainder {
            y[i] += alpha * x[i];
        }

        for i in 0..chunks {
            let base = remainder + i * 4;
            y[base] += alpha * x[base];
            y[base + 1] += alpha * x[base + 1];
            y[base + 2] += alpha * x[base + 2];
            y[base + 3] += alpha * x[base + 3];
        }
    } else {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } as usize;
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;

        for _ in 0..n {
            y[iy] += alpha * x[ix];
            ix = (ix as i32 + inc_x) as usize;
            iy = (iy as i32 + inc_y) as usize;
        }
    }
}