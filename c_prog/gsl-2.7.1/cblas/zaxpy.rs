#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cblas_zaxpy(
    N: libc::c_int,
    mut alpha: *const libc::c_void,
    mut X: *const libc::c_void,
    incX: libc::c_int,
    mut Y: *mut libc::c_void,
    incY: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = if incX > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incX
    };
    let mut iy: libc::c_int = if incY > 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (N - 1 as libc::c_int) * -incY
    };
    let alpha_real: libc::c_double = *(alpha as *const libc::c_double)
        .offset(0 as libc::c_int as isize);
    let alpha_imag: libc::c_double = *(alpha as *const libc::c_double)
        .offset(1 as libc::c_int as isize);
    if fabs(alpha_real) == 0 as libc::c_int as libc::c_double
        && fabs(alpha_imag) == 0 as libc::c_int as libc::c_double
    {
        return;
    }
    i = 0 as libc::c_int;
    while i < N {
        let x_real: libc::c_double = *(X as *const libc::c_double)
            .offset((2 as libc::c_int * ix) as isize);
        let x_imag: libc::c_double = *(X as *const libc::c_double)
            .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize);
        *(Y as *mut libc::c_double).offset((2 as libc::c_int * iy) as isize)
            += alpha_real * x_real - alpha_imag * x_imag;
        *(Y as *mut libc::c_double)
            .offset((2 as libc::c_int * iy + 1 as libc::c_int) as isize)
            += alpha_real * x_imag + alpha_imag * x_real;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}
