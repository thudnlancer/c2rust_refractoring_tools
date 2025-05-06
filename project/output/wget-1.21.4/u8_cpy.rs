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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn u8_cpy(
    mut dest: *mut uint8_t,
    mut src: *const uint8_t,
    mut n: size_t,
) -> *mut uint8_t {
    if n > 0 as i32 as u64 {
        memcpy(
            dest as *mut i8 as *mut libc::c_void,
            src as *const i8 as *const libc::c_void,
            n.wrapping_mul(::core::mem::size_of::<uint8_t>() as u64),
        );
    }
    return dest;
}