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
    fn nettle_pbkdf2(
        mac_ctx: *mut libc::c_void,
        update: Option<nettle_hash_update_func>,
        digest: Option<nettle_hash_digest_func>,
        digest_size: size_t,
        iterations: u32,
        salt_length: size_t,
        salt: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
    );
    fn nettle_hmac_sha1_set_key(
        ctx: *mut hmac_sha1_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_sha1_digest(
        ctx: *mut hmac_sha1_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_hmac_sha1_update(
        ctx: *mut hmac_sha1_ctx,
        length: size_t,
        data: *const uint8_t,
    );
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct hmac_sha1_ctx {
    pub outer: sha1_ctx,
    pub inner: sha1_ctx,
    pub state: sha1_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha1_ctx {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_pbkdf2_hmac_sha1(
    mut key_length: size_t,
    mut key: *const uint8_t,
    mut iterations: u32,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut sha1ctx: hmac_sha1_ctx = hmac_sha1_ctx {
        outer: sha1_ctx {
            state: [0; 5],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        inner: sha1_ctx {
            state: [0; 5],
            count: 0,
            index: 0,
            block: [0; 64],
        },
        state: sha1_ctx {
            state: [0; 5],
            count: 0,
            index: 0,
            block: [0; 64],
        },
    };
    nettle_hmac_sha1_set_key(&mut sha1ctx, key_length, key);
    if 0 as i32 != 0 {
        nettle_hmac_sha1_update(&mut sha1ctx, 0 as i32 as size_t, 0 as *mut uint8_t);
        nettle_hmac_sha1_digest(&mut sha1ctx, 0 as i32 as size_t, 0 as *mut uint8_t);
    } else {
        nettle_pbkdf2(
            &mut sha1ctx as *mut hmac_sha1_ctx as *mut libc::c_void,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut hmac_sha1_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_hash_update_func>,
            >(
                Some(
                    nettle_hmac_sha1_update
                        as unsafe extern "C" fn(
                            *mut hmac_sha1_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut hmac_sha1_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_hmac_sha1_digest
                        as unsafe extern "C" fn(
                            *mut hmac_sha1_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
            20 as i32 as size_t,
            iterations,
            salt_length,
            salt,
            length,
            dst,
        );
    };
}