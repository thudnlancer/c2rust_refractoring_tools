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
pub unsafe extern "C" fn hash_one_at_a_time(
    mut key: *const i8,
    mut key_length: size_t,
) -> uint32_t {
    let mut ptr: *const i8 = key;
    let mut value: uint32_t = 0 as i32 as uint32_t;
    loop {
        let fresh0 = key_length;
        key_length = key_length.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        let mut val: uint32_t = *fresh1 as uint32_t;
        value = (value as u32).wrapping_add(val) as uint32_t as uint32_t;
        value = (value as u32).wrapping_add(value << 10 as i32) as uint32_t as uint32_t;
        value ^= value >> 6 as i32;
    }
    value = (value as u32).wrapping_add(value << 3 as i32) as uint32_t as uint32_t;
    value ^= value >> 11 as i32;
    value = (value as u32).wrapping_add(value << 15 as i32) as uint32_t as uint32_t;
    return value;
}