pub fn cblas_srot(
    n: usize,
    x: &mut [f32],
    inc_x: usize,
    y: &mut [f32],
    inc_y: usize,
    c: f32,
    s: f32,
) -> Result<(), &'static str> {
    if n == 0 {
        return Ok(());
    }
    
    if inc_x == 0 || inc_y == 0 {
        return Err("Increment cannot be zero");
    }
    
    if x.len() < 1 + (n - 1) * inc_x {
        return Err("X vector too short");
    }
    
    if y.len() < 1 + (n - 1) * inc_y {
        return Err("Y vector too short");
    }
    
    for i in 0..n {
        let xi = x[i * inc_x];
        let yi = y[i * inc_y];
        
        x[i * inc_x] = c * xi + s * yi;
        y[i * inc_y] = c * yi - s * xi;
    }
    
    Ok(())
}