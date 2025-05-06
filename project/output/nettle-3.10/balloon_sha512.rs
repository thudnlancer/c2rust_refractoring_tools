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
    fn nettle_balloon(
        hash_ctx: *mut libc::c_void,
        update: Option<nettle_hash_update_func>,
        digest: Option<nettle_hash_digest_func>,
        digest_size: size_t,
        s_cost: size_t,
        t_cost: size_t,
        passwd_length: size_t,
        passwd: *const uint8_t,
        salt_length: size_t,
        salt: *const uint8_t,
        scratch: *mut uint8_t,
        dst: *mut uint8_t,
    );
    fn nettle_sha512_init(ctx: *mut sha512_ctx);
    fn nettle_sha512_update(ctx: *mut sha512_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha512_digest(ctx: *mut sha512_ctx, length: size_t, digest: *mut uint8_t);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
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
pub struct sha512_ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 128],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_balloon_sha512(
    mut s_cost: size_t,
    mut t_cost: size_t,
    mut passwd_length: size_t,
    mut passwd: *const uint8_t,
    mut salt_length: size_t,
    mut salt: *const uint8_t,
    mut scratch: *mut uint8_t,
    mut dst: *mut uint8_t,
) {
    let mut ctx: sha512_ctx = sha512_ctx {
        state: [0; 8],
        count_low: 0,
        count_high: 0,
        index: 0,
        block: [0; 128],
    };
    nettle_sha512_init(&mut ctx);
    nettle_balloon(
        &mut ctx as *mut sha512_ctx as *mut libc::c_void,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut sha512_ctx, size_t, *const uint8_t) -> ()>,
            Option<nettle_hash_update_func>,
        >(
            Some(
                nettle_sha512_update
                    as unsafe extern "C" fn(
                        *mut sha512_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut sha512_ctx, size_t, *mut uint8_t) -> ()>,
            Option<nettle_hash_digest_func>,
        >(
            Some(
                nettle_sha512_digest
                    as unsafe extern "C" fn(*mut sha512_ctx, size_t, *mut uint8_t) -> (),
            ),
        ),
        64 as i32 as size_t,
        s_cost,
        t_cost,
        passwd_length,
        passwd,
        salt_length,
        salt,
        scratch,
        dst,
    );
}