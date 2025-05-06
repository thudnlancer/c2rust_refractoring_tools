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
    fn _nettle_camellia_absorb(nkeys: u32, dst: *mut uint64_t, subkey: *mut uint64_t);
    static _nettle_camellia_table: camellia_table;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia_table {
    pub sp1110: [uint32_t; 256],
    pub sp0222: [uint32_t; 256],
    pub sp3033: [uint32_t; 256],
    pub sp4404: [uint32_t; 256],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_camellia128_set_encrypt_key(
    mut ctx: *mut camellia128_ctx,
    mut key: *const uint8_t,
) {
    let mut k0: uint64_t = 0;
    let mut k1: uint64_t = 0;
    let mut subkey: [uint64_t; 26] = [0; 26];
    let mut w: uint64_t = 0;
    k0 = (*key.offset(0 as i32 as isize) as uint64_t) << 56 as i32
        | (*key.offset(1 as i32 as isize) as uint64_t) << 48 as i32
        | (*key.offset(2 as i32 as isize) as uint64_t) << 40 as i32
        | (*key.offset(3 as i32 as isize) as uint64_t) << 32 as i32
        | (*key.offset(4 as i32 as isize) as uint64_t) << 24 as i32
        | (*key.offset(5 as i32 as isize) as uint64_t) << 16 as i32
        | (*key.offset(6 as i32 as isize) as uint64_t) << 8 as i32
        | *key.offset(7 as i32 as isize) as uint64_t;
    k1 = (*key.offset(8 as i32 as isize).offset(0 as i32 as isize) as uint64_t)
        << 56 as i32
        | (*key.offset(8 as i32 as isize).offset(1 as i32 as isize) as uint64_t)
            << 48 as i32
        | (*key.offset(8 as i32 as isize).offset(2 as i32 as isize) as uint64_t)
            << 40 as i32
        | (*key.offset(8 as i32 as isize).offset(3 as i32 as isize) as uint64_t)
            << 32 as i32
        | (*key.offset(8 as i32 as isize).offset(4 as i32 as isize) as uint64_t)
            << 24 as i32
        | (*key.offset(8 as i32 as isize).offset(5 as i32 as isize) as uint64_t)
            << 16 as i32
        | (*key.offset(8 as i32 as isize).offset(6 as i32 as isize) as uint64_t)
            << 8 as i32
        | *key.offset(8 as i32 as isize).offset(7 as i32 as isize) as uint64_t;
    subkey[0 as i32 as usize] = k0;
    subkey[1 as i32 as usize] = k1;
    let mut __rol128_t: uint64_t = k0;
    k0 = k0 << 15 as i32 | k1 >> 64 as i32 - 15 as i32;
    k1 = k1 << 15 as i32 | __rol128_t >> 64 as i32 - 15 as i32;
    subkey[4 as i32 as usize] = k0;
    subkey[5 as i32 as usize] = k1;
    let mut __rol128_t_0: uint64_t = k0;
    k0 = k0 << 30 as i32 | k1 >> 64 as i32 - 30 as i32;
    k1 = k1 << 30 as i32 | __rol128_t_0 >> 64 as i32 - 30 as i32;
    subkey[10 as i32 as usize] = k0;
    subkey[11 as i32 as usize] = k1;
    let mut __rol128_t_1: uint64_t = k0;
    k0 = k0 << 15 as i32 | k1 >> 64 as i32 - 15 as i32;
    k1 = k1 << 15 as i32 | __rol128_t_1 >> 64 as i32 - 15 as i32;
    subkey[13 as i32 as usize] = k1;
    let mut __rol128_t_2: uint64_t = k0;
    k0 = k0 << 17 as i32 | k1 >> 64 as i32 - 17 as i32;
    k1 = k1 << 17 as i32 | __rol128_t_2 >> 64 as i32 - 17 as i32;
    subkey[16 as i32 as usize] = k0;
    subkey[17 as i32 as usize] = k1;
    let mut __rol128_t_3: uint64_t = k0;
    k0 = k0 << 17 as i32 | k1 >> 64 as i32 - 17 as i32;
    k1 = k1 << 17 as i32 | __rol128_t_3 >> 64 as i32 - 17 as i32;
    subkey[18 as i32 as usize] = k0;
    subkey[19 as i32 as usize] = k1;
    let mut __rol128_t_4: uint64_t = k0;
    k0 = k0 << 17 as i32 | k1 >> 64 as i32 - 17 as i32;
    k1 = k1 << 17 as i32 | __rol128_t_4 >> 64 as i32 - 17 as i32;
    subkey[22 as i32 as usize] = k0;
    subkey[23 as i32 as usize] = k1;
    k0 = subkey[0 as i32 as usize];
    w = subkey[1 as i32 as usize];
    let mut __yl: uint32_t = 0;
    let mut __yr: uint32_t = 0;
    let mut __i: uint64_t = (k0 as libc::c_ulonglong
        ^ 0xa09e667f3bcc908b as libc::c_ulonglong) as uint64_t;
    __yl = _nettle_camellia_table.sp1110[(__i & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i >> 24 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i >> 16 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i >> 8 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yr = _nettle_camellia_table.sp1110[(__i >> 56 as i32) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i >> 48 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i >> 40 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i >> 32 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yl ^= __yr;
    __yr = __yr << 24 as i32 | __yr >> (-(24 as i32) & 31 as i32);
    __yr ^= __yl;
    k1 = (__yl as uint64_t) << 32 as i32 | __yr as u64;
    w ^= k1;
    let mut __yl_0: uint32_t = 0;
    let mut __yr_0: uint32_t = 0;
    let mut __i_0: uint64_t = (w as libc::c_ulonglong
        ^ 0xb67ae8584caa73b2 as libc::c_ulonglong) as uint64_t;
    __yl_0 = _nettle_camellia_table.sp1110[(__i_0 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_0 >> 24 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_0 >> 16 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_0 >> 8 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yr_0 = _nettle_camellia_table.sp1110[(__i_0 >> 56 as i32) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_0 >> 48 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_0 >> 40 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_0 >> 32 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yl_0 ^= __yr_0;
    __yr_0 = __yr_0 << 24 as i32 | __yr_0 >> (-(24 as i32) & 31 as i32);
    __yr_0 ^= __yl_0;
    k0 = (__yl_0 as uint64_t) << 32 as i32 | __yr_0 as u64;
    let mut __yl_1: uint32_t = 0;
    let mut __yr_1: uint32_t = 0;
    let mut __i_1: uint64_t = (k0 as libc::c_ulonglong
        ^ 0xc6ef372fe94f82be as libc::c_ulonglong) as uint64_t;
    __yl_1 = _nettle_camellia_table.sp1110[(__i_1 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_1 >> 24 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_1 >> 16 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_1 >> 8 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yr_1 = _nettle_camellia_table.sp1110[(__i_1 >> 56 as i32) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_1 >> 48 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_1 >> 40 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_1 >> 32 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yl_1 ^= __yr_1;
    __yr_1 = __yr_1 << 24 as i32 | __yr_1 >> (-(24 as i32) & 31 as i32);
    __yr_1 ^= __yl_1;
    w = (__yl_1 as uint64_t) << 32 as i32 | __yr_1 as u64;
    k1 ^= w;
    let mut __yl_2: uint32_t = 0;
    let mut __yr_2: uint32_t = 0;
    let mut __i_2: uint64_t = (k1 as libc::c_ulonglong
        ^ 0x54ff53a5f1d36f1c as libc::c_ulonglong) as uint64_t;
    __yl_2 = _nettle_camellia_table.sp1110[(__i_2 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_2 >> 24 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_2 >> 16 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_2 >> 8 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yr_2 = _nettle_camellia_table.sp1110[(__i_2 >> 56 as i32) as i32 as usize]
        ^ _nettle_camellia_table
            .sp0222[(__i_2 >> 48 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp3033[(__i_2 >> 40 as i32 & 0xff as i32 as u64) as i32 as usize]
        ^ _nettle_camellia_table
            .sp4404[(__i_2 >> 32 as i32 & 0xff as i32 as u64) as i32 as usize];
    __yl_2 ^= __yr_2;
    __yr_2 = __yr_2 << 24 as i32 | __yr_2 >> (-(24 as i32) & 31 as i32);
    __yr_2 ^= __yl_2;
    w = (__yl_2 as uint64_t) << 32 as i32 | __yr_2 as u64;
    k0 ^= w;
    subkey[2 as i32 as usize] = k0;
    subkey[3 as i32 as usize] = k1;
    let mut __rol128_t_5: uint64_t = k0;
    k0 = k0 << 15 as i32 | k1 >> 64 as i32 - 15 as i32;
    k1 = k1 << 15 as i32 | __rol128_t_5 >> 64 as i32 - 15 as i32;
    subkey[6 as i32 as usize] = k0;
    subkey[7 as i32 as usize] = k1;
    let mut __rol128_t_6: uint64_t = k0;
    k0 = k0 << 15 as i32 | k1 >> 64 as i32 - 15 as i32;
    k1 = k1 << 15 as i32 | __rol128_t_6 >> 64 as i32 - 15 as i32;
    subkey[8 as i32 as usize] = k0;
    subkey[9 as i32 as usize] = k1;
    let mut __rol128_t_7: uint64_t = k0;
    k0 = k0 << 15 as i32 | k1 >> 64 as i32 - 15 as i32;
    k1 = k1 << 15 as i32 | __rol128_t_7 >> 64 as i32 - 15 as i32;
    subkey[12 as i32 as usize] = k0;
    let mut __rol128_t_8: uint64_t = k0;
    k0 = k0 << 15 as i32 | k1 >> 64 as i32 - 15 as i32;
    k1 = k1 << 15 as i32 | __rol128_t_8 >> 64 as i32 - 15 as i32;
    subkey[14 as i32 as usize] = k0;
    subkey[15 as i32 as usize] = k1;
    let mut __rol128_t_9: uint64_t = k0;
    k0 = k0 << 34 as i32 | k1 >> 64 as i32 - 34 as i32;
    k1 = k1 << 34 as i32 | __rol128_t_9 >> 64 as i32 - 34 as i32;
    subkey[20 as i32 as usize] = k0;
    subkey[21 as i32 as usize] = k1;
    let mut __rol128_t_10: uint64_t = k0;
    k0 = k0 << 17 as i32 | k1 >> 64 as i32 - 17 as i32;
    k1 = k1 << 17 as i32 | __rol128_t_10 >> 64 as i32 - 17 as i32;
    subkey[24 as i32 as usize] = k0;
    subkey[25 as i32 as usize] = k1;
    _nettle_camellia_absorb(
        24 as i32 as u32,
        ((*ctx).keys).as_mut_ptr(),
        subkey.as_mut_ptr(),
    );
}