#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _nettle_sha3_update(
        state: *mut sha3_state,
        block_size: libc::c_uint,
        block: *mut uint8_t,
        pos: libc::c_uint,
        length: size_t,
        data: *const uint8_t,
    ) -> libc::c_uint;
    fn _nettle_sha3_shake(
        state: *mut sha3_state,
        block_size: libc::c_uint,
        block: *mut uint8_t,
        index: libc::c_uint,
        length: size_t,
        dst: *mut uint8_t,
    );
    fn _nettle_sha3_shake_output(
        state: *mut sha3_state,
        block_size: libc::c_uint,
        block: *mut uint8_t,
        index: libc::c_uint,
        length: size_t,
        dst: *mut uint8_t,
    ) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_state {
    pub a: [uint64_t; 25],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha3_128_ctx {
    pub state: sha3_state,
    pub index: libc::c_uint,
    pub block: [uint8_t; 168],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_128_init(mut ctx: *mut sha3_128_ctx) {
    memset(ctx as *mut libc::c_void, 0 as libc::c_int, 204 as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_128_update(
    mut ctx: *mut sha3_128_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    (*ctx)
        .index = _nettle_sha3_update(
        &mut (*ctx).state,
        168 as libc::c_int as libc::c_uint,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_128_shake(
    mut ctx: *mut sha3_128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
) {
    _nettle_sha3_shake(
        &mut (*ctx).state,
        ::core::mem::size_of::<[uint8_t; 168]>() as libc::c_ulong as libc::c_uint,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        dst,
    );
    nettle_sha3_128_init(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_128_shake_output(
    mut ctx: *mut sha3_128_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    (*ctx)
        .index = _nettle_sha3_shake_output(
        &mut (*ctx).state,
        ::core::mem::size_of::<[uint8_t; 168]>() as libc::c_ulong as libc::c_uint,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        digest,
    );
}
