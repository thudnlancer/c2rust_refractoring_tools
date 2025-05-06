#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn cblas_cdotu_sub(
    N: i32,
    mut X: *const libc::c_void,
    incX: i32,
    mut Y: *const libc::c_void,
    incY: i32,
    mut result: *mut libc::c_void,
) {
    let mut r_real: libc::c_float = 0.0f64 as libc::c_float;
    let mut r_imag: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: i32 = 0;
    let mut ix: i32 = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
    let mut iy: i32 = if incY > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incY };
    i = 0 as i32;
    while i < N {
        let x_real: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as i32 * ix) as isize);
        let x_imag: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as i32 * ix + 1 as i32) as isize);
        let y_real: libc::c_float = *(Y as *const libc::c_float)
            .offset((2 as i32 * iy) as isize);
        let y_imag: libc::c_float = *(Y as *const libc::c_float)
            .offset((2 as i32 * iy + 1 as i32) as isize);
        r_real = (r_real as libc::c_double
            + ((x_real * y_real) as libc::c_double
                - 1.0f64 * x_imag as libc::c_double * y_imag as libc::c_double))
            as libc::c_float;
        r_imag = (r_imag as libc::c_double
            + ((x_real * y_imag) as libc::c_double
                + 1.0f64 * x_imag as libc::c_double * y_real as libc::c_double))
            as libc::c_float;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
    *(result as *mut libc::c_float).offset(0 as i32 as isize) = r_real;
    *(result as *mut libc::c_float).offset(1 as i32 as isize) = r_imag;
}