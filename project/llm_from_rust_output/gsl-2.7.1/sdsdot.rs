pub fn cblas_sdsdot(
    n: i32,
    alpha: f32,
    x: &[f32],
    inc_x: i32,
    y: &[f32],
    inc_y: i32,
) -> f32 {
    assert!(n >= 0, "n must be non-negative");
    assert!(inc_x != 0, "inc_x must not be zero");
    assert!(inc_y != 0, "inc_y must not be zero");
    
    let mut r = alpha as f64;
    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
    
    for _ in 0..n {
        let x_val = if ix >= 0 && (ix as usize) < x.len() {
            x[ix as usize]
        } else {
            0.0
        };
        
        let y_val = if iy >= 0 && (iy as usize) < y.len() {
            y[iy as usize]
        } else {
            0.0
        };
        
        r += (x_val * y_val) as f64;
        ix += inc_x;
        iy += inc_y;
    }
    
    r as f32
}