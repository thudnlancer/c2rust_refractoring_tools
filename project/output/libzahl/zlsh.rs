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
pub type __ssize_t = i64;
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
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
unsafe extern "C" fn libzahl_memset_precise(
    mut a: *mut zahl_char_t,
    mut v: zahl_char_t,
    mut n: size_t,
) {
    let mut i: size_t = 0;
    if n <= 4 as i32 as u64 {
        if n >= 1 as i32 as u64 {
            *a.offset(0 as i32 as isize) = v;
        }
        if n >= 2 as i32 as u64 {
            *a.offset(1 as i32 as isize) = v;
        }
        if n >= 3 as i32 as u64 {
            *a.offset(2 as i32 as isize) = v;
        }
        if n >= 4 as i32 as u64 {
            *a.offset(3 as i32 as isize) = v;
        }
    } else {
        i = 0 as i32 as size_t;
        loop {
            i = (i as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
            if !(i <= n) {
                break;
            }
            *a.offset(i.wrapping_sub(1 as i32 as u64) as isize) = v;
            *a.offset(i.wrapping_sub(2 as i32 as u64) as isize) = v;
            *a.offset(i.wrapping_sub(3 as i32 as u64) as isize) = v;
            *a.offset(i.wrapping_sub(4 as i32 as u64) as isize) = v;
        }
        if i > n {
            i = (i as u64).wrapping_sub(4 as i32 as u64) as size_t as size_t;
            while i < n {
                *a.offset(i as isize) = v;
                i = i.wrapping_add(1);
                i;
            }
        }
    };
}
#[inline]
unsafe extern "C" fn libzahl_memmoveb(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut i: ssize_t = 0;
    let mut current_block_47: u64;
    match n {
        20 => {
            *d.offset((20 as i32 - 1 as i32) as isize) = *s
                .offset((20 as i32 - 1 as i32) as isize);
            current_block_47 = 3708020343659147998;
        }
        19 => {
            current_block_47 = 3708020343659147998;
        }
        18 => {
            current_block_47 = 10597256872703347551;
        }
        17 => {
            current_block_47 = 16368336330394982322;
        }
        16 => {
            current_block_47 = 9630013451124115526;
        }
        15 => {
            current_block_47 = 14283634218784605551;
        }
        14 => {
            current_block_47 = 1513176906760517186;
        }
        13 => {
            current_block_47 = 8451468998463349627;
        }
        12 => {
            current_block_47 = 9944861384781218757;
        }
        11 => {
            current_block_47 = 11763560597460547600;
        }
        10 => {
            current_block_47 = 15461180760796017574;
        }
        9 => {
            current_block_47 = 1083807695818635084;
        }
        8 => {
            current_block_47 = 10234853314349311949;
        }
        7 => {
            current_block_47 = 4192008369500975849;
        }
        6 => {
            current_block_47 = 302541485824227200;
        }
        5 => {
            current_block_47 = 17931195545865348945;
        }
        4 => {
            current_block_47 = 12533541166356859325;
        }
        3 => {
            current_block_47 = 14597459921621826422;
        }
        2 => {
            current_block_47 = 2597896670383610064;
        }
        1 => {
            current_block_47 = 14613590046668411832;
        }
        0 => {
            current_block_47 = 7226443171521532240;
        }
        _ => {
            i = n as ssize_t + 3 as i32 as i64 & !(3 as i32) as i64;
            loop {
                i -= 4 as i32 as i64;
                if !(i >= 0 as i32 as i64) {
                    break;
                }
                *d.offset((i + 3 as i32 as i64) as isize) = *s
                    .offset((i + 3 as i32 as i64) as isize);
                *d.offset((i + 2 as i32 as i64) as isize) = *s
                    .offset((i + 2 as i32 as i64) as isize);
                *d.offset((i + 1 as i32 as i64) as isize) = *s
                    .offset((i + 1 as i32 as i64) as isize);
                *d.offset((i + 0 as i32 as i64) as isize) = *s
                    .offset((i + 0 as i32 as i64) as isize);
            }
            current_block_47 = 7226443171521532240;
        }
    }
    match current_block_47 {
        3708020343659147998 => {
            *d.offset((19 as i32 - 1 as i32) as isize) = *s
                .offset((19 as i32 - 1 as i32) as isize);
            current_block_47 = 10597256872703347551;
        }
        _ => {}
    }
    match current_block_47 {
        10597256872703347551 => {
            *d.offset((18 as i32 - 1 as i32) as isize) = *s
                .offset((18 as i32 - 1 as i32) as isize);
            current_block_47 = 16368336330394982322;
        }
        _ => {}
    }
    match current_block_47 {
        16368336330394982322 => {
            *d.offset((17 as i32 - 1 as i32) as isize) = *s
                .offset((17 as i32 - 1 as i32) as isize);
            current_block_47 = 9630013451124115526;
        }
        _ => {}
    }
    match current_block_47 {
        9630013451124115526 => {
            *d.offset((16 as i32 - 1 as i32) as isize) = *s
                .offset((16 as i32 - 1 as i32) as isize);
            current_block_47 = 14283634218784605551;
        }
        _ => {}
    }
    match current_block_47 {
        14283634218784605551 => {
            *d.offset((15 as i32 - 1 as i32) as isize) = *s
                .offset((15 as i32 - 1 as i32) as isize);
            current_block_47 = 1513176906760517186;
        }
        _ => {}
    }
    match current_block_47 {
        1513176906760517186 => {
            *d.offset((14 as i32 - 1 as i32) as isize) = *s
                .offset((14 as i32 - 1 as i32) as isize);
            current_block_47 = 8451468998463349627;
        }
        _ => {}
    }
    match current_block_47 {
        8451468998463349627 => {
            *d.offset((13 as i32 - 1 as i32) as isize) = *s
                .offset((13 as i32 - 1 as i32) as isize);
            current_block_47 = 9944861384781218757;
        }
        _ => {}
    }
    match current_block_47 {
        9944861384781218757 => {
            *d.offset((12 as i32 - 1 as i32) as isize) = *s
                .offset((12 as i32 - 1 as i32) as isize);
            current_block_47 = 11763560597460547600;
        }
        _ => {}
    }
    match current_block_47 {
        11763560597460547600 => {
            *d.offset((11 as i32 - 1 as i32) as isize) = *s
                .offset((11 as i32 - 1 as i32) as isize);
            current_block_47 = 15461180760796017574;
        }
        _ => {}
    }
    match current_block_47 {
        15461180760796017574 => {
            *d.offset((10 as i32 - 1 as i32) as isize) = *s
                .offset((10 as i32 - 1 as i32) as isize);
            current_block_47 = 1083807695818635084;
        }
        _ => {}
    }
    match current_block_47 {
        1083807695818635084 => {
            *d.offset((9 as i32 - 1 as i32) as isize) = *s
                .offset((9 as i32 - 1 as i32) as isize);
            current_block_47 = 10234853314349311949;
        }
        _ => {}
    }
    match current_block_47 {
        10234853314349311949 => {
            *d.offset((8 as i32 - 1 as i32) as isize) = *s
                .offset((8 as i32 - 1 as i32) as isize);
            current_block_47 = 4192008369500975849;
        }
        _ => {}
    }
    match current_block_47 {
        4192008369500975849 => {
            *d.offset((7 as i32 - 1 as i32) as isize) = *s
                .offset((7 as i32 - 1 as i32) as isize);
            current_block_47 = 302541485824227200;
        }
        _ => {}
    }
    match current_block_47 {
        302541485824227200 => {
            *d.offset((6 as i32 - 1 as i32) as isize) = *s
                .offset((6 as i32 - 1 as i32) as isize);
            current_block_47 = 17931195545865348945;
        }
        _ => {}
    }
    match current_block_47 {
        17931195545865348945 => {
            *d.offset((5 as i32 - 1 as i32) as isize) = *s
                .offset((5 as i32 - 1 as i32) as isize);
            current_block_47 = 12533541166356859325;
        }
        _ => {}
    }
    match current_block_47 {
        12533541166356859325 => {
            *d.offset((4 as i32 - 1 as i32) as isize) = *s
                .offset((4 as i32 - 1 as i32) as isize);
            current_block_47 = 14597459921621826422;
        }
        _ => {}
    }
    match current_block_47 {
        14597459921621826422 => {
            *d.offset((3 as i32 - 1 as i32) as isize) = *s
                .offset((3 as i32 - 1 as i32) as isize);
            current_block_47 = 2597896670383610064;
        }
        _ => {}
    }
    match current_block_47 {
        2597896670383610064 => {
            *d.offset((2 as i32 - 1 as i32) as isize) = *s
                .offset((2 as i32 - 1 as i32) as isize);
            current_block_47 = 14613590046668411832;
        }
        _ => {}
    }
    match current_block_47 {
        14613590046668411832 => {
            *d.offset((1 as i32 - 1 as i32) as isize) = *s
                .offset((1 as i32 - 1 as i32) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut zahl) -> i32 {
    return ((*a).sign == 0) as i32;
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> i32 {
    return (*a).sign;
}
#[no_mangle]
pub unsafe extern "C" fn zlsh(mut a: *mut zahl, mut b: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    let mut carry: zahl_char_t = 0 as i32 as zahl_char_t;
    let mut tcarry: zahl_char_t = 0;
    if (zzero(b) != 0) as i32 as i64 != 0 {
        (*a).sign = 0 as i32;
        return;
    }
    chars = bits >> 6 as i32;
    bits = bits & (64 as i32 - 1 as i32) as u64;
    cbits = (64 as i32 as u64).wrapping_sub(bits);
    if (*a).alloced < ((*b).used).wrapping_add(chars).wrapping_add(1 as i32 as u64) {
        libzahl_realloc(
            a,
            ((*b).used).wrapping_add(chars).wrapping_add(1 as i32 as u64),
        );
    }
    if (a == b) as i32 as i64 != 0 {
        libzahl_memmoveb(((*a).chars).offset(chars as isize), (*b).chars, (*b).used);
    } else {
        libzahl_memcpy(((*a).chars).offset(chars as isize), (*b).chars, (*b).used);
    }
    libzahl_memset_precise((*a).chars, 0 as i32 as zahl_char_t, chars);
    (*a).used = ((*b).used).wrapping_add(chars);
    if (bits != 0) as i32 as i64 != 0 {
        i = chars;
        while i < (*a).used {
            tcarry = *((*a).chars).offset(i as isize) >> cbits;
            *((*a).chars).offset(i as isize) <<= bits;
            let ref mut fresh0 = *((*a).chars).offset(i as isize);
            *fresh0 |= carry;
            carry = tcarry;
            i = i.wrapping_add(1);
            i;
        }
        if carry != 0 {
            let fresh1 = (*a).used;
            (*a).used = ((*a).used).wrapping_add(1);
            *((*a).chars).offset(fresh1 as isize) = carry;
        }
    }
    (*a).sign = zsignum(b);
}