use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zmodsqr(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmodpow(_: *mut zahl, _: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zadd_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zsub_unsigned(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zrsh(_: *mut zahl, _: *mut zahl, _: size_t);
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_const_1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_const_2: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    fn zrand(_: *mut zahl, _: zranddev, _: zranddist, _: *mut zahl);
    static mut libzahl_const_4: z_t;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zprimality {
    NONPRIME = 0,
    PROBABLY_PRIME,
    PRIME,
}
impl zprimality {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zprimality::NONPRIME => 0,
            zprimality::PROBABLY_PRIME => 1,
            zprimality::PRIME => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> zprimality {
        match value {
            0 => zprimality::NONPRIME,
            1 => zprimality::PROBABLY_PRIME,
            2 => zprimality::PRIME,
            _ => panic!("Invalid value for zprimality: {}", value),
        }
    }
}
impl AddAssign<u32> for zprimality {
    fn add_assign(&mut self, rhs: u32) {
        *self = zprimality::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zprimality {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zprimality::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zprimality {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zprimality::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zprimality {
    fn div_assign(&mut self, rhs: u32) {
        *self = zprimality::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zprimality {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zprimality::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zprimality {
    type Output = zprimality;
    fn add(self, rhs: u32) -> zprimality {
        zprimality::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zprimality {
    type Output = zprimality;
    fn sub(self, rhs: u32) -> zprimality {
        zprimality::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zprimality {
    type Output = zprimality;
    fn mul(self, rhs: u32) -> zprimality {
        zprimality::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zprimality {
    type Output = zprimality;
    fn div(self, rhs: u32) -> zprimality {
        zprimality::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zprimality {
    type Output = zprimality;
    fn rem(self, rhs: u32) -> zprimality {
        zprimality::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zranddev {
    FAST_RANDOM = 0,
    SECURE_RANDOM,
    DEFAULT_RANDOM,
    FASTEST_RANDOM,
    LIBC_RAND_RANDOM,
    LIBC_RANDOM_RANDOM,
    LIBC_RAND48_RANDOM,
}
impl zranddev {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zranddev::FAST_RANDOM => 0,
            zranddev::SECURE_RANDOM => 1,
            zranddev::DEFAULT_RANDOM => 2,
            zranddev::FASTEST_RANDOM => 3,
            zranddev::LIBC_RAND_RANDOM => 4,
            zranddev::LIBC_RANDOM_RANDOM => 5,
            zranddev::LIBC_RAND48_RANDOM => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> zranddev {
        match value {
            0 => zranddev::FAST_RANDOM,
            1 => zranddev::SECURE_RANDOM,
            2 => zranddev::DEFAULT_RANDOM,
            3 => zranddev::FASTEST_RANDOM,
            4 => zranddev::LIBC_RAND_RANDOM,
            5 => zranddev::LIBC_RANDOM_RANDOM,
            6 => zranddev::LIBC_RAND48_RANDOM,
            _ => panic!("Invalid value for zranddev: {}", value),
        }
    }
}
impl AddAssign<u32> for zranddev {
    fn add_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zranddev {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zranddev {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zranddev {
    fn div_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zranddev {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zranddev::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zranddev {
    type Output = zranddev;
    fn add(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zranddev {
    type Output = zranddev;
    fn sub(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zranddev {
    type Output = zranddev;
    fn mul(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zranddev {
    type Output = zranddev;
    fn div(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zranddev {
    type Output = zranddev;
    fn rem(self, rhs: u32) -> zranddev {
        zranddev::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum zranddist {
    QUASIUNIFORM = 0,
    UNIFORM,
    MODUNIFORM,
}
impl zranddist {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            zranddist::QUASIUNIFORM => 0,
            zranddist::UNIFORM => 1,
            zranddist::MODUNIFORM => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> zranddist {
        match value {
            0 => zranddist::QUASIUNIFORM,
            1 => zranddist::UNIFORM,
            2 => zranddist::MODUNIFORM,
            _ => panic!("Invalid value for zranddist: {}", value),
        }
    }
}
impl AddAssign<u32> for zranddist {
    fn add_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for zranddist {
    fn sub_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for zranddist {
    fn mul_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for zranddist {
    fn div_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for zranddist {
    fn rem_assign(&mut self, rhs: u32) {
        *self = zranddist::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for zranddist {
    type Output = zranddist;
    fn add(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for zranddist {
    type Output = zranddist;
    fn sub(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for zranddist {
    type Output = zranddist;
    fn mul(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for zranddist {
    type Output = zranddist;
    fn div(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for zranddist {
    type Output = zranddist;
    fn rem(self, rhs: u32) -> zranddist {
        zranddist::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d_0: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d_0.offset((20 as i32 - 1 as i32) as isize) = *s
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
                lateout(reg) t, inlateout(reg) d_0, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        17254579315488500390 => {
            *d_0.offset((19 as i32 - 1 as i32) as isize) = *s
                .offset((19 as i32 - 1 as i32) as isize);
            current_block_42 = 16864574447450575806;
        }
        _ => {}
    }
    match current_block_42 {
        16864574447450575806 => {
            *d_0.offset((18 as i32 - 1 as i32) as isize) = *s
                .offset((18 as i32 - 1 as i32) as isize);
            current_block_42 = 4745558310823793980;
        }
        _ => {}
    }
    match current_block_42 {
        4745558310823793980 => {
            *d_0.offset((17 as i32 - 1 as i32) as isize) = *s
                .offset((17 as i32 - 1 as i32) as isize);
            current_block_42 = 3609958504996442327;
        }
        _ => {}
    }
    match current_block_42 {
        3609958504996442327 => {
            *d_0.offset((16 as i32 - 1 as i32) as isize) = *s
                .offset((16 as i32 - 1 as i32) as isize);
            current_block_42 = 7591989803176764182;
        }
        _ => {}
    }
    match current_block_42 {
        7591989803176764182 => {
            *d_0.offset((15 as i32 - 1 as i32) as isize) = *s
                .offset((15 as i32 - 1 as i32) as isize);
            current_block_42 = 18018901114339951972;
        }
        _ => {}
    }
    match current_block_42 {
        18018901114339951972 => {
            *d_0.offset((14 as i32 - 1 as i32) as isize) = *s
                .offset((14 as i32 - 1 as i32) as isize);
            current_block_42 = 12689878998560279573;
        }
        _ => {}
    }
    match current_block_42 {
        12689878998560279573 => {
            *d_0.offset((13 as i32 - 1 as i32) as isize) = *s
                .offset((13 as i32 - 1 as i32) as isize);
            current_block_42 = 5286769970458876423;
        }
        _ => {}
    }
    match current_block_42 {
        5286769970458876423 => {
            *d_0.offset((12 as i32 - 1 as i32) as isize) = *s
                .offset((12 as i32 - 1 as i32) as isize);
            current_block_42 = 7754037464575168929;
        }
        _ => {}
    }
    match current_block_42 {
        7754037464575168929 => {
            *d_0.offset((11 as i32 - 1 as i32) as isize) = *s
                .offset((11 as i32 - 1 as i32) as isize);
            current_block_42 = 3450148331727835525;
        }
        _ => {}
    }
    match current_block_42 {
        3450148331727835525 => {
            *d_0.offset((10 as i32 - 1 as i32) as isize) = *s
                .offset((10 as i32 - 1 as i32) as isize);
            current_block_42 = 17513148302838498461;
        }
        _ => {}
    }
    match current_block_42 {
        17513148302838498461 => {
            *d_0.offset((9 as i32 - 1 as i32) as isize) = *s
                .offset((9 as i32 - 1 as i32) as isize);
            current_block_42 = 5033545425852514390;
        }
        _ => {}
    }
    match current_block_42 {
        5033545425852514390 => {
            *d_0.offset((8 as i32 - 1 as i32) as isize) = *s
                .offset((8 as i32 - 1 as i32) as isize);
            current_block_42 = 7822129706622160761;
        }
        _ => {}
    }
    match current_block_42 {
        7822129706622160761 => {
            *d_0.offset((7 as i32 - 1 as i32) as isize) = *s
                .offset((7 as i32 - 1 as i32) as isize);
            current_block_42 = 14314526103220412958;
        }
        _ => {}
    }
    match current_block_42 {
        14314526103220412958 => {
            *d_0.offset((6 as i32 - 1 as i32) as isize) = *s
                .offset((6 as i32 - 1 as i32) as isize);
            current_block_42 = 11474465627670557597;
        }
        _ => {}
    }
    match current_block_42 {
        11474465627670557597 => {
            *d_0.offset((5 as i32 - 1 as i32) as isize) = *s
                .offset((5 as i32 - 1 as i32) as isize);
            current_block_42 = 13771482776398185125;
        }
        _ => {}
    }
    match current_block_42 {
        13771482776398185125 => {
            *d_0.offset((4 as i32 - 1 as i32) as isize) = *s
                .offset((4 as i32 - 1 as i32) as isize);
            current_block_42 = 519160363748337264;
        }
        _ => {}
    }
    match current_block_42 {
        519160363748337264 => {
            *d_0.offset((3 as i32 - 1 as i32) as isize) = *s
                .offset((3 as i32 - 1 as i32) as isize);
            current_block_42 = 17126217794288990979;
        }
        _ => {}
    }
    match current_block_42 {
        17126217794288990979 => {
            *d_0.offset((2 as i32 - 1 as i32) as isize) = *s
                .offset((2 as i32 - 1 as i32) as isize);
            current_block_42 = 11647970138234258413;
        }
        _ => {}
    }
    match current_block_42 {
        11647970138234258413 => {
            *d_0.offset((1 as i32 - 1 as i32) as isize) = *s
                .offset((1 as i32 - 1 as i32) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zswap(mut a_: *mut zahl, mut b_: *mut zahl) {
    let mut t: i64 = 0;
    let mut a_0: *mut i64 = a_ as *mut i64;
    let mut b: *mut i64 = b_ as *mut i64;
    t = *a_0.offset(0 as i32 as isize);
    *a_0.offset(0 as i32 as isize) = *b.offset(0 as i32 as isize);
    *b.offset(0 as i32 as isize) = t;
    t = *b.offset(1 as i32 as isize);
    *b.offset(1 as i32 as isize) = *a_0.offset(1 as i32 as isize);
    *a_0.offset(1 as i32 as isize) = t;
    t = *a_0.offset(2 as i32 as isize);
    *a_0.offset(2 as i32 as isize) = *b.offset(2 as i32 as isize);
    *b.offset(2 as i32 as isize) = t;
    t = *b.offset(3 as i32 as isize);
    *b.offset(3 as i32 as isize) = *a_0.offset(3 as i32 as isize);
    *a_0.offset(3 as i32 as isize) = t;
}
#[inline]
unsafe extern "C" fn zzero(mut a_0: *mut zahl) -> i32 {
    return ((*a_0).sign == 0) as i32;
}
#[inline]
unsafe extern "C" fn zset(mut a_0: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as i32) as i32 as i64 != 0 {
        (*a_0).sign = 0 as i32;
    } else {
        (*a_0).sign = (*b).sign;
        (*a_0).used = (*b).used;
        if (*a_0).alloced < (*b).used {
            libzahl_realloc(a_0, (*b).used);
        }
        libzahl_memcpy((*a_0).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zsetu(mut a_0: *mut zahl, mut b: uint64_t) {
    if (b == 0) as i32 as i64 != 0 {
        (*a_0).sign = 0 as i32;
        return;
    }
    if (*a_0).alloced < 1 as i32 as u64 {
        libzahl_realloc(a_0, 1 as i32 as size_t);
    }
    (*a_0).sign = 1 as i32;
    *((*a_0).chars).offset(0 as i32 as isize) = b;
    (*a_0).used = 1 as i32 as size_t;
}
#[inline]
unsafe extern "C" fn zcmp(mut a_0: *mut zahl, mut b: *mut zahl) -> i32 {
    if zsignum(a_0) != zsignum(b) {
        return if zsignum(a_0) < zsignum(b) {
            -(1 as i32)
        } else {
            (zsignum(a_0) > zsignum(b)) as i32
        };
    }
    return zsignum(a_0) * zcmpmag(a_0, b);
}
#[inline]
unsafe extern "C" fn zcmpmag(mut a_0: *mut zahl, mut b: *mut zahl) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if (zzero(a_0) != 0) as i32 as i64 != 0 {
        return -((zzero(b) == 0) as i32);
    }
    if (zzero(b) != 0) as i32 as i64 != 0 {
        return 1 as i32;
    }
    i = ((*a_0).used).wrapping_sub(1 as i32 as u64);
    j = ((*b).used).wrapping_sub(1 as i32 as u64);
    while i > j {
        if *((*a_0).chars).offset(i as isize) != 0 {
            return 1 as i32;
        }
        (*a_0).used = ((*a_0).used).wrapping_sub(1);
        (*a_0).used;
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
    while i != 0
        && *((*a_0).chars).offset(i as isize) == *((*b).chars).offset(i as isize)
    {
        i = i.wrapping_sub(1);
        i;
    }
    return if *((*a_0).chars).offset(i as isize) < *((*b).chars).offset(i as isize) {
        -(1 as i32)
    } else {
        (*((*a_0).chars).offset(i as isize) > *((*b).chars).offset(i as isize)) as i32
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a_0: *mut zahl) -> i32 {
    return (*a_0).sign;
}
#[inline]
unsafe extern "C" fn zcmpu(mut a_0: *mut zahl, mut b: uint64_t) -> i32 {
    if (b == 0) as i32 as i64 != 0 {
        return zsignum(a_0);
    }
    if (zsignum(a_0) <= 0 as i32) as i32 as i64 != 0 {
        return -(1 as i32);
    }
    while *((*a_0).chars).offset(((*a_0).used).wrapping_sub(1 as i32 as u64) as isize)
        == 0
    {
        (*a_0).used = ((*a_0).used).wrapping_sub(1);
        (*a_0).used;
    }
    if (*a_0).used > 1 as i32 as u64 {
        return 1 as i32;
    }
    return if *((*a_0).chars).offset(0 as i32 as isize) < b {
        -(1 as i32)
    } else {
        (*((*a_0).chars).offset(0 as i32 as isize) > b) as i32
    };
}
#[inline]
unsafe extern "C" fn zlsb(mut a_0: *mut zahl) -> size_t {
    let mut i: size_t = 0 as i32 as size_t;
    if (zzero(a_0) != 0) as i32 as i64 != 0 {
        return 18446744073709551615 as u64;
    }
    while *((*a_0).chars).offset(i as isize) == 0 {
        i = i.wrapping_add(1);
        i;
    }
    i = (i as u64)
        .wrapping_mul(
            (8 as i32 as u64).wrapping_mul(::core::mem::size_of::<zahl_char_t>() as u64),
        ) as size_t as size_t;
    i = (i as u64)
        .wrapping_add(
            (*((*a_0).chars).offset(i as isize) as libc::c_ulonglong).trailing_zeros()
                as i32 as size_t,
        ) as size_t as size_t;
    return i;
}
#[inline]
unsafe extern "C" fn zeven(mut a_0: *mut zahl) -> i32 {
    return ((*a_0).sign == 0
        || !*((*a_0).chars).offset(0 as i32 as isize) & 1 as i32 as u64 != 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn zptest(
    mut witness: *mut zahl,
    mut n: *mut zahl,
    mut t: i32,
) -> zprimality {
    let mut i: size_t = 0;
    let mut r: size_t = 0;
    if (zcmpu(n, 3 as i32 as uint64_t) <= 0 as i32) as i32 as i64 != 0 {
        if zcmpu(n, 1 as i32 as uint64_t) <= 0 as i32 {
            if !witness.is_null() {
                if witness != n {
                    zset(witness, n);
                }
            }
            return zprimality::NONPRIME;
        } else {
            return zprimality::PRIME
        }
    }
    if (zeven(n) != 0) as i32 as i64 != 0 {
        if !witness.is_null() {
            zsetu(witness, 2 as i32 as uint64_t);
        }
        return zprimality::NONPRIME;
    }
    zsub_unsigned(libzahl_tmp_ptest_n1.as_mut_ptr(), n, libzahl_const_1.as_mut_ptr());
    zsub_unsigned(libzahl_tmp_ptest_n4.as_mut_ptr(), n, libzahl_const_4.as_mut_ptr());
    r = zlsb(libzahl_tmp_ptest_n1.as_mut_ptr());
    zrsh(libzahl_tmp_ptest_d.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr(), r);
    loop {
        let fresh0 = t;
        t = t - 1;
        if !(fresh0 != 0) {
            break;
        }
        zrand(
            libzahl_tmp_ptest_a.as_mut_ptr(),
            zranddev::DEFAULT_RANDOM,
            zranddist::UNIFORM,
            libzahl_tmp_ptest_n4.as_mut_ptr(),
        );
        zadd_unsigned(
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_const_2.as_mut_ptr(),
        );
        zmodpow(
            libzahl_tmp_ptest_x.as_mut_ptr(),
            libzahl_tmp_ptest_a.as_mut_ptr(),
            libzahl_tmp_ptest_d.as_mut_ptr(),
            n,
        );
        if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_const_1.as_mut_ptr()) == 0
            || zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
        {
            continue;
        }
        i = 1 as i32 as size_t;
        while i < r {
            zmodsqr(
                libzahl_tmp_ptest_x.as_mut_ptr(),
                libzahl_tmp_ptest_x.as_mut_ptr(),
                n,
            );
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_const_1.as_mut_ptr()) == 0
            {
                if !witness.is_null() {
                    zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
                }
                return zprimality::NONPRIME;
            }
            if zcmp(libzahl_tmp_ptest_x.as_mut_ptr(), libzahl_tmp_ptest_n1.as_mut_ptr())
                == 0
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if i == r {
            if !witness.is_null() {
                zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
            }
            return zprimality::NONPRIME;
        }
    }
    return zprimality::PROBABLY_PRIME;
}