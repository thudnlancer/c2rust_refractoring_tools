use num_complex::Complex64;
use std::f64;

#[no_mangle]
pub extern "C" fn cblas_zaxpy(
    n: i32,
    alpha: *const std::ffi::c_void,
    x: *const std::ffi::c_void,
    inc_x: i32,
    y: *mut std::ffi::c_void,
    inc_y: i32,
) {
    if n <= 0 {
        return;
    }

    let alpha_slice = unsafe { std::slice::from_raw_parts(alpha as *const f64, 2) };
    let alpha = Complex64::new(alpha_slice[0], alpha_slice[1]);

    if alpha.norm_sqr() == 0.0 {
        return;
    }

    let x_slice = unsafe { std::slice::from_raw_parts(x as *const f64, (2 * n * inc_x.abs()) as usize) };
    let y_slice = unsafe { std::slice::from_raw_parts_mut(y as *mut f64, (2 * n * inc_y.abs()) as usize) };

    let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
    let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

    for _ in 0..n {
        let x_real = x_slice[(2 * ix) as usize];
        let x_imag = x_slice[(2 * ix + 1) as usize];
        let x = Complex64::new(x_real, x_imag);
        let product = alpha * x;

        y_slice[(2 * iy) as usize] += product.re;
        y_slice[(2 * iy + 1) as usize] += product.im;

        ix += inc_x;
        iy += inc_y;
    }
}