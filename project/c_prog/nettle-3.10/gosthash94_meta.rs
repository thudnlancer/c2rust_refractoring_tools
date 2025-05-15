use ::libc;
extern "C" {
    fn nettle_gosthash94_init(ctx: *mut gosthash94_ctx);
    fn nettle_gosthash94_update(
        ctx: *mut gosthash94_ctx,
        length: size_t,
        msg: *const uint8_t,
    );
    fn nettle_gosthash94_digest(
        ctx: *mut gosthash94_ctx,
        length: size_t,
        result: *mut uint8_t,
    );
    fn nettle_gosthash94cp_update(
        ctx: *mut gosthash94_ctx,
        length: size_t,
        msg: *const uint8_t,
    );
    fn nettle_gosthash94cp_digest(
        ctx: *mut gosthash94_ctx,
        length: size_t,
        result: *mut uint8_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub struct gosthash94_ctx {
    pub hash: [uint32_t; 8],
    pub sum: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 32],
}
#[no_mangle]
pub static mut nettle_gosthash94: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"gosthash94\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<gosthash94_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 32 as libc::c_int as libc::c_uint,
            block_size: 32 as libc::c_int as libc::c_uint,
            init: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut gosthash94_ctx) -> ()>,
                Option::<nettle_hash_init_func>,
            >(
                Some(
                    nettle_gosthash94_init
                        as unsafe extern "C" fn(*mut gosthash94_ctx) -> (),
                ),
            ),
            update: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gosthash94_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_gosthash94_update
                        as unsafe extern "C" fn(
                            *mut gosthash94_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut gosthash94_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_gosthash94_digest
                        as unsafe extern "C" fn(
                            *mut gosthash94_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut nettle_gosthash94cp: nettle_hash = unsafe {
    {
        let mut init = nettle_hash {
            name: b"gosthash94cp\0" as *const u8 as *const libc::c_char,
            context_size: ::core::mem::size_of::<gosthash94_ctx>() as libc::c_ulong
                as libc::c_uint,
            digest_size: 32 as libc::c_int as libc::c_uint,
            block_size: 32 as libc::c_int as libc::c_uint,
            init: ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut gosthash94_ctx) -> ()>,
                Option::<nettle_hash_init_func>,
            >(
                Some(
                    nettle_gosthash94_init
                        as unsafe extern "C" fn(*mut gosthash94_ctx) -> (),
                ),
            ),
            update: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut gosthash94_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option::<nettle_hash_update_func>,
            >(
                Some(
                    nettle_gosthash94cp_update
                        as unsafe extern "C" fn(
                            *mut gosthash94_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut gosthash94_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option::<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_gosthash94cp_digest
                        as unsafe extern "C" fn(
                            *mut gosthash94_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
