#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_spmatrix_cumsum(n: size_t, mut c: *mut i32) {
    let mut sum: i32 = 0 as i32;
    let mut k: size_t = 0;
    k = 0 as i32 as size_t;
    while k < n {
        let mut ck: i32 = *c.offset(k as isize);
        *c.offset(k as isize) = sum;
        sum += ck;
        k = k.wrapping_add(1);
        k;
    }
    *c.offset(n as isize) = sum;
}