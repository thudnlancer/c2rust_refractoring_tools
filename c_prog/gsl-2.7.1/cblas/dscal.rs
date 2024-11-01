#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn cblas_dscal(
    N: libc::c_int,
    alpha: libc::c_double,
    mut X: *mut libc::c_double,
    incX: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if incX <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < N {
        *X.offset(ix as isize) *= alpha;
        ix += incX;
        i += 1;
        i;
    }
}
