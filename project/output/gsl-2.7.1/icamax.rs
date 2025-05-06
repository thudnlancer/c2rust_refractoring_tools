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
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn cblas_icamax(
    N: i32,
    mut X: *const libc::c_void,
    incX: i32,
) -> size_t {
    let mut max: libc::c_float = 0.0f64 as libc::c_float;
    let mut ix: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut result: size_t = 0 as i32 as size_t;
    if incX <= 0 as i32 {
        return 0 as i32 as size_t;
    }
    i = 0 as i32;
    while i < N {
        let a: libc::c_float = (fabs(
            *(X as *const libc::c_float).offset((2 as i32 * ix) as isize)
                as libc::c_double,
        )
            + fabs(
                *(X as *const libc::c_float).offset((2 as i32 * ix + 1 as i32) as isize)
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