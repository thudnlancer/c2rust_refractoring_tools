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
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn strnlen1(mut string: *const i8, mut maxlen: size_t) -> size_t {
    let mut end: *const i8 = memchr(string as *const libc::c_void, '\0' as i32, maxlen)
        as *const i8;
    if !end.is_null() {
        return (end.offset_from(string) as i64 + 1 as i32 as i64) as size_t
    } else {
        return maxlen
    };
}