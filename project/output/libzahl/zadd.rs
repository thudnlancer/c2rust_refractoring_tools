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
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
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
unsafe extern "C" fn zabs(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign &= 1 as i32;
}
#[inline]
unsafe extern "C" fn zadd_impl_4(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
    mut n: size_t,
) {
    let mut ac: *mut zahl_char_t = (*a).chars;
    let mut bc: *mut zahl_char_t = (*b).chars;
    let mut cc: *mut zahl_char_t = (*c).chars;
    let mut carry: zahl_char_t = 0 as i32 as zahl_char_t;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    loop {
        ac = ac.offset(4 as i32 as isize);
        bc = bc.offset(4 as i32 as isize);
        cc = cc.offset(4 as i32 as isize);
        i = (i as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
        if !(i <= n) {
            break;
        }
        asm!(
            "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq -32({3}), {0}\n    movq {0}, -32({1})\n    movq -24({2}), {0}\n    adcq -24({3}), {0}\n    movq {0}, -24({1})\n    movq -16({2}), {0}\n    adcq -16({3}), {0}\n    movq {0}, -16({1})\n    movq -8({2}), {0}\n    adcq -8({3}), {0}\n    movq {0}, -8({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
            inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc, inlateout(reg)
            cc, options(preserves_flags, att_syntax)
        );
    }
    match n & 3 as i32 as u64 {
        3 => {
            asm!(
                "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq -32({3}), {0}\n    movq {0}, -32({1})\n    movq -24({2}), {0}\n    adcq -24({3}), {0}\n    movq {0}, -24({1})\n    movq -16({2}), {0}\n    adcq -16({3}), {0}\n    movq {0}, -16({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                inlateout(reg) cc, options(preserves_flags, att_syntax)
            );
        }
        2 => {
            asm!(
                "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq -32({3}), {0}\n    movq {0}, -32({1})\n    movq -24({2}), {0}\n    adcq -24({3}), {0}\n    movq {0}, -24({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                inlateout(reg) cc, options(preserves_flags, att_syntax)
            );
        }
        1 => {
            asm!(
                "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq -32({3}), {0}\n    movq {0}, -32({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                inlateout(reg) cc, options(preserves_flags, att_syntax)
            );
        }
        _ => {}
    }
    i = n;
    while carry != 0 {
        let (fresh0, fresh1) = (*((*a).chars).offset(i as isize))
            .overflowing_add(1 as i32 as u64);
        *((*a).chars).offset(i as isize) = fresh0;
        carry = fresh1 as zahl_char_t;
        i = i.wrapping_add(1);
        i;
    }
    if (*a).used < i {
        (*a).used = i;
    }
}
#[inline]
unsafe extern "C" fn zadd_impl_3(mut a: *mut zahl, mut b: *mut zahl, mut n: size_t) {
    let mut ac: *mut zahl_char_t = (*a).chars;
    let mut bc: *mut zahl_char_t = (*b).chars;
    let mut carry: zahl_char_t = 0 as i32 as zahl_char_t;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    loop {
        ac = ac.offset(4 as i32 as isize);
        bc = bc.offset(4 as i32 as isize);
        i = (i as u64).wrapping_add(4 as i32 as u64) as size_t as size_t;
        if !(i <= n) {
            break;
        }
        asm!(
            "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq {0}, -32({1})\n    movq -24({2}), {0}\n    adcq {0}, -24({1})\n    movq -16({2}), {0}\n    adcq {0}, -16({1})\n    movq -8({2}), {0}\n    adcq {0}, -8({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
            inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
            options(preserves_flags, att_syntax)
        );
    }
    match n & 3 as i32 as u64 {
        3 => {
            asm!(
                "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq {0}, -32({1})\n    movq -24({2}), {0}\n    adcq {0}, -24({1})\n    movq -16({2}), {0}\n    adcq {0}, -16({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                options(preserves_flags, att_syntax)
            );
        }
        2 => {
            asm!(
                "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq {0}, -32({1})\n    movq -24({2}), {0}\n    adcq {0}, -24({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                options(preserves_flags, att_syntax)
            );
        }
        1 => {
            asm!(
                "\n    addq $-1, {0}\n    movq -32({2}), {0}\n    adcq {0}, -32({1})\n    movq $1, {0}\n    jc 1f\n    movq $0, {0}\n 1:",
                inlateout(reg) carry, inlateout(reg) ac, inlateout(reg) bc,
                options(preserves_flags, att_syntax)
            );
        }
        _ => {}
    }
    i = n;
    while carry != 0 {
        let (fresh2, fresh3) = (*((*a).chars).offset(i as isize))
            .overflowing_add(1 as i32 as u64);
        *((*a).chars).offset(i as isize) = fresh2;
        carry = fresh3 as zahl_char_t;
        i = i.wrapping_add(1);
        i;
    }
    if (*a).used < i {
        (*a).used = i;
    }
}
#[inline]
unsafe extern "C" fn libzahl_zadd_unsigned(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
) {
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    if (zzero(b) != 0) as i32 as i64 != 0 {
        zabs(a, c);
        return;
    } else if (zzero(c) != 0) as i32 as i64 != 0 {
        zabs(a, b);
        return;
    }
    size = if (*b).used > (*c).used { (*b).used } else { (*c).used };
    n = ((*b).used).wrapping_add((*c).used).wrapping_sub(size);
    if (*a).alloced < size.wrapping_add(1 as i32 as u64) {
        libzahl_realloc(a, size.wrapping_add(1 as i32 as u64));
    }
    *((*a).chars).offset(size as isize) = 0 as i32 as zahl_char_t;
    if a == b {
        if (*a).used < (*c).used {
            n = (*c).used;
            libzahl_memset(
                ((*a).chars).offset((*a).used as isize),
                0 as i32 as zahl_char_t,
                n.wrapping_sub((*a).used),
            );
        }
        zadd_impl_3(a, c, n);
    } else if (a == c) as i32 as i64 != 0 {
        if (*a).used < (*b).used {
            n = (*b).used;
            libzahl_memset(
                ((*a).chars).offset((*a).used as isize),
                0 as i32 as zahl_char_t,
                n.wrapping_sub((*a).used),
            );
        }
        zadd_impl_3(a, b, n);
    } else if ((*b).used > (*c).used) as i32 as i64 != 0 {
        libzahl_memcpy(
            ((*a).chars).offset(n as isize),
            ((*b).chars).offset(n as isize),
            size.wrapping_sub(n),
        );
        (*a).used = size;
        zadd_impl_4(a, b, c, n);
    } else {
        libzahl_memcpy(
            ((*a).chars).offset(n as isize),
            ((*c).chars).offset(n as isize),
            size.wrapping_sub(n),
        );
        (*a).used = size;
        zadd_impl_4(a, b, c, n);
    }
    (*a).sign = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn zadd_unsigned(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
) {
    libzahl_zadd_unsigned(a, b, c);
}
#[no_mangle]
pub unsafe extern "C" fn zadd_unsigned_assign(mut a: *mut zahl, mut b: *mut zahl) {
    let mut size: size_t = 0;
    let mut n: size_t = 0;
    if (zzero(a) != 0) as i32 as i64 != 0 {
        zabs(a, b);
        return;
    } else if (zzero(b) != 0) as i32 as i64 != 0 {
        return
    }
    size = if (*a).used > (*b).used { (*a).used } else { (*b).used };
    n = ((*a).used).wrapping_add((*b).used).wrapping_sub(size);
    if (*a).alloced < size.wrapping_add(1 as i32 as u64) {
        libzahl_realloc(a, size.wrapping_add(1 as i32 as u64));
    }
    *((*a).chars).offset(size as isize) = 0 as i32 as zahl_char_t;
    if (*a).used < (*b).used {
        n = (*b).used;
        libzahl_memset(
            ((*a).chars).offset((*a).used as isize),
            0 as i32 as zahl_char_t,
            n.wrapping_sub((*a).used),
        );
    }
    zadd_impl_3(a, b, n);
    (*a).sign = 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn zadd(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    if (zzero(b) != 0) as i32 as i64 != 0 {
        if a != c {
            zset(a, c);
        }
    } else if (zzero(c) != 0) as i32 as i64 != 0 {
        if a != b {
            zset(a, b);
        }
    } else if (zsignum(b) < 0 as i32) as i32 as i64 != 0 {
        if zsignum(c) < 0 as i32 {
            libzahl_zadd_unsigned(a, b, c);
            (*a).sign = -zsignum(a);
        } else {
            zsub_unsigned(a, c, b);
        }
    } else if (zsignum(c) < 0 as i32) as i32 as i64 != 0 {
        zsub_unsigned(a, b, c);
    } else {
        libzahl_zadd_unsigned(a, b, c);
    };
}