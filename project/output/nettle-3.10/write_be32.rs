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
    fn abort() -> !;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub unsafe extern "C" fn _nettle_write_be32(
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint32_t,
) {
    let mut i: size_t = 0;
    let mut words: size_t = 0;
    let mut leftover: u32 = 0;
    words = length.wrapping_div(4 as i32 as u64);
    leftover = length.wrapping_rem(4 as i32 as u64) as u32;
    i = 0 as i32 as size_t;
    while i < words {
        *dst.offset(0 as i32 as isize) = (*src.offset(i as isize) >> 24 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(1 as i32 as isize) = (*src.offset(i as isize) >> 16 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(2 as i32 as isize) = (*src.offset(i as isize) >> 8 as i32
            & 0xff as i32 as u32) as uint8_t;
        *dst.offset(3 as i32 as isize) = (*src.offset(i as isize) & 0xff as i32 as u32)
            as uint8_t;
        i = i.wrapping_add(1);
        i;
        dst = dst.offset(4 as i32 as isize);
    }
    if leftover != 0 {
        let mut word: uint32_t = 0;
        let mut j: u32 = leftover;
        word = *src.offset(i as isize);
        let mut current_block_14: u64;
        match leftover {
            3 => {
                j = j.wrapping_sub(1);
                *dst.offset(j as isize) = (word >> 8 as i32 & 0xff as i32 as u32)
                    as uint8_t;
                current_block_14 = 11359316228224769559;
            }
            2 => {
                current_block_14 = 11359316228224769559;
            }
            1 => {
                current_block_14 = 14563843320413450866;
            }
            _ => {
                abort();
            }
        }
        match current_block_14 {
            11359316228224769559 => {
                j = j.wrapping_sub(1);
                *dst.offset(j as isize) = (word >> 16 as i32 & 0xff as i32 as u32)
                    as uint8_t;
            }
            _ => {}
        }
        j = j.wrapping_sub(1);
        *dst.offset(j as isize) = (word >> 24 as i32 & 0xff as i32 as u32) as uint8_t;
    }
}