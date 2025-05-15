pub fn cblas_sswap(n: usize, x: &mut [f32], inc_x: usize, y: &mut [f32], inc_y: usize) -> Result<(), &'static str> {
    if n == 0 {
        return Ok(());
    }

    if inc_x == 0 || inc_y == 0 {
        return Err("Increment cannot be zero");
    }

    let x_len = x.len();
    let y_len = y.len();
    
    let required_x_len = if n > 0 { 1 + (n - 1) * inc_x } else { 0 };
    let required_y_len = if n > 0 { 1 + (n - 1) * inc_y } else { 0 };
    
    if x_len < required_x_len || y_len < required_y_len {
        return Err("Input arrays too small for given dimensions");
    }

    for i in 0..n {
        let x_index = i * inc_x;
        let y_index = i * inc_y;
        
        let temp = x[x_index];
        x[x_index] = y[y_index];
        y[y_index] = temp;
    }

    Ok(())
}