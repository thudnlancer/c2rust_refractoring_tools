#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn c_strstr(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    return strstr(haystack, needle);
}
