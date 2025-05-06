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
pub unsafe extern "C" fn cblas_isamax(
    N: i32,
    mut X: *const libc::c_float,
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
        if fabs(*X.offset(ix as isize) as libc::c_double) > max as libc::c_double {
            max = fabs(*X.offset(ix as isize) as libc::c_double) as libc::c_float;
            result = i as size_t;
        }
        ix += incX;
        i += 1;
        i;
    }
    return result;
}