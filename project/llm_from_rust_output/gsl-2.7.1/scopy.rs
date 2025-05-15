pub fn cblas_scopy(
    n: i32,
    x: &[f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
) {
    assert!(n >= 0, "n must be non-negative");
    assert!(
        x.len() >= ((n - 1) * inc_x.abs()).max(0) as usize,
        "x slice too small"
    );
    assert!(
        y.len() >= ((n - 1) * inc_y.abs()).max(0) as usize,
        "y slice too small"
    );

    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        y[iy as usize] = x[ix as usize];
        ix += inc_x;
        iy += inc_y;
    }
}