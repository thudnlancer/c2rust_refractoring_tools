use ::libc;
extern "C" {
    fn nettle_sha3_224_init(ctx: *mut sha3_224_ctx);
    fn nettle_sha3_224_update(
        ctx: *mut sha3_224_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_sha3_224_digest(
        ctx: *mut sha3_224_ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
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
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_224_ctx {
    pub state: sha3_state,
    pub index: libc::c_uint,
    pub block: [uint8_t; 144],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
#[no_mangle]
pub static mut nettle_sha3_224: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"sha3_224\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<sha3_224_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 28 as libc::c_int as libc::c_uint,
            block_size: 144 as libc::c_int as libc::c_uint,
            init: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut sha3_224_ctx) -> ()>,
                Option::<nettle_hash_init_func>,
            >(
                Some(
                    nettle_sha3_224_init as unsafe extern "C" fn(*mut sha3_224_ctx) -> (),
                ),
            ),
            update: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut sha3_224_ctx, size_t, *const uint8_t) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_sha3_224_update
                        as unsafe extern "C" fn(
                            *mut sha3_224_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut sha3_224_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_sha3_224_digest
                        as unsafe extern "C" fn(
                            *mut sha3_224_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
