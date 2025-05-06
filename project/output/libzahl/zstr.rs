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
    fn zfree(_: *mut zahl);
    fn zdivmod(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    static mut libzahl_temp_allocation: *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __errno_location() -> *mut i32;
    static mut libzahl_jmp_buf: jmp_buf;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_error: i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
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
unsafe extern "C" fn libzahl_memfailure() {
    if *__errno_location() == 0 {
        *__errno_location() = 2 as i32;
    }
    libzahl_failure(*__errno_location());
}
unsafe extern "C" fn libzahl_failure(mut error: i32) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as i32);
}
#[inline]
unsafe extern "C" fn sprintint_fix(mut buf: *mut i8, mut v: zahl_char_t) {
    let mut partials: *const i8 = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\0"
        as *const u8 as *const i8;
    let mut buffer: *mut uint16_t = buf.offset(1 as i32 as isize) as *mut uint16_t;
    *buffer.offset(8 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(7 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(6 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(5 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(4 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(3 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(2 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(1 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buffer.offset(0 as i32 as isize) = *(partials
        .offset(
            (2 as i32 as u64).wrapping_mul(v.wrapping_rem(100 as i32 as u64)) as isize,
        ) as *const uint16_t);
    v = (v as u64).wrapping_div(100 as i32 as u64) as zahl_char_t as zahl_char_t;
    *buf = ('0' as i32 as u64).wrapping_add(v) as i8;
    *buf.offset(19 as i32 as isize) = 0 as i32 as i8;
}
#[inline]
unsafe extern "C" fn cmemmove(mut d: *mut i8, mut s: *const i8, mut n: i64) {
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0) {
            break;
        }
        let fresh1 = s;
        s = s.offset(1);
        let fresh2 = d;
        d = d.offset(1);
        *fresh2 = *fresh1;
    };
}
#[inline]
unsafe extern "C" fn sprintint_min(mut buf: *mut i8, mut v: zahl_char_t) -> size_t {
    let mut i: i64 = 0 as i32 as i64;
    let mut j: i64 = 0;
    sprintint_fix(buf, v);
    while *buf.offset(i as isize) as i32 == '0' as i32 {
        i += 1;
        i;
    }
    j = 19 as i32 as i64 - i;
    cmemmove(buf, buf.offset(i as isize), j);
    *buf.offset(j as isize) = 0 as i32 as i8;
    return j as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn zstr(
    mut a: *mut zahl,
    mut b: *mut i8,
    mut n: size_t,
) -> *mut i8 {
    let mut buf: [i8; 20] = [0; 20];
    let mut len: size_t = 0;
    let mut neg: size_t = 0;
    let mut last: size_t = 0;
    let mut tot: size_t = 0 as i32 as size_t;
    let mut overridden: i8 = 0 as i32 as i8;
    if (zzero(a) != 0) as i32 as i64 != 0 {
        if b.is_null() as i32 as i64 != 0
            && {
                b = malloc(2 as i32 as u64) as *mut i8;
                b.is_null() as i32 as i64 != 0
            }
        {
            libzahl_memfailure();
        }
        *b.offset(0 as i32 as isize) = '0' as i32 as i8;
        *b.offset(1 as i32 as isize) = 0 as i32 as i8;
        return b;
    }
    if n == 0 {
        n = ((20 as i32 * 64 as i32 / 64 as i32 + (64 as i32 == 8 as i32) as i32) as u64)
            .wrapping_mul((*a).used);
    }
    if b.is_null() as i32 as i64 != 0
        && {
            libzahl_temp_allocation = malloc(n.wrapping_add(1 as i32 as u64));
            b = libzahl_temp_allocation as *mut i8;
            b.is_null() as i32 as i64 != 0
        }
    {
        libzahl_memfailure();
    }
    neg = (zsignum(a) < 0 as i32) as i32 as size_t;
    zabs(libzahl_tmp_str_num.as_mut_ptr(), a);
    *b.offset(0 as i32 as isize) = '-' as i32 as i8;
    b = b.offset(neg as isize);
    n = (n as u64).wrapping_sub(neg) as size_t as size_t;
    last = n;
    n = if last > 19 as i32 as u64 {
        n.wrapping_sub(19 as i32 as u64)
    } else {
        0 as i32 as u64
    };
    loop {
        zdivmod(
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_tmp_str_rem.as_mut_ptr(),
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_const_1e19.as_mut_ptr(),
        );
        if (zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0) as i32 as i64 != 0 {
            sprintint_fix(
                b.offset(n as isize),
                if zzero(libzahl_tmp_str_rem.as_mut_ptr()) != 0 {
                    0 as i32 as u64
                } else {
                    *((*libzahl_tmp_str_rem.as_mut_ptr()).chars)
                        .offset(0 as i32 as isize)
                },
            );
            *b.offset(n.wrapping_add(19 as i32 as u64) as isize) = overridden;
            overridden = *b.offset(n as isize);
            last = n;
            n = if last > 19 as i32 as u64 {
                n.wrapping_sub(19 as i32 as u64)
            } else {
                0 as i32 as u64
            };
            tot = (tot as u64).wrapping_add(19 as i32 as u64) as size_t as size_t;
        } else {
            len = sprintint_min(
                buf.as_mut_ptr(),
                *((*libzahl_tmp_str_rem.as_mut_ptr()).chars).offset(0 as i32 as isize),
            );
            if tot != 0 {
                memcpy(
                    b as *mut libc::c_void,
                    buf.as_mut_ptr() as *const libc::c_void,
                    len,
                );
                memmove(
                    b.offset(len as isize) as *mut libc::c_void,
                    b.offset(last as isize) as *const libc::c_void,
                    tot.wrapping_add(1 as i32 as u64),
                );
            } else {
                memcpy(
                    b as *mut libc::c_void,
                    buf.as_mut_ptr() as *const libc::c_void,
                    len.wrapping_add(1 as i32 as u64),
                );
            }
            break;
        }
    }
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    return b.offset(-(neg as isize));
}