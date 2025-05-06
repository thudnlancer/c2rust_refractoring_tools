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
pub unsafe extern "C" fn cblas_scopy(
    N: i32,
    mut X: *const libc::c_float,
    incX: i32,
    mut Y: *mut libc::c_float,
    incY: i32,
) {
    let mut i: i32 = 0;
    let mut ix: i32 = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
    let mut iy: i32 = if incY > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incY };
    i = 0 as i32;
    while i < N {
        *Y.offset(iy as isize) = *X.offset(ix as isize);
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}