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
    fn _nettle_write_be32(length: size_t, dst: *mut uint8_t, src: *const uint32_t);
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
pub struct sm3_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
static mut K: [uint32_t; 64] = [
    0x79cc4519 as u64 as uint32_t,
    0xf3988a32 as u64 as uint32_t,
    0xe7311465 as u64 as uint32_t,
    0xce6228cb as u64 as uint32_t,
    0x9cc45197 as u64 as uint32_t,
    0x3988a32f as u64 as uint32_t,
    0x7311465e as u64 as uint32_t,
    0xe6228cbc as u64 as uint32_t,
    0xcc451979 as u64 as uint32_t,
    0x988a32f3 as u64 as uint32_t,
    0x311465e7 as u64 as uint32_t,
    0x6228cbce as u64 as uint32_t,
    0xc451979c as u64 as uint32_t,
    0x88a32f39 as u64 as uint32_t,
    0x11465e73 as u64 as uint32_t,
    0x228cbce6 as u64 as uint32_t,
    0x9d8a7a87 as u64 as uint32_t,
    0x3b14f50f as u64 as uint32_t,
    0x7629ea1e as u64 as uint32_t,
    0xec53d43c as u64 as uint32_t,
    0xd8a7a879 as u64 as uint32_t,
    0xb14f50f3 as u64 as uint32_t,
    0x629ea1e7 as u64 as uint32_t,
    0xc53d43ce as u64 as uint32_t,
    0x8a7a879d as u64 as uint32_t,
    0x14f50f3b as u64 as uint32_t,
    0x29ea1e76 as u64 as uint32_t,
    0x53d43cec as u64 as uint32_t,
    0xa7a879d8 as u64 as uint32_t,
    0x4f50f3b1 as u64 as uint32_t,
    0x9ea1e762 as u64 as uint32_t,
    0x3d43cec5 as u64 as uint32_t,
    0x7a879d8a as u64 as uint32_t,
    0xf50f3b14 as u64 as uint32_t,
    0xea1e7629 as u64 as uint32_t,
    0xd43cec53 as u64 as uint32_t,
    0xa879d8a7 as u64 as uint32_t,
    0x50f3b14f as u64 as uint32_t,
    0xa1e7629e as u64 as uint32_t,
    0x43cec53d as u64 as uint32_t,
    0x879d8a7a as u64 as uint32_t,
    0xf3b14f5 as u64 as uint32_t,
    0x1e7629ea as u64 as uint32_t,
    0x3cec53d4 as u64 as uint32_t,
    0x79d8a7a8 as u64 as uint32_t,
    0xf3b14f50 as u64 as uint32_t,
    0xe7629ea1 as u64 as uint32_t,
    0xcec53d43 as u64 as uint32_t,
    0x9d8a7a87 as u64 as uint32_t,
    0x3b14f50f as u64 as uint32_t,
    0x7629ea1e as u64 as uint32_t,
    0xec53d43c as u64 as uint32_t,
    0xd8a7a879 as u64 as uint32_t,
    0xb14f50f3 as u64 as uint32_t,
    0x629ea1e7 as u64 as uint32_t,
    0xc53d43ce as u64 as uint32_t,
    0x8a7a879d as u64 as uint32_t,
    0x14f50f3b as u64 as uint32_t,
    0x29ea1e76 as u64 as uint32_t,
    0x53d43cec as u64 as uint32_t,
    0xa7a879d8 as u64 as uint32_t,
    0x4f50f3b1 as u64 as uint32_t,
    0x9ea1e762 as u64 as uint32_t,
    0x3d43cec5 as u64 as uint32_t,
];
unsafe extern "C" fn sm3_compress(mut state: *mut uint32_t, mut input: *const uint8_t) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut f: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut ss1: uint32_t = 0;
    let mut ss2: uint32_t = 0;
    let mut w: [uint32_t; 16] = [0; 16];
    a = *state.offset(0 as i32 as isize);
    b = *state.offset(1 as i32 as isize);
    c = *state.offset(2 as i32 as isize);
    d = *state.offset(3 as i32 as isize);
    e = *state.offset(4 as i32 as isize);
    f = *state.offset(5 as i32 as isize);
    g = *state.offset(6 as i32 as isize);
    h = *state.offset(7 as i32 as isize);
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[0 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[0 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[0 as i32 as usize] = (*input
        .offset((0 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((0 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((0 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((0 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    w[4 as i32 as usize] = (*input
        .offset((4 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((4 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((4 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((4 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(w[0 as i32 as usize] ^ w[4 as i32 as usize]),
        ) as uint32_t as uint32_t;
    w[0 as i32 as usize] = (*input
        .offset((0 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((0 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((0 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((0 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    h = (h as u32)
        .wrapping_add((e ^ f ^ g).wrapping_add(ss1).wrapping_add(w[0 as i32 as usize]))
        as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[1 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[1 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[1 as i32 as usize] = (*input
        .offset((1 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((1 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((1 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((1 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    w[5 as i32 as usize] = (*input
        .offset((5 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((5 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((5 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((5 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(w[1 as i32 as usize] ^ w[5 as i32 as usize]),
        ) as uint32_t as uint32_t;
    w[1 as i32 as usize] = (*input
        .offset((1 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((1 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((1 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((1 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    g = (g as u32)
        .wrapping_add((h ^ e ^ f).wrapping_add(ss1).wrapping_add(w[1 as i32 as usize]))
        as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[2 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[2 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[2 as i32 as usize] = (*input
        .offset((2 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((2 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((2 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((2 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    w[6 as i32 as usize] = (*input
        .offset((6 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((6 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((6 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((6 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(w[2 as i32 as usize] ^ w[6 as i32 as usize]),
        ) as uint32_t as uint32_t;
    w[2 as i32 as usize] = (*input
        .offset((2 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((2 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((2 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((2 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    f = (f as u32)
        .wrapping_add((g ^ h ^ e).wrapping_add(ss1).wrapping_add(w[2 as i32 as usize]))
        as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[3 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[3 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[3 as i32 as usize] = (*input
        .offset((3 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((3 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((3 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((3 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    w[7 as i32 as usize] = (*input
        .offset((7 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((7 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((7 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((7 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(w[3 as i32 as usize] ^ w[7 as i32 as usize]),
        ) as uint32_t as uint32_t;
    w[3 as i32 as usize] = (*input
        .offset((3 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((3 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((3 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((3 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    e = (e as u32)
        .wrapping_add((f ^ g ^ h).wrapping_add(ss1).wrapping_add(w[3 as i32 as usize]))
        as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[4 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[4 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[8 as i32 as usize] = (*input
        .offset((8 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((8 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((8 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((8 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(w[(4 as i32 & 0xf as i32) as usize] ^ w[8 as i32 as usize]),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e ^ f ^ g)
                .wrapping_add(ss1)
                .wrapping_add(w[(4 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[5 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[5 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[9 as i32 as usize] = (*input
        .offset((9 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((9 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((9 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((9 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(w[(5 as i32 & 0xf as i32) as usize] ^ w[9 as i32 as usize]),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h ^ e ^ f)
                .wrapping_add(ss1)
                .wrapping_add(w[(5 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[6 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[6 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[10 as i32 as usize] = (*input
        .offset((10 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((10 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((10 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((10 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(6 as i32 & 0xf as i32) as usize] ^ w[10 as i32 as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g ^ h ^ e)
                .wrapping_add(ss1)
                .wrapping_add(w[(6 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[7 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[7 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[11 as i32 as usize] = (*input
        .offset((11 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((11 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((11 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((11 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(7 as i32 & 0xf as i32) as usize] ^ w[11 as i32 as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f ^ g ^ h)
                .wrapping_add(ss1)
                .wrapping_add(w[(7 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[8 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[8 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[12 as i32 as usize] = (*input
        .offset((12 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((12 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((12 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((12 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(8 as i32 & 0xf as i32) as usize] ^ w[12 as i32 as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e ^ f ^ g)
                .wrapping_add(ss1)
                .wrapping_add(w[(8 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[9 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[9 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[13 as i32 as usize] = (*input
        .offset((13 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((13 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((13 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((13 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(9 as i32 & 0xf as i32) as usize] ^ w[13 as i32 as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h ^ e ^ f)
                .wrapping_add(ss1)
                .wrapping_add(w[(9 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[10 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[10 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[14 as i32 as usize] = (*input
        .offset((14 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((14 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((14 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((14 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(10 as i32 & 0xf as i32) as usize] ^ w[14 as i32 as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g ^ h ^ e)
                .wrapping_add(ss1)
                .wrapping_add(w[(10 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[11 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[11 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[15 as i32 as usize] = (*input
        .offset((15 as i32 * 4 as i32) as isize)
        .offset(0 as i32 as isize) as uint32_t) << 24 as i32
        | (*input.offset((15 as i32 * 4 as i32) as isize).offset(1 as i32 as isize)
            as uint32_t) << 16 as i32
        | (*input.offset((15 as i32 * 4 as i32) as isize).offset(2 as i32 as isize)
            as uint32_t) << 8 as i32
        | *input.offset((15 as i32 * 4 as i32) as isize).offset(3 as i32 as isize)
            as uint32_t;
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(11 as i32 & 0xf as i32) as usize] ^ w[15 as i32 as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f ^ g ^ h)
                .wrapping_add(ss1)
                .wrapping_add(w[(11 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[12 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[12 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(16 as i32 & 0xf as i32) as usize] = w[(16 as i32 & 0xf as i32) as usize]
        ^ w[(16 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(16 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(16 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(16 as i32 & 0xf as i32) as usize]
            ^ w[(16 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(16 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(16 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(16 as i32 & 0xf as i32) as usize]
                ^ w[(16 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(16 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(16 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(16 as i32 & 0xf as i32) as usize]
            ^ w[(16 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(16 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(16 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(16 as i32 & 0xf as i32) as usize]
                ^ w[(16 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(16 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(16 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(16 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(16 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(16 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(12 as i32 & 0xf as i32) as usize]
                        ^ w[(16 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e ^ f ^ g)
                .wrapping_add(ss1)
                .wrapping_add(w[(12 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[13 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[13 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(17 as i32 & 0xf as i32) as usize] = w[(17 as i32 & 0xf as i32) as usize]
        ^ w[(17 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(17 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(17 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(17 as i32 & 0xf as i32) as usize]
            ^ w[(17 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(17 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(17 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(17 as i32 & 0xf as i32) as usize]
                ^ w[(17 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(17 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(17 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(17 as i32 & 0xf as i32) as usize]
            ^ w[(17 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(17 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(17 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(17 as i32 & 0xf as i32) as usize]
                ^ w[(17 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(17 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(17 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(17 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(17 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(17 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(13 as i32 & 0xf as i32) as usize]
                        ^ w[(17 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h ^ e ^ f)
                .wrapping_add(ss1)
                .wrapping_add(w[(13 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[14 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[14 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(18 as i32 & 0xf as i32) as usize] = w[(18 as i32 & 0xf as i32) as usize]
        ^ w[(18 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(18 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(18 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(18 as i32 & 0xf as i32) as usize]
            ^ w[(18 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(18 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(18 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(18 as i32 & 0xf as i32) as usize]
                ^ w[(18 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(18 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(18 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(18 as i32 & 0xf as i32) as usize]
            ^ w[(18 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(18 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(18 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(18 as i32 & 0xf as i32) as usize]
                ^ w[(18 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(18 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(18 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(18 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(18 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(18 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(14 as i32 & 0xf as i32) as usize]
                        ^ w[(18 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g ^ h ^ e)
                .wrapping_add(ss1)
                .wrapping_add(w[(14 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[15 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[15 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(19 as i32 & 0xf as i32) as usize] = w[(19 as i32 & 0xf as i32) as usize]
        ^ w[(19 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(19 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(19 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(19 as i32 & 0xf as i32) as usize]
            ^ w[(19 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(19 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(19 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(19 as i32 & 0xf as i32) as usize]
                ^ w[(19 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(19 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(19 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(19 as i32 & 0xf as i32) as usize]
            ^ w[(19 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(19 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(19 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(19 as i32 & 0xf as i32) as usize]
                ^ w[(19 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(19 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(19 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(19 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(19 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(19 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(15 as i32 & 0xf as i32) as usize]
                        ^ w[(19 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f ^ g ^ h)
                .wrapping_add(ss1)
                .wrapping_add(w[(15 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[16 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[16 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(20 as i32 & 0xf as i32) as usize] = w[(20 as i32 & 0xf as i32) as usize]
        ^ w[(20 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(20 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(20 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(20 as i32 & 0xf as i32) as usize]
            ^ w[(20 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(20 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(20 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(20 as i32 & 0xf as i32) as usize]
                ^ w[(20 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(20 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(20 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(20 as i32 & 0xf as i32) as usize]
            ^ w[(20 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(20 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(20 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(20 as i32 & 0xf as i32) as usize]
                ^ w[(20 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(20 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(20 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(20 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(20 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(20 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(16 as i32 & 0xf as i32) as usize]
                        ^ w[(20 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(16 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[17 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[17 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(21 as i32 & 0xf as i32) as usize] = w[(21 as i32 & 0xf as i32) as usize]
        ^ w[(21 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(21 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(21 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(21 as i32 & 0xf as i32) as usize]
            ^ w[(21 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(21 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(21 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(21 as i32 & 0xf as i32) as usize]
                ^ w[(21 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(21 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(21 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(21 as i32 & 0xf as i32) as usize]
            ^ w[(21 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(21 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(21 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(21 as i32 & 0xf as i32) as usize]
                ^ w[(21 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(21 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(21 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(21 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(21 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(21 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(17 as i32 & 0xf as i32) as usize]
                        ^ w[(21 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(17 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[18 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[18 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(22 as i32 & 0xf as i32) as usize] = w[(22 as i32 & 0xf as i32) as usize]
        ^ w[(22 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(22 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(22 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(22 as i32 & 0xf as i32) as usize]
            ^ w[(22 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(22 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(22 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(22 as i32 & 0xf as i32) as usize]
                ^ w[(22 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(22 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(22 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(22 as i32 & 0xf as i32) as usize]
            ^ w[(22 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(22 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(22 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(22 as i32 & 0xf as i32) as usize]
                ^ w[(22 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(22 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(22 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(22 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(22 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(22 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(18 as i32 & 0xf as i32) as usize]
                        ^ w[(22 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(18 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[19 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[19 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(23 as i32 & 0xf as i32) as usize] = w[(23 as i32 & 0xf as i32) as usize]
        ^ w[(23 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(23 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(23 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(23 as i32 & 0xf as i32) as usize]
            ^ w[(23 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(23 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(23 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(23 as i32 & 0xf as i32) as usize]
                ^ w[(23 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(23 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(23 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(23 as i32 & 0xf as i32) as usize]
            ^ w[(23 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(23 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(23 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(23 as i32 & 0xf as i32) as usize]
                ^ w[(23 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(23 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(23 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(23 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(23 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(23 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(19 as i32 & 0xf as i32) as usize]
                        ^ w[(23 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(19 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[20 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[20 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(24 as i32 & 0xf as i32) as usize] = w[(24 as i32 & 0xf as i32) as usize]
        ^ w[(24 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(24 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(24 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(24 as i32 & 0xf as i32) as usize]
            ^ w[(24 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(24 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(24 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(24 as i32 & 0xf as i32) as usize]
                ^ w[(24 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(24 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(24 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(24 as i32 & 0xf as i32) as usize]
            ^ w[(24 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(24 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(24 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(24 as i32 & 0xf as i32) as usize]
                ^ w[(24 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(24 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(24 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(24 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(24 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(24 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(20 as i32 & 0xf as i32) as usize]
                        ^ w[(24 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(20 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[21 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[21 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(25 as i32 & 0xf as i32) as usize] = w[(25 as i32 & 0xf as i32) as usize]
        ^ w[(25 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(25 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(25 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(25 as i32 & 0xf as i32) as usize]
            ^ w[(25 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(25 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(25 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(25 as i32 & 0xf as i32) as usize]
                ^ w[(25 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(25 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(25 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(25 as i32 & 0xf as i32) as usize]
            ^ w[(25 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(25 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(25 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(25 as i32 & 0xf as i32) as usize]
                ^ w[(25 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(25 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(25 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(25 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(25 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(25 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(21 as i32 & 0xf as i32) as usize]
                        ^ w[(25 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(21 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[22 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[22 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(26 as i32 & 0xf as i32) as usize] = w[(26 as i32 & 0xf as i32) as usize]
        ^ w[(26 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(26 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(26 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(26 as i32 & 0xf as i32) as usize]
            ^ w[(26 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(26 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(26 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(26 as i32 & 0xf as i32) as usize]
                ^ w[(26 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(26 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(26 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(26 as i32 & 0xf as i32) as usize]
            ^ w[(26 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(26 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(26 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(26 as i32 & 0xf as i32) as usize]
                ^ w[(26 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(26 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(26 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(26 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(26 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(26 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(22 as i32 & 0xf as i32) as usize]
                        ^ w[(26 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(22 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[23 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[23 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(27 as i32 & 0xf as i32) as usize] = w[(27 as i32 & 0xf as i32) as usize]
        ^ w[(27 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(27 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(27 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(27 as i32 & 0xf as i32) as usize]
            ^ w[(27 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(27 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(27 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(27 as i32 & 0xf as i32) as usize]
                ^ w[(27 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(27 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(27 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(27 as i32 & 0xf as i32) as usize]
            ^ w[(27 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(27 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(27 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(27 as i32 & 0xf as i32) as usize]
                ^ w[(27 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(27 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(27 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(27 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(27 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(27 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(23 as i32 & 0xf as i32) as usize]
                        ^ w[(27 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(23 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[24 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[24 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(28 as i32 & 0xf as i32) as usize] = w[(28 as i32 & 0xf as i32) as usize]
        ^ w[(28 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(28 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(28 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(28 as i32 & 0xf as i32) as usize]
            ^ w[(28 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(28 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(28 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(28 as i32 & 0xf as i32) as usize]
                ^ w[(28 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(28 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(28 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(28 as i32 & 0xf as i32) as usize]
            ^ w[(28 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(28 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(28 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(28 as i32 & 0xf as i32) as usize]
                ^ w[(28 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(28 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(28 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(28 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(28 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(28 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(24 as i32 & 0xf as i32) as usize]
                        ^ w[(28 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(24 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[25 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[25 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(29 as i32 & 0xf as i32) as usize] = w[(29 as i32 & 0xf as i32) as usize]
        ^ w[(29 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(29 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(29 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(29 as i32 & 0xf as i32) as usize]
            ^ w[(29 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(29 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(29 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(29 as i32 & 0xf as i32) as usize]
                ^ w[(29 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(29 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(29 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(29 as i32 & 0xf as i32) as usize]
            ^ w[(29 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(29 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(29 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(29 as i32 & 0xf as i32) as usize]
                ^ w[(29 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(29 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(29 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(29 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(29 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(29 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(25 as i32 & 0xf as i32) as usize]
                        ^ w[(29 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(25 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[26 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[26 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(30 as i32 & 0xf as i32) as usize] = w[(30 as i32 & 0xf as i32) as usize]
        ^ w[(30 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(30 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(30 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(30 as i32 & 0xf as i32) as usize]
            ^ w[(30 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(30 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(30 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(30 as i32 & 0xf as i32) as usize]
                ^ w[(30 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(30 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(30 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(30 as i32 & 0xf as i32) as usize]
            ^ w[(30 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(30 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(30 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(30 as i32 & 0xf as i32) as usize]
                ^ w[(30 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(30 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(30 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(30 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(30 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(30 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(26 as i32 & 0xf as i32) as usize]
                        ^ w[(30 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(26 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[27 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[27 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(31 as i32 & 0xf as i32) as usize] = w[(31 as i32 & 0xf as i32) as usize]
        ^ w[(31 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(31 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(31 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(31 as i32 & 0xf as i32) as usize]
            ^ w[(31 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(31 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(31 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(31 as i32 & 0xf as i32) as usize]
                ^ w[(31 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(31 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(31 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(31 as i32 & 0xf as i32) as usize]
            ^ w[(31 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(31 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(31 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(31 as i32 & 0xf as i32) as usize]
                ^ w[(31 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(31 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(31 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(31 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(31 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(31 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(27 as i32 & 0xf as i32) as usize]
                        ^ w[(31 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(27 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[28 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[28 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(32 as i32 & 0xf as i32) as usize] = w[(32 as i32 & 0xf as i32) as usize]
        ^ w[(32 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(32 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(32 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(32 as i32 & 0xf as i32) as usize]
            ^ w[(32 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(32 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(32 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(32 as i32 & 0xf as i32) as usize]
                ^ w[(32 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(32 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(32 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(32 as i32 & 0xf as i32) as usize]
            ^ w[(32 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(32 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(32 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(32 as i32 & 0xf as i32) as usize]
                ^ w[(32 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(32 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(32 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(32 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(32 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(32 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(28 as i32 & 0xf as i32) as usize]
                        ^ w[(32 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(28 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[29 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[29 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(33 as i32 & 0xf as i32) as usize] = w[(33 as i32 & 0xf as i32) as usize]
        ^ w[(33 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(33 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(33 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(33 as i32 & 0xf as i32) as usize]
            ^ w[(33 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(33 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(33 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(33 as i32 & 0xf as i32) as usize]
                ^ w[(33 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(33 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(33 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(33 as i32 & 0xf as i32) as usize]
            ^ w[(33 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(33 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(33 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(33 as i32 & 0xf as i32) as usize]
                ^ w[(33 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(33 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(33 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(33 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(33 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(33 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(29 as i32 & 0xf as i32) as usize]
                        ^ w[(33 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(29 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[30 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[30 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(34 as i32 & 0xf as i32) as usize] = w[(34 as i32 & 0xf as i32) as usize]
        ^ w[(34 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(34 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(34 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(34 as i32 & 0xf as i32) as usize]
            ^ w[(34 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(34 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(34 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(34 as i32 & 0xf as i32) as usize]
                ^ w[(34 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(34 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(34 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(34 as i32 & 0xf as i32) as usize]
            ^ w[(34 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(34 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(34 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(34 as i32 & 0xf as i32) as usize]
                ^ w[(34 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(34 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(34 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(34 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(34 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(34 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(30 as i32 & 0xf as i32) as usize]
                        ^ w[(34 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(30 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[31 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[31 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(35 as i32 & 0xf as i32) as usize] = w[(35 as i32 & 0xf as i32) as usize]
        ^ w[(35 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(35 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(35 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(35 as i32 & 0xf as i32) as usize]
            ^ w[(35 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(35 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(35 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(35 as i32 & 0xf as i32) as usize]
                ^ w[(35 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(35 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(35 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(35 as i32 & 0xf as i32) as usize]
            ^ w[(35 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(35 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(35 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(35 as i32 & 0xf as i32) as usize]
                ^ w[(35 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(35 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(35 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(35 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(35 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(35 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(31 as i32 & 0xf as i32) as usize]
                        ^ w[(35 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(31 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[32 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[32 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(36 as i32 & 0xf as i32) as usize] = w[(36 as i32 & 0xf as i32) as usize]
        ^ w[(36 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(36 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(36 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(36 as i32 & 0xf as i32) as usize]
            ^ w[(36 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(36 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(36 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(36 as i32 & 0xf as i32) as usize]
                ^ w[(36 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(36 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(36 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(36 as i32 & 0xf as i32) as usize]
            ^ w[(36 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(36 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(36 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(36 as i32 & 0xf as i32) as usize]
                ^ w[(36 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(36 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(36 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(36 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(36 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(36 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(32 as i32 & 0xf as i32) as usize]
                        ^ w[(36 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(32 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[33 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[33 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(37 as i32 & 0xf as i32) as usize] = w[(37 as i32 & 0xf as i32) as usize]
        ^ w[(37 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(37 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(37 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(37 as i32 & 0xf as i32) as usize]
            ^ w[(37 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(37 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(37 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(37 as i32 & 0xf as i32) as usize]
                ^ w[(37 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(37 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(37 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(37 as i32 & 0xf as i32) as usize]
            ^ w[(37 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(37 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(37 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(37 as i32 & 0xf as i32) as usize]
                ^ w[(37 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(37 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(37 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(37 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(37 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(37 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(33 as i32 & 0xf as i32) as usize]
                        ^ w[(37 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(33 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[34 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[34 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(38 as i32 & 0xf as i32) as usize] = w[(38 as i32 & 0xf as i32) as usize]
        ^ w[(38 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(38 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(38 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(38 as i32 & 0xf as i32) as usize]
            ^ w[(38 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(38 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(38 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(38 as i32 & 0xf as i32) as usize]
                ^ w[(38 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(38 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(38 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(38 as i32 & 0xf as i32) as usize]
            ^ w[(38 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(38 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(38 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(38 as i32 & 0xf as i32) as usize]
                ^ w[(38 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(38 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(38 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(38 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(38 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(38 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(34 as i32 & 0xf as i32) as usize]
                        ^ w[(38 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(34 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[35 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[35 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(39 as i32 & 0xf as i32) as usize] = w[(39 as i32 & 0xf as i32) as usize]
        ^ w[(39 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(39 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(39 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(39 as i32 & 0xf as i32) as usize]
            ^ w[(39 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(39 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(39 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(39 as i32 & 0xf as i32) as usize]
                ^ w[(39 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(39 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(39 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(39 as i32 & 0xf as i32) as usize]
            ^ w[(39 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(39 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(39 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(39 as i32 & 0xf as i32) as usize]
                ^ w[(39 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(39 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(39 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(39 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(39 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(39 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(35 as i32 & 0xf as i32) as usize]
                        ^ w[(39 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(35 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[36 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[36 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(40 as i32 & 0xf as i32) as usize] = w[(40 as i32 & 0xf as i32) as usize]
        ^ w[(40 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(40 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(40 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(40 as i32 & 0xf as i32) as usize]
            ^ w[(40 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(40 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(40 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(40 as i32 & 0xf as i32) as usize]
                ^ w[(40 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(40 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(40 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(40 as i32 & 0xf as i32) as usize]
            ^ w[(40 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(40 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(40 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(40 as i32 & 0xf as i32) as usize]
                ^ w[(40 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(40 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(40 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(40 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(40 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(40 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(36 as i32 & 0xf as i32) as usize]
                        ^ w[(40 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(36 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[37 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[37 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(41 as i32 & 0xf as i32) as usize] = w[(41 as i32 & 0xf as i32) as usize]
        ^ w[(41 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(41 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(41 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(41 as i32 & 0xf as i32) as usize]
            ^ w[(41 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(41 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(41 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(41 as i32 & 0xf as i32) as usize]
                ^ w[(41 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(41 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(41 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(41 as i32 & 0xf as i32) as usize]
            ^ w[(41 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(41 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(41 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(41 as i32 & 0xf as i32) as usize]
                ^ w[(41 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(41 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(41 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(41 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(41 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(41 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(37 as i32 & 0xf as i32) as usize]
                        ^ w[(41 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(37 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[38 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[38 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(42 as i32 & 0xf as i32) as usize] = w[(42 as i32 & 0xf as i32) as usize]
        ^ w[(42 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(42 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(42 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(42 as i32 & 0xf as i32) as usize]
            ^ w[(42 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(42 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(42 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(42 as i32 & 0xf as i32) as usize]
                ^ w[(42 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(42 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(42 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(42 as i32 & 0xf as i32) as usize]
            ^ w[(42 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(42 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(42 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(42 as i32 & 0xf as i32) as usize]
                ^ w[(42 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(42 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(42 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(42 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(42 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(42 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(38 as i32 & 0xf as i32) as usize]
                        ^ w[(42 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(38 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[39 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[39 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(43 as i32 & 0xf as i32) as usize] = w[(43 as i32 & 0xf as i32) as usize]
        ^ w[(43 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(43 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(43 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(43 as i32 & 0xf as i32) as usize]
            ^ w[(43 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(43 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(43 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(43 as i32 & 0xf as i32) as usize]
                ^ w[(43 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(43 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(43 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(43 as i32 & 0xf as i32) as usize]
            ^ w[(43 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(43 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(43 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(43 as i32 & 0xf as i32) as usize]
                ^ w[(43 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(43 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(43 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(43 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(43 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(43 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(39 as i32 & 0xf as i32) as usize]
                        ^ w[(43 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(39 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[40 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[40 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(44 as i32 & 0xf as i32) as usize] = w[(44 as i32 & 0xf as i32) as usize]
        ^ w[(44 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(44 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(44 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(44 as i32 & 0xf as i32) as usize]
            ^ w[(44 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(44 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(44 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(44 as i32 & 0xf as i32) as usize]
                ^ w[(44 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(44 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(44 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(44 as i32 & 0xf as i32) as usize]
            ^ w[(44 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(44 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(44 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(44 as i32 & 0xf as i32) as usize]
                ^ w[(44 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(44 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(44 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(44 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(44 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(44 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(40 as i32 & 0xf as i32) as usize]
                        ^ w[(44 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(40 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[41 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[41 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(45 as i32 & 0xf as i32) as usize] = w[(45 as i32 & 0xf as i32) as usize]
        ^ w[(45 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(45 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(45 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(45 as i32 & 0xf as i32) as usize]
            ^ w[(45 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(45 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(45 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(45 as i32 & 0xf as i32) as usize]
                ^ w[(45 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(45 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(45 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(45 as i32 & 0xf as i32) as usize]
            ^ w[(45 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(45 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(45 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(45 as i32 & 0xf as i32) as usize]
                ^ w[(45 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(45 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(45 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(45 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(45 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(45 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(41 as i32 & 0xf as i32) as usize]
                        ^ w[(45 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(41 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[42 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[42 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(46 as i32 & 0xf as i32) as usize] = w[(46 as i32 & 0xf as i32) as usize]
        ^ w[(46 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(46 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(46 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(46 as i32 & 0xf as i32) as usize]
            ^ w[(46 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(46 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(46 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(46 as i32 & 0xf as i32) as usize]
                ^ w[(46 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(46 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(46 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(46 as i32 & 0xf as i32) as usize]
            ^ w[(46 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(46 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(46 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(46 as i32 & 0xf as i32) as usize]
                ^ w[(46 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(46 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(46 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(46 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(46 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(46 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(42 as i32 & 0xf as i32) as usize]
                        ^ w[(46 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(42 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[43 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[43 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(47 as i32 & 0xf as i32) as usize] = w[(47 as i32 & 0xf as i32) as usize]
        ^ w[(47 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(47 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(47 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(47 as i32 & 0xf as i32) as usize]
            ^ w[(47 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(47 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(47 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(47 as i32 & 0xf as i32) as usize]
                ^ w[(47 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(47 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(47 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(47 as i32 & 0xf as i32) as usize]
            ^ w[(47 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(47 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(47 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(47 as i32 & 0xf as i32) as usize]
                ^ w[(47 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(47 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(47 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(47 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(47 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(47 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(43 as i32 & 0xf as i32) as usize]
                        ^ w[(47 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(43 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[44 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[44 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(48 as i32 & 0xf as i32) as usize] = w[(48 as i32 & 0xf as i32) as usize]
        ^ w[(48 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(48 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(48 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(48 as i32 & 0xf as i32) as usize]
            ^ w[(48 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(48 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(48 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(48 as i32 & 0xf as i32) as usize]
                ^ w[(48 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(48 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(48 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(48 as i32 & 0xf as i32) as usize]
            ^ w[(48 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(48 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(48 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(48 as i32 & 0xf as i32) as usize]
                ^ w[(48 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(48 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(48 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(48 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(48 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(48 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(44 as i32 & 0xf as i32) as usize]
                        ^ w[(48 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(44 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[45 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[45 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(49 as i32 & 0xf as i32) as usize] = w[(49 as i32 & 0xf as i32) as usize]
        ^ w[(49 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(49 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(49 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(49 as i32 & 0xf as i32) as usize]
            ^ w[(49 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(49 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(49 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(49 as i32 & 0xf as i32) as usize]
                ^ w[(49 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(49 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(49 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(49 as i32 & 0xf as i32) as usize]
            ^ w[(49 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(49 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(49 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(49 as i32 & 0xf as i32) as usize]
                ^ w[(49 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(49 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(49 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(49 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(49 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(49 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(45 as i32 & 0xf as i32) as usize]
                        ^ w[(49 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(45 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[46 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[46 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(50 as i32 & 0xf as i32) as usize] = w[(50 as i32 & 0xf as i32) as usize]
        ^ w[(50 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(50 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(50 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(50 as i32 & 0xf as i32) as usize]
            ^ w[(50 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(50 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(50 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(50 as i32 & 0xf as i32) as usize]
                ^ w[(50 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(50 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(50 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(50 as i32 & 0xf as i32) as usize]
            ^ w[(50 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(50 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(50 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(50 as i32 & 0xf as i32) as usize]
                ^ w[(50 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(50 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(50 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(50 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(50 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(50 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(46 as i32 & 0xf as i32) as usize]
                        ^ w[(50 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(46 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[47 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[47 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(51 as i32 & 0xf as i32) as usize] = w[(51 as i32 & 0xf as i32) as usize]
        ^ w[(51 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(51 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(51 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(51 as i32 & 0xf as i32) as usize]
            ^ w[(51 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(51 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(51 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(51 as i32 & 0xf as i32) as usize]
                ^ w[(51 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(51 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(51 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(51 as i32 & 0xf as i32) as usize]
            ^ w[(51 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(51 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(51 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(51 as i32 & 0xf as i32) as usize]
                ^ w[(51 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(51 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(51 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(51 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(51 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(51 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(47 as i32 & 0xf as i32) as usize]
                        ^ w[(51 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(47 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[48 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[48 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(52 as i32 & 0xf as i32) as usize] = w[(52 as i32 & 0xf as i32) as usize]
        ^ w[(52 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(52 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(52 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(52 as i32 & 0xf as i32) as usize]
            ^ w[(52 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(52 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(52 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(52 as i32 & 0xf as i32) as usize]
                ^ w[(52 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(52 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(52 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(52 as i32 & 0xf as i32) as usize]
            ^ w[(52 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(52 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(52 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(52 as i32 & 0xf as i32) as usize]
                ^ w[(52 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(52 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(52 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(52 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(52 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(52 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(48 as i32 & 0xf as i32) as usize]
                        ^ w[(52 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(48 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[49 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[49 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(53 as i32 & 0xf as i32) as usize] = w[(53 as i32 & 0xf as i32) as usize]
        ^ w[(53 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(53 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(53 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(53 as i32 & 0xf as i32) as usize]
            ^ w[(53 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(53 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(53 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(53 as i32 & 0xf as i32) as usize]
                ^ w[(53 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(53 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(53 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(53 as i32 & 0xf as i32) as usize]
            ^ w[(53 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(53 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(53 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(53 as i32 & 0xf as i32) as usize]
                ^ w[(53 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(53 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(53 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(53 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(53 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(53 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(49 as i32 & 0xf as i32) as usize]
                        ^ w[(53 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(49 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[50 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[50 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(54 as i32 & 0xf as i32) as usize] = w[(54 as i32 & 0xf as i32) as usize]
        ^ w[(54 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(54 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(54 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(54 as i32 & 0xf as i32) as usize]
            ^ w[(54 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(54 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(54 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(54 as i32 & 0xf as i32) as usize]
                ^ w[(54 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(54 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(54 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(54 as i32 & 0xf as i32) as usize]
            ^ w[(54 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(54 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(54 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(54 as i32 & 0xf as i32) as usize]
                ^ w[(54 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(54 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(54 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(54 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(54 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(54 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(50 as i32 & 0xf as i32) as usize]
                        ^ w[(54 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(50 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[51 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[51 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(55 as i32 & 0xf as i32) as usize] = w[(55 as i32 & 0xf as i32) as usize]
        ^ w[(55 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(55 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(55 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(55 as i32 & 0xf as i32) as usize]
            ^ w[(55 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(55 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(55 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(55 as i32 & 0xf as i32) as usize]
                ^ w[(55 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(55 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(55 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(55 as i32 & 0xf as i32) as usize]
            ^ w[(55 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(55 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(55 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(55 as i32 & 0xf as i32) as usize]
                ^ w[(55 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(55 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(55 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(55 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(55 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(55 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(51 as i32 & 0xf as i32) as usize]
                        ^ w[(55 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(51 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[52 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[52 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(56 as i32 & 0xf as i32) as usize] = w[(56 as i32 & 0xf as i32) as usize]
        ^ w[(56 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(56 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(56 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(56 as i32 & 0xf as i32) as usize]
            ^ w[(56 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(56 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(56 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(56 as i32 & 0xf as i32) as usize]
                ^ w[(56 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(56 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(56 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(56 as i32 & 0xf as i32) as usize]
            ^ w[(56 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(56 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(56 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(56 as i32 & 0xf as i32) as usize]
                ^ w[(56 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(56 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(56 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(56 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(56 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(56 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(52 as i32 & 0xf as i32) as usize]
                        ^ w[(56 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(52 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[53 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[53 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(57 as i32 & 0xf as i32) as usize] = w[(57 as i32 & 0xf as i32) as usize]
        ^ w[(57 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(57 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(57 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(57 as i32 & 0xf as i32) as usize]
            ^ w[(57 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(57 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(57 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(57 as i32 & 0xf as i32) as usize]
                ^ w[(57 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(57 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(57 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(57 as i32 & 0xf as i32) as usize]
            ^ w[(57 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(57 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(57 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(57 as i32 & 0xf as i32) as usize]
                ^ w[(57 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(57 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(57 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(57 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(57 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(57 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(53 as i32 & 0xf as i32) as usize]
                        ^ w[(57 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(53 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[54 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[54 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(58 as i32 & 0xf as i32) as usize] = w[(58 as i32 & 0xf as i32) as usize]
        ^ w[(58 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(58 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(58 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(58 as i32 & 0xf as i32) as usize]
            ^ w[(58 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(58 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(58 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(58 as i32 & 0xf as i32) as usize]
                ^ w[(58 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(58 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(58 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(58 as i32 & 0xf as i32) as usize]
            ^ w[(58 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(58 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(58 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(58 as i32 & 0xf as i32) as usize]
                ^ w[(58 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(58 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(58 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(58 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(58 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(58 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(54 as i32 & 0xf as i32) as usize]
                        ^ w[(58 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(54 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[55 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[55 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(59 as i32 & 0xf as i32) as usize] = w[(59 as i32 & 0xf as i32) as usize]
        ^ w[(59 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(59 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(59 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(59 as i32 & 0xf as i32) as usize]
            ^ w[(59 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(59 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(59 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(59 as i32 & 0xf as i32) as usize]
                ^ w[(59 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(59 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(59 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(59 as i32 & 0xf as i32) as usize]
            ^ w[(59 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(59 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(59 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(59 as i32 & 0xf as i32) as usize]
                ^ w[(59 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(59 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(59 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(59 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(59 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(59 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(55 as i32 & 0xf as i32) as usize]
                        ^ w[(59 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(55 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[56 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[56 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(60 as i32 & 0xf as i32) as usize] = w[(60 as i32 & 0xf as i32) as usize]
        ^ w[(60 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(60 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(60 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(60 as i32 & 0xf as i32) as usize]
            ^ w[(60 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(60 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(60 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(60 as i32 & 0xf as i32) as usize]
                ^ w[(60 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(60 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(60 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(60 as i32 & 0xf as i32) as usize]
            ^ w[(60 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(60 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(60 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(60 as i32 & 0xf as i32) as usize]
                ^ w[(60 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(60 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(60 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(60 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(60 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(60 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(56 as i32 & 0xf as i32) as usize]
                        ^ w[(60 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(56 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[57 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[57 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(61 as i32 & 0xf as i32) as usize] = w[(61 as i32 & 0xf as i32) as usize]
        ^ w[(61 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(61 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(61 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(61 as i32 & 0xf as i32) as usize]
            ^ w[(61 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(61 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(61 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(61 as i32 & 0xf as i32) as usize]
                ^ w[(61 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(61 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(61 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(61 as i32 & 0xf as i32) as usize]
            ^ w[(61 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(61 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(61 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(61 as i32 & 0xf as i32) as usize]
                ^ w[(61 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(61 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(61 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(61 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(61 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(61 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(57 as i32 & 0xf as i32) as usize]
                        ^ w[(61 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(57 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[58 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[58 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(62 as i32 & 0xf as i32) as usize] = w[(62 as i32 & 0xf as i32) as usize]
        ^ w[(62 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(62 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(62 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(62 as i32 & 0xf as i32) as usize]
            ^ w[(62 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(62 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(62 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(62 as i32 & 0xf as i32) as usize]
                ^ w[(62 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(62 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(62 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(62 as i32 & 0xf as i32) as usize]
            ^ w[(62 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(62 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(62 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(62 as i32 & 0xf as i32) as usize]
                ^ w[(62 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(62 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(62 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(62 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(62 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(62 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(58 as i32 & 0xf as i32) as usize]
                        ^ w[(62 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(58 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[59 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[59 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(63 as i32 & 0xf as i32) as usize] = w[(63 as i32 & 0xf as i32) as usize]
        ^ w[(63 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(63 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(63 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(63 as i32 & 0xf as i32) as usize]
            ^ w[(63 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(63 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(63 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(63 as i32 & 0xf as i32) as usize]
                ^ w[(63 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(63 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(63 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(63 as i32 & 0xf as i32) as usize]
            ^ w[(63 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(63 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(63 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(63 as i32 & 0xf as i32) as usize]
                ^ w[(63 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(63 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(63 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(63 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(63 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(63 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(59 as i32 & 0xf as i32) as usize]
                        ^ w[(63 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(59 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    ss1 = (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
        .wrapping_add(e)
        .wrapping_add(K[60 as i32 as usize]) << 7 as i32
        | (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32))
            .wrapping_add(e)
            .wrapping_add(K[60 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (a << 12 as i32 | a >> (-(12 as i32) & 31 as i32));
    w[(64 as i32 & 0xf as i32) as usize] = w[(64 as i32 & 0xf as i32) as usize]
        ^ w[(64 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(64 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(64 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(64 as i32 & 0xf as i32) as usize]
            ^ w[(64 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(64 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(64 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(64 as i32 & 0xf as i32) as usize]
                ^ w[(64 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(64 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(64 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(64 as i32 & 0xf as i32) as usize]
            ^ w[(64 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(64 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(64 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(64 as i32 & 0xf as i32) as usize]
                ^ w[(64 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(64 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(64 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(64 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(64 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(64 as i32 - 6 as i32 & 0xf as i32) as usize];
    d = (d as u32)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(60 as i32 & 0xf as i32) as usize]
                        ^ w[(64 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    h = (h as u32)
        .wrapping_add(
            (e & f | !e & g)
                .wrapping_add(ss1)
                .wrapping_add(w[(60 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    b = b << 9 as i32 | b >> (-(9 as i32) & 31 as i32);
    f = f << 19 as i32 | f >> (-(19 as i32) & 31 as i32);
    h = h ^ (h << 9 as i32 | h >> (-(9 as i32) & 31 as i32))
        ^ (h << 17 as i32 | h >> (-(17 as i32) & 31 as i32));
    ss1 = (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
        .wrapping_add(h)
        .wrapping_add(K[61 as i32 as usize]) << 7 as i32
        | (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32))
            .wrapping_add(h)
            .wrapping_add(K[61 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (d << 12 as i32 | d >> (-(12 as i32) & 31 as i32));
    w[(65 as i32 & 0xf as i32) as usize] = w[(65 as i32 & 0xf as i32) as usize]
        ^ w[(65 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(65 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(65 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(65 as i32 & 0xf as i32) as usize]
            ^ w[(65 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(65 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(65 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(65 as i32 & 0xf as i32) as usize]
                ^ w[(65 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(65 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(65 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(65 as i32 & 0xf as i32) as usize]
            ^ w[(65 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(65 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(65 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(65 as i32 & 0xf as i32) as usize]
                ^ w[(65 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(65 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(65 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(65 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(65 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(65 as i32 - 6 as i32 & 0xf as i32) as usize];
    c = (c as u32)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(61 as i32 & 0xf as i32) as usize]
                        ^ w[(65 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    g = (g as u32)
        .wrapping_add(
            (h & e | !h & f)
                .wrapping_add(ss1)
                .wrapping_add(w[(61 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    a = a << 9 as i32 | a >> (-(9 as i32) & 31 as i32);
    e = e << 19 as i32 | e >> (-(19 as i32) & 31 as i32);
    g = g ^ (g << 9 as i32 | g >> (-(9 as i32) & 31 as i32))
        ^ (g << 17 as i32 | g >> (-(17 as i32) & 31 as i32));
    ss1 = (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
        .wrapping_add(g)
        .wrapping_add(K[62 as i32 as usize]) << 7 as i32
        | (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32))
            .wrapping_add(g)
            .wrapping_add(K[62 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (c << 12 as i32 | c >> (-(12 as i32) & 31 as i32));
    w[(66 as i32 & 0xf as i32) as usize] = w[(66 as i32 & 0xf as i32) as usize]
        ^ w[(66 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(66 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(66 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(66 as i32 & 0xf as i32) as usize]
            ^ w[(66 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(66 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(66 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(66 as i32 & 0xf as i32) as usize]
                ^ w[(66 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(66 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(66 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(66 as i32 & 0xf as i32) as usize]
            ^ w[(66 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(66 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(66 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(66 as i32 & 0xf as i32) as usize]
                ^ w[(66 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(66 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(66 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(66 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(66 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(66 as i32 - 6 as i32 & 0xf as i32) as usize];
    b = (b as u32)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(62 as i32 & 0xf as i32) as usize]
                        ^ w[(66 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    f = (f as u32)
        .wrapping_add(
            (g & h | !g & e)
                .wrapping_add(ss1)
                .wrapping_add(w[(62 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    d = d << 9 as i32 | d >> (-(9 as i32) & 31 as i32);
    h = h << 19 as i32 | h >> (-(19 as i32) & 31 as i32);
    f = f ^ (f << 9 as i32 | f >> (-(9 as i32) & 31 as i32))
        ^ (f << 17 as i32 | f >> (-(17 as i32) & 31 as i32));
    ss1 = (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
        .wrapping_add(f)
        .wrapping_add(K[63 as i32 as usize]) << 7 as i32
        | (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32))
            .wrapping_add(f)
            .wrapping_add(K[63 as i32 as usize]) >> (-(7 as i32) & 31 as i32);
    ss2 = ss1 ^ (b << 12 as i32 | b >> (-(12 as i32) & 31 as i32));
    w[(67 as i32 & 0xf as i32) as usize] = w[(67 as i32 & 0xf as i32) as usize]
        ^ w[(67 as i32 - 9 as i32 & 0xf as i32) as usize]
        ^ (w[(67 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
            | w[(67 as i32 - 3 as i32 & 0xf as i32) as usize]
                >> (-(15 as i32) & 31 as i32))
        ^ ((w[(67 as i32 & 0xf as i32) as usize]
            ^ w[(67 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(67 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(67 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 15 as i32
            | (w[(67 as i32 & 0xf as i32) as usize]
                ^ w[(67 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(67 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(67 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(15 as i32) & 31 as i32))
        ^ ((w[(67 as i32 & 0xf as i32) as usize]
            ^ w[(67 as i32 - 9 as i32 & 0xf as i32) as usize]
            ^ (w[(67 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                | w[(67 as i32 - 3 as i32 & 0xf as i32) as usize]
                    >> (-(15 as i32) & 31 as i32))) << 23 as i32
            | (w[(67 as i32 & 0xf as i32) as usize]
                ^ w[(67 as i32 - 9 as i32 & 0xf as i32) as usize]
                ^ (w[(67 as i32 - 3 as i32 & 0xf as i32) as usize] << 15 as i32
                    | w[(67 as i32 - 3 as i32 & 0xf as i32) as usize]
                        >> (-(15 as i32) & 31 as i32))) >> (-(23 as i32) & 31 as i32))
        ^ (w[(67 as i32 - 13 as i32 & 0xf as i32) as usize] << 7 as i32
            | w[(67 as i32 - 13 as i32 & 0xf as i32) as usize]
                >> (-(7 as i32) & 31 as i32))
        ^ w[(67 as i32 - 6 as i32 & 0xf as i32) as usize];
    a = (a as u32)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(ss2)
                .wrapping_add(
                    w[(63 as i32 & 0xf as i32) as usize]
                        ^ w[(67 as i32 & 0xf as i32) as usize],
                ),
        ) as uint32_t as uint32_t;
    e = (e as u32)
        .wrapping_add(
            (f & g | !f & h)
                .wrapping_add(ss1)
                .wrapping_add(w[(63 as i32 & 0xf as i32) as usize]),
        ) as uint32_t as uint32_t;
    c = c << 9 as i32 | c >> (-(9 as i32) & 31 as i32);
    g = g << 19 as i32 | g >> (-(19 as i32) & 31 as i32);
    e = e ^ (e << 9 as i32 | e >> (-(9 as i32) & 31 as i32))
        ^ (e << 17 as i32 | e >> (-(17 as i32) & 31 as i32));
    let ref mut fresh0 = *state.offset(0 as i32 as isize);
    *fresh0 ^= a;
    let ref mut fresh1 = *state.offset(1 as i32 as isize);
    *fresh1 ^= b;
    let ref mut fresh2 = *state.offset(2 as i32 as isize);
    *fresh2 ^= c;
    let ref mut fresh3 = *state.offset(3 as i32 as isize);
    *fresh3 ^= d;
    let ref mut fresh4 = *state.offset(4 as i32 as isize);
    *fresh4 ^= e;
    let ref mut fresh5 = *state.offset(5 as i32 as isize);
    *fresh5 ^= f;
    let ref mut fresh6 = *state.offset(6 as i32 as isize);
    *fresh6 ^= g;
    let ref mut fresh7 = *state.offset(7 as i32 as isize);
    *fresh7 ^= h;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sm3_init(mut ctx: *mut sm3_ctx) {
    static mut H0: [uint32_t; 8] = [
        0x7380166f as u64 as uint32_t,
        0x4914b2b9 as u64 as uint32_t,
        0x172442d7 as u64 as uint32_t,
        0xda8a0600 as u64 as uint32_t,
        0xa96f30bc as u64 as uint32_t,
        0x163138aa as u64 as uint32_t,
        0xe38dee4d as u64 as uint32_t,
        0xb0fb0e4e as u64 as uint32_t,
    ];
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        H0.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 8]>() as u64,
    );
    (*ctx).count = 0 as i32 as uint64_t;
    (*ctx).index = 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sm3_update(
    mut ctx: *mut sm3_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    let mut current_block: u64;
    if !(length == 0) {
        if (*ctx).index != 0 {
            let mut __md_left: u32 = (::core::mem::size_of::<[uint8_t; 64]>() as u64)
                .wrapping_sub((*ctx).index as u64) as u32;
            if length < __md_left as u64 {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx).index = ((*ctx).index as u64).wrapping_add(length) as u32 as u32;
                current_block = 15652330335145281839;
            } else {
                memcpy(
                    ((*ctx).block).as_mut_ptr().offset((*ctx).index as isize)
                        as *mut libc::c_void,
                    data as *const libc::c_void,
                    __md_left as u64,
                );
                sm3_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
                (*ctx).count = ((*ctx).count).wrapping_add(1);
                (*ctx).count;
                data = data.offset(__md_left as isize);
                length = (length as u64).wrapping_sub(__md_left as u64) as size_t
                    as size_t;
                current_block = 11812396948646013369;
            }
        } else {
            current_block = 11812396948646013369;
        }
        match current_block {
            15652330335145281839 => {}
            _ => {
                while length >= ::core::mem::size_of::<[uint8_t; 64]>() as u64 {
                    sm3_compress(((*ctx).state).as_mut_ptr(), data);
                    (*ctx).count = ((*ctx).count).wrapping_add(1);
                    (*ctx).count;
                    data = data
                        .offset(::core::mem::size_of::<[uint8_t; 64]>() as u64 as isize);
                    length = (length as u64)
                        .wrapping_sub(::core::mem::size_of::<[uint8_t; 64]>() as u64)
                        as size_t as size_t;
                }
                memcpy(
                    ((*ctx).block).as_mut_ptr() as *mut libc::c_void,
                    data as *const libc::c_void,
                    length,
                );
                (*ctx).index = length as u32;
            }
        }
    }
}
unsafe extern "C" fn sm3_write_digest(
    mut ctx: *mut sm3_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    let mut bit_count: uint64_t = 0;
    if length <= 32 as i32 as u64 {} else {
        __assert_fail(
            b"length <= SM3_DIGEST_SIZE\0" as *const u8 as *const i8,
            b"sm3.c\0" as *const u8 as *const i8,
            228 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[i8; 59],
            >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_41715: {
        if length <= 32 as i32 as u64 {} else {
            __assert_fail(
                b"length <= SM3_DIGEST_SIZE\0" as *const u8 as *const i8,
                b"sm3.c\0" as *const u8 as *const i8,
                228 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[i8; 59],
                >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __md_i: u32 = 0;
    __md_i = (*ctx).index;
    if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
        __assert_fail(
            b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
            b"sm3.c\0" as *const u8 as *const i8,
            230 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[i8; 59],
            >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_41655: {
        if (__md_i as u64) < ::core::mem::size_of::<[uint8_t; 64]>() as u64 {} else {
            __assert_fail(
                b"__md_i < sizeof((ctx)->block)\0" as *const u8 as *const i8,
                b"sm3.c\0" as *const u8 as *const i8,
                230 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[i8; 59],
                >(b"void sm3_write_digest(struct sm3_ctx *, size_t, uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh8 = __md_i;
    __md_i = __md_i.wrapping_add(1);
    (*ctx).block[fresh8 as usize] = 0x80 as i32 as uint8_t;
    if __md_i as u64
        > (::core::mem::size_of::<[uint8_t; 64]>() as u64).wrapping_sub(8 as i32 as u64)
    {
        memset(
            ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
            0 as i32,
            (::core::mem::size_of::<[uint8_t; 64]>() as u64).wrapping_sub(__md_i as u64),
        );
        sm3_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
        __md_i = 0 as i32 as u32;
    }
    memset(
        ((*ctx).block).as_mut_ptr().offset(__md_i as isize) as *mut libc::c_void,
        0 as i32,
        (::core::mem::size_of::<[uint8_t; 64]>() as u64)
            .wrapping_sub(8 as i32 as u64)
            .wrapping_sub(__md_i as u64),
    );
    bit_count = (*ctx).count << 9 as i32 | ((*ctx).index << 3 as i32) as u64;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(0 as i32 as isize) = (bit_count >> 56 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(1 as i32 as isize) = (bit_count >> 48 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(2 as i32 as isize) = (bit_count >> 40 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(3 as i32 as isize) = (bit_count >> 32 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(4 as i32 as isize) = (bit_count >> 24 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(5 as i32 as isize) = (bit_count >> 16 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(6 as i32 as isize) = (bit_count >> 8 as i32 & 0xff as i32 as u64)
        as uint8_t;
    *((*ctx).block)
        .as_mut_ptr()
        .offset((64 as i32 - 8 as i32) as isize)
        .offset(7 as i32 as isize) = (bit_count & 0xff as i32 as u64) as uint8_t;
    sm3_compress(((*ctx).state).as_mut_ptr(), ((*ctx).block).as_mut_ptr());
    _nettle_write_be32(length, digest, ((*ctx).state).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sm3_digest(
    mut ctx: *mut sm3_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    sm3_write_digest(ctx, length, digest);
    nettle_sm3_init(ctx);
}