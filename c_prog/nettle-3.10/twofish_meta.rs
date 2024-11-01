#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn nettle_twofish128_set_key(context: *mut twofish_ctx, key: *const uint8_t);
    fn nettle_twofish192_set_key(context: *mut twofish_ctx, key: *const uint8_t);
    fn nettle_twofish256_set_key(context: *mut twofish_ctx, key: *const uint8_t);
    fn nettle_twofish_encrypt(
        ctx: *const twofish_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_twofish_decrypt(
        ctx: *const twofish_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub encrypt: Option::<nettle_cipher_func>,
    pub decrypt: Option::<nettle_cipher_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct twofish_ctx {
    pub keys: [uint32_t; 40],
    pub s_box: [[uint32_t; 256]; 4],
}
#[no_mangle]
pub static mut nettle_twofish128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"twofish128\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<twofish_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_twofish128_set_key
                        as unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_twofish128_set_key
                        as unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const twofish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_twofish_encrypt
                        as unsafe extern "C" fn(
                            *const twofish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const twofish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_twofish_decrypt
                        as unsafe extern "C" fn(
                            *const twofish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut nettle_twofish192: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"twofish192\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<twofish_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 24 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_twofish192_set_key
                        as unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_twofish192_set_key
                        as unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const twofish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_twofish_encrypt
                        as unsafe extern "C" fn(
                            *const twofish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const twofish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_twofish_decrypt
                        as unsafe extern "C" fn(
                            *const twofish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut nettle_twofish256: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"twofish256\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<twofish_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_twofish256_set_key
                        as unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> ()>,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_twofish256_set_key
                        as unsafe extern "C" fn(*mut twofish_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const twofish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_twofish_encrypt
                        as unsafe extern "C" fn(
                            *const twofish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *const twofish_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_cipher_func>,
            >(
                Some(
                    nettle_twofish_decrypt
                        as unsafe extern "C" fn(
                            *const twofish_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
