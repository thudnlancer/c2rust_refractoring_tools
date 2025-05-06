#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type ptrdiff_t = i64;
#[no_mangle]
pub unsafe extern "C" fn u8_uctomb_aux(
    mut s: *mut uint8_t,
    mut uc: ucs4_t,
    mut n: ptrdiff_t,
) -> i32 {
    let mut count: i32 = 0;
    if uc < 0x80 as i32 as u32 {
        return -(2 as i32)
    } else if uc < 0x800 as i32 as u32 {
        count = 2 as i32;
    } else if uc < 0x10000 as i32 as u32 {
        if uc < 0xd800 as i32 as u32 || uc >= 0xe000 as i32 as u32 {
            count = 3 as i32;
        } else {
            return -(1 as i32)
        }
    } else if uc < 0x110000 as i32 as u32 {
        count = 4 as i32;
    } else {
        return -(1 as i32)
    }
    if n < count as i64 {
        return -(2 as i32);
    }
    let mut current_block_25: u64;
    match count {
        4 => {
            *s.offset(3 as i32 as isize) = (0x80 as i32 as u32 | uc & 0x3f as i32 as u32)
                as uint8_t;
            uc = uc >> 6 as i32;
            uc |= 0x10000 as i32 as u32;
            current_block_25 = 7092055183921229348;
        }
        3 => {
            current_block_25 = 7092055183921229348;
        }
        2 => {
            current_block_25 = 5795868061627929324;
        }
        _ => {
            current_block_25 = 224731115979188411;
        }
    }
    match current_block_25 {
        7092055183921229348 => {
            *s.offset(2 as i32 as isize) = (0x80 as i32 as u32 | uc & 0x3f as i32 as u32)
                as uint8_t;
            uc = uc >> 6 as i32;
            uc |= 0x800 as i32 as u32;
            current_block_25 = 5795868061627929324;
        }
        _ => {}
    }
    match current_block_25 {
        5795868061627929324 => {
            *s.offset(1 as i32 as isize) = (0x80 as i32 as u32 | uc & 0x3f as i32 as u32)
                as uint8_t;
            uc = uc >> 6 as i32;
            uc |= 0xc0 as i32 as u32;
            *s.offset(0 as i32 as isize) = uc as uint8_t;
        }
        _ => {}
    }
    return count;
}