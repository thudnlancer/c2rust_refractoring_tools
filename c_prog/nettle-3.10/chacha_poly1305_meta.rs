#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn nettle_chacha_poly1305_digest(
        ctx: *mut chacha_poly1305_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_chacha_poly1305_set_key(
        ctx: *mut chacha_poly1305_ctx,
        key: *const uint8_t,
    );
    fn nettle_chacha_poly1305_update(
        ctx: *mut chacha_poly1305_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_chacha_poly1305_set_nonce(
        ctx: *mut chacha_poly1305_ctx,
        nonce: *const uint8_t,
    );
    fn nettle_chacha_poly1305_encrypt(
        ctx: *mut chacha_poly1305_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_chacha_poly1305_decrypt(
        ctx: *mut chacha_poly1305_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub struct chacha_poly1305_ctx {
    pub chacha: chacha_ctx,
    pub poly1305: poly1305_ctx,
    pub s: nettle_block16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
    pub block: [uint8_t; 16],
    pub index: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct poly1305_ctx {
    pub r: C2RustUnnamed_0,
    pub s32: [uint32_t; 3],
    pub hh: uint32_t,
    pub h: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub h32: [uint32_t; 4],
    pub h64: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub r32: [uint32_t; 6],
    pub r64: [uint64_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
#[no_mangle]
pub static mut nettle_chacha_poly1305: nettle_aead = unsafe {
    {
        let mut init = nettle_aead {
            name: b"chacha_poly1305\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<chacha_poly1305_ctx>() as libc::c_ulong
                as libc::c_uint,
            block_size: 64 as libc::c_int as libc::c_uint,
            key_size: 32 as libc::c_int as libc::c_uint,
            nonce_size: 12 as libc::c_int as libc::c_uint,
            digest_size: 16 as libc::c_int as libc::c_uint,
            set_encrypt_key: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut chacha_poly1305_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_chacha_poly1305_set_key
                        as unsafe extern "C" fn(
                            *mut chacha_poly1305_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_decrypt_key: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut chacha_poly1305_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_chacha_poly1305_set_key
                        as unsafe extern "C" fn(
                            *mut chacha_poly1305_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            set_nonce: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut chacha_poly1305_ctx, *const uint8_t) -> (),
                >,
                Option::<nettle_set_key_func>,
            >(
                Some(
                    nettle_chacha_poly1305_set_nonce
                        as unsafe extern "C" fn(
                            *mut chacha_poly1305_ctx,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            update: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut chacha_poly1305_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_chacha_poly1305_update
                        as unsafe extern "C" fn(
                            *mut chacha_poly1305_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            encrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut chacha_poly1305_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_chacha_poly1305_encrypt
                        as unsafe extern "C" fn(
                            *mut chacha_poly1305_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            decrypt: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut chacha_poly1305_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_crypt_func>,
            >(
                Some(
                    nettle_chacha_poly1305_decrypt
                        as unsafe extern "C" fn(
                            *mut chacha_poly1305_ctx,
                            size_t,
                            *mut uint8_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut chacha_poly1305_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_chacha_poly1305_digest
                        as unsafe extern "C" fn(
                            *mut chacha_poly1305_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
