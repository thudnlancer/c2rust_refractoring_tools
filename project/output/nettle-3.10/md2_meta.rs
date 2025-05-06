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
    fn nettle_md2_init(ctx: *mut md2_ctx);
    fn nettle_md2_update(ctx: *mut md2_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md2_digest(ctx: *mut md2_ctx, length: size_t, digest: *mut uint8_t);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
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
pub struct md2_ctx {
    pub C: [uint8_t; 16],
    pub X: [uint8_t; 48],
    pub index: u32,
    pub block: [uint8_t; 16],
}
#[no_mangle]
pub static mut nettle_md2: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"md2\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<md2_ctx>() as u64 as u32,
            digest_size: 16 as i32 as u32,
            block_size: 16 as i32 as u32,
            init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut md2_ctx) -> ()>,
                Option<nettle_hash_init_func>,
            >(Some(nettle_md2_init as unsafe extern "C" fn(*mut md2_ctx) -> ())),
            update: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut md2_ctx, size_t, *const uint8_t) -> ()>,
                Option<nettle_hash_update_func>,
            >(
                Some(
                    nettle_md2_update
                        as unsafe extern "C" fn(
                            *mut md2_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut md2_ctx, size_t, *mut uint8_t) -> ()>,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_md2_digest
                        as unsafe extern "C" fn(*mut md2_ctx, size_t, *mut uint8_t) -> (),
                ),
            ),
        };
        init
    }
};