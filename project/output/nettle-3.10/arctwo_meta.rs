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
    fn nettle_arctwo40_set_key(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo64_set_key(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo128_set_key(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo128_set_key_gutmann(ctx: *mut arctwo_ctx, key: *const uint8_t);
    fn nettle_arctwo_encrypt(
        ctx: *mut arctwo_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_arctwo_decrypt(
        ctx: *mut arctwo_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
    pub name: *const i8,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub encrypt: Option<nettle_cipher_func>,
    pub decrypt: Option<nettle_cipher_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arctwo_ctx {
    pub S: [uint16_t; 64],
}
#[no_mangle]
pub static mut nettle_arctwo40: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo40\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<arctwo_ctx>() as u64 as u32,
            block_size: 8 as i32 as u32,
            key_size: (40 as i32 / 8 as i32) as u32,
            set_encrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo40_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo40_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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
pub static mut nettle_arctwo64: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo64\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<arctwo_ctx>() as u64 as u32,
            block_size: 8 as i32 as u32,
            key_size: (64 as i32 / 8 as i32) as u32,
            set_encrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo64_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo64_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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
pub static mut nettle_arctwo128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo128\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<arctwo_ctx>() as u64 as u32,
            block_size: 8 as i32 as u32,
            key_size: (128 as i32 / 8 as i32) as u32,
            set_encrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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
pub static mut nettle_arctwo_gutmann128: nettle_cipher = unsafe {
    {
        let mut init = nettle_cipher {
            name: b"arctwo_gutmann128\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<arctwo_ctx>() as u64 as u32,
            block_size: 8 as i32 as u32,
            key_size: 16 as i32 as u32,
            set_encrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key_gutmann
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_arctwo128_set_key_gutmann
                        as unsafe extern "C" fn(*mut arctwo_ctx, *const uint8_t) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_encrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut arctwo_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_cipher_func>,
            >(
                Some(
                    nettle_arctwo_decrypt
                        as unsafe extern "C" fn(
                            *mut arctwo_ctx,
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