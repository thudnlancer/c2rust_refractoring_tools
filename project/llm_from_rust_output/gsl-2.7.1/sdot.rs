pub fn cblas_sdot(
    n: i32,
    x: &[f32],
    inc_x: i32,
    y: &[f32],
    inc_y: i32,
) -> f32 {
    let mut result = 0.0;
    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        result += x[ix as usize] * y[iy as usize];
        ix += inc_x;
        iy += inc_y;
    }

    result
}