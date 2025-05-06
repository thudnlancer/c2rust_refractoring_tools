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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn nettle_sha3_permute(state: *mut sha3_state);
    fn _nettle_sha3_update(
        state: *mut sha3_state,
        block_size: u32,
        block: *mut uint8_t,
        pos: u32,
        length: size_t,
        data: *const uint8_t,
    ) -> u32;
    fn _nettle_sha3_pad(
        state: *mut sha3_state,
        block_size: u32,
        block: *mut uint8_t,
        pos: u32,
        magic: uint8_t,
    );
    fn _nettle_write_le64(length: size_t, dst: *mut uint8_t, src: *const uint64_t);
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
pub struct sha3_224_ctx {
    pub state: sha3_state,
    pub index: u32,
    pub block: [uint8_t; 144],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_224_init(mut ctx: *mut sha3_224_ctx) {
    memset(ctx as *mut libc::c_void, 0 as i32, 204 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_224_update(
    mut ctx: *mut sha3_224_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    (*ctx).index = _nettle_sha3_update(
        &mut (*ctx).state,
        144 as i32 as u32,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        length,
        data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha3_224_digest(
    mut ctx: *mut sha3_224_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    _nettle_sha3_pad(
        &mut (*ctx).state,
        144 as i32 as u32,
        ((*ctx).block).as_mut_ptr(),
        (*ctx).index,
        6 as i32 as uint8_t,
    );
    nettle_sha3_permute(&mut (*ctx).state);
    _nettle_write_le64(length, digest, ((*ctx).state.a).as_mut_ptr());
    nettle_sha3_224_init(ctx);
}