use std::f64;

pub type size_t = usize;

#[no_mangle]
pub fn cblas_icamax(N: i32, X: &[f32], incX: i32) -> size_t {
    let mut max = 0.0f32;
    let mut ix = 0;
    let mut result = 0;

    if incX <= 0 || N <= 0 {
        return result;
    }

    for i in 0..N {
        let real = X[(2 * ix) as usize];
        let imag = X[(2 * ix + 1) as usize];
        let a = (f64::from(real).abs() + f64::from(imag).abs()) as f32;

        if a > max {
            max = a;
            result = i as size_t;
        }

        ix += incX;
    }

    result
}