use std::f64;

pub fn cblas_dznrm2(N: i32, X: &[f64], incX: i32) -> f64 {
    let mut scale = 0.0;
    let mut ssq = 1.0;

    if N == 0 || incX < 1 {
        return 0.0;
    }

    let mut ix = 0;
    for _ in 0..N {
        let x = X[2 * ix as usize];
        let y = X[2 * ix as usize + 1];

        if x != 0.0 {
            let ax = x.abs();
            if scale < ax {
                ssq = 1.0 + ssq * (scale / ax).powi(2);
                scale = ax;
            } else {
                ssq += (ax / scale).powi(2);
            }
        }

        if y != 0.0 {
            let ay = y.abs();
            if scale < ay {
                ssq = 1.0 + ssq * (scale / ay).powi(2);
                scale = ay;
            } else {
                ssq += (ay / scale).powi(2);
            }
        }

        ix += incX;
    }

    scale * ssq.sqrt()
}