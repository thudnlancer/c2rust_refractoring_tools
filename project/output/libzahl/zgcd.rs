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
    fn zsub_positive_assign(_: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    static mut libzahl_tmp_gcd_v: z_t;
    static mut libzahl_tmp_gcd_u: z_t;
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
unsafe extern "C" fn zcmpmag(mut a: *mut zahl, mut b: *mut zahl) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a) != 0) as i32 as i64 != 0 {
        return -((zzero(b) == 0) as i32);
    }
    if (zzero(b) != 0) as i32 as i64 != 0 {
        return 1 as i32;
    }
    i = ((*a).used).wrapping_sub(1 as i32 as u64);
    j = ((*b).used).wrapping_sub(1 as i32 as u64);
    while i > j {
        if *((*a).chars).offset(i as isize) != 0 {
            return 1 as i32;
        }
        (*a).used = ((*a).used).wrapping_sub(1);
        (*a).used;
        i = i.wrapping_sub(1);
        i;
    }
    while j > i {
        if *((*b).chars).offset(j as isize) != 0 {
            return -(1 as i32);
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
        -(1 as i32)
    } else {
        (*((*a).chars).offset(i as isize) > *((*b).chars).offset(i as isize)) as i32
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> i32 {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zlsb(mut a: *mut zahl) -> size_t {
    let mut i: size_t = 0 as i32 as size_t;
    if (zzero(a) != 0) as i32 as i64 != 0 {
        return 18446744073709551615 as u64;
    }
    while *((*a).chars).offset(i as isize) == 0 {
        i = i.wrapping_add(1);
        i;
    }
    i = (i as u64)
        .wrapping_mul(
            (8 as i32 as u64).wrapping_mul(::core::mem::size_of::<zahl_char_t>() as u64),
        ) as size_t as size_t;
    i = (i as u64)
        .wrapping_add(
            (*((*a).chars).offset(i as isize) as libc::c_ulonglong).trailing_zeros()
                as i32 as size_t,
        ) as size_t as size_t;
    return i;
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
#[inline]
unsafe extern "C" fn zrsh_taint(mut a: *mut zahl, mut bits: size_t) {
    let mut i: size_t = 0;
    let mut chars: size_t = 0;
    let mut cbits: size_t = 0;
    if (bits == 0) as i32 as i64 != 0 {
        return;
    }
    if (zzero(a) != 0) as i32 as i64 != 0 {
        return;
    }
    chars = bits >> 6 as i32;
    if (chars >= (*a).used || zbits(a) <= bits) as i32 as i64 != 0 {
        (*a).sign = 0 as i32;
        return;
    }
    bits = bits & (64 as i32 - 1 as i32) as u64;
    cbits = (64 as i32 as u64).wrapping_sub(bits);
    if (chars != 0) as i32 as i64 != 0 {
        (*a).used = ((*a).used as u64).wrapping_sub(chars) as size_t as size_t;
        (*a).chars = ((*a).chars).offset(chars as isize);
    }
    if (bits != 0) as i32 as i64 != 0 {
        *((*a).chars).offset(0 as i32 as isize) >>= bits;
        i = 1 as i32 as size_t;
        while i < (*a).used {
            let ref mut fresh0 = *((*a).chars)
                .offset(i.wrapping_sub(1 as i32 as u64) as isize);
            *fresh0 |= *((*a).chars).offset(i as isize) << cbits;
            *((*a).chars).offset(i as isize) >>= bits;
            i = i.wrapping_add(1);
            i;
        }
        while *((*a).chars).offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize)
            == 0
        {
            (*a).used = ((*a).used).wrapping_sub(1);
            (*a).used;
        }
    }
}
#[inline]
unsafe extern "C" fn zswap_tainted_unsigned(mut a: *mut zahl, mut b: *mut zahl) {
    let mut t: z_t = [zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    (*t.as_mut_ptr()).used = (*a).used;
    (*a).used = (*b).used;
    (*b).used = (*t.as_mut_ptr()).used;
    let ref mut fresh1 = (*t.as_mut_ptr()).chars;
    *fresh1 = (*b).chars;
    (*b).chars = (*a).chars;
    (*a).chars = (*t.as_mut_ptr()).chars;
}
#[no_mangle]
pub unsafe extern "C" fn zgcd(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut shifts: size_t = 0;
    let mut u_orig: *mut zahl_char_t = 0 as *mut zahl_char_t;
    let mut v_orig: *mut zahl_char_t = 0 as *mut zahl_char_t;
    let mut u_lsb: size_t = 0;
    let mut v_lsb: size_t = 0;
    let mut neg: i32 = 0;
    let mut cmpmag: i32 = 0;
    if (zzero(b) != 0) as i32 as i64 != 0 {
        if a != c {
            zset(a, c);
        }
        return;
    }
    if (zzero(c) != 0) as i32 as i64 != 0 {
        if a != b {
            zset(a, b);
        }
        return;
    }
    neg = (zsignum(b) & zsignum(c) < 0 as i32) as i32;
    u_lsb = zlsb(b);
    v_lsb = zlsb(c);
    shifts = if u_lsb < v_lsb { u_lsb } else { v_lsb };
    zrsh(libzahl_tmp_gcd_u.as_mut_ptr(), b, u_lsb);
    zrsh(libzahl_tmp_gcd_v.as_mut_ptr(), c, v_lsb);
    u_orig = (*libzahl_tmp_gcd_u.as_mut_ptr()).chars;
    v_orig = (*libzahl_tmp_gcd_v.as_mut_ptr()).chars;
    loop {
        cmpmag = zcmpmag(libzahl_tmp_gcd_u.as_mut_ptr(), libzahl_tmp_gcd_v.as_mut_ptr());
        if (cmpmag >= 0 as i32) as i32 as i64 != 0 {
            if (cmpmag == 0 as i32) as i32 as i64 != 0 {
                break;
            }
            zswap_tainted_unsigned(
                libzahl_tmp_gcd_u.as_mut_ptr(),
                libzahl_tmp_gcd_v.as_mut_ptr(),
            );
        }
        zsub_positive_assign(
            libzahl_tmp_gcd_v.as_mut_ptr(),
            libzahl_tmp_gcd_u.as_mut_ptr(),
        );
        zrsh_taint(libzahl_tmp_gcd_v.as_mut_ptr(), zlsb(libzahl_tmp_gcd_v.as_mut_ptr()));
    }
    zlsh(a, libzahl_tmp_gcd_u.as_mut_ptr(), shifts);
    (*a).sign = if neg != 0 { -(1 as i32) } else { 1 as i32 };
    let ref mut fresh2 = (*libzahl_tmp_gcd_u.as_mut_ptr()).chars;
    *fresh2 = u_orig;
    let ref mut fresh3 = (*libzahl_tmp_gcd_v.as_mut_ptr()).chars;
    *fresh3 = v_orig;
}