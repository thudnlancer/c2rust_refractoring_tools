#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn u8_cpy(
    mut dest: *mut uint8_t,
    mut src: *const uint8_t,
    mut n: size_t,
) -> *mut uint8_t {
    if n > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            dest as *mut libc::c_char as *mut libc::c_void,
            src as *const libc::c_char as *const libc::c_void,
            n.wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
    }
    return dest;
}
