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
    fn gsl_stream_printf(
        label: *const i8,
        file: *const i8,
        line: i32,
        reason: *const i8,
    );
}
#[no_mangle]
pub static mut gsl_message_mask: u32 = 0xffffffff as u32;
#[no_mangle]
pub unsafe extern "C" fn gsl_message(
    mut reason: *const i8,
    mut file: *const i8,
    mut line: i32,
    mut mask: u32,
) {
    if mask & gsl_message_mask != 0 {
        gsl_stream_printf(b"MESSAGE\0" as *const u8 as *const i8, file, line, reason);
    }
}