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
    fn xcharalloc(n: size_t) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn xmemdup0(mut p: *const libc::c_void, mut s: size_t) -> *mut i8 {
    let mut result: *mut i8 = xcharalloc(s.wrapping_add(1 as i32 as u64));
    if s > 0 as i32 as u64 {
        memcpy(result as *mut libc::c_void, p, s);
    }
    *result.offset(s as isize) = 0 as i32 as i8;
    return result;
}