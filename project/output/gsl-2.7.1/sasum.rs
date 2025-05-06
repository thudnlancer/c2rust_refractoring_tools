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
pub unsafe extern "C" fn cblas_sasum(
    N: i32,
    mut X: *const libc::c_float,
    incX: i32,
) -> libc::c_float {
    let mut r: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: i32 = 0;
    let mut ix: i32 = 0 as i32;
    if incX <= 0 as i32 {
        return 0 as i32 as libc::c_float;
    }
    i = 0 as i32;
    while i < N {
        r = (r as libc::c_double + fabs(*X.offset(ix as isize) as libc::c_double))
            as libc::c_float;
        ix += incX;
        i += 1;
        i;
    }
    return r;
}