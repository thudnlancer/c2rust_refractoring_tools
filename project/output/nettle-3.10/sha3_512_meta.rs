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
    fn nettle_sha3_512_init(ctx: *mut sha3_512_ctx);
    fn nettle_sha3_512_update(
        ctx: *mut sha3_512_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_sha3_512_digest(
        ctx: *mut sha3_512_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
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
pub struct sha3_512_ctx {
    pub state: sha3_state,
    pub index: u32,
    pub block: [uint8_t; 72],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
#[no_mangle]
pub static mut nettle_sha3_512: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"sha3_512\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<sha3_512_ctx>() as u64 as u32,
            digest_size: 64 as i32 as u32,
            block_size: 72 as i32 as u32,
            init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut sha3_512_ctx) -> ()>,
                Option<nettle_hash_init_func>,
            >(
                Some(
                    nettle_sha3_512_init as unsafe extern "C" fn(*mut sha3_512_ctx) -> (),
                ),
            ),
            update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut sha3_512_ctx, size_t, *const uint8_t) -> (),
                >,
                Option<nettle_hash_update_func>,
            >(
                Some(
                    nettle_sha3_512_update
                        as unsafe extern "C" fn(
                            *mut sha3_512_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut sha3_512_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_sha3_512_digest
                        as unsafe extern "C" fn(
                            *mut sha3_512_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};