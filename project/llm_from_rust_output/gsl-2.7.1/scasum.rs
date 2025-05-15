pub fn cblas_scasum(N: i32, X: &[f32], incX: i32) -> f32 {
    if incX <= 0 || N <= 0 {
        return 0.0;
    }

    let mut r = 0.0f32;
    let mut ix = 0usize;

    for _ in 0..N {
        let real = X[2 * ix];
        let imag = X[2 * ix + 1];
        r += (real.abs() + imag.abs()) as f32;
        
        ix = (ix as i32 + incX) as usize;
        if ix * 2 + 1 >= X.len() {
            break;
        }
    }

    r
}