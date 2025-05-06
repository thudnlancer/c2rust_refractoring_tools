#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cblas_caxpy(
    N: i32,
    mut alpha: *const libc::c_void,
    mut X: *const libc::c_void,
    incX: i32,
    mut Y: *mut libc::c_void,
    incY: i32,
) {
    let mut i: i32 = 0;
    let mut ix: i32 = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
    let mut iy: i32 = if incY > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incY };
    let alpha_real: libc::c_float = *(alpha as *const libc::c_float)
        .offset(0 as i32 as isize);
    let alpha_imag: libc::c_float = *(alpha as *const libc::c_float)
        .offset(1 as i32 as isize);
    if fabs(alpha_real as libc::c_double) == 0 as i32 as libc::c_double
        && fabs(alpha_imag as libc::c_double) == 0 as i32 as libc::c_double
    {
        return;
    }
    i = 0 as i32;
    while i < N {
        let x_real: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as i32 * ix) as isize);
        let x_imag: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as i32 * ix + 1 as i32) as isize);
        *(Y as *mut libc::c_float).offset((2 as i32 * iy) as isize)
            += alpha_real * x_real - alpha_imag * x_imag;
        *(Y as *mut libc::c_float).offset((2 as i32 * iy + 1 as i32) as isize)
            += alpha_real * x_imag + alpha_imag * x_real;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}