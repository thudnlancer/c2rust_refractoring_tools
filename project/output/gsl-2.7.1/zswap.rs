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
pub unsafe extern "C" fn cblas_zswap(
    N: i32,
    mut X: *mut libc::c_void,
    incX: i32,
    mut Y: *mut libc::c_void,
    incY: i32,
) {
    let mut i: i32 = 0;
    let mut ix: i32 = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
    let mut iy: i32 = if incY > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incY };
    i = 0 as i32;
    while i < N {
        let tmp_real: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as i32 * ix) as isize);
        let tmp_imag: libc::c_double = *(X as *mut libc::c_double)
            .offset((2 as i32 * ix + 1 as i32) as isize);
        *(X as *mut libc::c_double).offset((2 as i32 * ix) as isize) = *(Y
            as *mut libc::c_double)
            .offset((2 as i32 * iy) as isize);
        *(X as *mut libc::c_double).offset((2 as i32 * ix + 1 as i32) as isize) = *(Y
            as *mut libc::c_double)
            .offset((2 as i32 * iy + 1 as i32) as isize);
        *(Y as *mut libc::c_double).offset((2 as i32 * iy) as isize) = tmp_real;
        *(Y as *mut libc::c_double).offset((2 as i32 * iy + 1 as i32) as isize) = tmp_imag;
        ix += incX;
        iy += incY;
        i += 1;
        i;
    }
}