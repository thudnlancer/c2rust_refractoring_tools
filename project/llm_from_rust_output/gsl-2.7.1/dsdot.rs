pub fn cblas_dsdot(
    n: i32,
    x: &[f32],
    inc_x: i32,
    y: &[f32],
    inc_y: i32,
) -> f64 {
    assert!(n >= 0, "n must be non-negative");
    assert!(x.len() >= ((n - 1) * inc_x.abs()).max(0) as usize, "x array too small");
    assert!(y.len() >= ((n - 1) * inc_y.abs()).max(0) as usize, "y array too small");
    assert!(inc_x != 0, "inc_x must not be zero");
    assert!(inc_y != 0, "inc_y must not be zero");

    let mut result = 0.0;
    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        result += (x[ix as usize] * y[iy as usize]) as f64;
        ix += inc_x;
        iy += inc_y;
    }

    result
}