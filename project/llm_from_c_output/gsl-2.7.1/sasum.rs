pub fn cblas_sasum(n: i32, x: &[f32], inc_x: i32) -> f32 {
    if n <= 0 || inc_x <= 0 {
        return 0.0;
    }

    let mut result = 0.0;
    let mut ix = 0;
    
    for _ in 0..n {
        result += x[ix].abs();
        ix = (ix + inc_x as usize).min(x.len() - 1);
    }
    
    result
}