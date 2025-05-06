#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_l3_init(mut size: u32, mut k: *mut uint64_t) {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < size {
        let mut w: uint64_t = *k.offset(i as isize);
        w = (w as libc::c_ulonglong).swap_bytes() as uint64_t;
        *k.offset(i as isize) = (w as libc::c_ulonglong)
            .wrapping_rem(0xffffffffb as libc::c_ulonglong) as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn umac_l3_word(mut k: *const uint64_t, mut w: uint64_t) -> uint64_t {
    let mut i: u32 = 0;
    let mut y: uint64_t = 0;
    y = 0 as i32 as uint64_t;
    i = y as u32;
    while i < 4 as i32 as u32 {
        y = (y as u64)
            .wrapping_add(
                (w & 0xffff as i32 as u64)
                    .wrapping_mul(*k.offset((3 as i32 as u32).wrapping_sub(i) as isize)),
            ) as uint64_t as uint64_t;
        i = i.wrapping_add(1);
        i;
        w >>= 16 as i32;
    }
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_l3(
    mut key: *const uint64_t,
    mut m: *const uint64_t,
) -> uint32_t {
    let mut y: uint32_t = ((umac_l3_word(key, *m.offset(0 as i32 as isize)))
        .wrapping_add(
            umac_l3_word(key.offset(4 as i32 as isize), *m.offset(1 as i32 as isize)),
        ) as libc::c_ulonglong)
        .wrapping_rem(0xffffffffb as libc::c_ulonglong) as uint32_t;
    return y.swap_bytes();
}