use std::f64;
use std::ptr;

pub fn cblas_scnrm2(N: i32, X: &[f32], incX: i32) -> f32 {
    let mut scale = 0.0f32;
    let mut ssq = 1.0f32;
    
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
                ssq = (1.0 + (ssq * (scale / ax) * (scale / ax)) as f32;
                scale = ax;
            } else {
                ssq += (ax / scale) * (ax / scale);
            }
        }

        if y != 0.0 {
            let ay = y.abs();
            if scale < ay {
                ssq = (1.0 + (ssq * (scale / ay) * (scale / ay)) as f32;
                scale = ay;
            } else {
                ssq += (ay / scale) * (ay / scale);
            }
        }

        ix += incX;
    }

    (scale as f64 * f64::sqrt(ssq as f64)) as f32
}