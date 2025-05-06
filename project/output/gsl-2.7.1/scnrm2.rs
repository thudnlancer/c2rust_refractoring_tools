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
pub unsafe extern "C" fn cblas_scnrm2(
    N: i32,
    mut X: *const libc::c_void,
    incX: i32,
) -> libc::c_float {
    let mut scale: libc::c_float = 0.0f64 as libc::c_float;
    let mut ssq: libc::c_float = 1.0f64 as libc::c_float;
    let mut i: i32 = 0;
    let mut ix: i32 = 0 as i32;
    if N == 0 as i32 || incX < 1 as i32 {
        return 0 as i32 as libc::c_float;
    }
    i = 0 as i32;
    while i < N {
        let x: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as i32 * ix) as isize);
        let y: libc::c_float = *(X as *const libc::c_float)
            .offset((2 as i32 * ix + 1 as i32) as isize);
        if x as libc::c_double != 0.0f64 {
            let ax: libc::c_float = fabs(x as libc::c_double) as libc::c_float;
            if scale < ax {
                ssq = (1.0f64 + (ssq * (scale / ax) * (scale / ax)) as libc::c_double)
                    as libc::c_float;
                scale = ax;
            } else {
                ssq += ax / scale * (ax / scale);
            }
        }
        if y as libc::c_double != 0.0f64 {
            let ay: libc::c_float = fabs(y as libc::c_double) as libc::c_float;
            if scale < ay {
                ssq = (1.0f64 + (ssq * (scale / ay) * (scale / ay)) as libc::c_double)
                    as libc::c_float;
                scale = ay;
            } else {
                ssq += ay / scale * (ay / scale);
            }
        }
        ix += incX;
        i += 1;
        i;
    }
    return (scale as libc::c_double * sqrt(ssq as libc::c_double)) as libc::c_float;
}