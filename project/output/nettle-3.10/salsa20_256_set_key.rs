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
pub unsafe extern "C" fn nettle_salsa20_256_set_key(
    mut ctx: *mut salsa20_ctx,
    mut key: *const uint8_t,
) {
    (*ctx).input[1 as i32 as usize] = (*key
        .offset(0 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[2 as i32 as usize] = (*key
        .offset(4 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[3 as i32 as usize] = (*key
        .offset(8 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(8 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(8 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(8 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[4 as i32 as usize] = (*key
        .offset(12 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(12 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(12 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(12 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[11 as i32 as usize] = (*key
        .offset(16 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(16 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(16 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(16 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[12 as i32 as usize] = (*key
        .offset(20 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(20 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(20 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(20 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[13 as i32 as usize] = (*key
        .offset(24 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(24 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(24 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(24 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[14 as i32 as usize] = (*key
        .offset(28 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(28 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(28 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(28 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).input[0 as i32 as usize] = 0x61707865 as i32 as uint32_t;
    (*ctx).input[5 as i32 as usize] = 0x3320646e as i32 as uint32_t;
    (*ctx).input[10 as i32 as usize] = 0x79622d32 as i32 as uint32_t;
    (*ctx).input[15 as i32 as usize] = 0x6b206574 as i32 as uint32_t;
}