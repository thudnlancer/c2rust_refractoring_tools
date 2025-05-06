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
    static mut program_invocation_short_name: *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn getprogname() -> *const i8 {
    return program_invocation_short_name;
}