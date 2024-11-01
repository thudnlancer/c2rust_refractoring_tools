#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cblas_dasum(
    N: libc::c_int,
    mut X: *const libc::c_double,
    incX: libc::c_int,
) -> libc::c_double {
    let mut r: libc::c_double = 0.0f64;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if incX <= 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < N {
        r += fabs(*X.offset(ix as isize));
        ix += incX;
        i += 1;
        i;
    }
    return r;
}
