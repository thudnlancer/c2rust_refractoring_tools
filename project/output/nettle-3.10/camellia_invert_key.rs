#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn _nettle_camellia_invert_key(
    mut nkeys: u32,
    mut dst: *mut uint64_t,
    mut src: *const uint64_t,
) {
    let mut i: u32 = 0;
    if dst == src as *mut uint64_t {
        i = 0 as i32 as u32;
        while i < nkeys.wrapping_sub(1 as i32 as u32).wrapping_sub(i) {
            let mut t_swap: uint64_t = *dst.offset(i as isize);
            *dst.offset(i as isize) = *dst
                .offset(nkeys.wrapping_sub(1 as i32 as u32).wrapping_sub(i) as isize);
            *dst.offset(nkeys.wrapping_sub(1 as i32 as u32).wrapping_sub(i) as isize) = t_swap;
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as i32 as u32;
        while i < nkeys {
            *dst.offset(i as isize) = *src
                .offset(nkeys.wrapping_sub(1 as i32 as u32).wrapping_sub(i) as isize);
            i = i.wrapping_add(1);
            i;
        }
    };
}