pub fn cblas_sdsdot(
    n: usize,
    alpha: f32,
    x: &[f32],
    inc_x: usize,
    y: &[f32],
    inc_y: usize,
) -> Result<f32, &'static str> {
    if n == 0 {
        return Ok(alpha);
    }
    
    if inc_x == 0 || inc_y == 0 {
        return Err("Increment cannot be zero");
    }
    
    if x.len() < 1 + (n - 1) * inc_x {
        return Err("X vector length too short");
    }
    
    if y.len() < 1 + (n - 1) * inc_y {
        return Err("Y vector length too short");
    }
    
    let mut acc: f64 = alpha.into();
    
    for i in 0..n {
        let x_idx = i * inc_x;
        let y_idx = i * inc_y;
        
        acc += f64::from(x[x_idx]) * f64::from(y[y_idx]);
    }
    
    Ok(acc as f32)
}