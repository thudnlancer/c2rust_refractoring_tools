pub fn cblas_idamax(N: i32, X: &[f64], incX: i32) -> usize {
    if incX <= 0 || N <= 0 || X.is_empty() {
        return 0;
    }

    let mut max = 0.0;
    let mut result = 0;
    let mut ix = 0;

    for i in 0..N {
        let current = X[ix as usize].abs();
        if current > max {
            max = current;
            result = i as usize;
        }
        ix += incX;
        if ix as usize >= X.len() {
            break;
        }
    }

    result
}