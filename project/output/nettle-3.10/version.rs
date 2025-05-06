#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn nettle_version_major() -> i32 {
    return 3 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_version_minor() -> i32 {
    return 10 as i32;
}