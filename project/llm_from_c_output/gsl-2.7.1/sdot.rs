pub fn cblas_sdot(n: i32, x: &[f32], inc_x: i32, y: &[f32], inc_y: i32) -> f32 {
    if n <= 0 {
        return 0.0;
    }

    let mut sum = 0.0;
    let mut x_idx = 0;
    let mut y_idx = 0;
    
    for _ in 0..n {
        sum += x[x_idx as usize] * y[y_idx as usize];
        x_idx += inc_x;
        y_idx += inc_y;
        
        if x_idx as usize >= x.len() || y_idx as usize >= y.len() {
            break;
        }
    }
    
    sum
}