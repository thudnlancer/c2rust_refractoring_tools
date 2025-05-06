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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn _nettle_ripemd160_compress(
    mut state: *mut uint32_t,
    mut data: *const uint8_t,
) {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut aa: uint32_t = 0;
    let mut bb: uint32_t = 0;
    let mut cc: uint32_t = 0;
    let mut dd: uint32_t = 0;
    let mut ee: uint32_t = 0;
    let mut t: uint32_t = 0;
    let mut x: [uint32_t; 16] = [0; 16];
    memcpy(
        x.as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 16]>() as u64,
    );
    a = *state.offset(0 as i32 as isize);
    b = *state.offset(1 as i32 as isize);
    c = *state.offset(2 as i32 as isize);
    d = *state.offset(3 as i32 as isize);
    e = *state.offset(4 as i32 as isize);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    a = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    e = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    d = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    c = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    b = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    a = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    e = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    c = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    b = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    a = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    e = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    d = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    c = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    b = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    a = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    e = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    d = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    c = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    b = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    a = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    e = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    d = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    c = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    b = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    a = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    e = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    c = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    b = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    a = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x5a827999 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    e = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    d = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    c = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    b = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    a = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    e = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    c = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    b = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    a = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    e = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    d = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    c = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    b = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    a = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    e = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6ed9eba1 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    d = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[1 as i32 as usize]);
    c = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[9 as i32 as usize]);
    b = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[11 as i32 as usize]);
    a = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[10 as i32 as usize]);
    e = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[0 as i32 as usize]);
    d = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[8 as i32 as usize]);
    c = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[12 as i32 as usize]);
    b = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[4 as i32 as usize]);
    a = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[13 as i32 as usize]);
    e = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[3 as i32 as usize]);
    d = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[7 as i32 as usize]);
    c = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[15 as i32 as usize]);
    b = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[14 as i32 as usize]);
    a = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[5 as i32 as usize]);
    e = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[6 as i32 as usize]);
    d = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x8f1bbcdc as u32)
        .wrapping_add(x[2 as i32 as usize]);
    c = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[4 as i32 as usize]);
    b = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[0 as i32 as usize]);
    a = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[5 as i32 as usize]);
    e = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[9 as i32 as usize]);
    d = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[7 as i32 as usize]);
    c = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[12 as i32 as usize]);
    b = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[2 as i32 as usize]);
    a = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[10 as i32 as usize]);
    e = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[14 as i32 as usize]);
    d = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[1 as i32 as usize]);
    c = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[3 as i32 as usize]);
    b = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[8 as i32 as usize]);
    a = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[11 as i32 as usize]);
    e = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[6 as i32 as usize]);
    d = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[15 as i32 as usize]);
    c = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0xa953fd4e as u32)
        .wrapping_add(x[13 as i32 as usize]);
    b = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    aa = a;
    bb = b;
    cc = c;
    dd = d;
    ee = e;
    a = *state.offset(0 as i32 as isize);
    b = *state.offset(1 as i32 as isize);
    c = *state.offset(2 as i32 as isize);
    d = *state.offset(3 as i32 as isize);
    e = *state.offset(4 as i32 as isize);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    a = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    e = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    c = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    b = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    a = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    e = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    d = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    c = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    b = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    a = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ (b | !c))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    e = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ (a | !b))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    d = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ (e | !a))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    c = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ (d | !e))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    b = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ (c | !d))
        .wrapping_add(0x50a28be6 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    a = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    e = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    d = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    c = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    b = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    a = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    e = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    c = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    b = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    a = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    e = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & b | a & !b)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    d = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & a | e & !a)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    c = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & e | d & !e)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    b = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    a = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & c | b & !c)
        .wrapping_add(0x5c4dd124 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    e = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    c = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    b = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    a = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    e = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    d = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    c = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    b = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    a = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    e = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    d = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add((d | !e) ^ a)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    c = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add((c | !d) ^ e)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    b = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add((b | !c) ^ d)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    a = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add((a | !b) ^ c)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    e = (t << 7 as i32 | t >> (-(7 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add((e | !a) ^ b)
        .wrapping_add(0x6d703ef3 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    d = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    c = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    b = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    a = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    e = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    d = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    c = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    b = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    a = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    e = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    c = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c & d | !c & e)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    b = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    a = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a & b | !a & c)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    e = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e & a | !e & b)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    d = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d & e | !d & a)
        .wrapping_add(0x7a6d76e9 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    c = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[12 as i32 as usize]);
    b = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[15 as i32 as usize]);
    a = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[10 as i32 as usize]);
    e = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[4 as i32 as usize]);
    d = (t << 9 as i32 | t >> (-(9 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[1 as i32 as usize]);
    c = (t << 12 as i32 | t >> (-(12 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[5 as i32 as usize]);
    b = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[8 as i32 as usize]);
    a = (t << 14 as i32 | t >> (-(14 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[7 as i32 as usize]);
    e = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[6 as i32 as usize]);
    d = (t << 8 as i32 | t >> (-(8 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[2 as i32 as usize]);
    c = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[13 as i32 as usize]);
    b = (t << 6 as i32 | t >> (-(6 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[14 as i32 as usize]);
    a = (t << 5 as i32 | t >> (-(5 as i32) & 31 as i32)).wrapping_add(e);
    c = c << 10 as i32 | c >> (-(10 as i32) & 31 as i32);
    t = e
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[0 as i32 as usize]);
    e = (t << 15 as i32 | t >> (-(15 as i32) & 31 as i32)).wrapping_add(d);
    b = b << 10 as i32 | b >> (-(10 as i32) & 31 as i32);
    t = d
        .wrapping_add(e ^ a ^ b)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[3 as i32 as usize]);
    d = (t << 13 as i32 | t >> (-(13 as i32) & 31 as i32)).wrapping_add(c);
    a = a << 10 as i32 | a >> (-(10 as i32) & 31 as i32);
    t = c
        .wrapping_add(d ^ e ^ a)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[9 as i32 as usize]);
    c = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(b);
    e = e << 10 as i32 | e >> (-(10 as i32) & 31 as i32);
    t = b
        .wrapping_add(c ^ d ^ e)
        .wrapping_add(0 as i32 as u32)
        .wrapping_add(x[11 as i32 as usize]);
    b = (t << 11 as i32 | t >> (-(11 as i32) & 31 as i32)).wrapping_add(a);
    d = d << 10 as i32 | d >> (-(10 as i32) & 31 as i32);
    t = (*state.offset(1 as i32 as isize)).wrapping_add(d).wrapping_add(cc);
    *state.offset(1 as i32 as isize) = (*state.offset(2 as i32 as isize))
        .wrapping_add(e)
        .wrapping_add(dd);
    *state.offset(2 as i32 as isize) = (*state.offset(3 as i32 as isize))
        .wrapping_add(a)
        .wrapping_add(ee);
    *state.offset(3 as i32 as isize) = (*state.offset(4 as i32 as isize))
        .wrapping_add(b)
        .wrapping_add(aa);
    *state.offset(4 as i32 as isize) = (*state.offset(0 as i32 as isize))
        .wrapping_add(c)
        .wrapping_add(bb);
    *state.offset(0 as i32 as isize) = t;
}