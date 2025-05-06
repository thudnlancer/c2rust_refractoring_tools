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
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn c_strstr(
    mut haystack: *const i8,
    mut needle: *const i8,
) -> *mut i8 {
    return strstr(haystack, needle);
}