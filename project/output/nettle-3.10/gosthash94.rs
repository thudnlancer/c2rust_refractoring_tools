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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn _nettle_write_le32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
    static _nettle_gost28147_param_test_3411: gost28147_param;
    static _nettle_gost28147_param_CryptoPro_3411: gost28147_param;
    fn _nettle_gost28147_encrypt_block(
        key: *const uint32_t,
        sbox: *const [uint32_t; 256],
        in_0: *const uint32_t,
        out: *mut uint32_t,
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
pub struct gosthash94_ctx {
    pub hash: [uint32_t; 8],
    pub sum: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gost28147_param {
    pub sbox: [[uint32_t; 256]; 4],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gosthash94_init(mut ctx: *mut gosthash94_ctx) {
    memset(
        ctx as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<gosthash94_ctx>() as u64,
    );
}
unsafe extern "C" fn gost_block_compress(
    mut ctx: *mut gosthash94_ctx,
    mut block: *const uint32_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut i: u32 = 0;
    let mut key: [uint32_t; 8] = [0; 8];
    let mut u: [uint32_t; 8] = [0; 8];
    let mut v: [uint32_t; 8] = [0; 8];
    let mut w: [uint32_t; 8] = [0; 8];
    let mut s: [uint32_t; 8] = [0; 8];
    memcpy(
        u.as_mut_ptr() as *mut libc::c_void,
        ((*ctx).hash).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 8]>() as u64,
    );
    memcpy(
        v.as_mut_ptr() as *mut libc::c_void,
        block as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 8]>() as u64,
    );
    w[0 as i32 as usize] = u[0 as i32 as usize] ^ v[0 as i32 as usize];
    w[1 as i32 as usize] = u[1 as i32 as usize] ^ v[1 as i32 as usize];
    w[2 as i32 as usize] = u[2 as i32 as usize] ^ v[2 as i32 as usize];
    w[3 as i32 as usize] = u[3 as i32 as usize] ^ v[3 as i32 as usize];
    w[4 as i32 as usize] = u[4 as i32 as usize] ^ v[4 as i32 as usize];
    w[5 as i32 as usize] = u[5 as i32 as usize] ^ v[5 as i32 as usize];
    w[6 as i32 as usize] = u[6 as i32 as usize] ^ v[6 as i32 as usize];
    w[7 as i32 as usize] = u[7 as i32 as usize] ^ v[7 as i32 as usize];
    i = 0 as i32 as u32;
    loop {
        key[0 as i32 as usize] = w[0 as i32 as usize] & 0xff as i32 as u32
            | (w[2 as i32 as usize] & 0xff as i32 as u32) << 8 as i32
            | (w[4 as i32 as usize] & 0xff as i32 as u32) << 16 as i32
            | (w[6 as i32 as usize] & 0xff as i32 as u32) << 24 as i32;
        key[1 as i32 as usize] = (w[0 as i32 as usize] & 0xff00 as i32 as u32)
            >> 8 as i32 | w[2 as i32 as usize] & 0xff00 as i32 as u32
            | (w[4 as i32 as usize] & 0xff00 as i32 as u32) << 8 as i32
            | (w[6 as i32 as usize] & 0xff00 as i32 as u32) << 16 as i32;
        key[2 as i32 as usize] = (w[0 as i32 as usize] & 0xff0000 as i32 as u32)
            >> 16 as i32 | (w[2 as i32 as usize] & 0xff0000 as i32 as u32) >> 8 as i32
            | w[4 as i32 as usize] & 0xff0000 as i32 as u32
            | (w[6 as i32 as usize] & 0xff0000 as i32 as u32) << 8 as i32;
        key[3 as i32 as usize] = (w[0 as i32 as usize] & 0xff000000 as u32) >> 24 as i32
            | (w[2 as i32 as usize] & 0xff000000 as u32) >> 16 as i32
            | (w[4 as i32 as usize] & 0xff000000 as u32) >> 8 as i32
            | w[6 as i32 as usize] & 0xff000000 as u32;
        key[4 as i32 as usize] = w[1 as i32 as usize] & 0xff as i32 as u32
            | (w[3 as i32 as usize] & 0xff as i32 as u32) << 8 as i32
            | (w[5 as i32 as usize] & 0xff as i32 as u32) << 16 as i32
            | (w[7 as i32 as usize] & 0xff as i32 as u32) << 24 as i32;
        key[5 as i32 as usize] = (w[1 as i32 as usize] & 0xff00 as i32 as u32)
            >> 8 as i32 | w[3 as i32 as usize] & 0xff00 as i32 as u32
            | (w[5 as i32 as usize] & 0xff00 as i32 as u32) << 8 as i32
            | (w[7 as i32 as usize] & 0xff00 as i32 as u32) << 16 as i32;
        key[6 as i32 as usize] = (w[1 as i32 as usize] & 0xff0000 as i32 as u32)
            >> 16 as i32 | (w[3 as i32 as usize] & 0xff0000 as i32 as u32) >> 8 as i32
            | w[5 as i32 as usize] & 0xff0000 as i32 as u32
            | (w[7 as i32 as usize] & 0xff0000 as i32 as u32) << 8 as i32;
        key[7 as i32 as usize] = (w[1 as i32 as usize] & 0xff000000 as u32) >> 24 as i32
            | (w[3 as i32 as usize] & 0xff000000 as u32) >> 16 as i32
            | (w[5 as i32 as usize] & 0xff000000 as u32) >> 8 as i32
            | w[7 as i32 as usize] & 0xff000000 as u32;
        _nettle_gost28147_encrypt_block(
            key.as_mut_ptr(),
            sbox,
            &mut *((*ctx).hash).as_mut_ptr().offset(i as isize),
            &mut *s.as_mut_ptr().offset(i as isize),
        );
        if i == 0 as i32 as u32 {
            w[0 as i32 as usize] = u[2 as i32 as usize] ^ v[4 as i32 as usize];
            w[1 as i32 as usize] = u[3 as i32 as usize] ^ v[5 as i32 as usize];
            w[2 as i32 as usize] = u[4 as i32 as usize] ^ v[6 as i32 as usize];
            w[3 as i32 as usize] = u[5 as i32 as usize] ^ v[7 as i32 as usize];
            v[0 as i32 as usize] ^= v[2 as i32 as usize];
            w[4 as i32 as usize] = u[6 as i32 as usize] ^ v[0 as i32 as usize];
            v[1 as i32 as usize] ^= v[3 as i32 as usize];
            w[5 as i32 as usize] = u[7 as i32 as usize] ^ v[1 as i32 as usize];
            u[0 as i32 as usize] ^= u[2 as i32 as usize];
            v[2 as i32 as usize] ^= v[4 as i32 as usize];
            w[6 as i32 as usize] = u[0 as i32 as usize] ^ v[2 as i32 as usize];
            u[1 as i32 as usize] ^= u[3 as i32 as usize];
            v[3 as i32 as usize] ^= v[5 as i32 as usize];
            w[7 as i32 as usize] = u[1 as i32 as usize] ^ v[3 as i32 as usize];
        } else if i & 2 as i32 as u32 != 0 as i32 as u32 {
            if i == 6 as i32 as u32 {
                break;
            }
            u[2 as i32 as usize] ^= u[4 as i32 as usize] ^ 0xff as i32 as u32;
            u[3 as i32 as usize] ^= u[5 as i32 as usize] ^ 0xff00ffff as u32;
            u[4 as i32 as usize] ^= 0xff00ff00 as u32;
            u[5 as i32 as usize] ^= 0xff00ff00 as u32;
            u[6 as i32 as usize] ^= 0xff00ff as i32 as u32;
            u[7 as i32 as usize] ^= 0xff00ff as i32 as u32;
            u[0 as i32 as usize] ^= 0xffff00 as i32 as u32;
            u[1 as i32 as usize] ^= 0xff0000ff as u32;
            w[0 as i32 as usize] = u[4 as i32 as usize] ^ v[0 as i32 as usize];
            w[2 as i32 as usize] = u[6 as i32 as usize] ^ v[2 as i32 as usize];
            v[4 as i32 as usize] ^= v[6 as i32 as usize];
            w[4 as i32 as usize] = u[0 as i32 as usize] ^ v[4 as i32 as usize];
            v[6 as i32 as usize] ^= v[0 as i32 as usize];
            w[6 as i32 as usize] = u[2 as i32 as usize] ^ v[6 as i32 as usize];
            w[1 as i32 as usize] = u[5 as i32 as usize] ^ v[1 as i32 as usize];
            w[3 as i32 as usize] = u[7 as i32 as usize] ^ v[3 as i32 as usize];
            v[5 as i32 as usize] ^= v[7 as i32 as usize];
            w[5 as i32 as usize] = u[1 as i32 as usize] ^ v[5 as i32 as usize];
            v[7 as i32 as usize] ^= v[1 as i32 as usize];
            w[7 as i32 as usize] = u[3 as i32 as usize] ^ v[7 as i32 as usize];
        } else {
            w[0 as i32 as usize] = u[6 as i32 as usize] ^ v[4 as i32 as usize];
            w[1 as i32 as usize] = u[7 as i32 as usize] ^ v[5 as i32 as usize];
            w[2 as i32 as usize] = u[0 as i32 as usize] ^ v[6 as i32 as usize];
            w[3 as i32 as usize] = u[1 as i32 as usize] ^ v[7 as i32 as usize];
            v[0 as i32 as usize] ^= v[2 as i32 as usize];
            w[4 as i32 as usize] = u[2 as i32 as usize] ^ v[0 as i32 as usize];
            v[1 as i32 as usize] ^= v[3 as i32 as usize];
            w[5 as i32 as usize] = u[3 as i32 as usize] ^ v[1 as i32 as usize];
            u[4 as i32 as usize] ^= u[6 as i32 as usize];
            v[2 as i32 as usize] ^= v[4 as i32 as usize];
            w[6 as i32 as usize] = u[4 as i32 as usize] ^ v[2 as i32 as usize];
            u[5 as i32 as usize] ^= u[7 as i32 as usize];
            v[3 as i32 as usize] ^= v[5 as i32 as usize];
            w[7 as i32 as usize] = u[5 as i32 as usize] ^ v[3 as i32 as usize];
        }
        i = i.wrapping_add(2 as i32 as u32);
    }
    u[0 as i32 as usize] = *block.offset(0 as i32 as isize) ^ s[6 as i32 as usize];
    u[1 as i32 as usize] = *block.offset(1 as i32 as isize) ^ s[7 as i32 as usize];
    u[2 as i32 as usize] = *block.offset(2 as i32 as isize)
        ^ s[0 as i32 as usize] << 16 as i32 ^ s[0 as i32 as usize] >> 16 as i32
        ^ s[0 as i32 as usize] & 0xffff as i32 as u32
        ^ s[1 as i32 as usize] & 0xffff as i32 as u32 ^ s[1 as i32 as usize] >> 16 as i32
        ^ s[2 as i32 as usize] << 16 as i32 ^ s[6 as i32 as usize]
        ^ s[6 as i32 as usize] << 16 as i32 ^ s[7 as i32 as usize] & 0xffff0000 as u32
        ^ s[7 as i32 as usize] >> 16 as i32;
    u[3 as i32 as usize] = *block.offset(3 as i32 as isize)
        ^ s[0 as i32 as usize] & 0xffff as i32 as u32 ^ s[0 as i32 as usize] << 16 as i32
        ^ s[1 as i32 as usize] & 0xffff as i32 as u32 ^ s[1 as i32 as usize] << 16 as i32
        ^ s[1 as i32 as usize] >> 16 as i32 ^ s[2 as i32 as usize] << 16 as i32
        ^ s[2 as i32 as usize] >> 16 as i32 ^ s[3 as i32 as usize] << 16 as i32
        ^ s[6 as i32 as usize] ^ s[6 as i32 as usize] << 16 as i32
        ^ s[6 as i32 as usize] >> 16 as i32 ^ s[7 as i32 as usize] & 0xffff as i32 as u32
        ^ s[7 as i32 as usize] << 16 as i32 ^ s[7 as i32 as usize] >> 16 as i32;
    u[4 as i32 as usize] = *block.offset(4 as i32 as isize)
        ^ s[0 as i32 as usize] & 0xffff0000 as u32 ^ s[0 as i32 as usize] << 16 as i32
        ^ s[0 as i32 as usize] >> 16 as i32 ^ s[1 as i32 as usize] & 0xffff0000 as u32
        ^ s[1 as i32 as usize] >> 16 as i32 ^ s[2 as i32 as usize] << 16 as i32
        ^ s[2 as i32 as usize] >> 16 as i32 ^ s[3 as i32 as usize] << 16 as i32
        ^ s[3 as i32 as usize] >> 16 as i32 ^ s[4 as i32 as usize] << 16 as i32
        ^ s[6 as i32 as usize] << 16 as i32 ^ s[6 as i32 as usize] >> 16 as i32
        ^ s[7 as i32 as usize] & 0xffff as i32 as u32 ^ s[7 as i32 as usize] << 16 as i32
        ^ s[7 as i32 as usize] >> 16 as i32;
    u[5 as i32 as usize] = *block.offset(5 as i32 as isize)
        ^ s[0 as i32 as usize] << 16 as i32 ^ s[0 as i32 as usize] >> 16 as i32
        ^ s[0 as i32 as usize] & 0xffff0000 as u32
        ^ s[1 as i32 as usize] & 0xffff as i32 as u32 ^ s[2 as i32 as usize]
        ^ s[2 as i32 as usize] >> 16 as i32 ^ s[3 as i32 as usize] << 16 as i32
        ^ s[3 as i32 as usize] >> 16 as i32 ^ s[4 as i32 as usize] << 16 as i32
        ^ s[4 as i32 as usize] >> 16 as i32 ^ s[5 as i32 as usize] << 16 as i32
        ^ s[6 as i32 as usize] << 16 as i32 ^ s[6 as i32 as usize] >> 16 as i32
        ^ s[7 as i32 as usize] & 0xffff0000 as u32 ^ s[7 as i32 as usize] << 16 as i32
        ^ s[7 as i32 as usize] >> 16 as i32;
    u[6 as i32 as usize] = *block.offset(6 as i32 as isize) ^ s[0 as i32 as usize]
        ^ s[1 as i32 as usize] >> 16 as i32 ^ s[2 as i32 as usize] << 16 as i32
        ^ s[3 as i32 as usize] ^ s[3 as i32 as usize] >> 16 as i32
        ^ s[4 as i32 as usize] << 16 as i32 ^ s[4 as i32 as usize] >> 16 as i32
        ^ s[5 as i32 as usize] << 16 as i32 ^ s[5 as i32 as usize] >> 16 as i32
        ^ s[6 as i32 as usize] ^ s[6 as i32 as usize] << 16 as i32
        ^ s[6 as i32 as usize] >> 16 as i32 ^ s[7 as i32 as usize] << 16 as i32;
    u[7 as i32 as usize] = *block.offset(7 as i32 as isize)
        ^ s[0 as i32 as usize] & 0xffff0000 as u32 ^ s[0 as i32 as usize] << 16 as i32
        ^ s[1 as i32 as usize] & 0xffff as i32 as u32 ^ s[1 as i32 as usize] << 16 as i32
        ^ s[2 as i32 as usize] >> 16 as i32 ^ s[3 as i32 as usize] << 16 as i32
        ^ s[4 as i32 as usize] ^ s[4 as i32 as usize] >> 16 as i32
        ^ s[5 as i32 as usize] << 16 as i32 ^ s[5 as i32 as usize] >> 16 as i32
        ^ s[6 as i32 as usize] >> 16 as i32 ^ s[7 as i32 as usize] & 0xffff as i32 as u32
        ^ s[7 as i32 as usize] << 16 as i32 ^ s[7 as i32 as usize] >> 16 as i32;
    v[0 as i32 as usize] = (*ctx).hash[0 as i32 as usize]
        ^ u[1 as i32 as usize] << 16 as i32 ^ u[0 as i32 as usize] >> 16 as i32;
    v[1 as i32 as usize] = (*ctx).hash[1 as i32 as usize]
        ^ u[2 as i32 as usize] << 16 as i32 ^ u[1 as i32 as usize] >> 16 as i32;
    v[2 as i32 as usize] = (*ctx).hash[2 as i32 as usize]
        ^ u[3 as i32 as usize] << 16 as i32 ^ u[2 as i32 as usize] >> 16 as i32;
    v[3 as i32 as usize] = (*ctx).hash[3 as i32 as usize]
        ^ u[4 as i32 as usize] << 16 as i32 ^ u[3 as i32 as usize] >> 16 as i32;
    v[4 as i32 as usize] = (*ctx).hash[4 as i32 as usize]
        ^ u[5 as i32 as usize] << 16 as i32 ^ u[4 as i32 as usize] >> 16 as i32;
    v[5 as i32 as usize] = (*ctx).hash[5 as i32 as usize]
        ^ u[6 as i32 as usize] << 16 as i32 ^ u[5 as i32 as usize] >> 16 as i32;
    v[6 as i32 as usize] = (*ctx).hash[6 as i32 as usize]
        ^ u[7 as i32 as usize] << 16 as i32 ^ u[6 as i32 as usize] >> 16 as i32;
    v[7 as i32 as usize] = (*ctx).hash[7 as i32 as usize]
        ^ u[0 as i32 as usize] & 0xffff0000 as u32 ^ u[0 as i32 as usize] << 16 as i32
        ^ u[1 as i32 as usize] & 0xffff0000 as u32 ^ u[1 as i32 as usize] << 16 as i32
        ^ u[6 as i32 as usize] << 16 as i32 ^ u[7 as i32 as usize] & 0xffff0000 as u32
        ^ u[7 as i32 as usize] >> 16 as i32;
    (*ctx).hash[0 as i32 as usize] = v[0 as i32 as usize] & 0xffff0000 as u32
        ^ v[0 as i32 as usize] << 16 as i32 ^ v[0 as i32 as usize] >> 16 as i32
        ^ v[1 as i32 as usize] >> 16 as i32 ^ v[1 as i32 as usize] & 0xffff0000 as u32
        ^ v[2 as i32 as usize] << 16 as i32 ^ v[3 as i32 as usize] >> 16 as i32
        ^ v[4 as i32 as usize] << 16 as i32 ^ v[5 as i32 as usize] >> 16 as i32
        ^ v[5 as i32 as usize] ^ v[6 as i32 as usize] >> 16 as i32
        ^ v[7 as i32 as usize] << 16 as i32 ^ v[7 as i32 as usize] >> 16 as i32
        ^ v[7 as i32 as usize] & 0xffff as i32 as u32;
    (*ctx).hash[1 as i32 as usize] = v[0 as i32 as usize] << 16 as i32
        ^ v[0 as i32 as usize] >> 16 as i32 ^ v[0 as i32 as usize] & 0xffff0000 as u32
        ^ v[1 as i32 as usize] & 0xffff as i32 as u32 ^ v[2 as i32 as usize]
        ^ v[2 as i32 as usize] >> 16 as i32 ^ v[3 as i32 as usize] << 16 as i32
        ^ v[4 as i32 as usize] >> 16 as i32 ^ v[5 as i32 as usize] << 16 as i32
        ^ v[6 as i32 as usize] << 16 as i32 ^ v[6 as i32 as usize]
        ^ v[7 as i32 as usize] & 0xffff0000 as u32 ^ v[7 as i32 as usize] >> 16 as i32;
    (*ctx).hash[2 as i32 as usize] = v[0 as i32 as usize] & 0xffff as i32 as u32
        ^ v[0 as i32 as usize] << 16 as i32 ^ v[1 as i32 as usize] << 16 as i32
        ^ v[1 as i32 as usize] >> 16 as i32 ^ v[1 as i32 as usize] & 0xffff0000 as u32
        ^ v[2 as i32 as usize] << 16 as i32 ^ v[3 as i32 as usize] >> 16 as i32
        ^ v[3 as i32 as usize] ^ v[4 as i32 as usize] << 16 as i32
        ^ v[5 as i32 as usize] >> 16 as i32 ^ v[6 as i32 as usize]
        ^ v[6 as i32 as usize] >> 16 as i32 ^ v[7 as i32 as usize] & 0xffff as i32 as u32
        ^ v[7 as i32 as usize] << 16 as i32 ^ v[7 as i32 as usize] >> 16 as i32;
    (*ctx).hash[3 as i32 as usize] = v[0 as i32 as usize] << 16 as i32
        ^ v[0 as i32 as usize] >> 16 as i32 ^ v[0 as i32 as usize] & 0xffff0000 as u32
        ^ v[1 as i32 as usize] & 0xffff0000 as u32 ^ v[1 as i32 as usize] >> 16 as i32
        ^ v[2 as i32 as usize] << 16 as i32 ^ v[2 as i32 as usize] >> 16 as i32
        ^ v[2 as i32 as usize] ^ v[3 as i32 as usize] << 16 as i32
        ^ v[4 as i32 as usize] >> 16 as i32 ^ v[4 as i32 as usize]
        ^ v[5 as i32 as usize] << 16 as i32 ^ v[6 as i32 as usize] << 16 as i32
        ^ v[7 as i32 as usize] & 0xffff as i32 as u32
        ^ v[7 as i32 as usize] >> 16 as i32;
    (*ctx).hash[4 as i32 as usize] = v[0 as i32 as usize] >> 16 as i32
        ^ v[1 as i32 as usize] << 16 as i32 ^ v[1 as i32 as usize]
        ^ v[2 as i32 as usize] >> 16 as i32 ^ v[2 as i32 as usize]
        ^ v[3 as i32 as usize] << 16 as i32 ^ v[3 as i32 as usize] >> 16 as i32
        ^ v[3 as i32 as usize] ^ v[4 as i32 as usize] << 16 as i32
        ^ v[5 as i32 as usize] >> 16 as i32 ^ v[5 as i32 as usize]
        ^ v[6 as i32 as usize] << 16 as i32 ^ v[6 as i32 as usize] >> 16 as i32
        ^ v[7 as i32 as usize] << 16 as i32;
    (*ctx).hash[5 as i32 as usize] = v[0 as i32 as usize] << 16 as i32
        ^ v[0 as i32 as usize] & 0xffff0000 as u32 ^ v[1 as i32 as usize] << 16 as i32
        ^ v[1 as i32 as usize] >> 16 as i32 ^ v[1 as i32 as usize] & 0xffff0000 as u32
        ^ v[2 as i32 as usize] << 16 as i32 ^ v[2 as i32 as usize]
        ^ v[3 as i32 as usize] >> 16 as i32 ^ v[3 as i32 as usize]
        ^ v[4 as i32 as usize] << 16 as i32 ^ v[4 as i32 as usize] >> 16 as i32
        ^ v[4 as i32 as usize] ^ v[5 as i32 as usize] << 16 as i32
        ^ v[6 as i32 as usize] << 16 as i32 ^ v[6 as i32 as usize] >> 16 as i32
        ^ v[6 as i32 as usize] ^ v[7 as i32 as usize] << 16 as i32
        ^ v[7 as i32 as usize] >> 16 as i32 ^ v[7 as i32 as usize] & 0xffff0000 as u32;
    (*ctx).hash[6 as i32 as usize] = v[0 as i32 as usize] ^ v[2 as i32 as usize]
        ^ v[2 as i32 as usize] >> 16 as i32 ^ v[3 as i32 as usize]
        ^ v[3 as i32 as usize] << 16 as i32 ^ v[4 as i32 as usize]
        ^ v[4 as i32 as usize] >> 16 as i32 ^ v[5 as i32 as usize] << 16 as i32
        ^ v[5 as i32 as usize] >> 16 as i32 ^ v[5 as i32 as usize]
        ^ v[6 as i32 as usize] << 16 as i32 ^ v[6 as i32 as usize] >> 16 as i32
        ^ v[6 as i32 as usize] ^ v[7 as i32 as usize] << 16 as i32
        ^ v[7 as i32 as usize];
    (*ctx).hash[7 as i32 as usize] = v[0 as i32 as usize]
        ^ v[0 as i32 as usize] >> 16 as i32 ^ v[1 as i32 as usize] << 16 as i32
        ^ v[1 as i32 as usize] >> 16 as i32 ^ v[2 as i32 as usize] << 16 as i32
        ^ v[3 as i32 as usize] >> 16 as i32 ^ v[3 as i32 as usize]
        ^ v[4 as i32 as usize] << 16 as i32 ^ v[4 as i32 as usize]
        ^ v[5 as i32 as usize] >> 16 as i32 ^ v[5 as i32 as usize]
        ^ v[6 as i32 as usize] << 16 as i32 ^ v[6 as i32 as usize] >> 16 as i32
        ^ v[7 as i32 as usize] << 16 as i32 ^ v[7 as i32 as usize];
}
unsafe extern "C" fn gost_compute_sum_and_hash(
    mut ctx: *mut gosthash94_ctx,
    mut block: *const uint8_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut block_le: [uint32_t; 8] = [0; 8];
    let mut i: u32 = 0;
    let mut carry: u32 = 0;
    carry = 0 as i32 as u32;
    i = carry;
    while i < 8 as i32 as u32 {
        block_le[i as usize] = (*block.offset(3 as i32 as isize) as uint32_t)
            << 24 as i32 | (*block.offset(2 as i32 as isize) as uint32_t) << 16 as i32
            | (*block.offset(1 as i32 as isize) as uint32_t) << 8 as i32
            | *block.offset(0 as i32 as isize) as uint32_t;
        (*ctx).sum[i as usize] = ((*ctx).sum[i as usize] as u32).wrapping_add(carry)
            as uint32_t as uint32_t;
        carry = ((*ctx).sum[i as usize] < carry) as i32 as u32;
        (*ctx).sum[i as usize] = ((*ctx).sum[i as usize] as u32)
            .wrapping_add(block_le[i as usize]) as uint32_t as uint32_t;
        carry = carry
            .wrapping_add(((*ctx).sum[i as usize] < block_le[i as usize]) as i32 as u32);
        i = i.wrapping_add(1);
        i;
        block = block.offset(4 as i32 as isize);
    }
    gost_block_compress(ctx, block_le.as_mut_ptr(), sbox);
}
unsafe extern "C" fn gosthash94_update_int(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut msg: *const uint8_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut current_block: u64;
    if !(length == 0) {
        if (*ctx).index != 0 {
            let mut __md_left: u32 = (::core::mem::size_of::<[uint8_t; 32]>() as u64)
                .wrapping_sub((*ctx).index as u64) as u32;
            if length < __md_left as u64 {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    msg as *const libc::c_void,
                    length,
                );
                (*ctx).index = ((*ctx).index as u64).wrapping_add(length) as u32 as u32;
                current_block = 15904375183555213903;
            } else {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    msg as *const libc::c_void,
                    __md_left as u64,
                );
                gost_compute_sum_and_hash(ctx, ((*ctx).block).as_mut_ptr(), sbox);
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                (*ctx).count;
                msg = msg.offset(__md_left as isize);
                length = (length as u64).wrapping_sub(__md_left as u64) as size_t
                    as size_t;
                current_block = 2868539653012386629;
            }
        } else {
            current_block = 2868539653012386629;
        }
        match current_block {
            15904375183555213903 => {}
            _ => {
                while length >= ::core::mem::size_of::<[uint8_t; 32]>() as u64 {
                    gost_compute_sum_and_hash(ctx, msg, sbox);
                    (*ctx).count = ((*ctx).count).wrapping_add(1);
                    (*ctx).count;
                    msg = msg
                        .offset(::core::mem::size_of::<[uint8_t; 32]>() as u64 as isize);
                    length = (length as u64)
                        .wrapping_sub(::core::mem::size_of::<[uint8_t; 32]>() as u64)
                        as size_t as size_t;
                }
                memcpy(
                    ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                    msg as *const libc::c_void,
                    length,
                );
                (*ctx).index = length as u32;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gosthash94_update(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut msg: *const uint8_t,
) {
    gosthash94_update_int(
        ctx,
        length,
        msg,
        (_nettle_gost28147_param_test_3411.sbox).as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gosthash94cp_update(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut msg: *const uint8_t,
) {
    gosthash94_update_int(
        ctx,
        length,
        msg,
        (_nettle_gost28147_param_CryptoPro_3411.sbox).as_ptr(),
    );
}
unsafe extern "C" fn gosthash94_write_digest(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut result: *mut uint8_t,
    mut sbox: *const [uint32_t; 256],
) {
    let mut msg32: [uint32_t; 8] = [0; 8];
    if length <= 32 as i32 as u64 {} else {
        __assert_fail(
            b"length <= GOSTHASH94_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"gosthash94.c\0" as *const u8 as *const i8,
            349 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[i8; 98],
            >(
                b"void gosthash94_write_digest(struct gosthash94_ctx *, size_t, uint8_t *, const uint32_t (*)[256])\0",
            ))
                .as_ptr(),
        );
    }
    'c_4526: {
        if length <= 32 as i32 as u64 {} else {
            __assert_fail(
                b"length <= GOSTHASH94_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"gosthash94.c\0" as *const u8 as *const i8,
                349 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[i8; 98],
                >(
                    b"void gosthash94_write_digest(struct gosthash94_ctx *, size_t, uint8_t *, const uint32_t (*)[256])\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*ctx).index > 0 as i32 as u32 {
        memset(
            ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                as *mut libc::c_void,
            0 as i32,
            (32 as i32 as u32).wrapping_sub((*ctx).index) as u64,
        );
        gost_compute_sum_and_hash(ctx, ((*ctx).block).as_mut_ptr(), sbox);
    }
    msg32[0 as i32 as usize] = ((*ctx).count << 8 as i32
        | ((*ctx).index << 3 as i32) as u64) as uint32_t;
    msg32[1 as i32 as usize] = ((*ctx).count >> 24 as i32) as uint32_t;
    memset(
        msg32.as_mut_ptr().offset(2 as i32 as isize) as *mut libc::c_void,
        0 as i32,
        (::core::mem::size_of::<uint32_t>() as u64).wrapping_mul(6 as i32 as u64),
    );
    gost_block_compress(ctx, msg32.as_mut_ptr(), sbox);
    gost_block_compress(ctx, ((*ctx).sum).as_mut_ptr(), sbox);
    _nettle_write_le32(length, result, ((*ctx).hash).as_mut_ptr());
    nettle_gosthash94_init(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gosthash94_digest(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut result: *mut uint8_t,
) {
    gosthash94_write_digest(
        ctx,
        length,
        result,
        (_nettle_gost28147_param_test_3411.sbox).as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_gosthash94cp_digest(
    mut ctx: *mut gosthash94_ctx,
    mut length: size_t,
    mut result: *mut uint8_t,
) {
    gosthash94_write_digest(
        ctx,
        length,
        result,
        (_nettle_gost28147_param_CryptoPro_3411.sbox).as_ptr(),
    );
}