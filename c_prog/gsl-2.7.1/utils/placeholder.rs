#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn gsl_utils_placeholder() {
    let mut i: libc::c_int = 0 as libc::c_int;
    i += 1;
    i;
}
