pub fn cblas_srot(
    n: i32,
    x: &mut [f32],
    inc_x: i32,
    y: &mut [f32],
    inc_y: i32,
    c: f32,
    s: f32,
) {
    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        let x_val = x[ix as usize];
        let y_val = y[iy as usize];
        
        x[ix as usize] = c * x_val + s * y_val;
        y[iy as usize] = -s * x_val + c * y_val;
        
        ix += inc_x;
        iy += inc_y;
    }
}