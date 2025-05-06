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
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[no_mangle]
pub unsafe extern "C" fn hash_hsieh(
    mut key: *const i8,
    mut key_length: size_t,
) -> uint32_t {
    let mut hash: uint32_t = 0 as i32 as uint32_t;
    let mut tmp: uint32_t = 0;
    let mut rem: i32 = 0;
    if key_length <= 0 as i32 as u64 || key.is_null() {
        return 0 as i32 as uint32_t;
    }
    rem = (key_length & 3 as i32 as u64) as i32;
    key_length >>= 2 as i32;
    while key_length > 0 as i32 as u64 {
        hash = (hash as u32)
            .wrapping_add(
                ((*(key as *const uint8_t).offset(1 as i32 as isize) as uint32_t)
                    << 8 as i32)
                    .wrapping_add(
                        *(key as *const uint8_t).offset(0 as i32 as isize) as uint32_t,
                    ),
            ) as uint32_t as uint32_t;
        tmp = ((*(key.offset(2 as i32 as isize) as *const uint8_t)
            .offset(1 as i32 as isize) as uint32_t) << 8 as i32)
            .wrapping_add(
                *(key.offset(2 as i32 as isize) as *const uint8_t)
                    .offset(0 as i32 as isize) as uint32_t,
            ) << 11 as i32 ^ hash;
        hash = hash << 16 as i32 ^ tmp;
        key = key
            .offset(
                (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<uint16_t>() as u64)
                    as isize,
            );
        hash = (hash as u32).wrapping_add(hash >> 11 as i32) as uint32_t as uint32_t;
        key_length = key_length.wrapping_sub(1);
        key_length;
    }
    match rem {
        3 => {
            hash = (hash as u32)
                .wrapping_add(
                    ((*(key as *const uint8_t).offset(1 as i32 as isize) as uint32_t)
                        << 8 as i32)
                        .wrapping_add(
                            *(key as *const uint8_t).offset(0 as i32 as isize)
                                as uint32_t,
                        ),
                ) as uint32_t as uint32_t;
            hash ^= hash << 16 as i32;
            hash
                ^= (*key.offset(::core::mem::size_of::<uint16_t>() as u64 as isize)
                    as uint32_t) << 18 as i32;
            hash = (hash as u32).wrapping_add(hash >> 11 as i32) as uint32_t as uint32_t;
        }
        2 => {
            hash = (hash as u32)
                .wrapping_add(
                    ((*(key as *const uint8_t).offset(1 as i32 as isize) as uint32_t)
                        << 8 as i32)
                        .wrapping_add(
                            *(key as *const uint8_t).offset(0 as i32 as isize)
                                as uint32_t,
                        ),
                ) as uint32_t as uint32_t;
            hash ^= hash << 11 as i32;
            hash = (hash as u32).wrapping_add(hash >> 17 as i32) as uint32_t as uint32_t;
        }
        1 => {
            hash = (hash as u32).wrapping_add(*key as u8 as u32) as uint32_t as uint32_t;
            hash ^= hash << 10 as i32;
            hash = (hash as u32).wrapping_add(hash >> 1 as i32) as uint32_t as uint32_t;
        }
        _ => {}
    }
    hash ^= hash << 3 as i32;
    hash = (hash as u32).wrapping_add(hash >> 5 as i32) as uint32_t as uint32_t;
    hash ^= hash << 4 as i32;
    hash = (hash as u32).wrapping_add(hash >> 17 as i32) as uint32_t as uint32_t;
    hash ^= hash << 25 as i32;
    hash = (hash as u32).wrapping_add(hash >> 6 as i32) as uint32_t as uint32_t;
    return hash;
}