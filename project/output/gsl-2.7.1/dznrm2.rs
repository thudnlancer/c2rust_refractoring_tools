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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cblas_dznrm2(
    N: i32,
    mut X: *const libc::c_void,
    incX: i32,
) -> libc::c_double {
    let mut scale: libc::c_double = 0.0f64;
    let mut ssq: libc::c_double = 1.0f64;
    let mut i: i32 = 0;
    let mut ix: i32 = 0 as i32;
    if N == 0 as i32 || incX < 1 as i32 {
        return 0 as i32 as libc::c_double;
    }
    i = 0 as i32;
    while i < N {
        let x: libc::c_double = *(X as *const libc::c_double)
            .offset((2 as i32 * ix) as isize);
        let y: libc::c_double = *(X as *const libc::c_double)
            .offset((2 as i32 * ix + 1 as i32) as isize);
        if x != 0.0f64 {
            let ax: libc::c_double = fabs(x);
            if scale < ax {
                ssq = 1.0f64 + ssq * (scale / ax) * (scale / ax);
                scale = ax;
            } else {
                ssq += ax / scale * (ax / scale);
            }
        }
        if y != 0.0f64 {
            let ay: libc::c_double = fabs(y);
            if scale < ay {
                ssq = 1.0f64 + ssq * (scale / ay) * (scale / ay);
                scale = ay;
            } else {
                ssq += ay / scale * (ay / scale);
            }
        }
        ix += incX;
        i += 1;
        i;
    }
    return scale * sqrt(ssq);
}