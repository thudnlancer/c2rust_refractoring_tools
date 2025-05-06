#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn _nettle_write_le64(
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint64_t,
) {
    let mut i: size_t = 0;
    let mut words: size_t = 0;
    let mut leftover: u32 = 0;
    words = length.wrapping_div(8 as i32 as u64);
    leftover = length.wrapping_rem(8 as i32 as u64) as u32;
    i = 0 as i32 as size_t;
    while i < words {
        *dst.offset(7 as i32 as isize) = (*src.offset(i as isize) >> 56 as i32
            & 0xff as i32 as u64) as uint8_t;
        *dst.offset(6 as i32 as isize) = (*src.offset(i as isize) >> 48 as i32
            & 0xff as i32 as u64) as uint8_t;
        *dst.offset(5 as i32 as isize) = (*src.offset(i as isize) >> 40 as i32
            & 0xff as i32 as u64) as uint8_t;
        *dst.offset(4 as i32 as isize) = (*src.offset(i as isize) >> 32 as i32
            & 0xff as i32 as u64) as uint8_t;
        *dst.offset(3 as i32 as isize) = (*src.offset(i as isize) >> 24 as i32
            & 0xff as i32 as u64) as uint8_t;
        *dst.offset(2 as i32 as isize) = (*src.offset(i as isize) >> 16 as i32
            & 0xff as i32 as u64) as uint8_t;
        *dst.offset(1 as i32 as isize) = (*src.offset(i as isize) >> 8 as i32
            & 0xff as i32 as u64) as uint8_t;
        *dst.offset(0 as i32 as isize) = (*src.offset(i as isize) & 0xff as i32 as u64)
            as uint8_t;
        i = i.wrapping_add(1);
        i;
        dst = dst.offset(8 as i32 as isize);
    }
    if leftover != 0 {
        let mut word: uint64_t = 0;
        word = *src.offset(i as isize);
        loop {
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = (word & 0xff as i32 as u64) as uint8_t;
            word >>= 8 as i32;
            leftover = leftover.wrapping_sub(1);
            if !(leftover != 0) {
                break;
            }
        }
    }
}