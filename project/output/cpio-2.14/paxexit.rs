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
    fn exit(_: i32) -> !;
    static mut exit_status: i32;
}
#[no_mangle]
pub unsafe extern "C" fn pax_exit() {
    exit(exit_status);
}