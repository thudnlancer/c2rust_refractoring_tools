use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use core::arch::asm;
extern "C" {
    fn zfree(_: *mut zahl);
    static mut libzahl_tmp_div: [zahl; 1];
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zdivmod(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsqr_ll(_: *mut zahl, _: *mut zahl);
    static mut libzahl_tmp_str_div: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_tmp_str_mag: z_t;
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
unsafe extern "C" fn zdiv(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    zdivmod(a, libzahl_tmp_div.as_mut_ptr(), b, c);
}
#[inline]
unsafe extern "C" fn zsqr(mut a: *mut zahl, mut b: *mut zahl) {
    if (zzero(b) != 0) as i32 as i64 != 0 {
        (*a).sign = 0 as i32;
    } else {
        zsqr_ll(a, b);
        (*a).sign = 1 as i32;
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
#[no_mangle]
pub unsafe extern "C" fn zstr_length(
    mut a: *mut zahl,
    mut radix: libc::c_ulonglong,
) -> size_t {
    let mut size_total: size_t = 1 as i32 as size_t;
    let mut size_temp: size_t = 0;
    if (radix < 2 as i32 as libc::c_ulonglong) as i32 as i64 != 0 {
        libzahl_failure(-(zerror::ZERROR_INVALID_RADIX as i32));
    }
    zset(libzahl_tmp_str_num.as_mut_ptr(), a);
    while zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0 {
        zsetu(libzahl_tmp_str_mag.as_mut_ptr(), radix as uint64_t);
        zset(libzahl_tmp_str_div.as_mut_ptr(), libzahl_tmp_str_mag.as_mut_ptr());
        size_temp = 1 as i32 as size_t;
        while zcmpmag(libzahl_tmp_str_mag.as_mut_ptr(), libzahl_tmp_str_num.as_mut_ptr())
            <= 0 as i32
        {
            zset(libzahl_tmp_str_div.as_mut_ptr(), libzahl_tmp_str_mag.as_mut_ptr());
            zsqr(libzahl_tmp_str_mag.as_mut_ptr(), libzahl_tmp_str_mag.as_mut_ptr());
            size_temp <<= 1 as i32;
        }
        size_temp >>= 1 as i32;
        size_total = (size_total as u64).wrapping_add(size_temp) as size_t as size_t;
        zdiv(
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_tmp_str_num.as_mut_ptr(),
            libzahl_tmp_str_div.as_mut_ptr(),
        );
    }
    return size_total.wrapping_add((zsignum(a) < 0 as i32) as i32 as u64);
}