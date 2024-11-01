#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn nettle_gcm_camellia128_set_key(
        ctx: *mut gcm_camellia128_ctx,
        key: *const uint8_t,
    );
    fn nettle_gcm_camellia128_set_iv(
        ctx: *mut gcm_camellia128_ctx,
        length: size_t,
        iv: *const uint8_t,
    );
    fn nettle_gcm_camellia128_update(
        ctx: *mut gcm_camellia128_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_gcm_camellia128_encrypt(
        ctx: *mut gcm_camellia128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_camellia128_decrypt(
        ctx: *mut gcm_camellia128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_camellia128_digest(
        ctx: *mut gcm_camellia128_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
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
pub struct nettle_aead {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub nonce_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub set_encrypt_key: Option::<nettle_set_key_func>,
    pub set_decrypt_key: Option::<nettle_set_key_func>,
    pub set_nonce: Option::<nettle_set_key_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub encrypt: Option::<nettle_crypt_func>,
    pub decrypt: Option::<nettle_crypt_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_camellia128_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: camellia128_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_ctx {
    pub iv: nettle_block16,
    pub ctr: nettle_block16,
    pub x: nettle_block16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
unsafe extern "C" fn gcm_camellia128_set_nonce_wrapper(
    mut ctx: *mut libc::c_void,
    mut nonce: *const uint8_t,
) {
    nettle_gcm_camellia128_set_iv(
        ctx as *mut gcm_camellia128_ctx,
        (16 as libc::c_int - 4 as libc::c_int) as size_t,
        nonce,
    );
}
#[no_mangle]
pub static mut nettle_gcm_camellia128: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"gcm_camellia128\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<gcm_camellia128_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 16 as libc::c_int as libc::c_uint,
            key_size: 16 as libc::c_int as libc::c_uint,
            nonce_size: (16 as libc::c_int - 4 as libc::c_int) as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut gcm_camellia128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_gcm_camellia128_set_key
                        as unsafe extern "C" fn(
                            *mut gcm_camellia128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut gcm_camellia128_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_gcm_camellia128_set_key
                        as unsafe extern "C" fn(
                            *mut gcm_camellia128_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_nonce: Some(gcm_camellia128_set_nonce_wrapper as nettle_set_key_func),
            update: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gcm_camellia128_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_gcm_camellia128_update
                        as unsafe extern "C" fn(
                            *mut gcm_camellia128_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gcm_camellia128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_gcm_camellia128_encrypt
                        as unsafe extern "C" fn(
                            *mut gcm_camellia128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gcm_camellia128_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_gcm_camellia128_decrypt
                        as unsafe extern "C" fn(
                            *mut gcm_camellia128_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gcm_camellia128_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_gcm_camellia128_digest
                        as unsafe extern "C" fn(
                            *mut gcm_camellia128_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
