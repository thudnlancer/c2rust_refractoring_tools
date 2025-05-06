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
pub unsafe extern "C" fn cblas_dzasum(
    N: i32,
    mut X: *const libc::c_void,
    incX: i32,
) -> libc::c_double {
    let mut r: libc::c_double = 0.0f64;
    let mut i: i32 = 0;
    let mut ix: i32 = 0 as i32;
    if incX <= 0 as i32 {
        return 0 as i32 as libc::c_double;
    }
    i = 0 as i32;
    while i < N {
        r
            += fabs(*(X as *const libc::c_double).offset((2 as i32 * ix) as isize))
                + fabs(
                    *(X as *const libc::c_double)
                        .offset((2 as i32 * ix + 1 as i32) as isize),
                );
        ix += incX;
        i += 1;
        i;
    }
    return r;
}