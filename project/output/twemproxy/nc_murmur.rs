#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn hash_murmur(
    mut key: *const i8,
    mut length: size_t,
) -> uint32_t {
    let m: u32 = 0x5bd1e995 as i32 as u32;
    let seed: uint32_t = (0xdeadbeef as u32).wrapping_mul(length as uint32_t);
    let r: i32 = 24 as i32;
    let mut h: uint32_t = seed ^ length as uint32_t;
    let mut data: *const u8 = key as *const u8;
    while length >= 4 as i32 as u64 {
        let mut k: u32 = *(data as *mut u32);
        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h = (h as u32).wrapping_mul(m) as uint32_t as uint32_t;
        h ^= k;
        data = data.offset(4 as i32 as isize);
        length = (length as u64).wrapping_sub(4 as i32 as u64) as size_t as size_t;
    }
    let mut current_block_12: u64;
    match length {
        3 => {
            h ^= (*data.offset(2 as i32 as isize) as uint32_t) << 16 as i32;
            current_block_12 = 1271714645385924685;
        }
        2 => {
            current_block_12 = 1271714645385924685;
        }
        1 => {
            current_block_12 = 2606630792939804770;
        }
        _ => {
            current_block_12 = 12800627514080957624;
        }
    }
    match current_block_12 {
        1271714645385924685 => {
            h ^= (*data.offset(1 as i32 as isize) as uint32_t) << 8 as i32;
            current_block_12 = 2606630792939804770;
        }
        _ => {}
    }
    match current_block_12 {
        2606630792939804770 => {
            h ^= *data.offset(0 as i32 as isize) as u32;
            h = (h as u32).wrapping_mul(m) as uint32_t as uint32_t;
        }
        _ => {}
    }
    h ^= h >> 13 as i32;
    h = (h as u32).wrapping_mul(m) as uint32_t as uint32_t;
    h ^= h >> 15 as i32;
    return h;
}