#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = u64;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl64(mut x: uint64_t, mut n: i32) -> uint64_t {
    return (x << n | x >> 64 as i32 - n) & 18446744073709551615 as u64;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr8(mut x: uint8_t, mut n: i32) -> uint8_t {
    return ((x as u32 >> n | (x as u32) << 8 as i32 - n) & 255 as i32 as u32) as uint8_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr64(mut x: uint64_t, mut n: i32) -> uint64_t {
    return (x >> n | x << 64 as i32 - n) & 18446744073709551615 as u64;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl32(mut x: uint32_t, mut n: i32) -> uint32_t {
    return (x << n | x >> 32 as i32 - n) & 4294967295 as u32;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl8(mut x: uint8_t, mut n: i32) -> uint8_t {
    return (((x as u32) << n | x as u32 >> 8 as i32 - n) & 255 as i32 as u32) as uint8_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr32(mut x: uint32_t, mut n: i32) -> uint32_t {
    return (x >> n | x << 32 as i32 - n) & 4294967295 as u32;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl_sz(mut x: size_t, mut n: i32) -> size_t {
    return (x << n
        | x
            >> (8 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<size_t>() as u64)
                .wrapping_sub(n as u64)) & 18446744073709551615 as u64;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr16(mut x: uint16_t, mut n: i32) -> uint16_t {
    return ((x as u32 >> n | (x as u32) << 16 as i32 - n) & 65535 as i32 as u32)
        as uint16_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotl16(mut x: uint16_t, mut n: i32) -> uint16_t {
    return (((x as u32) << n | x as u32 >> 16 as i32 - n) & 65535 as i32 as u32)
        as uint16_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn rotr_sz(mut x: size_t, mut n: i32) -> size_t {
    return (x >> n
        | x
            << (8 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<size_t>() as u64)
                .wrapping_sub(n as u64)) & 18446744073709551615 as u64;
}