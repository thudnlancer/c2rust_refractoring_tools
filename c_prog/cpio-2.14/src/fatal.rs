#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn exit(_: libc::c_int) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn fatal_exit() {
    exit(2 as libc::c_int);
}
