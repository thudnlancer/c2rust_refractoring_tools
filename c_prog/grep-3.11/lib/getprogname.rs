#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    static mut program_invocation_short_name: *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getprogname() -> *const libc::c_char {
    return program_invocation_short_name;
}
