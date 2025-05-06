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
    fn libzahl_realloc(_: *mut zahl, _: size_t);
}
pub type size_t = u64;
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: i32,
    pub padding__: i32,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[inline]
unsafe extern "C" fn libzahl_memset(
    mut a: *mut zahl_char_t,
    mut v: zahl_char_t,
    mut n: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n {
        *a.offset(i.wrapping_add(0 as i32 as u64) as isize) = v;
        *a.offset(i.wrapping_add(1 as i32 as u64) as isize) = v;
        *a.offset(i.wrapping_add(2 as i32 as u64) as isize) = v;
        *a.offset(i.wrapping_add(3 as i32 as u64) as isize) = v;
        i = (i as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
    }
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> i32 {
    return ((*a).sign == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_set(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as i32 as zahl_char_t;
    let mut chars: size_t = bit >> 6 as i32;
    if zzero(a) != 0 {
        (*a).used = 0 as i32 as size_t;
        (*a).sign = 1 as i32;
    }
    if (chars >= (*a).used) as i32 as i64 != 0 {
        if (*a).alloced < chars.wrapping_add(1 as i32 as u64) {
            libzahl_realloc(a, chars.wrapping_add(1 as i32 as u64));
        }
        libzahl_memset(
            ((*a).chars).offset((*a).used as isize),
            0 as i32 as zahl_char_t,
            chars.wrapping_add(1 as i32 as u64).wrapping_sub((*a).used),
        );
        (*a).used = chars.wrapping_add(1 as i32 as u64);
    }
    bit = bit & (64 as i32 - 1 as i32) as u64;
    mask <<= bit;
    let ref mut fresh0 = *((*a).chars).offset(chars as isize);
    *fresh0 |= mask;
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_clear(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as i32 as zahl_char_t;
    let mut chars: size_t = bit >> 6 as i32;
    if (chars >= (*a).used) as i32 as i64 != 0 {
        return;
    }
    bit = bit & (64 as i32 - 1 as i32) as u64;
    mask <<= bit;
    let ref mut fresh1 = *((*a).chars).offset(chars as isize);
    *fresh1 &= !mask;
    while (*a).used != 0
        && *((*a).chars).offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize) == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    if (*a).used == 0 {
        (*a).sign = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn zbset_ll_flip(mut a: *mut zahl, mut bit: size_t) {
    let mut mask: zahl_char_t = 1 as i32 as zahl_char_t;
    let mut chars: size_t = bit >> 6 as i32;
    if zzero(a) != 0 {
        (*a).used = 0 as i32 as size_t;
        (*a).sign = 1 as i32;
    }
    if (chars >= (*a).used) as i32 as i64 != 0 {
        if (*a).alloced < chars.wrapping_add(1 as i32 as u64) {
            libzahl_realloc(a, chars.wrapping_add(1 as i32 as u64));
        }
        libzahl_memset(
            ((*a).chars).offset((*a).used as isize),
            0 as i32 as zahl_char_t,
            chars.wrapping_add(1 as i32 as u64).wrapping_sub((*a).used),
        );
        (*a).used = chars.wrapping_add(1 as i32 as u64);
    }
    bit = bit & (64 as i32 - 1 as i32) as u64;
    mask <<= bit;
    let ref mut fresh2 = *((*a).chars).offset(chars as isize);
    *fresh2 ^= mask;
    while (*a).used != 0
        && *((*a).chars).offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize) == 0
    {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    if (*a).used == 0 {
        (*a).sign = 0 as i32;
    }
}