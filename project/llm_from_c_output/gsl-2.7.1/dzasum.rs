pub fn cblas_dzasum(n: i32, x: &[f64], inc_x: i32) -> f64 {
    if n <= 0 || inc_x <= 0 {
        return 0.0;
    }

    let mut sum = 0.0;
    let mut ix = 0;
    
    for _ in 0..n {
        sum += x[ix].abs();
        ix = (ix + inc_x as usize) % x.len();
    }
    
    sum
}