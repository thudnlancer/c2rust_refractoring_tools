pub fn cblas_dasum(N: i32, X: &[f64], incX: i32) -> f64 {
    if incX <= 0 {
        return 0.0;
    }

    let mut r = 0.0;
    let mut ix = 0;
    
    for _ in 0..N {
        if ix >= X.len() as i32 {
            break;
        }
        r += X[ix as usize].abs();
        ix += incX;
    }
    
    r
}