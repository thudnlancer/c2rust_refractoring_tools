pub fn cblas_ccopy(
    n: i32,
    x: &[f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
) {
    if n <= 0 || inc_x == 0 || inc_y == 0 {
        return;
    }

    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        let x_idx = (2 * ix) as usize;
        let y_idx = (2 * iy) as usize;

        if x_idx + 1 < x.len() && y_idx + 1 < y.len() {
            y[y_idx] = x[x_idx];
            y[y_idx + 1] = x[x_idx + 1];
        }

        ix += inc_x;
        iy += inc_y;
    }
}