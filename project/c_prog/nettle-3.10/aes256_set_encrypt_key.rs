use ::libc;
extern "C" {
    fn _nettle_aes_set_key(
        nr: libc::c_uint,
        nk: libc::c_uint,
        subkeys: *mut uint32_t,
        key: *const uint8_t,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes256_set_encrypt_key(
    mut ctx: *mut aes256_ctx,
    mut key: *const uint8_t,
) {
    _nettle_aes_set_key(
        14 as libc::c_int as libc::c_uint,
        (32 as libc::c_int / 4 as libc::c_int) as libc::c_uint,
        ((*ctx).keys).as_mut_ptr(),
        key,
    );
}
