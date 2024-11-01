#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn nettle_hmac_sha256_update(
        ctx: *mut hmac_sha256_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_sha224_digest(
        ctx: *mut hmac_sha256_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_hmac_sha224_set_key(
        ctx: *mut hmac_sha256_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
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
pub struct nettle_mac {
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub key_size: libc::c_uint,
    pub set_key: Option::<nettle_set_key_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha256_ctx {
    pub outer: sha256_ctx,
    pub inner: sha256_ctx,
    pub state: sha256_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
unsafe extern "C" fn hmac_sha224_set_key_wrapper(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    nettle_hmac_sha224_set_key(
        ctx as *mut hmac_sha256_ctx,
        28 as libc::c_int as size_t,
        key,
    );
}
#[no_mangle]
pub static mut nettle_hmac_sha224: nettle_mac = unsafe {
    {
        let mut init = nettle_mac {
            name: b"hmac_sha224\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<hmac_sha256_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 28 as libc::c_int as libc::c_uint,
            key_size: 28 as libc::c_int as libc::c_uint,
            set_key: Some(
                hmac_sha224_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            update: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut hmac_sha256_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_hmac_sha256_update
                        as unsafe extern "C" fn(
                            *mut hmac_sha256_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut hmac_sha256_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_hmac_sha224_digest
                        as unsafe extern "C" fn(
                            *mut hmac_sha256_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
