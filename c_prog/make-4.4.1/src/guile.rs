#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
#[no_mangle]
pub unsafe extern "C" fn guile_gmake_setup(mut flocp: *const floc) -> libc::c_int {
    return 1 as libc::c_int;
}
