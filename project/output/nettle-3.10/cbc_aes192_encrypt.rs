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
    fn nettle_cbc_encrypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_encrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_cbc_aes192_encrypt_c(
    mut ctx: *const aes192_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    nettle_cbc_encrypt(
        ctx as *const libc::c_void,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *const aes192_ctx,
                    size_t,
                    *mut uint8_t,
                    *const uint8_t,
                ) -> (),
            >,
            Option<nettle_cipher_func>,
        >(
            Some(
                nettle_aes192_encrypt
                    as unsafe extern "C" fn(
                        *const aes192_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        16 as i32 as size_t,
        iv,
        length,
        dst,
        src,
    );
}