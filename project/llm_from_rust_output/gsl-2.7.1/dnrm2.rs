pub fn cblas_dnrm2(N: i32, X: &[f64], incX: i32) -> f64 {
    let mut scale = 0.0;
    let mut ssq = 1.0;
    
    if N <= 0 || incX <= 0 {
        return 0.0;
    } else if N == 1 {
        return X[0].abs();
    }

    let mut ix = 0;
    for _ in 0..N {
        let x = X[ix as usize];
        if x != 0.0 {
            let ax = x.abs();
            if scale < ax {
                ssq = 1.0 + ssq * (scale / ax).powi(2);
                scale = ax;
            } else {
                ssq += (ax / scale).powi(2);
            }
        }
        ix += incX;
        if ix as usize >= X.len() {
            break;
        }
    }
    scale * ssq.sqrt()
}