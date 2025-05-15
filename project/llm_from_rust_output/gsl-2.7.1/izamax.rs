use std::f64;

pub type size_t = usize;

pub fn cblas_izamax(N: i32, X: &[f64], incX: i32) -> size_t {
    let mut max = 0.0f64;
    let mut ix = 0;
    let mut result = 0 as size_t;

    if incX <= 0 {
        return result;
    }

    for i in 0..N {
        let real = X[(2 * ix) as usize];
        let imag = X[(2 * ix + 1) as usize];
        let a = f64::abs(real) + f64::abs(imag);

        if a > max {
            max = a;
            result = i as size_t;
        }

        ix += incX;
    }

    result
}