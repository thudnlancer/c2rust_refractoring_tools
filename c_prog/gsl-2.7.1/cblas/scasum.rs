#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn cblas_scasum(
    N: libc::c_int,
    mut X: *const libc::c_void,
    incX: libc::c_int,
) -> libc::c_float {
    let mut r: libc::c_float = 0.0f64 as libc::c_float;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0 as libc::c_int;
    if incX <= 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_float;
    }
    i = 0 as libc::c_int;
    while i < N {
        r = (r as libc::c_double
            + (fabs(
                *(X as *const libc::c_float).offset((2 as libc::c_int * ix) as isize)
                    as libc::c_double,
            )
                + fabs(
                    *(X as *const libc::c_float)
                        .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize)
                        as libc::c_double,
                ))) as libc::c_float;
        ix += incX;
        i += 1;
        i;
    }
    return r;
}
