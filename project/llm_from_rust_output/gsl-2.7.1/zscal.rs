use num_complex::Complex64;

#[no_mangle]
pub extern "C" fn cblas_zscal(
    N: i32,
    alpha: *const libc::c_void,
    X: *mut libc::c_void,
    incX: i32,
) {
    if incX <= 0 || N <= 0 {
        return;
    }

    let alpha_slice = unsafe { std::slice::from_raw_parts(alpha as *const f64, 2) };
    let alpha = Complex64::new(alpha_slice[0], alpha_slice[1]);

    let x_slice = unsafe { std::slice::from_raw_parts_mut(X as *mut f64, (2 * N as usize).max(0)) };

    let mut ix = 0;
    for _ in 0..N {
        let x = Complex64::new(x_slice[2 * ix], x_slice[2 * ix + 1]);
        let result = alpha * x;
        x_slice[2 * ix] = result.re;
        x_slice[2 * ix + 1] = result.im;
        ix += incX as usize;
    }
}