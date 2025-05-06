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
pub unsafe extern "C" fn cblas_sdsdot(
    N: i32,
    alpha: libc::c_float,
    mut X: *const libc::c_float,
    incX: i32,
    mut Y: *const libc::c_float,
    incY: i32,
) -> libc::c_float {
    let mut r: libc::c_double = alpha as libc::c_double;
    let mut i: i32 = 0;
    let mut ix: i32 = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
    let mut iy: i32 = if incY > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incY };
    i = 0 as i32;
    while i < N {
        r += (*X.offset(ix as isize) * *Y.offset(iy as isize)) as libc::c_double;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
    return r as libc::c_float;
}