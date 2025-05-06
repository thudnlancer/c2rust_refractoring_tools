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
pub struct salsa20_ctx {
    pub input: [uint32_t; 16],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_salsa20_set_nonce(
    mut ctx: *mut salsa20_ctx,
    mut nonce: *const uint8_t,
) {
    (*ctx).input[6 as i32 as usize] = (*nonce
        .offset(0 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*nonce.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*nonce.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *nonce.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[7 as i32 as usize] = (*nonce
        .offset(4 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*nonce.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*nonce.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *nonce.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[8 as i32 as usize] = 0 as i32 as uint32_t;
    (*ctx).input[9 as i32 as usize] = 0 as i32 as uint32_t;
}