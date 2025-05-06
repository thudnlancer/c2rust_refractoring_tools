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
    fn nettle_sha3_256_init(ctx: *mut sha3_256_ctx);
    fn _nettle_sha3_shake(
        state: *mut sha3_state,
        block_size: u32,
        block: *mut uint8_t,
        index: u32,
        length: size_t,
        dst: *mut uint8_t,
    );
    fn _nettle_sha3_shake_output(
        state: *mut sha3_state,
        block_size: u32,
        block: *mut uint8_t,
        index: u32,
        length: size_t,
        dst: *mut uint8_t,
    ) -> u32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_256_ctx {
    pub state: sha3_state,
    pub index: u32,
    pub block: [uint8_t; 136],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_256_shake(
    mut ctx: *mut sha3_256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    _nettle_sha3_shake(
        &mut (*ctx).state,
        ::core::mem::size_of::<[uint8_t; 136]>() as u64 as u32,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        dst,
    );
    nettle_sha3_256_init(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_256_shake_output(
    mut ctx: *mut sha3_256_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    (*ctx).index = _nettle_sha3_shake_output(
        &mut (*ctx).state,
        ::core::mem::size_of::<[uint8_t; 136]>() as u64 as u32,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        digest,
    );
}