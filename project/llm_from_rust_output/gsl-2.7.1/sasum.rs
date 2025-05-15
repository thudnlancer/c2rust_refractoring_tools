pub fn cblas_sasum(N: i32, X: &[f32], incX: i32) -> f32 {
    if incX <= 0 {
        return 0.0;
    }

    let mut r = 0.0f32;
    let mut ix = 0usize;
    
    for _ in 0..N {
        if ix >= X.len() {
            break;
        }
        r += X[ix].abs();
        ix = ix.wrapping_add(incX as usize);
    }

    r
}