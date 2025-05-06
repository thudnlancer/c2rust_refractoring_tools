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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use core::arch::asm;
extern "C" {
    fn zfree(_: *mut zahl);
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zlsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    fn zbset_ll_clear(_: *mut zahl, _: size_t);
    fn zbset_ll_flip(_: *mut zahl, _: size_t);
    fn zbset_ll_set(_: *mut zahl, _: size_t);
    static mut libzahl_tmp_divmod_ds: [z_t; 64];
    static mut libzahl_tmp_divmod_d: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_temp_allocation: *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_error: i32;
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
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zerror {
    ZERROR_ERRNO_SET = 0,
    ZERROR_0_POW_0,
    ZERROR_0_DIV_0,
    ZERROR_DIV_0,
    ZERROR_NEGATIVE,
    ZERROR_INVALID_RADIX,
}
impl zerror {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zerror::ZERROR_ERRNO_SET => 0,
            zerror::ZERROR_0_POW_0 => 1,
            zerror::ZERROR_0_DIV_0 => 2,
            zerror::ZERROR_DIV_0 => 3,
            zerror::ZERROR_NEGATIVE => 4,
            zerror::ZERROR_INVALID_RADIX => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> zerror {
        match value {
            0 => zerror::ZERROR_ERRNO_SET,
            1 => zerror::ZERROR_0_POW_0,
            2 => zerror::ZERROR_0_DIV_0,
            3 => zerror::ZERROR_DIV_0,
            4 => zerror::ZERROR_NEGATIVE,
            5 => zerror::ZERROR_INVALID_RADIX,
            _ => panic!("Invalid value for zerror: {}", value),
        }
    }
}
impl AddAssign<u32> for zerror {
    fn add_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zerror {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zerror {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zerror {
    fn div_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zerror {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zerror::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zerror {
    type Output = zerror;
    fn add(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zerror {
    type Output = zerror;
    fn sub(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zerror {
    type Output = zerror;
    fn mul(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zerror {
    type Output = zerror;
    fn div(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zerror {
    type Output = zerror;
    fn rem(self, rhs: u32) -> zerror {
        zerror::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
unsafe extern "C" fn zswap(mut a_: *mut zahl, mut b_: *mut zahl) {
    let mut t: i64 = 0;
    let mut a: *mut i64 = a_ as *mut i64;
    let mut b: *mut i64 = b_ as *mut i64;
    t = *a.offset(0 as i32 as isize);
    *a.offset(0 as i32 as isize) = *b.offset(0 as i32 as isize);
    *b.offset(0 as i32 as isize) = t;
    t = *b.offset(1 as i32 as isize);
    *b.offset(1 as i32 as isize) = *a.offset(1 as i32 as isize);
    *a.offset(1 as i32 as isize) = t;
    t = *a.offset(2 as i32 as isize);
    *a.offset(2 as i32 as isize) = *b.offset(2 as i32 as isize);
    *b.offset(2 as i32 as isize) = t;
    t = *b.offset(3 as i32 as isize);
    *b.offset(3 as i32 as isize) = *a.offset(3 as i32 as isize);
    *a.offset(3 as i32 as isize) = t;
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
unsafe extern "C" fn zsetu(mut a: *mut zahl, mut b: uint64_t) {
    if (b == 0) as i32 as i64 != 0 {
        (*a).sign = 0 as i32;
        return;
    }
    if (*a).alloced < 1 as i32 as u64 {
        libzahl_realloc(a, 1 as i32 as size_t);
    }
    (*a).sign = 1 as i32;
    *((*a).chars).offset(0 as i32 as isize) = b;
    (*a).used = 1 as i32 as size_t;
}
#[inline]
unsafe extern "C" fn zseti(mut a: *mut zahl, mut b: int64_t) {
    if (b >= 0 as i32 as i64) as i32 as i64 != 0 {
        zsetu(a, b as uint64_t);
        return;
    }
    if (*a).alloced < 1 as i32 as u64 {
        libzahl_realloc(a, 1 as i32 as size_t);
    }
    (*a).sign = -(1 as i32);
    *((*a).chars).offset(0 as i32 as isize) = -b as zahl_char_t;
    (*a).used = 1 as i32 as size_t;
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
unsafe extern "C" fn zabs(mut a: *mut zahl, mut b: *mut zahl) {
    if a != b {
        zset(a, b);
    }
    (*a).sign &= 1 as i32;
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
unsafe extern "C" fn zbset(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut bit: size_t,
    mut action: i32,
) {
    if (a != b) as i32 as i64 != 0 {
        zset(a, b);
    }
    if 0 != 0 && 0 != 0 {
        let mut mask: zahl_char_t = 1 as i32 as zahl_char_t;
        if zzero(a) != 0 || bit >> 6 as i32 >= (*a).used {
            if action == 0 {
                return;
            }
        } else {
            mask <<= bit & (64 as i32 - 1 as i32) as u64;
            if action > 0 as i32 {
                let ref mut fresh1 = *((*a).chars).offset((bit >> 6 as i32) as isize);
                *fresh1 |= mask;
                return;
            } else if action < 0 as i32 {
                let ref mut fresh2 = *((*a).chars).offset((bit >> 6 as i32) as isize);
                *fresh2 ^= mask;
            } else {
                let ref mut fresh3 = *((*a).chars).offset((bit >> 6 as i32) as isize);
                *fresh3 &= !mask;
            }
            while (*a).used != 0
                && *((*a).chars)
                    .offset(((*a).used).wrapping_sub(1 as i32 as u64) as isize) == 0
            {
                (*a).used = ((*a).used).wrapping_sub(1);
                (*a).used;
            }
            if (*a).used == 0 {
                (*a).sign = 0 as i32;
            }
            return;
        }
    }
    if action > 0 as i32 {
        zbset_ll_set(a, bit);
    } else if action < 0 as i32 {
        zbset_ll_flip(a, bit);
    } else {
        zbset_ll_clear(a, bit);
    };
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
unsafe extern "C" fn zdivmod_impl(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
    mut d: *mut zahl,
) {
    let mut c_bits: size_t = 0;
    let mut d_bits: size_t = 0;
    let mut bit: size_t = 0;
    let mut i: size_t = 0;
    static mut tds: [z_t; 64] = [[zahl {
        sign: 0,
        padding__: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1]; 64];
    c_bits = zbits(c);
    d_bits = zbits(d);
    bit = c_bits.wrapping_sub(d_bits);
    zlsh(libzahl_tmp_divmod_d.as_mut_ptr(), d, bit);
    (*libzahl_tmp_divmod_d.as_mut_ptr()).sign = 1 as i32;
    if zcmpmag(libzahl_tmp_divmod_d.as_mut_ptr(), c) > 0 as i32 {
        zrsh(
            libzahl_tmp_divmod_d.as_mut_ptr(),
            libzahl_tmp_divmod_d.as_mut_ptr(),
            1 as i32 as size_t,
        );
        bit = (bit as u64).wrapping_sub(1 as i32 as u64) as size_t as size_t;
    }
    (*libzahl_tmp_divmod_a.as_mut_ptr()).sign = 0 as i32;
    zabs(libzahl_tmp_divmod_b.as_mut_ptr(), c);
    if (bit <= 64 as i32 as u64) as i32 as i64 != 0 {
        loop {
            if zcmpmag(
                libzahl_tmp_divmod_d.as_mut_ptr(),
                libzahl_tmp_divmod_b.as_mut_ptr(),
            ) <= 0 as i32
            {
                zsub_unsigned(
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                    libzahl_tmp_divmod_d.as_mut_ptr(),
                );
                zbset(
                    libzahl_tmp_divmod_a.as_mut_ptr(),
                    libzahl_tmp_divmod_a.as_mut_ptr(),
                    bit,
                    1 as i32,
                );
            }
            let fresh4 = bit;
            bit = bit.wrapping_sub(1);
            if fresh4 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                break;
            }
            zrsh(
                libzahl_tmp_divmod_d.as_mut_ptr(),
                libzahl_tmp_divmod_d.as_mut_ptr(),
                1 as i32 as size_t,
            );
        }
    } else {
        i = 0 as i32 as size_t;
        while i < 64 as i32 as u64 {
            zrsh(
                (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                libzahl_tmp_divmod_d.as_mut_ptr(),
                i,
            );
            (*(tds[i as usize]).as_mut_ptr()).used = (*(libzahl_tmp_divmod_ds[i
                as usize])
                .as_mut_ptr())
                .used;
            (*(tds[i as usize]).as_mut_ptr()).sign = (*(libzahl_tmp_divmod_ds[i
                as usize])
                .as_mut_ptr())
                .sign;
            let ref mut fresh5 = (*(tds[i as usize]).as_mut_ptr()).chars;
            *fresh5 = (*(libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr()).chars;
            i = i.wrapping_add(1);
            i;
        }
        's_114: loop {
            i = 0 as i32 as size_t;
            while i < 64 as i32 as u64 {
                if zcmpmag(
                    (tds[i as usize]).as_mut_ptr(),
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                ) <= 0 as i32
                {
                    zsub_unsigned(
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        (tds[i as usize]).as_mut_ptr(),
                    );
                    zbset(
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        bit,
                        1 as i32,
                    );
                }
                let fresh6 = bit;
                bit = bit.wrapping_sub(1);
                if fresh6 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                    break 's_114;
                }
                i = i.wrapping_add(1);
                i;
            }
            i = (if bit < (64 as i32 - 1 as i32) as u64 {
                bit
            } else {
                (64 as i32 - 1 as i32) as u64
            })
                .wrapping_add(1 as i32 as u64);
            loop {
                let fresh7 = i;
                i = i.wrapping_sub(1);
                if !(fresh7 != 0) {
                    break;
                }
                zrsh_taint((tds[i as usize]).as_mut_ptr(), 64 as i32 as size_t);
            }
        }
    }
    zswap(a, libzahl_tmp_divmod_a.as_mut_ptr());
    zswap(b, libzahl_tmp_divmod_b.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn zdivmod(
    mut a: *mut zahl,
    mut b: *mut zahl,
    mut c: *mut zahl,
    mut d: *mut zahl,
) {
    let mut c_sign: i32 = 0;
    let mut sign: i32 = 0;
    let mut cmpmag: i32 = 0;
    c_sign = zsignum(c);
    sign = c_sign * zsignum(d);
    if (sign == 0) as i32 as i64 != 0 {
        if (zzero(c) == 0) as i32 as i64 != 0 {
            libzahl_failure(-(zerror::ZERROR_DIV_0 as i32));
        } else if (zzero(d) != 0) as i32 as i64 != 0 {
            libzahl_failure(-(zerror::ZERROR_0_DIV_0 as i32));
        } else {
            (*a).sign = 0 as i32;
            (*b).sign = 0 as i32;
        }
        return;
    } else {
        cmpmag = zcmpmag(c, d);
        if (cmpmag <= 0 as i32) as i32 as i64 != 0 {
            if (cmpmag == 0 as i32) as i32 as i64 != 0 {
                zseti(a, sign as int64_t);
                (*b).sign = 0 as i32;
            } else {
                if b != c {
                    zset(b, c);
                }
                (*a).sign = 0 as i32;
            }
            return;
        }
    }
    zdivmod_impl(a, b, c, d);
    (*a).sign = sign;
    if zsignum(b) > 0 as i32 {
        (*b).sign = c_sign;
    }
}