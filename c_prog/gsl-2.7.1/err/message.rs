#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn gsl_stream_printf(
        label: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        reason: *const libc::c_char,
    );
}
#[no_mangle]
pub static mut gsl_message_mask: libc::c_uint = 0xffffffff as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn gsl_message(
    mut reason: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut mask: libc::c_uint,
) {
    if mask & gsl_message_mask != 0 {
        gsl_stream_printf(
            b"MESSAGE\0" as *const u8 as *const libc::c_char,
            file,
            line,
            reason,
        );
    }
}
