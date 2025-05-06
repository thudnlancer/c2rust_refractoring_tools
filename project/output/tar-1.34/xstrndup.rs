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
    fn strndup(_: *const i8, _: u64) -> *mut i8;
    fn xalloc_die();
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn xstrndup(mut string: *const i8, mut n: size_t) -> *mut i8 {
    let mut s: *mut i8 = strndup(string, n);
    if s.is_null() {
        xalloc_die();
    }
    return s;
}