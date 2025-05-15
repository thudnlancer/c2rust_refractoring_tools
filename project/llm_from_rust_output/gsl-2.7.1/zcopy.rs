use libc::{c_int, c_void, c_double};

#[no_mangle]
pub extern "C" fn cblas_zcopy(
    N: c_int,
    X: *const c_void,
    incX: c_int,
    Y: *mut c_void,
    incY: c_int,
) {
    if N <= 0 {
        return;
    }

    let x_ptr = X as *const c_double;
    let y_ptr = Y as *mut c_double;

    let mut ix = if incX > 0 { 0 } else { (N - 1) * -incX };
    let mut iy = if incY > 0 { 0 } else { (N - 1) * -incY };

    for _ in 0..N {
        unsafe {
            *y_ptr.offset((2 * iy) as isize) = *x_ptr.offset((2 * ix) as isize);
            *y_ptr.offset((2 * iy + 1) as isize) = *x_ptr.offset((2 * ix + 1) as isize);
        }
        ix += incX;
        iy += incY;
    }
}