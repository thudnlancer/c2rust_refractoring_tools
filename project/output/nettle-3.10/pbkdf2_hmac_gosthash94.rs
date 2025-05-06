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
    fn nettle_hmac_gosthash94cp_update(
        ctx: *mut hmac_gosthash94cp_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_hmac_gosthash94cp_set_key(
        ctx: *mut hmac_gosthash94cp_ctx,
        key_length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_gosthash94cp_digest(
        ctx: *mut hmac_gosthash94cp_ctx,
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
pub struct hmac_gosthash94cp_ctx {
    pub outer: gosthash94_ctx,
    pub inner: gosthash94_ctx,
    pub state: gosthash94_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gosthash94_ctx {
    pub hash: [uint32_t; 8],
    pub sum: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 32],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_pbkdf2_hmac_gosthash94cp(
    mut key_length: size_t,
    mut key: *const uint8_t,
    mut iterations: u32,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    let mut gosthash94cpctx: hmac_gosthash94cp_ctx = hmac_gosthash94cp_ctx {
        outer: gosthash94_ctx {
            hash: [0; 8],
            sum: [0; 8],
            count: 0,
            index: 0,
            block: [0; 32],
        },
        inner: gosthash94_ctx {
            hash: [0; 8],
            sum: [0; 8],
            count: 0,
            index: 0,
            block: [0; 32],
        },
        state: gosthash94_ctx {
            hash: [0; 8],
            sum: [0; 8],
            count: 0,
            index: 0,
            block: [0; 32],
        },
    };
    nettle_hmac_gosthash94cp_set_key(&mut gosthash94cpctx, key_length, key);
    if 0 as i32 != 0 {
        nettle_hmac_gosthash94cp_update(
            &mut gosthash94cpctx,
            0 as i32 as size_t,
            0 as *mut uint8_t,
        );
        nettle_hmac_gosthash94cp_digest(
            &mut gosthash94cpctx,
            0 as i32 as size_t,
            0 as *mut uint8_t,
        );
    } else {
        nettle_pbkdf2(
            &mut gosthash94cpctx as *mut hmac_gosthash94cp_ctx as *mut libc::c_void,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut hmac_gosthash94cp_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_hash_update_func>,
            >(
                Some(
                    nettle_hmac_gosthash94cp_update
                        as unsafe extern "C" fn(
                            *mut hmac_gosthash94cp_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut hmac_gosthash94cp_ctx,
                        size_t,
                        *mut uint8_t,
                    ) -> (),
                >,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_hmac_gosthash94cp_digest
                        as unsafe extern "C" fn(
                            *mut hmac_gosthash94cp_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
            32 as i32 as size_t,
            iterations,
            salt_length,
            salt,
            length,
            dst,
        );
    };
}