#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn cblas_icamax(
    N: libc::c_int,
    mut X: *const libc::c_void,
    incX: libc::c_int,
) -> size_t {
    let mut max: libc::c_float = 0.0f64 as libc::c_float;
    let mut ix: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut result: size_t = 0 as libc::c_int as size_t;
    if incX <= 0 as libc::c_int {
        return 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int;
    while i < N {
        let a: libc::c_float = (fabs(
            *(X as *const libc::c_float).offset((2 as libc::c_int * ix) as isize)
                as libc::c_double,
        )
            + fabs(
                *(X as *const libc::c_float)
                    .offset((2 as libc::c_int * ix + 1 as libc::c_int) as isize)
                    as libc::c_double,
            )) as libc::c_float;
        if a > max {
            max = a;
            result = i as size_t;
        }
        ix += incX;
        i += 1;
        i;
    }
    return result;
}
