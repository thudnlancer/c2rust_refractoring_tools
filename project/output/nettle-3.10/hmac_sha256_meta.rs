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
    fn nettle_hmac_sha256_set_key(
        ctx: *mut hmac_sha256_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_sha256_update(
        ctx: *mut hmac_sha256_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_sha256_digest(
        ctx: *mut hmac_sha256_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
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
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub key_size: u32,
    pub set_key: Option<nettle_set_key_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
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
    pub index: u32,
    pub block: [uint8_t; 64],
}
unsafe extern "C" fn hmac_sha256_set_key_wrapper(
    mut ctx: *mut libc::c_void,
    mut key: *const uint8_t,
) {
    nettle_hmac_sha256_set_key(ctx as *mut hmac_sha256_ctx, 32 as i32 as size_t, key);
}
#[no_mangle]
pub static mut nettle_hmac_sha256: nettle_mac = unsafe {
    {
        let mut init = nettle_mac {
            name: b"hmac_sha256\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<hmac_sha256_ctx>() as u64 as u32,
            digest_size: 32 as i32 as u32,
            key_size: 32 as i32 as u32,
            set_key: Some(
                hmac_sha256_set_key_wrapper
                    as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t) -> (),
            ),
            update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut hmac_sha256_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_hash_update_func>,
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
                Option<
                    unsafe extern "C" fn(
                        *mut hmac_sha256_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
                >,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_hmac_sha256_digest
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