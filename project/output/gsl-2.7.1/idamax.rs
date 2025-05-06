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
pub unsafe extern "C" fn cblas_idamax(
    N: i32,
    mut X: *const libc::c_double,
    incX: i32,
) -> size_t {
    let mut max: libc::c_double = 0.0f64;
    let mut ix: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut result: size_t = 0 as i32 as size_t;
    if incX <= 0 as i32 {
        return 0 as i32 as size_t;
    }
    i = 0 as i32;
    while i < N {
        if fabs(*X.offset(ix as isize)) > max {
            max = fabs(*X.offset(ix as isize));
            result = i as size_t;
        }
        ix += incX;
        i += 1;
        i;
    }
    return result;
}