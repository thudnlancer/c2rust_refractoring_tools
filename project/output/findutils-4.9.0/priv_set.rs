#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn priv_set_remove_linkdir() -> i32 {
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn priv_set_restore_linkdir() -> i32 {
    return -(1 as i32);
}