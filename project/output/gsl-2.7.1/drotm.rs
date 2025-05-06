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
    fn cblas_xerbla(p: i32, rout: *const i8, form: *const i8, _: ...);
}
#[no_mangle]
pub unsafe extern "C" fn cblas_drotm(
    N: i32,
    mut X: *mut libc::c_double,
    incX: i32,
    mut Y: *mut libc::c_double,
    incY: i32,
    mut P: *const libc::c_double,
) {
    let mut n: i32 = 0;
    let mut i: i32 = if incX > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incX };
    let mut j: i32 = if incY > 0 as i32 { 0 as i32 } else { (N - 1 as i32) * -incY };
    let mut h11: libc::c_double = 0.;
    let mut h21: libc::c_double = 0.;
    let mut h12: libc::c_double = 0.;
    let mut h22: libc::c_double = 0.;
    if *P.offset(0 as i32 as isize) == -1.0f64 {
        h11 = *P.offset(1 as i32 as isize);
        h21 = *P.offset(2 as i32 as isize);
        h12 = *P.offset(3 as i32 as isize);
        h22 = *P.offset(4 as i32 as isize);
    } else if *P.offset(0 as i32 as isize) == 0.0f64 {
        h11 = 1.0f64;
        h21 = *P.offset(2 as i32 as isize);
        h12 = *P.offset(3 as i32 as isize);
        h22 = 1.0f64;
    } else if *P.offset(0 as i32 as isize) == 1.0f64 {
        h11 = *P.offset(1 as i32 as isize);
        h21 = -1.0f64;
        h12 = 1.0f64;
        h22 = *P.offset(4 as i32 as isize);
    } else if *P.offset(0 as i32 as isize) == -2.0f64 {
        return
    } else {
        cblas_xerbla(
            0 as i32,
            b"./source_rotm.h\0" as *const u8 as *const i8,
            b"unrecognized value of P[0]\0" as *const u8 as *const i8,
        );
        return;
    }
    n = 0 as i32;
    while n < N {
        let w: libc::c_double = *X.offset(i as isize);
        let z: libc::c_double = *Y.offset(j as isize);
        *X.offset(i as isize) = h11 * w + h12 * z;
        *Y.offset(j as isize) = h21 * w + h22 * z;
        i += incX;
        j += incY;
        n += 1;
        n;
    }
}