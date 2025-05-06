#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sm4_ctx {
    pub rkey: [uint32_t; 32],
}
static mut fk: [uint32_t; 4] = [
    0xa3b1bac6 as u32,
    0x56aa3350 as i32 as uint32_t,
    0x677d9197 as i32 as uint32_t,
    0xb27022dc as u32,
];
static mut ck: [uint32_t; 32] = [
    0x70e15 as i32 as uint32_t,
    0x1c232a31 as i32 as uint32_t,
    0x383f464d as i32 as uint32_t,
    0x545b6269 as i32 as uint32_t,
    0x70777e85 as i32 as uint32_t,
    0x8c939aa1 as u32,
    0xa8afb6bd as u32,
    0xc4cbd2d9 as u32,
    0xe0e7eef5 as u32,
    0xfc030a11 as u32,
    0x181f262d as i32 as uint32_t,
    0x343b4249 as i32 as uint32_t,
    0x50575e65 as i32 as uint32_t,
    0x6c737a81 as i32 as uint32_t,
    0x888f969d as u32,
    0xa4abb2b9 as u32,
    0xc0c7ced5 as u32,
    0xdce3eaf1 as u32,
    0xf8ff060d as u32,
    0x141b2229 as i32 as uint32_t,
    0x30373e45 as i32 as uint32_t,
    0x4c535a61 as i32 as uint32_t,
    0x686f767d as i32 as uint32_t,
    0x848b9299 as u32,
    0xa0a7aeb5 as u32,
    0xbcc3cad1 as u32,
    0xd8dfe6ed as u32,
    0xf4fb0209 as u32,
    0x10171e25 as i32 as uint32_t,
    0x2c333a41 as i32 as uint32_t,
    0x484f565d as i32 as uint32_t,
    0x646b7279 as i32 as uint32_t,
];
static mut sbox: [uint8_t; 256] = [
    0xd6 as i32 as uint8_t,
    0x90 as i32 as uint8_t,
    0xe9 as i32 as uint8_t,
    0xfe as i32 as uint8_t,
    0xcc as i32 as uint8_t,
    0xe1 as i32 as uint8_t,
    0x3d as i32 as uint8_t,
    0xb7 as i32 as uint8_t,
    0x16 as i32 as uint8_t,
    0xb6 as i32 as uint8_t,
    0x14 as i32 as uint8_t,
    0xc2 as i32 as uint8_t,
    0x28 as i32 as uint8_t,
    0xfb as i32 as uint8_t,
    0x2c as i32 as uint8_t,
    0x5 as i32 as uint8_t,
    0x2b as i32 as uint8_t,
    0x67 as i32 as uint8_t,
    0x9a as i32 as uint8_t,
    0x76 as i32 as uint8_t,
    0x2a as i32 as uint8_t,
    0xbe as i32 as uint8_t,
    0x4 as i32 as uint8_t,
    0xc3 as i32 as uint8_t,
    0xaa as i32 as uint8_t,
    0x44 as i32 as uint8_t,
    0x13 as i32 as uint8_t,
    0x26 as i32 as uint8_t,
    0x49 as i32 as uint8_t,
    0x86 as i32 as uint8_t,
    0x6 as i32 as uint8_t,
    0x99 as i32 as uint8_t,
    0x9c as i32 as uint8_t,
    0x42 as i32 as uint8_t,
    0x50 as i32 as uint8_t,
    0xf4 as i32 as uint8_t,
    0x91 as i32 as uint8_t,
    0xef as i32 as uint8_t,
    0x98 as i32 as uint8_t,
    0x7a as i32 as uint8_t,
    0x33 as i32 as uint8_t,
    0x54 as i32 as uint8_t,
    0xb as i32 as uint8_t,
    0x43 as i32 as uint8_t,
    0xed as i32 as uint8_t,
    0xcf as i32 as uint8_t,
    0xac as i32 as uint8_t,
    0x62 as i32 as uint8_t,
    0xe4 as i32 as uint8_t,
    0xb3 as i32 as uint8_t,
    0x1c as i32 as uint8_t,
    0xa9 as i32 as uint8_t,
    0xc9 as i32 as uint8_t,
    0x8 as i32 as uint8_t,
    0xe8 as i32 as uint8_t,
    0x95 as i32 as uint8_t,
    0x80 as i32 as uint8_t,
    0xdf as i32 as uint8_t,
    0x94 as i32 as uint8_t,
    0xfa as i32 as uint8_t,
    0x75 as i32 as uint8_t,
    0x8f as i32 as uint8_t,
    0x3f as i32 as uint8_t,
    0xa6 as i32 as uint8_t,
    0x47 as i32 as uint8_t,
    0x7 as i32 as uint8_t,
    0xa7 as i32 as uint8_t,
    0xfc as i32 as uint8_t,
    0xf3 as i32 as uint8_t,
    0x73 as i32 as uint8_t,
    0x17 as i32 as uint8_t,
    0xba as i32 as uint8_t,
    0x83 as i32 as uint8_t,
    0x59 as i32 as uint8_t,
    0x3c as i32 as uint8_t,
    0x19 as i32 as uint8_t,
    0xe6 as i32 as uint8_t,
    0x85 as i32 as uint8_t,
    0x4f as i32 as uint8_t,
    0xa8 as i32 as uint8_t,
    0x68 as i32 as uint8_t,
    0x6b as i32 as uint8_t,
    0x81 as i32 as uint8_t,
    0xb2 as i32 as uint8_t,
    0x71 as i32 as uint8_t,
    0x64 as i32 as uint8_t,
    0xda as i32 as uint8_t,
    0x8b as i32 as uint8_t,
    0xf8 as i32 as uint8_t,
    0xeb as i32 as uint8_t,
    0xf as i32 as uint8_t,
    0x4b as i32 as uint8_t,
    0x70 as i32 as uint8_t,
    0x56 as i32 as uint8_t,
    0x9d as i32 as uint8_t,
    0x35 as i32 as uint8_t,
    0x1e as i32 as uint8_t,
    0x24 as i32 as uint8_t,
    0xe as i32 as uint8_t,
    0x5e as i32 as uint8_t,
    0x63 as i32 as uint8_t,
    0x58 as i32 as uint8_t,
    0xd1 as i32 as uint8_t,
    0xa2 as i32 as uint8_t,
    0x25 as i32 as uint8_t,
    0x22 as i32 as uint8_t,
    0x7c as i32 as uint8_t,
    0x3b as i32 as uint8_t,
    0x1 as i32 as uint8_t,
    0x21 as i32 as uint8_t,
    0x78 as i32 as uint8_t,
    0x87 as i32 as uint8_t,
    0xd4 as i32 as uint8_t,
    0 as i32 as uint8_t,
    0x46 as i32 as uint8_t,
    0x57 as i32 as uint8_t,
    0x9f as i32 as uint8_t,
    0xd3 as i32 as uint8_t,
    0x27 as i32 as uint8_t,
    0x52 as i32 as uint8_t,
    0x4c as i32 as uint8_t,
    0x36 as i32 as uint8_t,
    0x2 as i32 as uint8_t,
    0xe7 as i32 as uint8_t,
    0xa0 as i32 as uint8_t,
    0xc4 as i32 as uint8_t,
    0xc8 as i32 as uint8_t,
    0x9e as i32 as uint8_t,
    0xea as i32 as uint8_t,
    0xbf as i32 as uint8_t,
    0x8a as i32 as uint8_t,
    0xd2 as i32 as uint8_t,
    0x40 as i32 as uint8_t,
    0xc7 as i32 as uint8_t,
    0x38 as i32 as uint8_t,
    0xb5 as i32 as uint8_t,
    0xa3 as i32 as uint8_t,
    0xf7 as i32 as uint8_t,
    0xf2 as i32 as uint8_t,
    0xce as i32 as uint8_t,
    0xf9 as i32 as uint8_t,
    0x61 as i32 as uint8_t,
    0x15 as i32 as uint8_t,
    0xa1 as i32 as uint8_t,
    0xe0 as i32 as uint8_t,
    0xae as i32 as uint8_t,
    0x5d as i32 as uint8_t,
    0xa4 as i32 as uint8_t,
    0x9b as i32 as uint8_t,
    0x34 as i32 as uint8_t,
    0x1a as i32 as uint8_t,
    0x55 as i32 as uint8_t,
    0xad as i32 as uint8_t,
    0x93 as i32 as uint8_t,
    0x32 as i32 as uint8_t,
    0x30 as i32 as uint8_t,
    0xf5 as i32 as uint8_t,
    0x8c as i32 as uint8_t,
    0xb1 as i32 as uint8_t,
    0xe3 as i32 as uint8_t,
    0x1d as i32 as uint8_t,
    0xf6 as i32 as uint8_t,
    0xe2 as i32 as uint8_t,
    0x2e as i32 as uint8_t,
    0x82 as i32 as uint8_t,
    0x66 as i32 as uint8_t,
    0xca as i32 as uint8_t,
    0x60 as i32 as uint8_t,
    0xc0 as i32 as uint8_t,
    0x29 as i32 as uint8_t,
    0x23 as i32 as uint8_t,
    0xab as i32 as uint8_t,
    0xd as i32 as uint8_t,
    0x53 as i32 as uint8_t,
    0x4e as i32 as uint8_t,
    0x6f as i32 as uint8_t,
    0xd5 as i32 as uint8_t,
    0xdb as i32 as uint8_t,
    0x37 as i32 as uint8_t,
    0x45 as i32 as uint8_t,
    0xde as i32 as uint8_t,
    0xfd as i32 as uint8_t,
    0x8e as i32 as uint8_t,
    0x2f as i32 as uint8_t,
    0x3 as i32 as uint8_t,
    0xff as i32 as uint8_t,
    0x6a as i32 as uint8_t,
    0x72 as i32 as uint8_t,
    0x6d as i32 as uint8_t,
    0x6c as i32 as uint8_t,
    0x5b as i32 as uint8_t,
    0x51 as i32 as uint8_t,
    0x8d as i32 as uint8_t,
    0x1b as i32 as uint8_t,
    0xaf as i32 as uint8_t,
    0x92 as i32 as uint8_t,
    0xbb as i32 as uint8_t,
    0xdd as i32 as uint8_t,
    0xbc as i32 as uint8_t,
    0x7f as i32 as uint8_t,
    0x11 as i32 as uint8_t,
    0xd9 as i32 as uint8_t,
    0x5c as i32 as uint8_t,
    0x41 as i32 as uint8_t,
    0x1f as i32 as uint8_t,
    0x10 as i32 as uint8_t,
    0x5a as i32 as uint8_t,
    0xd8 as i32 as uint8_t,
    0xa as i32 as uint8_t,
    0xc1 as i32 as uint8_t,
    0x31 as i32 as uint8_t,
    0x88 as i32 as uint8_t,
    0xa5 as i32 as uint8_t,
    0xcd as i32 as uint8_t,
    0x7b as i32 as uint8_t,
    0xbd as i32 as uint8_t,
    0x2d as i32 as uint8_t,
    0x74 as i32 as uint8_t,
    0xd0 as i32 as uint8_t,
    0x12 as i32 as uint8_t,
    0xb8 as i32 as uint8_t,
    0xe5 as i32 as uint8_t,
    0xb4 as i32 as uint8_t,
    0xb0 as i32 as uint8_t,
    0x89 as i32 as uint8_t,
    0x69 as i32 as uint8_t,
    0x97 as i32 as uint8_t,
    0x4a as i32 as uint8_t,
    0xc as i32 as uint8_t,
    0x96 as i32 as uint8_t,
    0x77 as i32 as uint8_t,
    0x7e as i32 as uint8_t,
    0x65 as i32 as uint8_t,
    0xb9 as i32 as uint8_t,
    0xf1 as i32 as uint8_t,
    0x9 as i32 as uint8_t,
    0xc5 as i32 as uint8_t,
    0x6e as i32 as uint8_t,
    0xc6 as i32 as uint8_t,
    0x84 as i32 as uint8_t,
    0x18 as i32 as uint8_t,
    0xf0 as i32 as uint8_t,
    0x7d as i32 as uint8_t,
    0xec as i32 as uint8_t,
    0x3a as i32 as uint8_t,
    0xdc as i32 as uint8_t,
    0x4d as i32 as uint8_t,
    0x20 as i32 as uint8_t,
    0x79 as i32 as uint8_t,
    0xee as i32 as uint8_t,
    0x5f as i32 as uint8_t,
    0x3e as i32 as uint8_t,
    0xd7 as i32 as uint8_t,
    0xcb as i32 as uint8_t,
    0x39 as i32 as uint8_t,
    0x48 as i32 as uint8_t,
];
unsafe extern "C" fn sm4_t_non_lin_sub(mut x: uint32_t) -> uint32_t {
    let mut out: uint32_t = 0;
    out = sbox[(x & 0xff as i32 as u32) as usize] as uint32_t;
    out |= (sbox[(x >> 8 as i32 & 0xff as i32 as u32) as usize] as uint32_t) << 8 as i32;
    out
        |= (sbox[(x >> 16 as i32 & 0xff as i32 as u32) as usize] as uint32_t)
            << 16 as i32;
    out
        |= (sbox[(x >> 24 as i32 & 0xff as i32 as u32) as usize] as uint32_t)
            << 24 as i32;
    return out;
}
unsafe extern "C" fn sm4_key_lin_sub(mut x: uint32_t) -> uint32_t {
    return x ^ (x << 13 as i32 | x >> (-(13 as i32) & 31 as i32))
        ^ (x << 23 as i32 | x >> (-(23 as i32) & 31 as i32));
}
unsafe extern "C" fn sm4_enc_lin_sub(mut x: uint32_t) -> uint32_t {
    return x ^ (x << 2 as i32 | x >> (-(2 as i32) & 31 as i32))
        ^ (x << 10 as i32 | x >> (-(10 as i32) & 31 as i32))
        ^ (x << 18 as i32 | x >> (-(18 as i32) & 31 as i32))
        ^ (x << 24 as i32 | x >> (-(24 as i32) & 31 as i32));
}
unsafe extern "C" fn sm4_key_sub(mut x: uint32_t) -> uint32_t {
    return sm4_key_lin_sub(sm4_t_non_lin_sub(x));
}
unsafe extern "C" fn sm4_enc_sub(mut x: uint32_t) -> uint32_t {
    return sm4_enc_lin_sub(sm4_t_non_lin_sub(x));
}
unsafe extern "C" fn sm4_round(
    mut x0: uint32_t,
    mut x1: uint32_t,
    mut x2: uint32_t,
    mut x3: uint32_t,
    mut rk: uint32_t,
) -> uint32_t {
    return x0 ^ sm4_enc_sub(x1 ^ x2 ^ x3 ^ rk);
}
unsafe extern "C" fn sm4_set_key(
    mut ctx: *mut sm4_ctx,
    mut key: *const uint8_t,
    mut encrypt: i32,
) {
    let mut rk0: uint32_t = 0;
    let mut rk1: uint32_t = 0;
    let mut rk2: uint32_t = 0;
    let mut rk3: uint32_t = 0;
    let mut i: u32 = 0;
    rk0 = ((*key.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
        << 24 as i32
        | (*key.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(0 as i32 as isize).offset(3 as i32 as isize) as uint32_t)
        ^ fk[0 as i32 as usize];
    rk1 = ((*key.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
        << 24 as i32
        | (*key.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(4 as i32 as isize).offset(3 as i32 as isize) as uint32_t)
        ^ fk[1 as i32 as usize];
    rk2 = ((*key.offset(8 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
        << 24 as i32
        | (*key.offset(8 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(8 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(8 as i32 as isize).offset(3 as i32 as isize) as uint32_t)
        ^ fk[2 as i32 as usize];
    rk3 = ((*key.offset(12 as i32 as isize).offset(0 as i32 as isize) as uint32_t)
        << 24 as i32
        | (*key.offset(12 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(12 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(12 as i32 as isize).offset(3 as i32 as isize) as uint32_t)
        ^ fk[3 as i32 as usize];
    i = 0 as i32 as u32;
    while i < 32 as i32 as u32 {
        rk0
            ^= sm4_key_sub(
                rk1 ^ rk2 ^ rk3 ^ ck[i.wrapping_add(0 as i32 as u32) as usize],
            );
        rk1
            ^= sm4_key_sub(
                rk2 ^ rk3 ^ rk0 ^ ck[i.wrapping_add(1 as i32 as u32) as usize],
            );
        rk2
            ^= sm4_key_sub(
                rk3 ^ rk0 ^ rk1 ^ ck[i.wrapping_add(2 as i32 as u32) as usize],
            );
        rk3
            ^= sm4_key_sub(
                rk0 ^ rk1 ^ rk2 ^ ck[i.wrapping_add(3 as i32 as u32) as usize],
            );
        if encrypt != 0 {
            (*ctx).rkey[i.wrapping_add(0 as i32 as u32) as usize] = rk0;
            (*ctx).rkey[i.wrapping_add(1 as i32 as u32) as usize] = rk1;
            (*ctx).rkey[i.wrapping_add(2 as i32 as u32) as usize] = rk2;
            (*ctx).rkey[i.wrapping_add(3 as i32 as u32) as usize] = rk3;
        } else {
            (*ctx).rkey[((31 as i32 - 0 as i32) as u32).wrapping_sub(i) as usize] = rk0;
            (*ctx).rkey[((31 as i32 - 1 as i32) as u32).wrapping_sub(i) as usize] = rk1;
            (*ctx).rkey[((31 as i32 - 2 as i32) as u32).wrapping_sub(i) as usize] = rk2;
            (*ctx).rkey[((31 as i32 - 3 as i32) as u32).wrapping_sub(i) as usize] = rk3;
        }
        i = i.wrapping_add(4 as i32 as u32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sm4_set_encrypt_key(
    mut ctx: *mut sm4_ctx,
    mut key: *const uint8_t,
) {
    sm4_set_key(ctx, key, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sm4_set_decrypt_key(
    mut ctx: *mut sm4_ctx,
    mut key: *const uint8_t,
) {
    sm4_set_key(ctx, key, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sm4_crypt(
    mut context: *const sm4_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut rk: *const uint32_t = ((*context).rkey).as_ptr();
    if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
        __assert_fail(
            b"!(length % SM4_BLOCK_SIZE)\0" as *const u8 as *const i8,
            b"sm4.c\0" as *const u8 as *const i8,
            195 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 82],
                &[i8; 82],
            >(
                b"void nettle_sm4_crypt(const struct sm4_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2599: {
        if length.wrapping_rem(16 as i32 as u64) == 0 {} else {
            __assert_fail(
                b"!(length % SM4_BLOCK_SIZE)\0" as *const u8 as *const i8,
                b"sm4.c\0" as *const u8 as *const i8,
                195 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[i8; 82],
                >(
                    b"void nettle_sm4_crypt(const struct sm4_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while length != 0 {
        let mut x0: uint32_t = 0;
        let mut x1: uint32_t = 0;
        let mut x2: uint32_t = 0;
        let mut x3: uint32_t = 0;
        let mut i: u32 = 0;
        x0 = (*src.offset((0 as i32 * 4 as i32) as isize).offset(0 as i32 as isize)
            as uint32_t) << 24 as i32
            | (*src.offset((0 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
                as uint32_t) << 16 as i32
            | (*src.offset((0 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
                as uint32_t) << 8 as i32
            | *src.offset((0 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
                as uint32_t;
        x1 = (*src.offset((1 as i32 * 4 as i32) as isize).offset(0 as i32 as isize)
            as uint32_t) << 24 as i32
            | (*src.offset((1 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
                as uint32_t) << 16 as i32
            | (*src.offset((1 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
                as uint32_t) << 8 as i32
            | *src.offset((1 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
                as uint32_t;
        x2 = (*src.offset((2 as i32 * 4 as i32) as isize).offset(0 as i32 as isize)
            as uint32_t) << 24 as i32
            | (*src.offset((2 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
                as uint32_t) << 16 as i32
            | (*src.offset((2 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
                as uint32_t) << 8 as i32
            | *src.offset((2 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
                as uint32_t;
        x3 = (*src.offset((3 as i32 * 4 as i32) as isize).offset(0 as i32 as isize)
            as uint32_t) << 24 as i32
            | (*src.offset((3 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
                as uint32_t) << 16 as i32
            | (*src.offset((3 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
                as uint32_t) << 8 as i32
            | *src.offset((3 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
                as uint32_t;
        i = 0 as i32 as u32;
        while i < 32 as i32 as u32 {
            x0 = sm4_round(
                x0,
                x1,
                x2,
                x3,
                *rk.offset(i.wrapping_add(0 as i32 as u32) as isize),
            );
            x1 = sm4_round(
                x1,
                x2,
                x3,
                x0,
                *rk.offset(i.wrapping_add(1 as i32 as u32) as isize),
            );
            x2 = sm4_round(
                x2,
                x3,
                x0,
                x1,
                *rk.offset(i.wrapping_add(2 as i32 as u32) as isize),
            );
            x3 = sm4_round(
                x3,
                x0,
                x1,
                x2,
                *rk.offset(i.wrapping_add(3 as i32 as u32) as isize),
            );
            i = i.wrapping_add(4 as i32 as u32);
        }
        *dst.offset((0 as i32 * 4 as i32) as isize).offset(0 as i32 as isize) = (x3
            >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((0 as i32 * 4 as i32) as isize).offset(1 as i32 as isize) = (x3
            >> 16 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((0 as i32 * 4 as i32) as isize).offset(2 as i32 as isize) = (x3
            >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((0 as i32 * 4 as i32) as isize).offset(3 as i32 as isize) = (x3
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset((1 as i32 * 4 as i32) as isize).offset(0 as i32 as isize) = (x2
            >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((1 as i32 * 4 as i32) as isize).offset(1 as i32 as isize) = (x2
            >> 16 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((1 as i32 * 4 as i32) as isize).offset(2 as i32 as isize) = (x2
            >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((1 as i32 * 4 as i32) as isize).offset(3 as i32 as isize) = (x2
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset((2 as i32 * 4 as i32) as isize).offset(0 as i32 as isize) = (x1
            >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((2 as i32 * 4 as i32) as isize).offset(1 as i32 as isize) = (x1
            >> 16 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((2 as i32 * 4 as i32) as isize).offset(2 as i32 as isize) = (x1
            >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((2 as i32 * 4 as i32) as isize).offset(3 as i32 as isize) = (x1
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset((3 as i32 * 4 as i32) as isize).offset(0 as i32 as isize) = (x0
            >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((3 as i32 * 4 as i32) as isize).offset(1 as i32 as isize) = (x0
            >> 16 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((3 as i32 * 4 as i32) as isize).offset(2 as i32 as isize) = (x0
            >> 8 as i32 & 0xff as i32 as u32) as uint8_t;
        *dst.offset((3 as i32 * 4 as i32) as isize).offset(3 as i32 as isize) = (x0
            & 0xff as i32 as u32) as uint8_t;
        src = src.offset(16 as i32 as isize);
        dst = dst.offset(16 as i32 as isize);
        length = (length as u64).wrapping_sub(16 as i32 as u64) as size_t as size_t;
    }
}