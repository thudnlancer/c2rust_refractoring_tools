#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(asm)]
use core::arch::asm;
extern "C" {
    fn zfree(_: *mut zahl);
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsqr_ll(_: *mut zahl, _: *mut zahl);
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_temp_allocation: *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_error: libc::c_int;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zerror {
    ZERROR_INVALID_RADIX = 5,
    ZERROR_NEGATIVE = 4,
    ZERROR_DIV_0 = 3,
    ZERROR_0_DIV_0 = 2,
    ZERROR_0_POW_0 = 1,
    ZERROR_ERRNO_SET = 0,
}  // end of enum

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
unsafe extern "C" fn zsetu(mut a: *mut zahl, mut b: uint64_t) {
    if (b == 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    if (*a).alloced < 1 as libc::c_int as libc::c_ulong {
        libzahl_realloc(a, 1 as libc::c_int as size_t);
    }
    (*a).sign = 1 as libc::c_int;
    *((*a).chars).offset(0 as libc::c_int as isize) = b;
    (*a).used = 1 as libc::c_int as size_t;
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
unsafe extern "C" fn libzahl_failure(mut error: libc::c_int) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zpowu(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: libc::c_ulonglong,
) {
    let mut neg: libc::c_int = 0;
    if (c == 0) as libc::c_int as libc::c_long != 0 {
        if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
            libzahl_failure(-(ZERROR_0_POW_0 as libc::c_int));
        }
        zsetu(a, 1 as libc::c_int as uint64_t);
        return;
    } else if (zzero(b) != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
        return;
    }
    neg = (zsignum(b) < 0 as libc::c_int
        && c & 1 as libc::c_int as libc::c_ulonglong != 0) as libc::c_int;
    zabs(libzahl_tmp_pow_b.as_mut_ptr(), b);
    if c & 1 as libc::c_int as libc::c_ulonglong != 0 {
        zset(a, libzahl_tmp_pow_b.as_mut_ptr());
    } else {
        zsetu(a, 1 as libc::c_int as uint64_t);
    }
    loop {
        c >>= 1 as libc::c_int;
        if !(c != 0) {
            break;
        }
        zsqr_ll(libzahl_tmp_pow_b.as_mut_ptr(), libzahl_tmp_pow_b.as_mut_ptr());
        if c & 1 as libc::c_int as libc::c_ulonglong != 0 {
            zmul_ll(a, a, libzahl_tmp_pow_b.as_mut_ptr());
        }
    }
    if neg != 0 {
        zneg(a, a);
    }
}
