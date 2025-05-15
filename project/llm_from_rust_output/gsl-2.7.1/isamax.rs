pub fn cblas_isamax(N: i32, X: &[f32], incX: i32) -> usize {
    if incX <= 0 || N <= 0 || X.is_empty() {
        return 0;
    }

    let mut max = 0.0f32;
    let mut result = 0usize;
    let mut ix = 0usize;

    for i in 0..N {
        let current = X[ix].abs();
        if current > max {
            max = current;
            result = i as usize;
        }
        ix = (ix as i32 + incX) as usize;
        if ix >= X.len() {
            break;
        }
    }

    result
}