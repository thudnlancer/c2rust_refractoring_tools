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
pub unsafe extern "C" fn cblas_zscal(
    N: i32,
    mut alpha: *const libc::c_void,
    mut X: *mut libc::c_void,
    incX: i32,
) {
    let mut i: i32 = 0;
    let mut ix: i32 = 0 as i32;
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as i32 as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as i32 as isize);
    if incX <= 0 as i32 {
        return;
    }
    i = 0 as i32;
    while i < N {
        let x_real: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as i32 * ix) as isize);
        let x_imag: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as i32 * ix + 1 as i32) as isize);
        *(X as *mut libc::c_double).offset((2 as i32 * ix) as isize) = x_real
            * alpha_real - x_imag * alpha_imag;
        *(X as *mut libc::c_double).offset((2 as i32 * ix + 1 as i32) as isize) = x_real
            * alpha_imag + x_imag * alpha_real;
        ix += incX;
        i += 1;
        i;
    }
}