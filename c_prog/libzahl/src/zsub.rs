#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(asm)]
use core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    static mut libzahl_tmp_sub: z_t;
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: libc::c_int,
    pub padding__: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [zahl; 1];
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17254579315488500390;
        }
        19 => {
            current_block_42 = 17254579315488500390;
        }
        18 => {
            current_block_42 = 16864574447450575806;
        }
        17 => {
            current_block_42 = 4745558310823793980;
        }
        16 => {
            current_block_42 = 3609958504996442327;
        }
        15 => {
            current_block_42 = 7591989803176764182;
        }
        14 => {
            current_block_42 = 18018901114339951972;
        }
        13 => {
            current_block_42 = 12689878998560279573;
        }
        12 => {
            current_block_42 = 5286769970458876423;
        }
        11 => {
            current_block_42 = 7754037464575168929;
        }
        10 => {
            current_block_42 = 3450148331727835525;
        }
        9 => {
            current_block_42 = 17513148302838498461;
        }
        8 => {
            current_block_42 = 5033545425852514390;
        }
        7 => {
            current_block_42 = 7822129706622160761;
        }
        6 => {
            current_block_42 = 14314526103220412958;
        }
        5 => {
            current_block_42 = 11474465627670557597;
        }
        4 => {
            current_block_42 = 13771482776398185125;
        }
        3 => {
            current_block_42 = 519160363748337264;
        }
        2 => {
            current_block_42 = 17126217794288990979;
        }
        1 => {
            current_block_42 = 11647970138234258413;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n    shlq $3, {3}\n    addq {1}, {3}\n 1:\n    movq 0({2}), {0}\n    movq {0}, 0({1})\n    movq 8({2}), {0}\n    movq {0}, 8({1})\n    movq 16({2}), {0}\n    movq {0}, 16({1})\n    movq 24({2}), {0}\n    movq {0}, 24({1})\n    addq $32, {2}\n    addq $32, {1}\n    cmpq {3}, {1}\n    jl 1b",
                lateout(reg) t, inlateout(reg) d, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        17254579315488500390 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16864574447450575806;
        }
        _ => {}
    }
    match current_block_42 {
        16864574447450575806 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4745558310823793980;
        }
        _ => {}
    }
    match current_block_42 {
        4745558310823793980 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3609958504996442327;
        }
        _ => {}
    }
    match current_block_42 {
        3609958504996442327 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7591989803176764182;
        }
        _ => {}
    }
    match current_block_42 {
        7591989803176764182 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 18018901114339951972;
        }
        _ => {}
    }
    match current_block_42 {
        18018901114339951972 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12689878998560279573;
        }
        _ => {}
    }
    match current_block_42 {
        12689878998560279573 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5286769970458876423;
        }
        _ => {}
    }
    match current_block_42 {
        5286769970458876423 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7754037464575168929;
        }
        _ => {}
    }
    match current_block_42 {
        7754037464575168929 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3450148331727835525;
        }
        _ => {}
    }
    match current_block_42 {
        3450148331727835525 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17513148302838498461;
        }
        _ => {}
    }
    match current_block_42 {
        17513148302838498461 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 5033545425852514390;
        }
        _ => {}
    }
    match current_block_42 {
        5033545425852514390 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7822129706622160761;
        }
        _ => {}
    }
    match current_block_42 {
        7822129706622160761 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 14314526103220412958;
        }
        _ => {}
    }
    match current_block_42 {
        14314526103220412958 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11474465627670557597;
        }
        _ => {}
    }
    match current_block_42 {
        11474465627670557597 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13771482776398185125;
        }
        _ => {}
    }
    match current_block_42 {
        13771482776398185125 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 519160363748337264;
        }
        _ => {}
    }
    match current_block_42 {
        519160363748337264 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17126217794288990979;
        }
        _ => {}
    }
    match current_block_42 {
        17126217794288990979 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11647970138234258413;
        }
        _ => {}
    }
    match current_block_42 {
        11647970138234258413 => {
            *d
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used {
            libzahl_realloc(a, (*b).used);
        }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl) -> libc::c_int {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as libc::c_int as libc::c_long != 0 {
        return -((zzero(b) == 0) as libc::c_int);
    }
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        return 1 as libc::c_int;
    }
    i = ((*a).used).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    j = ((*b).used).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    while i > j {
        if *((*a).chars).offset(i as isize) != 0 {
            return 1 as libc::c_int;
        }
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
        i = i.wrapping_sub(1);
        i;
    }
    while j > i {
        if *((*b).chars).offset(j as isize) != 0 {
            return -(1 as libc::c_int);
        }
        (*b).used = ((*b).used).wrapping_sub(1);
        (*b).used;
        j = j.wrapping_sub(1);
        j;
    }
    while i != 0 && *((*a).chars).offset(i as isize) == *((*b).chars).offset(i as isize)
    {
        i = i.wrapping_sub(1);
        i;
    }
    return if *((*a).chars).offset(i as isize) < *((*b).chars).offset(i as isize) {
        -(1 as libc::c_int)
    } else {
        (*((*a).chars).offset(i as isize) > *((*b).chars).offset(i as isize))
            as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zabs(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign &= 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn zneg(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign = -(*a).sign;
}
#[inline]
unsafe extern "C" fn zsub_impl(mut a: *mut zahl, mut b: *mut zahl, mut n: size_t) {
    let mut carry: zahl_char_t = 0 as libc::c_int as zahl_char_t;
    let mut tcarry: zahl_char_t = 0;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n {
        tcarry = (if carry != 0 {
            (*((*a).chars).offset(i as isize) <= *((*b).chars).offset(i as isize))
                as libc::c_int
        } else {
            (*((*a).chars).offset(i as isize) < *((*b).chars).offset(i as isize))
                as libc::c_int
        }) as zahl_char_t;
        let ref mut fresh0 = *((*a).chars).offset(i as isize);
        *fresh0 = (*fresh0 as libc::c_ulong)
            .wrapping_sub(*((*b).chars).offset(i as isize)) as zahl_char_t
            as zahl_char_t;
        let ref mut fresh1 = *((*a).chars).offset(i as isize);
        *fresh1 = (*fresh1 as libc::c_ulong).wrapping_sub(carry) as zahl_char_t
            as zahl_char_t;
        carry = tcarry;
        i = i.wrapping_add(1);
        i;
    }
    if carry != 0 {
        while *((*a).chars).offset(i as isize) == 0 {
            let fresh2 = i;
            i = i.wrapping_add(1);
            *((*a).chars)
                .offset(fresh2 as isize) = 18446744073709551615 as libc::c_ulong;
        }
        if *((*a).chars).offset(i as isize) == 1 as libc::c_int as libc::c_ulong {
            (*a).used = ((*a).used).wrapping_sub(1);
            (*a).used;
        } else {
            let ref mut fresh3 = *((*a).chars).offset(i as isize);
            *fresh3 = (*fresh3 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as zahl_char_t
                as zahl_char_t;
        }
    }
}
#[inline]
unsafe extern "C" fn libzahl_zsub_unsigned(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
) {
    let mut magcmp: libc::c_int = 0;
    let mut n: size_t = 0;
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        zabs(a, c);
        zneg(a, a);
        return;
    } else if (zzero(c) != 0) as libc::c_int as libc::c_long != 0 {
        zabs(a, b);
        return;
    }
    magcmp = zcmpmag(b, c);
    if (magcmp <= 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if (magcmp == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
            (*a).sign = 0 as libc::c_int;
            return;
        }
        n = (*b).used;
        if a == b {
            zset(libzahl_tmp_sub.as_mut_ptr(), b);
            if a != c {
                zset(a, c);
            }
            zsub_impl(a, libzahl_tmp_sub.as_mut_ptr(), n);
        } else {
            if a != c {
                zset(a, c);
            }
            zsub_impl(a, b, n);
        }
    } else {
        n = (*c).used;
        if (a == c) as libc::c_int as libc::c_long != 0 {
            zset(libzahl_tmp_sub.as_mut_ptr(), c);
            if a != b {
                zset(a, b);
            }
            zsub_impl(a, libzahl_tmp_sub.as_mut_ptr(), n);
        } else {
            if a != b {
                zset(a, b);
            }
            zsub_impl(a, c, n);
        }
    }
    (*a).sign = magcmp;
}
#[no_mangle]
pub unsafe extern "C" fn zsub_unsigned(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
) {
    libzahl_zsub_unsigned(a, b, c);
}
#[no_mangle]
pub unsafe extern "C" fn zsub_nonnegative_assign(mut a: *mut zahl, mut b: *mut zahl) {
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        zabs(a, a);
    } else if (zcmpmag(a, b) == 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        zsub_impl(a, b, (*b).used);
    };
}
#[no_mangle]
pub unsafe extern "C" fn zsub_positive_assign(mut a: *mut zahl, mut b: *mut zahl) {
    zsub_impl(a, b, (*b).used);
}
#[no_mangle]
pub unsafe extern "C" fn zsub(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        zneg(a, c);
    } else if (zzero(c) != 0) as libc::c_int as libc::c_long != 0 {
        if a != b {
            zset(a, b);
        }
    } else if (zsignum(b) < 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if zsignum(c) < 0 as libc::c_int {
            libzahl_zsub_unsigned(a, c, b);
        } else {
            zadd_unsigned(a, b, c);
            (*a).sign = -zsignum(a);
        }
    } else if zsignum(c) < 0 as libc::c_int {
        zadd_unsigned(a, b, c);
    } else {
        libzahl_zsub_unsigned(a, b, c);
    };
}
