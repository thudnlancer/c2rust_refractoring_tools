use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn u8_strlen(mut s: *const uint8_t) -> size_t {
    return strlen(s as *const libc::c_char);
}
