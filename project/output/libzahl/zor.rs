#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm)]
use core::arch::asm;
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
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d.offset((20 as i32 - 1 as i32) as isize) = *s
                .offset((20 as i32 - 1 as i32) as isize);
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
            *d.offset((19 as i32 - 1 as i32) as isize) = *s
                .offset((19 as i32 - 1 as i32) as isize);
            current_block_42 = 16864574447450575806;
        }
        _ => {}
    }
    match current_block_42 {
        16864574447450575806 => {
            *d.offset((18 as i32 - 1 as i32) as isize) = *s
                .offset((18 as i32 - 1 as i32) as isize);
            current_block_42 = 4745558310823793980;
        }
        _ => {}
    }
    match current_block_42 {
        4745558310823793980 => {
            *d.offset((17 as i32 - 1 as i32) as isize) = *s
                .offset((17 as i32 - 1 as i32) as isize);
            current_block_42 = 3609958504996442327;
        }
        _ => {}
    }
    match current_block_42 {
        3609958504996442327 => {
            *d.offset((16 as i32 - 1 as i32) as isize) = *s
                .offset((16 as i32 - 1 as i32) as isize);
            current_block_42 = 7591989803176764182;
        }
        _ => {}
    }
    match current_block_42 {
        7591989803176764182 => {
            *d.offset((15 as i32 - 1 as i32) as isize) = *s
                .offset((15 as i32 - 1 as i32) as isize);
            current_block_42 = 18018901114339951972;
        }
        _ => {}
    }
    match current_block_42 {
        18018901114339951972 => {
            *d.offset((14 as i32 - 1 as i32) as isize) = *s
                .offset((14 as i32 - 1 as i32) as isize);
            current_block_42 = 12689878998560279573;
        }
        _ => {}
    }
    match current_block_42 {
        12689878998560279573 => {
            *d.offset((13 as i32 - 1 as i32) as isize) = *s
                .offset((13 as i32 - 1 as i32) as isize);
            current_block_42 = 5286769970458876423;
        }
        _ => {}
    }
    match current_block_42 {
        5286769970458876423 => {
            *d.offset((12 as i32 - 1 as i32) as isize) = *s
                .offset((12 as i32 - 1 as i32) as isize);
            current_block_42 = 7754037464575168929;
        }
        _ => {}
    }
    match current_block_42 {
        7754037464575168929 => {
            *d.offset((11 as i32 - 1 as i32) as isize) = *s
                .offset((11 as i32 - 1 as i32) as isize);
            current_block_42 = 3450148331727835525;
        }
        _ => {}
    }
    match current_block_42 {
        3450148331727835525 => {
            *d.offset((10 as i32 - 1 as i32) as isize) = *s
                .offset((10 as i32 - 1 as i32) as isize);
            current_block_42 = 17513148302838498461;
        }
        _ => {}
    }
    match current_block_42 {
        17513148302838498461 => {
            *d.offset((9 as i32 - 1 as i32) as isize) = *s
                .offset((9 as i32 - 1 as i32) as isize);
            current_block_42 = 5033545425852514390;
        }
        _ => {}
    }
    match current_block_42 {
        5033545425852514390 => {
            *d.offset((8 as i32 - 1 as i32) as isize) = *s
                .offset((8 as i32 - 1 as i32) as isize);
            current_block_42 = 7822129706622160761;
        }
        _ => {}
    }
    match current_block_42 {
        7822129706622160761 => {
            *d.offset((7 as i32 - 1 as i32) as isize) = *s
                .offset((7 as i32 - 1 as i32) as isize);
            current_block_42 = 14314526103220412958;
        }
        _ => {}
    }
    match current_block_42 {
        14314526103220412958 => {
            *d.offset((6 as i32 - 1 as i32) as isize) = *s
                .offset((6 as i32 - 1 as i32) as isize);
            current_block_42 = 11474465627670557597;
        }
        _ => {}
    }
    match current_block_42 {
        11474465627670557597 => {
            *d.offset((5 as i32 - 1 as i32) as isize) = *s
                .offset((5 as i32 - 1 as i32) as isize);
            current_block_42 = 13771482776398185125;
        }
        _ => {}
    }
    match current_block_42 {
        13771482776398185125 => {
            *d.offset((4 as i32 - 1 as i32) as isize) = *s
                .offset((4 as i32 - 1 as i32) as isize);
            current_block_42 = 519160363748337264;
        }
        _ => {}
    }
    match current_block_42 {
        519160363748337264 => {
            *d.offset((3 as i32 - 1 as i32) as isize) = *s
                .offset((3 as i32 - 1 as i32) as isize);
            current_block_42 = 17126217794288990979;
        }
        _ => {}
    }
    match current_block_42 {
        17126217794288990979 => {
            *d.offset((2 as i32 - 1 as i32) as isize) = *s
                .offset((2 as i32 - 1 as i32) as isize);
            current_block_42 = 11647970138234258413;
        }
        _ => {}
    }
    match current_block_42 {
        11647970138234258413 => {
            *d.offset((1 as i32 - 1 as i32) as isize) = *s
                .offset((1 as i32 - 1 as i32) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zmemcpy_range(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut i: size_t,
    mut n: size_t,
) {
    d = d.offset(i as isize);
    s = s.offset(i as isize);
    n = (n as u64).wrapping_sub(i) as size_t as size_t;
    libzahl_memcpy(d, s, n);
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as i32) as i32 as i64 != 0 {
        (*a).sign = 0 as i32;
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
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> i32 {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> i32 {
    return ((*a).sign == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn zor(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut n: size_t = 0;
    let mut m: size_t = 0;
    if (zzero(b) != 0) as i32 as i64 != 0 {
        if a != c {
            zset(a, c);
        }
        return;
    } else if (zzero(c) != 0) as i32 as i64 != 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    n = (if (*b).used < (*c).used { (*b).used } else { (*c).used });
    m = (if (*b).used > (*c).used { (*b).used } else { (*c).used });
    if (*a).alloced < m {
        libzahl_realloc(a, m);
    }
    if a == b {
        let mut a__: *mut zahl_char_t = (*a).chars;
        let mut b__: *const zahl_char_t = (*a).chars;
        let mut c__: *const zahl_char_t = (*c).chars;
        let mut i__: size_t = 0;
        let mut n__: size_t = n;
        if n__ <= 4 as i32 as u64 {
            if n__ >= 1 as i32 as u64 {
                *a__.offset(0 as i32 as isize) = *b__.offset(0 as i32 as isize)
                    | *c__.offset(0 as i32 as isize);
            }
            if n__ >= 2 as i32 as u64 {
                *a__.offset(1 as i32 as isize) = *b__.offset(1 as i32 as isize)
                    | *c__.offset(1 as i32 as isize);
            }
            if n__ >= 3 as i32 as u64 {
                *a__.offset(2 as i32 as isize) = *b__.offset(2 as i32 as isize)
                    | *c__.offset(2 as i32 as isize);
            }
            if n__ >= 4 as i32 as u64 {
                *a__.offset(3 as i32 as isize) = *b__.offset(3 as i32 as isize)
                    | *c__.offset(3 as i32 as isize);
            }
        } else {
            i__ = 0 as i32 as size_t;
            loop {
                i__ = (i__ as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
                if !(i__ < n__) {
                    break;
                }
                *a__.offset(i__.wrapping_sub(1 as i32 as u64) as isize) = *b__
                    .offset(i__.wrapping_sub(1 as i32 as u64) as isize)
                    | *c__.offset(i__.wrapping_sub(1 as i32 as u64) as isize);
                *a__.offset(i__.wrapping_sub(2 as i32 as u64) as isize) = *b__
                    .offset(i__.wrapping_sub(2 as i32 as u64) as isize)
                    | *c__.offset(i__.wrapping_sub(2 as i32 as u64) as isize);
                *a__.offset(i__.wrapping_sub(3 as i32 as u64) as isize) = *b__
                    .offset(i__.wrapping_sub(3 as i32 as u64) as isize)
                    | *c__.offset(i__.wrapping_sub(3 as i32 as u64) as isize);
                *a__.offset(i__.wrapping_sub(4 as i32 as u64) as isize) = *b__
                    .offset(i__.wrapping_sub(4 as i32 as u64) as isize)
                    | *c__.offset(i__.wrapping_sub(4 as i32 as u64) as isize);
            }
            if i__ > n__ {
                i__ = (i__ as u64).wrapping_sub(4 as i32 as u64) as size_t as size_t;
                while i__ < n__ {
                    *a__.offset(i__ as isize) = *b__.offset(i__ as isize)
                        | *c__.offset(i__ as isize);
                    i__ = i__.wrapping_add(1);
                    i__;
                }
            }
        }
        if (*a).used < (*c).used {
            zmemcpy_range((*a).chars, (*c).chars, n, m);
        }
    } else if (a == c) as i32 as i64 != 0 {
        let mut a___0: *mut zahl_char_t = (*a).chars;
        let mut b___0: *const zahl_char_t = (*a).chars;
        let mut c___0: *const zahl_char_t = (*b).chars;
        let mut i___0: size_t = 0;
        let mut n___0: size_t = n;
        if n___0 <= 4 as i32 as u64 {
            if n___0 >= 1 as i32 as u64 {
                *a___0.offset(0 as i32 as isize) = *b___0.offset(0 as i32 as isize)
                    | *c___0.offset(0 as i32 as isize);
            }
            if n___0 >= 2 as i32 as u64 {
                *a___0.offset(1 as i32 as isize) = *b___0.offset(1 as i32 as isize)
                    | *c___0.offset(1 as i32 as isize);
            }
            if n___0 >= 3 as i32 as u64 {
                *a___0.offset(2 as i32 as isize) = *b___0.offset(2 as i32 as isize)
                    | *c___0.offset(2 as i32 as isize);
            }
            if n___0 >= 4 as i32 as u64 {
                *a___0.offset(3 as i32 as isize) = *b___0.offset(3 as i32 as isize)
                    | *c___0.offset(3 as i32 as isize);
            }
        } else {
            i___0 = 0 as i32 as size_t;
            loop {
                i___0 = (i___0 as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
                if !(i___0 < n___0) {
                    break;
                }
                *a___0.offset(i___0.wrapping_sub(1 as i32 as u64) as isize) = *b___0
                    .offset(i___0.wrapping_sub(1 as i32 as u64) as isize)
                    | *c___0.offset(i___0.wrapping_sub(1 as i32 as u64) as isize);
                *a___0.offset(i___0.wrapping_sub(2 as i32 as u64) as isize) = *b___0
                    .offset(i___0.wrapping_sub(2 as i32 as u64) as isize)
                    | *c___0.offset(i___0.wrapping_sub(2 as i32 as u64) as isize);
                *a___0.offset(i___0.wrapping_sub(3 as i32 as u64) as isize) = *b___0
                    .offset(i___0.wrapping_sub(3 as i32 as u64) as isize)
                    | *c___0.offset(i___0.wrapping_sub(3 as i32 as u64) as isize);
                *a___0.offset(i___0.wrapping_sub(4 as i32 as u64) as isize) = *b___0
                    .offset(i___0.wrapping_sub(4 as i32 as u64) as isize)
                    | *c___0.offset(i___0.wrapping_sub(4 as i32 as u64) as isize);
            }
            if i___0 > n___0 {
                i___0 = (i___0 as u64).wrapping_sub(4 as i32 as u64) as size_t as size_t;
                while i___0 < n___0 {
                    *a___0.offset(i___0 as isize) = *b___0.offset(i___0 as isize)
                        | *c___0.offset(i___0 as isize);
                    i___0 = i___0.wrapping_add(1);
                    i___0;
                }
            }
        }
        if (*a).used < (*b).used {
            zmemcpy_range((*a).chars, (*b).chars, n, m);
        }
    } else if m == (*b).used {
        let mut a___1: *mut zahl_char_t = (*a).chars;
        let mut b___1: *const zahl_char_t = (*c).chars;
        let mut c___1: *const zahl_char_t = (*b).chars;
        let mut i___1: size_t = 0;
        let mut n___1: size_t = n;
        i___1 = 0 as i32 as size_t;
        while i___1 < n___1 {
            *a___1.offset(i___1.wrapping_add(0 as i32 as u64) as isize) = *b___1
                .offset(i___1.wrapping_add(0 as i32 as u64) as isize)
                | *c___1.offset(i___1.wrapping_add(0 as i32 as u64) as isize);
            *a___1.offset(i___1.wrapping_add(1 as i32 as u64) as isize) = *b___1
                .offset(i___1.wrapping_add(1 as i32 as u64) as isize)
                | *c___1.offset(i___1.wrapping_add(1 as i32 as u64) as isize);
            *a___1.offset(i___1.wrapping_add(2 as i32 as u64) as isize) = *b___1
                .offset(i___1.wrapping_add(2 as i32 as u64) as isize)
                | *c___1.offset(i___1.wrapping_add(2 as i32 as u64) as isize);
            *a___1.offset(i___1.wrapping_add(3 as i32 as u64) as isize) = *b___1
                .offset(i___1.wrapping_add(3 as i32 as u64) as isize)
                | *c___1.offset(i___1.wrapping_add(3 as i32 as u64) as isize);
            i___1 = (i___1 as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
        }
        zmemcpy_range((*a).chars, (*b).chars, n, m);
    } else {
        let mut a___2: *mut zahl_char_t = (*a).chars;
        let mut b___2: *const zahl_char_t = (*b).chars;
        let mut c___2: *const zahl_char_t = (*c).chars;
        let mut i___2: size_t = 0;
        let mut n___2: size_t = n;
        i___2 = 0 as i32 as size_t;
        while i___2 < n___2 {
            *a___2.offset(i___2.wrapping_add(0 as i32 as u64) as isize) = *b___2
                .offset(i___2.wrapping_add(0 as i32 as u64) as isize)
                | *c___2.offset(i___2.wrapping_add(0 as i32 as u64) as isize);
            *a___2.offset(i___2.wrapping_add(1 as i32 as u64) as isize) = *b___2
                .offset(i___2.wrapping_add(1 as i32 as u64) as isize)
                | *c___2.offset(i___2.wrapping_add(1 as i32 as u64) as isize);
            *a___2.offset(i___2.wrapping_add(2 as i32 as u64) as isize) = *b___2
                .offset(i___2.wrapping_add(2 as i32 as u64) as isize)
                | *c___2.offset(i___2.wrapping_add(2 as i32 as u64) as isize);
            *a___2.offset(i___2.wrapping_add(3 as i32 as u64) as isize) = *b___2
                .offset(i___2.wrapping_add(3 as i32 as u64) as isize)
                | *c___2.offset(i___2.wrapping_add(3 as i32 as u64) as isize);
            i___2 = (i___2 as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
        }
        zmemcpy_range((*a).chars, (*c).chars, n, m);
    }
    (*a).used = m;
    (*a).sign = (zsignum(b) + zsignum(c) == 2 as i32) as i32 * 2 as i32 - 1 as i32;
}