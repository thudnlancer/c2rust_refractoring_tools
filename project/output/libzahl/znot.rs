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
unsafe extern "C" fn zzero(mut a: *mut zahl) -> i32 {
    return ((*a).sign == 0) as i32;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> i32 {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zbits(mut a: *mut zahl) -> size_t {
    let mut rc: size_t = 0;
    if (zzero(a) != 0) as i32 as i64 != 0 {
        return 1 as i32 as size_t;
    }
    while *((*a).chars).offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize) == 0 {
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
    }
    rc = ((*a).used)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as u64);
    rc = (rc as u64)
        .wrapping_sub(
            (*((*a).chars).offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize)
                as libc::c_ulonglong)
                .leading_zeros() as i32 as size_t,
        ) as size_t as size_t;
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn znot(mut a: *mut zahl, mut b: *mut zahl) {
    let mut bits: size_t = 0;
    if (zzero(b) != 0) as i32 as i64 != 0 {
        (*a).sign = 0 as i32;
        return;
    }
    bits = zbits(b);
    (*a).used = (*b).used;
    (*a).sign = -zsignum(b);
    let mut a__: *mut zahl_char_t = (*a).chars;
    let mut b__: *const zahl_char_t = (*b).chars;
    let mut i__: size_t = 0;
    let mut n__: size_t = (*a).used;
    i__ = 0 as i32 as size_t;
    while i__ < n__ {
        *a__.offset(i__.wrapping_add(0 as i32 as u64) as isize) = !*b__
            .offset(i__.wrapping_add(0 as i32 as u64) as isize);
        *a__.offset(i__.wrapping_add(1 as i32 as u64) as isize) = !*b__
            .offset(i__.wrapping_add(1 as i32 as u64) as isize);
        *a__.offset(i__.wrapping_add(2 as i32 as u64) as isize) = !*b__
            .offset(i__.wrapping_add(2 as i32 as u64) as isize);
        *a__.offset(i__.wrapping_add(3 as i32 as u64) as isize) = !*b__
            .offset(i__.wrapping_add(3 as i32 as u64) as isize);
        i__ = (i__ as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
    }
    bits = bits & (64 as i32 - 1 as i32) as u64;
    if bits != 0 {
        let ref mut fresh0 = *((*a).chars)
            .offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize);
        *fresh0 &= ((1 as i32 as zahl_char_t) << bits).wrapping_sub(1 as i32 as u64);
    }
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