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
pub unsafe extern "C" fn cblas_sscal(
    N: i32,
    alpha: libc::c_float,
    mut X: *mut libc::c_float,
    incX: i32,
) {
    let mut i: i32 = 0;
    let mut ix: i32 = 0 as i32;
    if incX <= 0 as i32 {
        return;
    }
    i = 0 as i32;
    while i < N {
        *X.offset(ix as isize) *= alpha;
        ix += incX;
        i += 1;
        i;
    }
}