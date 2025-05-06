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
    fn nettle_sha256_init(ctx: *mut sha256_ctx);
    fn nettle_sha256_update(ctx: *mut sha256_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha256_digest(ctx: *mut sha256_ctx, length: size_t, digest: *mut uint8_t);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
#[no_mangle]
pub static mut nettle_sha256: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"sha256\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<sha256_ctx>() as u64 as u32,
            digest_size: 32 as i32 as u32,
            block_size: 64 as i32 as u32,
            init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut sha256_ctx) -> ()>,
                Option<nettle_hash_init_func>,
            >(Some(nettle_sha256_init as unsafe extern "C" fn(*mut sha256_ctx) -> ())),
            update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut sha256_ctx, size_t, *const uint8_t) -> (),
                >,
                Option<nettle_hash_update_func>,
            >(
                Some(
                    nettle_sha256_update
                        as unsafe extern "C" fn(
                            *mut sha256_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut sha256_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_sha256_digest
                        as unsafe extern "C" fn(
                            *mut sha256_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};