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
    fn nettle_cmac_des3_update(
        ctx: *mut cmac_des3_ctx,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_cmac_des3_set_key(ctx: *mut cmac_des3_ctx, key: *const uint8_t);
    fn nettle_cmac_des3_digest(
        ctx: *mut cmac_des3_ctx,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block8 {
    pub b: [uint8_t; 8],
    pub u64_0: uint64_t,
}
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
pub struct cmac_des3_ctx {
    pub key: cmac64_key,
    pub ctx: cmac64_ctx,
    pub cipher: des3_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des3_ctx {
    pub des: [des_ctx; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac64_ctx {
    pub X: nettle_block8,
    pub block: nettle_block8,
    pub index: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac64_key {
    pub K1: nettle_block8,
    pub K2: nettle_block8,
}
#[no_mangle]
pub static mut nettle_cmac_des3: nettle_mac = unsafe {
    {
        let mut init = nettle_mac {
            name: b"cmac_des3\0" as *const u8 as *const i8,
            context_size: ::core::mem::size_of::<cmac_des3_ctx>() as u64 as u32,
            digest_size: 8 as i32 as u32,
            key_size: 24 as i32 as u32,
            set_key: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut cmac_des3_ctx, *const uint8_t) -> ()>,
                Option<nettle_set_key_func>,
            >(
                Some(
                    nettle_cmac_des3_set_key
                        as unsafe extern "C" fn(*mut cmac_des3_ctx, *const uint8_t) -> (),
                ),
            ),
            update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut cmac_des3_ctx,
                        size_t,
                        *const uint8_t,
                    ) -> (),
                >,
                Option<nettle_hash_update_func>,
            >(
                Some(
                    nettle_cmac_des3_update
                        as unsafe extern "C" fn(
                            *mut cmac_des3_ctx,
                            size_t,
                            *const uint8_t,
                        ) -> (),
                ),
            ),
            digest: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(*mut cmac_des3_ctx, size_t, *mut uint8_t) -> (),
                >,
                Option<nettle_hash_digest_func>,
            >(
                Some(
                    nettle_cmac_des3_digest
                        as unsafe extern "C" fn(
                            *mut cmac_des3_ctx,
                            size_t,
                            *mut uint8_t,
                        ) -> (),
                ),
            ),
        };
        init
    }
};