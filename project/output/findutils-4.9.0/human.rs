#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use num_traits::ToPrimitive;
extern "C" {
    fn xstrtoumax(
        _: *const i8,
        _: *mut *mut i8,
        _: i32,
        _: *mut uintmax_t,
        _: *const i8,
    ) -> strtol_error;
    fn localeconv() -> *mut lconv;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn argmatch(
        arg: *const i8,
        arglist: *const *const i8,
        vallist: *const libc::c_void,
        valsize: size_t,
    ) -> ptrdiff_t;
}
pub type __uintmax_t = u64;
pub type uintmax_t = __uintmax_t;
pub type size_t = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum strtol_error {
    LONGINT_OK = 0,
    LONGINT_OVERFLOW,
    LONGINT_INVALID_SUFFIX_CHAR,
    LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
    LONGINT_INVALID = 4,
}
impl strtol_error {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            strtol_error::LONGINT_OK => 0,
            strtol_error::LONGINT_OVERFLOW => 1,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR => 2,
            strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW => 3,
            strtol_error::LONGINT_INVALID => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> strtol_error {
        match value {
            0 => strtol_error::LONGINT_OK,
            1 => strtol_error::LONGINT_OVERFLOW,
            2 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR,
            3 => strtol_error::LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
            4 => strtol_error::LONGINT_INVALID,
            _ => panic!("Invalid value for strtol_error: {}", value),
        }
    }
}
impl AddAssign<u32> for strtol_error {
    fn add_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for strtol_error {
    fn sub_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for strtol_error {
    fn mul_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for strtol_error {
    fn div_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for strtol_error {
    fn rem_assign(&mut self, rhs: u32) {
        *self = strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for strtol_error {
    type Output = strtol_error;
    fn add(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for strtol_error {
    type Output = strtol_error;
    fn sub(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for strtol_error {
    type Output = strtol_error;
    fn mul(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for strtol_error {
    type Output = strtol_error;
    fn div(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for strtol_error {
    type Output = strtol_error;
    fn rem(self, rhs: u32) -> strtol_error {
        strtol_error::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    human_ceiling = 0,
    human_round_to_nearest = 1,
    human_floor = 2,
    human_group_digits = 4,
    human_suppress_point_zero = 8,
    human_autoscale = 16,
    human_base_1024 = 32,
    human_space_before_unit = 64,
    human_SI = 128,
    human_B = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::human_ceiling => 0,
            C2RustUnnamed::human_round_to_nearest => 1,
            C2RustUnnamed::human_floor => 2,
            C2RustUnnamed::human_group_digits => 4,
            C2RustUnnamed::human_suppress_point_zero => 8,
            C2RustUnnamed::human_autoscale => 16,
            C2RustUnnamed::human_base_1024 => 32,
            C2RustUnnamed::human_space_before_unit => 64,
            C2RustUnnamed::human_SI => 128,
            C2RustUnnamed::human_B => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            0 => C2RustUnnamed::human_ceiling,
            1 => C2RustUnnamed::human_round_to_nearest,
            2 => C2RustUnnamed::human_floor,
            4 => C2RustUnnamed::human_group_digits,
            8 => C2RustUnnamed::human_suppress_point_zero,
            16 => C2RustUnnamed::human_autoscale,
            32 => C2RustUnnamed::human_base_1024,
            64 => C2RustUnnamed::human_space_before_unit,
            128 => C2RustUnnamed::human_SI,
            256 => C2RustUnnamed::human_B,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut i8,
    pub thousands_sep: *mut i8,
    pub grouping: *mut i8,
    pub int_curr_symbol: *mut i8,
    pub currency_symbol: *mut i8,
    pub mon_decimal_point: *mut i8,
    pub mon_thousands_sep: *mut i8,
    pub mon_grouping: *mut i8,
    pub positive_sign: *mut i8,
    pub negative_sign: *mut i8,
    pub int_frac_digits: i8,
    pub frac_digits: i8,
    pub p_cs_precedes: i8,
    pub p_sep_by_space: i8,
    pub n_cs_precedes: i8,
    pub n_sep_by_space: i8,
    pub p_sign_posn: i8,
    pub n_sign_posn: i8,
    pub int_p_cs_precedes: i8,
    pub int_p_sep_by_space: i8,
    pub int_n_cs_precedes: i8,
    pub int_n_sep_by_space: i8,
    pub int_p_sign_posn: i8,
    pub int_n_sign_posn: i8,
}
pub type ptrdiff_t = i64;
static mut power_letter: [i8; 9] = [
    0 as i32 as i8,
    'K' as i32 as i8,
    'M' as i32 as i8,
    'G' as i32 as i8,
    'T' as i32 as i8,
    'P' as i32 as i8,
    'E' as i32 as i8,
    'Z' as i32 as i8,
    'Y' as i32 as i8,
];
unsafe extern "C" fn adjust_value(
    mut inexact_style: i32,
    mut value: f128::f128,
) -> f128::f128 {
    if inexact_style != C2RustUnnamed::human_round_to_nearest as i32
        && value < f128::f128::new(18446744073709551615 as u64)
    {
        let mut u: uintmax_t = value.to_u64().unwrap();
        value = f128::f128::new(
            u
                .wrapping_add(
                    (inexact_style == C2RustUnnamed::human_ceiling as i32
                        && f128::f128::new(u) != value) as i32 as u64,
                ),
        );
    }
    return value;
}
unsafe extern "C" fn group_number(
    mut number: *mut i8,
    mut numberlen: size_t,
    mut grouping: *const i8,
    mut thousands_sep: *const i8,
) -> *mut i8 {
    let mut d: *mut i8 = 0 as *mut i8;
    let mut grouplen: size_t = 18446744073709551615 as u64;
    let mut thousands_seplen: size_t = strlen(thousands_sep);
    let mut i: size_t = numberlen;
    let mut buf: [i8; 41] = [0; 41];
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        number as *const libc::c_void,
        numberlen,
    );
    d = number.offset(numberlen as isize);
    loop {
        let mut g: u8 = *grouping as u8;
        if g != 0 {
            grouplen = if (g as i32) < 127 as i32 { g as u64 } else { i };
            grouping = grouping.offset(1);
            grouping;
        }
        if i < grouplen {
            grouplen = i;
        }
        d = d.offset(-(grouplen as isize));
        i = (i as u64).wrapping_sub(grouplen) as size_t as size_t;
        memcpy(
            d as *mut libc::c_void,
            buf.as_mut_ptr().offset(i as isize) as *const libc::c_void,
            grouplen,
        );
        if i == 0 as i32 as u64 {
            return d;
        }
        d = d.offset(-(thousands_seplen as isize));
        memcpy(
            d as *mut libc::c_void,
            thousands_sep as *const libc::c_void,
            thousands_seplen,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn human_readable(
    mut n: uintmax_t,
    mut buf: *mut i8,
    mut opts: i32,
    mut from_block_size: uintmax_t,
    mut to_block_size: uintmax_t,
) -> *mut i8 {
    let mut current_block: u64;
    let mut inexact_style: i32 = opts
        & (C2RustUnnamed::human_round_to_nearest as i32
            | C2RustUnnamed::human_floor as i32 | C2RustUnnamed::human_ceiling as i32);
    let mut base: u32 = (if opts & C2RustUnnamed::human_base_1024 as i32 != 0 {
        1024 as i32
    } else {
        1000 as i32
    }) as u32;
    let mut amt: uintmax_t = 0;
    let mut tenths: i32 = 0;
    let mut exponent: i32 = -(1 as i32);
    let mut exponent_max: i32 = (::core::mem::size_of::<[i8; 9]>() as u64)
        .wrapping_sub(1 as i32 as u64) as i32;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut psuffix: *mut i8 = 0 as *mut i8;
    let mut integerlim: *const i8 = 0 as *const i8;
    let mut rounding: i32 = 0;
    let mut decimal_point: *const i8 = b".\0" as *const u8 as *const i8;
    let mut decimal_pointlen: size_t = 1 as i32 as size_t;
    let mut grouping: *const i8 = b"\0" as *const u8 as *const i8;
    let mut thousands_sep: *const i8 = b"\0" as *const u8 as *const i8;
    let mut l: *const lconv = localeconv();
    let mut pointlen: size_t = strlen((*l).decimal_point);
    if (0 as i32 as u64) < pointlen && pointlen <= 16 as i32 as u64 {
        decimal_point = (*l).decimal_point;
        decimal_pointlen = pointlen;
    }
    grouping = (*l).grouping;
    if strlen((*l).thousands_sep) <= 16 as i32 as u64 {
        thousands_sep = (*l).thousands_sep;
    }
    psuffix = buf
        .offset(
            (2 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_mul(146 as i32 as u64)
                .wrapping_div(485 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
                .wrapping_mul((16 as i32 + 1 as i32) as u64)
                .wrapping_sub(16 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(3 as i32 as u64) as isize,
        )
        .offset(-(1 as i32 as isize))
        .offset(-(3 as i32 as isize));
    p = psuffix;
    if to_block_size <= from_block_size {
        if from_block_size.wrapping_rem(to_block_size) == 0 as i32 as u64 {
            let mut multiplier: uintmax_t = from_block_size.wrapping_div(to_block_size);
            amt = n.wrapping_mul(multiplier);
            if amt.wrapping_div(multiplier) == n {
                tenths = 0 as i32;
                rounding = 0 as i32;
                current_block = 472718568467245738;
            } else {
                current_block = 11298138898191919651;
            }
        } else {
            current_block = 11298138898191919651;
        }
    } else if from_block_size != 0 as i32 as u64
        && to_block_size.wrapping_rem(from_block_size) == 0 as i32 as u64
    {
        let mut divisor: uintmax_t = to_block_size.wrapping_div(from_block_size);
        let mut r10: uintmax_t = n.wrapping_rem(divisor).wrapping_mul(10 as i32 as u64);
        let mut r2: uintmax_t = r10.wrapping_rem(divisor).wrapping_mul(2 as i32 as u64);
        amt = n.wrapping_div(divisor);
        tenths = r10.wrapping_div(divisor) as i32;
        rounding = if r2 < divisor {
            ((0 as i32 as u64) < r2) as i32
        } else {
            2 as i32 + (divisor < r2) as i32
        };
        current_block = 472718568467245738;
    } else {
        current_block = 11298138898191919651;
    }
    match current_block {
        11298138898191919651 => {
            let mut dto_block_size: f128::f128 = f128::f128::new(to_block_size);
            let mut damt: f128::f128 = f128::f128::new(n)
                * (f128::f128::new(from_block_size) / dto_block_size);
            let mut buflen: size_t = 0;
            let mut nonintegerlen: size_t = 0;
            if opts & C2RustUnnamed::human_autoscale as i32 == 0 {
                sprintf(
                    buf,
                    b"%.0Lf\0" as *const u8 as *const i8,
                    adjust_value(inexact_style, damt),
                );
                buflen = strlen(buf);
                nonintegerlen = 0 as i32 as size_t;
            } else {
                let mut e: f128::f128 = f128::f128::new(1 as i32);
                exponent = 0 as i32;
                loop {
                    e *= f128::f128::new(base);
                    exponent += 1;
                    exponent;
                    if !(e * f128::f128::new(base) <= damt && exponent < exponent_max) {
                        break;
                    }
                }
                damt /= e;
                sprintf(
                    buf,
                    b"%.1Lf\0" as *const u8 as *const i8,
                    adjust_value(inexact_style, damt),
                );
                buflen = strlen(buf);
                nonintegerlen = decimal_pointlen.wrapping_add(1 as i32 as u64);
                if (1 as i32 as u64)
                    .wrapping_add(nonintegerlen)
                    .wrapping_add(
                        (opts & C2RustUnnamed::human_base_1024 as i32 == 0) as i32 as u64,
                    ) < buflen
                    || opts & C2RustUnnamed::human_suppress_point_zero as i32 != 0
                        && *buf.offset(buflen.wrapping_sub(1 as i32 as u64) as isize)
                            as i32 == '0' as i32
                {
                    sprintf(
                        buf,
                        b"%.0Lf\0" as *const u8 as *const i8,
                        adjust_value(inexact_style, damt * f128::f128::new(10 as i32))
                            / f128::f128::new(10 as i32),
                    );
                    buflen = strlen(buf);
                    nonintegerlen = 0 as i32 as size_t;
                }
            }
            p = psuffix.offset(-(buflen as isize));
            memmove(p as *mut libc::c_void, buf as *const libc::c_void, buflen);
            integerlim = p.offset(buflen as isize).offset(-(nonintegerlen as isize));
        }
        _ => {
            if opts & C2RustUnnamed::human_autoscale as i32 != 0 {
                exponent = 0 as i32;
                if base as u64 <= amt {
                    loop {
                        let mut r10_0: u32 = amt
                            .wrapping_rem(base as u64)
                            .wrapping_mul(10 as i32 as u64)
                            .wrapping_add(tenths as u64) as u32;
                        let mut r2_0: u32 = r10_0
                            .wrapping_rem(base)
                            .wrapping_mul(2 as i32 as u32)
                            .wrapping_add((rounding >> 1 as i32) as u32);
                        amt = (amt as u64).wrapping_div(base as u64) as uintmax_t
                            as uintmax_t;
                        tenths = r10_0.wrapping_div(base) as i32;
                        rounding = if r2_0 < base {
                            (r2_0.wrapping_add(rounding as u32) != 0 as i32 as u32)
                                as i32
                        } else {
                            2 as i32 + (base < r2_0.wrapping_add(rounding as u32)) as i32
                        };
                        exponent += 1;
                        exponent;
                        if !(base as u64 <= amt && exponent < exponent_max) {
                            break;
                        }
                    }
                    if amt < 10 as i32 as u64 {
                        if if inexact_style
                            == C2RustUnnamed::human_round_to_nearest as i32
                        {
                            ((2 as i32) < rounding + (tenths & 1 as i32)) as i32
                        } else {
                            (inexact_style == C2RustUnnamed::human_ceiling as i32
                                && (0 as i32) < rounding) as i32
                        } != 0
                        {
                            tenths += 1;
                            tenths;
                            rounding = 0 as i32;
                            if tenths == 10 as i32 {
                                amt = amt.wrapping_add(1);
                                amt;
                                tenths = 0 as i32;
                            }
                        }
                        if amt < 10 as i32 as u64
                            && (tenths != 0
                                || opts & C2RustUnnamed::human_suppress_point_zero as i32
                                    == 0)
                        {
                            p = p.offset(-1);
                            *p = ('0' as i32 + tenths) as i8;
                            p = p.offset(-(decimal_pointlen as isize));
                            memcpy(
                                p as *mut libc::c_void,
                                decimal_point as *const libc::c_void,
                                decimal_pointlen,
                            );
                            rounding = 0 as i32;
                            tenths = rounding;
                        }
                    }
                }
            }
            if if inexact_style == C2RustUnnamed::human_round_to_nearest as i32 {
                ((5 as i32)
                    < tenths
                        + ((0 as i32 as u64)
                            < (rounding as u64).wrapping_add(amt & 1 as i32 as u64))
                            as i32) as i32
            } else {
                (inexact_style == C2RustUnnamed::human_ceiling as i32
                    && (0 as i32) < tenths + rounding) as i32
            } != 0
            {
                amt = amt.wrapping_add(1);
                amt;
                if opts & C2RustUnnamed::human_autoscale as i32 != 0
                    && amt == base as u64 && exponent < exponent_max
                {
                    exponent += 1;
                    exponent;
                    if opts & C2RustUnnamed::human_suppress_point_zero as i32 == 0 {
                        p = p.offset(-1);
                        *p = '0' as i32 as i8;
                        p = p.offset(-(decimal_pointlen as isize));
                        memcpy(
                            p as *mut libc::c_void,
                            decimal_point as *const libc::c_void,
                            decimal_pointlen,
                        );
                    }
                    amt = 1 as i32 as uintmax_t;
                }
            }
            integerlim = p;
            loop {
                let mut digit: i32 = amt.wrapping_rem(10 as i32 as u64) as i32;
                p = p.offset(-1);
                *p = (digit + '0' as i32) as i8;
                amt = (amt as u64).wrapping_div(10 as i32 as u64) as uintmax_t
                    as uintmax_t;
                if !(amt != 0 as i32 as u64) {
                    break;
                }
            }
        }
    }
    if opts & C2RustUnnamed::human_group_digits as i32 != 0 {
        p = group_number(
            p,
            integerlim.offset_from(p) as i64 as size_t,
            grouping,
            thousands_sep,
        );
    }
    if opts & C2RustUnnamed::human_SI as i32 != 0 {
        if exponent < 0 as i32 {
            let mut power: uintmax_t = 0;
            exponent = 0 as i32;
            power = 1 as i32 as uintmax_t;
            while power < to_block_size {
                exponent += 1;
                if exponent == exponent_max {
                    break;
                }
                power = (power as u64).wrapping_mul(base as u64) as uintmax_t
                    as uintmax_t;
            }
        }
        if exponent | opts & C2RustUnnamed::human_B as i32 != 0
            && opts & C2RustUnnamed::human_space_before_unit as i32 != 0
        {
            let fresh0 = psuffix;
            psuffix = psuffix.offset(1);
            *fresh0 = ' ' as i32 as i8;
        }
        if exponent != 0 {
            let fresh1 = psuffix;
            psuffix = psuffix.offset(1);
            *fresh1 = (if opts & C2RustUnnamed::human_base_1024 as i32 == 0
                && exponent == 1 as i32
            {
                'k' as i32
            } else {
                power_letter[exponent as usize] as i32
            }) as i8;
        }
        if opts & C2RustUnnamed::human_B as i32 != 0 {
            if opts & C2RustUnnamed::human_base_1024 as i32 != 0 && exponent != 0 {
                let fresh2 = psuffix;
                psuffix = psuffix.offset(1);
                *fresh2 = 'i' as i32 as i8;
            }
            let fresh3 = psuffix;
            psuffix = psuffix.offset(1);
            *fresh3 = 'B' as i32 as i8;
        }
    }
    *psuffix = '\0' as i32 as i8;
    return p;
}
static mut block_size_args: [*const i8; 3] = [
    b"human-readable\0" as *const u8 as *const i8,
    b"si\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut block_size_opts: [i32; 2] = [
    C2RustUnnamed::human_autoscale as i32 + C2RustUnnamed::human_SI as i32
        + C2RustUnnamed::human_base_1024 as i32,
    C2RustUnnamed::human_autoscale as i32 + C2RustUnnamed::human_SI as i32,
];
unsafe extern "C" fn default_block_size() -> uintmax_t {
    return (if !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const i8)).is_null() {
        512 as i32
    } else {
        1024 as i32
    }) as uintmax_t;
}
unsafe extern "C" fn humblock(
    mut spec: *const i8,
    mut block_size: *mut uintmax_t,
    mut options: *mut i32,
) -> strtol_error {
    let mut i: i32 = 0;
    let mut opts: i32 = 0 as i32;
    if spec.is_null()
        && {
            spec = getenv(b"BLOCK_SIZE\0" as *const u8 as *const i8);
            spec.is_null()
        }
        && {
            spec = getenv(b"BLOCKSIZE\0" as *const u8 as *const i8);
            spec.is_null()
        }
    {
        *block_size = default_block_size();
    } else {
        if *spec as i32 == '\'' as i32 {
            opts |= C2RustUnnamed::human_group_digits as i32;
            spec = spec.offset(1);
            spec;
        }
        i = argmatch(
            spec,
            block_size_args.as_ptr(),
            block_size_opts.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<i32>() as u64,
        ) as i32;
        if 0 as i32 <= i {
            opts |= block_size_opts[i as usize];
            *block_size = 1 as i32 as uintmax_t;
        } else {
            let mut ptr: *mut i8 = 0 as *mut i8;
            let mut e: strtol_error = xstrtoumax(
                spec,
                &mut ptr,
                0 as i32,
                block_size,
                b"eEgGkKmMpPtTyYzZ0\0" as *const u8 as *const i8,
            );
            if e as u32 != strtol_error::LONGINT_OK as i32 as u32 {
                *options = 0 as i32;
                return e;
            }
            while !('0' as i32 <= *spec as i32 && *spec as i32 <= '9' as i32) {
                if spec == ptr {
                    opts |= C2RustUnnamed::human_SI as i32;
                    if *ptr.offset(-(1 as i32) as isize) as i32 == 'B' as i32 {
                        opts |= C2RustUnnamed::human_B as i32;
                    }
                    if *ptr.offset(-(1 as i32) as isize) as i32 != 'B' as i32
                        || *ptr.offset(-(2 as i32) as isize) as i32 == 'i' as i32
                    {
                        opts |= C2RustUnnamed::human_base_1024 as i32;
                    }
                    break;
                } else {
                    spec = spec.offset(1);
                    spec;
                }
            }
        }
    }
    *options = opts;
    return strtol_error::LONGINT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn human_options(
    mut spec: *const i8,
    mut opts: *mut i32,
    mut block_size: *mut uintmax_t,
) -> strtol_error {
    let mut e: strtol_error = humblock(spec, block_size, opts);
    if *block_size == 0 as i32 as u64 {
        *block_size = default_block_size();
        e = strtol_error::LONGINT_INVALID;
    }
    return e;
}