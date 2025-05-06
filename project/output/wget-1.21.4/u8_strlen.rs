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
    fn strlen(_: *const i8) -> u64;
}
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn u8_strlen(mut s: *const uint8_t) -> size_t {
    return strlen(s as *const i8);
}