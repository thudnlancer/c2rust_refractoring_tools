#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_set_nonce(
    mut ctx: *mut chacha_ctx,
    mut nonce: *const uint8_t,
) {
    (*ctx).state[12 as i32 as usize] = 0 as i32 as uint32_t;
    (*ctx).state[13 as i32 as usize] = 0 as i32 as uint32_t;
    (*ctx).state[14 as i32 as usize] = (*nonce
        .offset(0 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*nonce.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*nonce.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *nonce.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[15 as i32 as usize] = (*nonce
        .offset(4 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*nonce.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*nonce.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *nonce.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_set_nonce96(
    mut ctx: *mut chacha_ctx,
    mut nonce: *const uint8_t,
) {
    (*ctx).state[12 as i32 as usize] = 0 as i32 as uint32_t;
    (*ctx).state[13 as i32 as usize] = (*nonce
        .offset(0 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*nonce.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*nonce.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *nonce.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[14 as i32 as usize] = (*nonce
        .offset(4 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*nonce.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*nonce.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *nonce.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[15 as i32 as usize] = (*nonce
        .offset(8 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*nonce.offset(8 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*nonce.offset(8 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *nonce.offset(8 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_set_counter(
    mut ctx: *mut chacha_ctx,
    mut counter: *const uint8_t,
) {
    (*ctx).state[12 as i32 as usize] = (*counter
        .offset(0 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*counter.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*counter.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *counter.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[13 as i32 as usize] = (*counter
        .offset(4 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*counter.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*counter.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *counter.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_set_counter32(
    mut ctx: *mut chacha_ctx,
    mut counter: *const uint8_t,
) {
    (*ctx).state[12 as i32 as usize] = (*counter
        .offset(0 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*counter.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*counter.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *counter.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
}