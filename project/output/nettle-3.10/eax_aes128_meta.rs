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
    fn nettle_eax_aes128_digest(
        ctx: *mut eax_aes128_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_eax_aes128_decrypt(
        ctx: *mut eax_aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_eax_aes128_encrypt(
        ctx: *mut eax_aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_eax_aes128_update(
        ctx: *mut eax_aes128_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_eax_aes128_set_nonce(
        ctx: *mut eax_aes128_ctx,
        length: size_t,
        iv: *const uint8_t,
    );
    fn nettle_eax_aes128_set_key(ctx: *mut eax_aes128_ctx, key: *const uint8_t);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_crypt_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eax_key {
    pub pad_block: nettle_block16,
    pub pad_partial: nettle_block16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eax_ctx {
    pub omac_nonce: nettle_block16,
    pub omac_data: nettle_block16,
    pub omac_message: nettle_block16,
    pub ctr: nettle_block16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eax_aes128_ctx {
    pub key: eax_key,
    pub eax: eax_ctx,
    pub cipher: aes128_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_aead {
    pub name: *const i8,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub nonce_size: u32,
    pub digest_size: u32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub set_nonce: Option<nettle_set_key_func>,
    pub update: Option<nettle_hash_update_func>,
    pub encrypt: Option<nettle_crypt_func>,
    pub decrypt: Option<nettle_crypt_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
unsafe extern "C" fn eax_aes128_set_nonce_wrapper(
    mut ctx: *mut libc::c_void,
    mut nonce: *const uint8_t,
) {
    nettle_eax_aes128_set_nonce(ctx as *mut eax_aes128_ctx, 16 as i32 as size_t, nonce);
}
#[no_mangle]
pub static mut nettle_eax_aes128: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"eax_aes128\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<eax_aes128_ctx>() as u64 as u32,
            block_size: 16 as i32 as u32,
            key_size: 16 as i32 as u32,
            nonce_size: 16 as i32 as u32,
            digest_size: 16 as i32 as u32,
            set_encrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut eax_aes128_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_eax_aes128_set_key
                        as unsafe extern "C" fn(
                            *mut eax_aes128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut eax_aes128_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_eax_aes128_set_key
                        as unsafe extern "C" fn(
                            *mut eax_aes128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_nonce: Some(eax_aes128_set_nonce_wrapper as nettle_set_key_func),
            update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut eax_aes128_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_hash_update_func>,
            >(
                Some(
                    nettle_eax_aes128_update
                        as unsafe extern "C" fn(
                            *mut eax_aes128_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut eax_aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_crypt_func>,
            >(
                Some(
                    nettle_eax_aes128_encrypt
                        as unsafe extern "C" fn(
                            *mut eax_aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut eax_aes128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_crypt_func>,
            >(
                Some(
                    nettle_eax_aes128_decrypt
                        as unsafe extern "C" fn(
                            *mut eax_aes128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut eax_aes128_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_eax_aes128_digest
                        as unsafe extern "C" fn(
                            *mut eax_aes128_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};