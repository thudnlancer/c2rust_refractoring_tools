#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn nettle_version_major() -> libc::c_int {
    return 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_version_minor() -> libc::c_int {
    return 10 as libc::c_int;
}
