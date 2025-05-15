pub fn cblas_dzasum(N: i32, X: &[f64], incX: i32) -> f64 {
    if incX <= 0 || N <= 0 {
        return 0.0;
    }

    let mut r = 0.0;
    let mut ix = 0;

    for _ in 0..N {
        if ix * 2 + 1 >= X.len() {
            break;
        }
        r += X[ix * 2].abs() + X[ix * 2 + 1].abs();
        ix += incX as usize;
    }

    r
}