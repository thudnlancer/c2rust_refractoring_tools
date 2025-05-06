#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn __strtol_internal(
        __nptr: *const i8,
        __endptr: *mut *mut i8,
        __base: i32,
        __group: i32,
    ) -> i64;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type __intmax_t = i64;
pub type intmax_t = __intmax_t;
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
    _ISspace = 8192,
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8192 => C2RustUnnamed::_ISspace,
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
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
#[no_mangle]
pub unsafe extern "C" fn xstrtoimax(
    mut s: *const i8,
    mut ptr: *mut *mut i8,
    mut strtol_base: i32,
    mut val: *mut intmax_t,
    mut valid_suffixes: *const i8,
) -> strtol_error {
    let mut t_ptr: *mut i8 = 0 as *mut i8;
    let mut p: *mut *mut i8 = 0 as *mut *mut i8;
    let mut tmp: intmax_t = 0;
    let mut err: strtol_error = strtol_error::LONGINT_OK;
    if 0 as i32 <= strtol_base && strtol_base <= 36 as i32 {} else {
        __assert_fail(
            b"0 <= strtol_base && strtol_base <= 36\0" as *const u8 as *const i8,
            b"./xstrtol.c\0" as *const u8 as *const i8,
            85 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 78],
                &[i8; 78],
            >(
                b"strtol_error xstrtoimax(const char *, char **, int, intmax_t *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11726: {
        if 0 as i32 <= strtol_base && strtol_base <= 36 as i32 {} else {
            __assert_fail(
                b"0 <= strtol_base && strtol_base <= 36\0" as *const u8 as *const i8,
                b"./xstrtol.c\0" as *const u8 as *const i8,
                85 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[i8; 78],
                >(
                    b"strtol_error xstrtoimax(const char *, char **, int, intmax_t *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    p = if !ptr.is_null() { ptr } else { &mut t_ptr };
    *__errno_location() = 0 as i32;
    if (0 as i32 as intmax_t) < -(1 as i32) as intmax_t {
        let mut q: *const i8 = s;
        let mut ch: u8 = *q as u8;
        while *(*__ctype_b_loc()).offset(ch as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
        {
            q = q.offset(1);
            ch = *q as u8;
        }
        if ch as i32 == '-' as i32 {
            return strtol_error::LONGINT_INVALID;
        }
    }
    tmp = strtoimax(s, p, strtol_base);
    if *p == s as *mut i8 {
        if !valid_suffixes.is_null() && **p as i32 != 0
            && !(strchr(valid_suffixes, **p as i32)).is_null()
        {
            tmp = 1 as i32 as intmax_t;
        } else {
            return strtol_error::LONGINT_INVALID
        }
    } else if *__errno_location() != 0 as i32 {
        if *__errno_location() != 34 as i32 {
            return strtol_error::LONGINT_INVALID;
        }
        err = strtol_error::LONGINT_OVERFLOW;
    }
    if valid_suffixes.is_null() {
        *val = tmp;
        return err;
    }
    if **p as i32 != '\0' as i32 {
        let mut base: i32 = 1024 as i32;
        let mut suffixes: i32 = 1 as i32;
        let mut overflow: strtol_error = strtol_error::LONGINT_OK;
        if (strchr(valid_suffixes, **p as i32)).is_null() {
            *val = tmp;
            return strtol_error::from_libc_c_uint(
                (err as u32 | strtol_error::LONGINT_INVALID_SUFFIX_CHAR as i32 as u32)
                    as u32,
            );
        }
        match **p as i32 {
            69 | 71 | 103 | 107 | 75 | 77 | 109 | 80 | 81 | 82 | 84 | 116 | 89 | 90 => {
                if !(strchr(valid_suffixes, '0' as i32)).is_null() {
                    match *(*p.offset(0 as i32 as isize)).offset(1 as i32 as isize)
                        as i32
                    {
                        105 => {
                            if *(*p.offset(0 as i32 as isize)).offset(2 as i32 as isize)
                                as i32 == 'B' as i32
                            {
                                suffixes += 2 as i32;
                            }
                        }
                        66 | 68 => {
                            base = 1000 as i32;
                            suffixes += 1;
                            suffixes;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        match **p as i32 {
            98 => {
                overflow = bkm_scale(&mut tmp, 512 as i32);
            }
            66 => {
                overflow = bkm_scale(&mut tmp, 1024 as i32);
            }
            99 => {
                overflow = strtol_error::LONGINT_OK;
            }
            69 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 6 as i32);
            }
            71 | 103 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 3 as i32);
            }
            107 | 75 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 1 as i32);
            }
            77 | 109 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 2 as i32);
            }
            80 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 5 as i32);
            }
            81 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 10 as i32);
            }
            82 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 9 as i32);
            }
            84 | 116 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 4 as i32);
            }
            119 => {
                overflow = bkm_scale(&mut tmp, 2 as i32);
            }
            89 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 8 as i32);
            }
            90 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 7 as i32);
            }
            _ => {
                *val = tmp;
                return strtol_error::from_libc_c_uint(
                    (err as u32
                        | strtol_error::LONGINT_INVALID_SUFFIX_CHAR as i32 as u32) as u32,
                );
            }
        }
        err = ::core::mem::transmute::<u32, strtol_error>(err as u32 | overflow as u32);
        *p = (*p).offset(suffixes as isize);
        if **p != 0 {
            err = ::core::mem::transmute::<
                u32,
                strtol_error,
            >(err as u32 | strtol_error::LONGINT_INVALID_SUFFIX_CHAR as i32 as u32);
        }
    }
    *val = tmp;
    return err;
}
unsafe extern "C" fn bkm_scale_by_power(
    mut x: *mut intmax_t,
    mut base: i32,
    mut power: i32,
) -> strtol_error {
    let mut err: strtol_error = strtol_error::LONGINT_OK;
    loop {
        let fresh0 = power;
        power = power - 1;
        if !(fresh0 != 0) {
            break;
        }
        err = ::core::mem::transmute::<
            u32,
            strtol_error,
        >(err as u32 | bkm_scale(x, base) as u32);
    }
    return err;
}
unsafe extern "C" fn bkm_scale(
    mut x: *mut intmax_t,
    mut scale_factor: i32,
) -> strtol_error {
    let mut scaled: intmax_t = 0;
    if if ::core::mem::size_of::<intmax_t>() as u64
        == ::core::mem::size_of::<libc::c_schar>() as u64
    {
        if !((0 as i32 as intmax_t) < -(1 as i32) as intmax_t) {
            if if scale_factor < 0 as i32 {
                if *x < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 127 as i32 })
                            + scale_factor
                    }) - 1 as i32) < 0 as i32
                    {
                        (*x < (127 as i32 / scale_factor) as i64) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            scale_factor
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) < 0 as i32
                        {
                            (scale_factor
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    scale_factor
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < scale_factor) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 127 as i32
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            127 as i32 / -scale_factor
                        }) as i64 <= -(1 as i32) as i64 - *x) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        + (-(127 as i32) - 1 as i32)
                }) - 1 as i32) < 0 as i32
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(127 as i32) - 1 as i32)
                    }) + 1 as i32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                        + 1 as i32)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(127 as i32) - 1 as i32)
                    }) + 0 as i32
                }) < 0 as i32
                {
                    ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        + (-(127 as i32) - 1 as i32)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + (-(127 as i32) - 1 as i32)
                        }) - 1 as i32) < 0 as i32
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + (-(127 as i32) - 1 as i32)
                            }) + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + (-(127 as i32) - 1 as i32)
                            }) - 1 as i32
                        })) as i32
                } else {
                    ((0 as i32)
                        < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(127 as i32) - 1 as i32)) as i32
                }) != 0 && scale_factor == -(1 as i32)
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64) < *x + (-(127 as i32) - 1 as i32) as i64)
                            as i32
                    } else {
                        ((0 as i32 as i64) < *x
                            && ((-(1 as i32) - (-(127 as i32) - 1 as i32)) as i64)
                                < *x - 1 as i32 as i64) as i32
                    }
                } else {
                    ((((-(127 as i32) - 1 as i32) / scale_factor) as i64) < *x) as i32
                }
            } else if scale_factor == 0 as i32 {
                0 as i32
            } else if *x < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(127 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(127 as i32) - 1 as i32) as i64
                    }) + 1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(127 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(127 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + (-(127 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(127 as i32) - 1 as i32) as i64) as i32
                }) != 0 && *x == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                        < 0 as i32
                    {
                        ((0 as i32) < scale_factor + (-(127 as i32) - 1 as i32)) as i32
                    } else {
                        (-(1 as i32) - (-(127 as i32) - 1 as i32)
                            < scale_factor - 1 as i32) as i32
                    }
                } else {
                    ((-(127 as i32) - 1 as i32) as i64 / *x < scale_factor as i64) as i32
                }
            } else {
                (((127 as i32 / scale_factor) as i64) < *x) as i32
            } != 0
            {
                scaled = (*x as u32).wrapping_mul(scale_factor as u32) as libc::c_schar
                    as intmax_t;
                1 as i32
            } else {
                scaled = (*x as u32).wrapping_mul(scale_factor as u32) as libc::c_schar
                    as intmax_t;
                0 as i32
            }
        } else if if scale_factor < 0 as i32 {
            if *x < 0 as i32 as i64 {
                if ((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        127 as i32 * 2 as i32 + 1 as i32
                    }) + scale_factor
                }) - 1 as i32) < 0 as i32
                {
                    (*x < ((127 as i32 * 2 as i32 + 1 as i32) / scale_factor) as i64)
                        as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) < 0 as i32
                    {
                        (scale_factor
                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32) < scale_factor) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (127 as i32 * 2 as i32 + 1 as i32)
                            >> (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (127 as i32 * 2 as i32 + 1 as i32) / -scale_factor
                    }) as i64 <= -(1 as i32) as i64 - *x) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32
            } else {
                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
            }) - 1 as i32) < 0 as i32
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 1 as i32)
                    << (::core::mem::size_of::<i32>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                    + 1 as i32)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 0 as i32
            }) < 0 as i32
            {
                (((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) - 1 as i32
                    })) as i32
            } else {
                ((0 as i32)
                    < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    as i32
            }) != 0 && scale_factor == -(1 as i32)
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < *x + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < *x
                        && ((-(1 as i32) - 0 as i32) as i64) < *x - 1 as i32 as i64)
                        as i32
                }
            } else {
                (((0 as i32 / scale_factor) as i64) < *x) as i32
            }
        } else if scale_factor == 0 as i32 {
            0 as i32
        } else if *x < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) + 1 as i32 as i64)
                            << (::core::mem::size_of::<i64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + 0 as i32 as i64) as i32
            }) != 0 && *x == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                    < 0 as i32
                {
                    ((0 as i32) < scale_factor + 0 as i32) as i32
                } else {
                    ((-(1 as i32) - 0 as i32) < scale_factor - 1 as i32) as i32
                }
            } else {
                (0 as i32 as i64 / *x < scale_factor as i64) as i32
            }
        } else {
            ((((127 as i32 * 2 as i32 + 1 as i32) / scale_factor) as i64) < *x) as i32
        } != 0
        {
            scaled = (*x as u32).wrapping_mul(scale_factor as u32) as u8 as intmax_t;
            1 as i32
        } else {
            scaled = (*x as u32).wrapping_mul(scale_factor as u32) as u8 as intmax_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<intmax_t>() as u64
        == ::core::mem::size_of::<libc::c_short>() as u64
    {
        if !((0 as i32 as intmax_t) < -(1 as i32) as intmax_t) {
            if if scale_factor < 0 as i32 {
                if *x < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 32767 as i32 })
                            + scale_factor
                    }) - 1 as i32) < 0 as i32
                    {
                        (*x < (32767 as i32 / scale_factor) as i64) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            scale_factor
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) < 0 as i32
                        {
                            (scale_factor
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    scale_factor
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < scale_factor) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 32767 as i32
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            32767 as i32 / -scale_factor
                        }) as i64 <= -(1 as i32) as i64 - *x) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        + (-(32767 as i32) - 1 as i32)
                }) - 1 as i32) < 0 as i32
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(32767 as i32) - 1 as i32)
                    }) + 1 as i32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                        + 1 as i32)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(32767 as i32) - 1 as i32)
                    }) + 0 as i32
                }) < 0 as i32
                {
                    ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        + (-(32767 as i32) - 1 as i32)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + (-(32767 as i32) - 1 as i32)
                        }) - 1 as i32) < 0 as i32
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + (-(32767 as i32) - 1 as i32)
                            }) + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + (-(32767 as i32) - 1 as i32)
                            }) - 1 as i32
                        })) as i32
                } else {
                    ((0 as i32)
                        < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(32767 as i32) - 1 as i32)) as i32
                }) != 0 && scale_factor == -(1 as i32)
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64) < *x + (-(32767 as i32) - 1 as i32) as i64)
                            as i32
                    } else {
                        ((0 as i32 as i64) < *x
                            && ((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as i64)
                                < *x - 1 as i32 as i64) as i32
                    }
                } else {
                    ((((-(32767 as i32) - 1 as i32) / scale_factor) as i64) < *x) as i32
                }
            } else if scale_factor == 0 as i32 {
                0 as i32
            } else if *x < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(32767 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(32767 as i32) - 1 as i32) as i64
                    }) + 1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(32767 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(32767 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + (-(32767 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(32767 as i32) - 1 as i32) as i64
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(32767 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(32767 as i32) - 1 as i32) as i64) as i32
                }) != 0 && *x == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                        < 0 as i32
                    {
                        ((0 as i32) < scale_factor + (-(32767 as i32) - 1 as i32)) as i32
                    } else {
                        (-(1 as i32) - (-(32767 as i32) - 1 as i32)
                            < scale_factor - 1 as i32) as i32
                    }
                } else {
                    ((-(32767 as i32) - 1 as i32) as i64 / *x < scale_factor as i64)
                        as i32
                }
            } else {
                (((32767 as i32 / scale_factor) as i64) < *x) as i32
            } != 0
            {
                scaled = (*x as u32).wrapping_mul(scale_factor as u32) as libc::c_short
                    as intmax_t;
                1 as i32
            } else {
                scaled = (*x as u32).wrapping_mul(scale_factor as u32) as libc::c_short
                    as intmax_t;
                0 as i32
            }
        } else if if scale_factor < 0 as i32 {
            if *x < 0 as i32 as i64 {
                if ((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        32767 as i32 * 2 as i32 + 1 as i32
                    }) + scale_factor
                }) - 1 as i32) < 0 as i32
                {
                    (*x < ((32767 as i32 * 2 as i32 + 1 as i32) / scale_factor) as i64)
                        as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) < 0 as i32
                    {
                        (scale_factor
                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32) < scale_factor) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (32767 as i32 * 2 as i32 + 1 as i32)
                            >> (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (32767 as i32 * 2 as i32 + 1 as i32) / -scale_factor
                    }) as i64 <= -(1 as i32) as i64 - *x) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32
            } else {
                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
            }) - 1 as i32) < 0 as i32
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 1 as i32)
                    << (::core::mem::size_of::<i32>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                    + 1 as i32)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 0 as i32
            }) < 0 as i32
            {
                (((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) - 1 as i32
                    })) as i32
            } else {
                ((0 as i32)
                    < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    as i32
            }) != 0 && scale_factor == -(1 as i32)
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < *x + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < *x
                        && ((-(1 as i32) - 0 as i32) as i64) < *x - 1 as i32 as i64)
                        as i32
                }
            } else {
                (((0 as i32 / scale_factor) as i64) < *x) as i32
            }
        } else if scale_factor == 0 as i32 {
            0 as i32
        } else if *x < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) + 1 as i32 as i64)
                            << (::core::mem::size_of::<i64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + 0 as i32 as i64) as i32
            }) != 0 && *x == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                    < 0 as i32
                {
                    ((0 as i32) < scale_factor + 0 as i32) as i32
                } else {
                    ((-(1 as i32) - 0 as i32) < scale_factor - 1 as i32) as i32
                }
            } else {
                (0 as i32 as i64 / *x < scale_factor as i64) as i32
            }
        } else {
            ((((32767 as i32 * 2 as i32 + 1 as i32) / scale_factor) as i64) < *x) as i32
        } != 0
        {
            scaled = (*x as u32).wrapping_mul(scale_factor as u32) as libc::c_ushort
                as intmax_t;
            1 as i32
        } else {
            scaled = (*x as u32).wrapping_mul(scale_factor as u32) as libc::c_ushort
                as intmax_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<intmax_t>() as u64
        == ::core::mem::size_of::<i32>() as u64
    {
        if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { scaled }) - 1 as i32 as i64)
            < 0 as i32 as i64
        {
            if if scale_factor < 0 as i32 {
                if *x < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 2147483647 as i32 })
                            + scale_factor
                    }) - 1 as i32) < 0 as i32
                    {
                        (*x < (2147483647 as i32 / scale_factor) as i64) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            scale_factor
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) < 0 as i32
                        {
                            (scale_factor
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    scale_factor
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < scale_factor) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 2147483647 as i32
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            2147483647 as i32 / -scale_factor
                        }) as i64 <= -(1 as i32) as i64 - *x) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        + (-(2147483647 as i32) - 1 as i32)
                }) - 1 as i32) < 0 as i32
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(2147483647 as i32) - 1 as i32)
                    }) + 1 as i32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                        + 1 as i32)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(2147483647 as i32) - 1 as i32)
                    }) + 0 as i32
                }) < 0 as i32
                {
                    ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        + (-(2147483647 as i32) - 1 as i32)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + (-(2147483647 as i32) - 1 as i32)
                        }) - 1 as i32) < 0 as i32
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + (-(2147483647 as i32) - 1 as i32)
                            }) + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + (-(2147483647 as i32) - 1 as i32)
                            }) - 1 as i32
                        })) as i32
                } else {
                    ((0 as i32)
                        < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + (-(2147483647 as i32) - 1 as i32)) as i32
                }) != 0 && scale_factor == -(1 as i32)
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64)
                            < *x + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                    } else {
                        ((0 as i32 as i64) < *x
                            && ((-(1 as i32) - (-(2147483647 as i32) - 1 as i32)) as i64)
                                < *x - 1 as i32 as i64) as i32
                    }
                } else {
                    ((((-(2147483647 as i32) - 1 as i32) / scale_factor) as i64) < *x)
                        as i32
                }
            } else if scale_factor == 0 as i32 {
                0 as i32
            } else if *x < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(2147483647 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(2147483647 as i32) - 1 as i32) as i64
                    }) + 1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(2147483647 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(2147483647 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + (-(2147483647 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(2147483647 as i32) - 1 as i32) as i64
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(2147483647 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                }) != 0 && *x == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                        < 0 as i32
                    {
                        ((0 as i32) < scale_factor + (-(2147483647 as i32) - 1 as i32))
                            as i32
                    } else {
                        (-(1 as i32) - (-(2147483647 as i32) - 1 as i32)
                            < scale_factor - 1 as i32) as i32
                    }
                } else {
                    ((-(2147483647 as i32) - 1 as i32) as i64 / *x < scale_factor as i64)
                        as i32
                }
            } else {
                (((2147483647 as i32 / scale_factor) as i64) < *x) as i32
            } != 0
            {
                scaled = (*x as u32).wrapping_mul(scale_factor as u32) as i32
                    as intmax_t;
                1 as i32
            } else {
                scaled = (*x as u32).wrapping_mul(scale_factor as u32) as i32
                    as intmax_t;
                0 as i32
            }
        } else if if scale_factor < 0 as i32 {
            if *x < 0 as i32 as i64 {
                if (if 1 as i32 != 0 {
                    0 as i32 as u32
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u32
                    } else {
                        (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32)
                    })
                        .wrapping_add(scale_factor as u32)
                })
                    .wrapping_sub(1 as i32 as u32) < 0 as i32 as u32
                {
                    (*x
                        < (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32)
                            .wrapping_div(scale_factor as u32) as i64) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) < 0 as i32
                    {
                        (scale_factor
                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32) < scale_factor) as i32
                    }) != 0
                    {
                        ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as u32)
                            .wrapping_add(
                                (2147483647 as i32 as u32)
                                    .wrapping_mul(2 as u32)
                                    .wrapping_add(1 as u32),
                            )
                            >> (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32)
                            .wrapping_div(-scale_factor as u32)
                    }) as i64 <= -(1 as i32) as i64 - *x) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32
            } else {
                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
            }) - 1 as i32) < 0 as i32
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 1 as i32)
                    << (::core::mem::size_of::<i32>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                    + 1 as i32)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 0 as i32
            }) < 0 as i32
            {
                (((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) - 1 as i32
                    })) as i32
            } else {
                ((0 as i32)
                    < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    as i32
            }) != 0 && scale_factor == -(1 as i32)
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < *x + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < *x
                        && ((-(1 as i32) - 0 as i32) as i64) < *x - 1 as i32 as i64)
                        as i32
                }
            } else {
                (((0 as i32 / scale_factor) as i64) < *x) as i32
            }
        } else if scale_factor == 0 as i32 {
            0 as i32
        } else if *x < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) + 1 as i32 as i64)
                            << (::core::mem::size_of::<i64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + 0 as i32 as i64) as i32
            }) != 0 && *x == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                    < 0 as i32
                {
                    ((0 as i32) < scale_factor + 0 as i32) as i32
                } else {
                    ((-(1 as i32) - 0 as i32) < scale_factor - 1 as i32) as i32
                }
            } else {
                (0 as i32 as i64 / *x < scale_factor as i64) as i32
            }
        } else {
            (((2147483647 as i32 as u32)
                .wrapping_mul(2 as u32)
                .wrapping_add(1 as u32)
                .wrapping_div(scale_factor as u32) as i64) < *x) as i32
        } != 0
        {
            scaled = (*x as u32).wrapping_mul(scale_factor as u32) as intmax_t;
            1 as i32
        } else {
            scaled = (*x as u32).wrapping_mul(scale_factor as u32) as intmax_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<intmax_t>() as u64
        == ::core::mem::size_of::<i64>() as u64
    {
        if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { scaled }) - 1 as i32 as i64)
            < 0 as i32 as i64
        {
            if if scale_factor < 0 as i32 {
                if *x < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            9223372036854775807 as i64
                        }) + scale_factor as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (*x < 9223372036854775807 as i64 / scale_factor as i64) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            scale_factor
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) < 0 as i32
                        {
                            (scale_factor
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    scale_factor
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                        - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < scale_factor) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                                + 9223372036854775807 as i64
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            9223372036854775807 as i64 / -scale_factor as i64
                        }) <= -(1 as i32) as i64 - *x) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                        + (-(9223372036854775807 as i64) - 1 as i64)
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                            + (-(9223372036854775807 as i64) - 1 as i64)
                    }) + 1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                            + (-(9223372036854775807 as i64) - 1 as i64)
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                        + (-(9223372036854775807 as i64) - 1 as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                                + (-(9223372036854775807 as i64) - 1 as i64)
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                                    + (-(9223372036854775807 as i64) - 1 as i64)
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                                    + (-(9223372036854775807 as i64) - 1 as i64)
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as i64
                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                }) != 0 && scale_factor == -(1 as i32)
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64)
                            < *x + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                    } else {
                        ((0 as i32 as i64) < *x
                            && -(1 as i32) as i64
                                - (-(9223372036854775807 as i64) - 1 as i64)
                                < *x - 1 as i32 as i64) as i32
                    }
                } else {
                    (((-(9223372036854775807 as i64) - 1 as i64) / scale_factor as i64)
                        < *x) as i32
                }
            } else if scale_factor == 0 as i32 {
                0 as i32
            } else if *x < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(9223372036854775807 as i64) - 1 as i64)
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(9223372036854775807 as i64) - 1 as i64)
                    }) + 1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(9223372036854775807 as i64) - 1 as i64)
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + (-(9223372036854775807 as i64) - 1 as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + (-(9223372036854775807 as i64) - 1 as i64)
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(9223372036854775807 as i64) - 1 as i64)
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                    + (-(9223372036854775807 as i64) - 1 as i64)
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                }) != 0 && *x == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                        < 0 as i32
                    {
                        ((0 as i32 as i64)
                            < scale_factor as i64
                                + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                    } else {
                        (-(1 as i32) as i64 - (-(9223372036854775807 as i64) - 1 as i64)
                            < (scale_factor - 1 as i32) as i64) as i32
                    }
                } else {
                    ((-(9223372036854775807 as i64) - 1 as i64) / *x
                        < scale_factor as i64) as i32
                }
            } else {
                ((9223372036854775807 as i64 / scale_factor as i64) < *x) as i32
            } != 0
            {
                scaled = (*x as u64).wrapping_mul(scale_factor as u64) as i64;
                1 as i32
            } else {
                scaled = (*x as u64).wrapping_mul(scale_factor as u64) as i64;
                0 as i32
            }
        } else if if scale_factor < 0 as i32 {
            if *x < 0 as i32 as i64 {
                if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                    })
                        .wrapping_add(scale_factor as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((*x as u64)
                        < (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                            .wrapping_div(scale_factor as u64)) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) < 0 as i32
                    {
                        (scale_factor
                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32) < scale_factor) as i32
                    }) != 0
                    {
                        ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as u64)
                            .wrapping_add(
                                (9223372036854775807 as i64 as u64)
                                    .wrapping_mul(2 as u64)
                                    .wrapping_add(1 as u64),
                            )
                            >> (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                            .wrapping_div(-scale_factor as u64)
                    }) <= (-(1 as i32) as i64 - *x) as u64) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32
            } else {
                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
            }) - 1 as i32) < 0 as i32
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 1 as i32)
                    << (::core::mem::size_of::<i32>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                    + 1 as i32)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) + 0 as i32
            }) < 0 as i32
            {
                (((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 0 as i32
                        }) - 1 as i32
                    })) as i32
            } else {
                ((0 as i32)
                    < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                    as i32
            }) != 0 && scale_factor == -(1 as i32)
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < *x + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < *x
                        && ((-(1 as i32) - 0 as i32) as i64) < *x - 1 as i32 as i64)
                        as i32
                }
            } else {
                (((0 as i32 / scale_factor) as i64) < *x) as i32
            }
        } else if scale_factor == 0 as i32 {
            0 as i32
        } else if *x < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) + 1 as i32 as i64)
                            << (::core::mem::size_of::<i64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        + 0 as i32 as i64) as i32
            }) != 0 && *x == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                    < 0 as i32
                {
                    ((0 as i32) < scale_factor + 0 as i32) as i32
                } else {
                    ((-(1 as i32) - 0 as i32) < scale_factor - 1 as i32) as i32
                }
            } else {
                (0 as i32 as i64 / *x < scale_factor as i64) as i32
            }
        } else {
            ((9223372036854775807 as i64 as u64)
                .wrapping_mul(2 as u64)
                .wrapping_add(1 as u64)
                .wrapping_div(scale_factor as u64) < *x as u64) as i32
        } != 0
        {
            scaled = (*x as u64).wrapping_mul(scale_factor as u64) as intmax_t;
            1 as i32
        } else {
            scaled = (*x as u64).wrapping_mul(scale_factor as u64) as intmax_t;
            0 as i32
        }
    } else if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { scaled }) - 1 as i32 as i64)
        < 0 as i32 as i64
    {
        if if scale_factor < 0 as i32 {
            if *x < 0 as i32 as i64 {
                if ((if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as libc::c_longlong
                    } else {
                        9223372036854775807 as libc::c_longlong
                    }) + scale_factor as libc::c_longlong
                }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
                {
                    ((*x as libc::c_longlong)
                        < 9223372036854775807 as libc::c_longlong
                            / scale_factor as libc::c_longlong) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) < 0 as i32
                    {
                        (scale_factor
                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                    - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32) < scale_factor) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            as libc::c_longlong + 9223372036854775807 as libc::c_longlong
                            >> (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        9223372036854775807 as libc::c_longlong
                            / -scale_factor as libc::c_longlong
                    }) <= (-(1 as i32) as i64 - *x) as libc::c_longlong) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32 as libc::c_longlong
            } else {
                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
            }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 1 as i32 as libc::c_longlong)
                    << (::core::mem::size_of::<libc::c_longlong>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as libc::c_longlong)
                    * 2 as i32 as libc::c_longlong + 1 as i32 as libc::c_longlong)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as i32 as libc::c_longlong
            }) < 0 as i32 as libc::c_longlong
            {
                ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                    as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as libc::c_longlong
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as libc::c_longlong
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) + 1 as i32 as libc::c_longlong)
                            << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            - 1 as i32 as libc::c_longlong)
                            * 2 as i32 as libc::c_longlong + 1 as i32 as libc::c_longlong
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as libc::c_longlong
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as i32 as libc::c_longlong
                    })) as i32
            } else {
                ((0 as i32 as libc::c_longlong)
                    < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as i32
            }) != 0 && scale_factor == -(1 as i32)
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as libc::c_longlong)
                        < *x as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as i32
                } else {
                    ((0 as i32 as i64) < *x
                        && -(1 as i32) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                            < (*x - 1 as i32 as i64) as libc::c_longlong) as i32
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / scale_factor as libc::c_longlong) < *x as libc::c_longlong) as i32
            }
        } else if scale_factor == 0 as i32 {
            0 as i32
        } else if *x < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as libc::c_longlong
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
            }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 1 as i32 as libc::c_longlong)
                    << (::core::mem::size_of::<libc::c_longlong>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as libc::c_longlong)
                    * 2 as i32 as libc::c_longlong + 1 as i32 as libc::c_longlong)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as i32 as libc::c_longlong
            }) < 0 as i32 as libc::c_longlong
            {
                ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as libc::c_longlong
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as libc::c_longlong
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) + 1 as i32 as libc::c_longlong)
                            << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            - 1 as i32 as libc::c_longlong)
                            * 2 as i32 as libc::c_longlong + 1 as i32 as libc::c_longlong
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as libc::c_longlong
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                                as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as i32 as libc::c_longlong
                    })) as i32
            } else {
                ((0 as i32 as libc::c_longlong)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as i32
            }) != 0 && *x == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                    < 0 as i32
                {
                    ((0 as i32 as libc::c_longlong)
                        < scale_factor as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as i32
                } else {
                    (-(1 as i32) as libc::c_longlong
                        - (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                        < (scale_factor - 1 as i32) as libc::c_longlong) as i32
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / *x as libc::c_longlong) < scale_factor as libc::c_longlong) as i32
            }
        } else {
            ((9223372036854775807 as libc::c_longlong / scale_factor as libc::c_longlong)
                < *x as libc::c_longlong) as i32
        } != 0
        {
            scaled = (*x as libc::c_ulonglong)
                .wrapping_mul(scale_factor as libc::c_ulonglong) as libc::c_longlong
                as intmax_t;
            1 as i32
        } else {
            scaled = (*x as libc::c_ulonglong)
                .wrapping_mul(scale_factor as libc::c_ulonglong) as libc::c_longlong
                as intmax_t;
            0 as i32
        }
    } else if if scale_factor < 0 as i32 {
        if *x < 0 as i32 as i64 {
            if (if 1 as i32 != 0 {
                0 as i32 as libc::c_ulonglong
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as libc::c_ulonglong
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                })
                    .wrapping_add(scale_factor as libc::c_ulonglong)
            })
                .wrapping_sub(1 as i32 as libc::c_ulonglong)
                < 0 as i32 as libc::c_ulonglong
            {
                ((*x as libc::c_ulonglong)
                    < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(scale_factor as libc::c_ulonglong)) as i32
            } else {
                ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                    - 1 as i32) < 0 as i32
                {
                    !(((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        + 1 as i32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                        + 1 as i32)
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) < 0 as i32
                {
                    (scale_factor
                        < -(if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                            - 1 as i32) < 0 as i32
                        {
                            ((((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                                - 1 as i32
                        })) as i32
                } else {
                    ((0 as i32) < scale_factor) as i32
                }) != 0
                {
                    ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong),
                        )
                        >> (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(1 as i32 as u64)
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(-scale_factor as libc::c_ulonglong)
                }) <= (-(1 as i32) as i64 - *x) as libc::c_ulonglong) as i32
            }
        } else if (if (if ((if 1 as i32 != 0 {
            0 as i32
        } else {
            (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
        }) - 1 as i32) < 0 as i32
        {
            !(((((if 1 as i32 != 0 {
                0 as i32
            } else {
                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
            }) + 1 as i32)
                << (::core::mem::size_of::<i32>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32 + 1 as i32)
        } else {
            (if 1 as i32 != 0 {
                0 as i32
            } else {
                (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
            }) + 0 as i32
        }) < 0 as i32
        {
            (((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                < -(if ((if 1 as i32 != 0 {
                    0 as i32
                } else {
                    (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                }) - 1 as i32) < 0 as i32
                {
                    ((((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) + 1 as i32)
                        << (::core::mem::size_of::<i32>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                        + 1 as i32
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32
                    }) - 1 as i32
                })) as i32
        } else {
            ((0 as i32)
                < (if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) + 0 as i32)
                as i32
        }) != 0 && scale_factor == -(1 as i32)
        {
            if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) - 1 as i32 as i64)
                < 0 as i32 as i64
            {
                ((0 as i32 as i64) < *x + 0 as i32 as i64) as i32
            } else {
                ((0 as i32 as i64) < *x
                    && ((-(1 as i32) - 0 as i32) as i64) < *x - 1 as i32 as i64) as i32
            }
        } else {
            (((0 as i32 / scale_factor) as i64) < *x) as i32
        }
    } else if scale_factor == 0 as i32 {
        0 as i32
    } else if *x < 0 as i32 as i64 {
        if (if (if ((if 1 as i32 != 0 {
            0 as i32 as i64
        } else {
            (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
        }) - 1 as i32 as i64) < 0 as i32 as i64
        {
            !(((((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
            }) + 1 as i32 as i64)
                << (::core::mem::size_of::<i64>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64)
        } else {
            (if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
            }) + 0 as i32 as i64
        }) < 0 as i32 as i64
        {
            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64)
                < -(if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    ((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + 0 as i32 as i64
                    }) + 1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64
                })) as i32
        } else {
            ((0 as i32 as i64)
                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { *x }) + 0 as i32 as i64)
                as i32
        }) != 0 && *x == -(1 as i32) as i64
        {
            if ((if 1 as i32 != 0 { 0 as i32 } else { scale_factor }) - 1 as i32)
                < 0 as i32
            {
                ((0 as i32) < scale_factor + 0 as i32) as i32
            } else {
                ((-(1 as i32) - 0 as i32) < scale_factor - 1 as i32) as i32
            }
        } else {
            (0 as i32 as i64 / *x < scale_factor as i64) as i32
        }
    } else {
        ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(scale_factor as libc::c_ulonglong) < *x as libc::c_ulonglong)
            as i32
    } != 0
    {
        scaled = (*x as libc::c_ulonglong)
            .wrapping_mul(scale_factor as libc::c_ulonglong) as intmax_t;
        1 as i32
    } else {
        scaled = (*x as libc::c_ulonglong)
            .wrapping_mul(scale_factor as libc::c_ulonglong) as intmax_t;
        0 as i32
    } != 0
    {
        *x = if *x < 0 as i32 as i64 {
            !if (0 as i32 as intmax_t) < -(1 as i32) as intmax_t {
                -(1 as i32) as intmax_t
            } else {
                (((1 as i32 as intmax_t)
                    << (::core::mem::size_of::<intmax_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64
            }
        } else if (0 as i32 as intmax_t) < -(1 as i32) as intmax_t {
            -(1 as i32) as intmax_t
        } else {
            (((1 as i32 as intmax_t)
                << (::core::mem::size_of::<intmax_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64
        };
        return strtol_error::LONGINT_OVERFLOW;
    }
    *x = scaled;
    return strtol_error::LONGINT_OK;
}
#[inline]
unsafe extern "C" fn strtoimax(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as i32);
}