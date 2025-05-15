use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::f128;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type va_list = __builtin_va_list;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
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
unsafe extern "C" fn dopr(
    mut buffer: *mut i8,
    mut maxlen: size_t,
    mut retlen: *mut size_t,
    mut format: *const i8,
    mut args: ::core::ffi::VaList,
) {
    let mut ch: i8 = 0;
    let mut value: libc::c_longlong = 0;
    let mut fvalue: f128::f128 = f128::f128::ZERO;
    let mut strvalue: *mut i8 = 0 as *mut i8;
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    let mut state: i32 = 0;
    let mut flags: i32 = 0;
    let mut cflags: i32 = 0;
    let mut currlen: size_t = 0;
    state = 0 as i32;
    min = 0 as i32;
    cflags = min;
    currlen = cflags as size_t;
    flags = currlen as i32;
    max = -(1 as i32);
    let fresh0 = format;
    format = format.offset(1);
    ch = *fresh0;
    if maxlen == -(1 as i32) as u64 {
        maxlen = !((1 as i32 as size_t)
            << (::core::mem::size_of::<size_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64));
    }
    while state != 7 as i32 {
        if ch as i32 == '\0' as i32 || currlen >= maxlen {
            state = 7 as i32;
        }
        match state {
            0 => {
                if ch as i32 == '%' as i32 {
                    state = 1 as i32;
                } else {
                    dopr_outch(buffer, &mut currlen, maxlen, ch as i32);
                }
                let fresh1 = format;
                format = format.offset(1);
                ch = *fresh1;
            }
            1 => {
                match ch as i32 {
                    45 => {
                        flags |= (1 as i32) << 0 as i32;
                        let fresh2 = format;
                        format = format.offset(1);
                        ch = *fresh2;
                    }
                    43 => {
                        flags |= (1 as i32) << 1 as i32;
                        let fresh3 = format;
                        format = format.offset(1);
                        ch = *fresh3;
                    }
                    32 => {
                        flags |= (1 as i32) << 2 as i32;
                        let fresh4 = format;
                        format = format.offset(1);
                        ch = *fresh4;
                    }
                    35 => {
                        flags |= (1 as i32) << 3 as i32;
                        let fresh5 = format;
                        format = format.offset(1);
                        ch = *fresh5;
                    }
                    48 => {
                        flags |= (1 as i32) << 4 as i32;
                        let fresh6 = format;
                        format = format.offset(1);
                        ch = *fresh6;
                    }
                    _ => {
                        state = 2 as i32;
                    }
                }
            }
            2 => {
                if *(*__ctype_b_loc()).offset(ch as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    min = 10 as i32 * min + (ch as i32 - '0' as i32);
                    let fresh7 = format;
                    format = format.offset(1);
                    ch = *fresh7;
                } else if ch as i32 == '*' as i32 {
                    min = args.arg::<i32>();
                    let fresh8 = format;
                    format = format.offset(1);
                    ch = *fresh8;
                    state = 3 as i32;
                } else {
                    state = 3 as i32;
                }
            }
            3 => {
                if ch as i32 == '.' as i32 {
                    state = 4 as i32;
                    let fresh9 = format;
                    format = format.offset(1);
                    ch = *fresh9;
                } else {
                    state = 5 as i32;
                }
            }
            4 => {
                if *(*__ctype_b_loc()).offset(ch as u8 as i32 as isize) as i32
                    & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
                {
                    if max < 0 as i32 {
                        max = 0 as i32;
                    }
                    max = 10 as i32 * max + (ch as i32 - '0' as i32);
                    let fresh10 = format;
                    format = format.offset(1);
                    ch = *fresh10;
                } else if ch as i32 == '*' as i32 {
                    max = args.arg::<i32>();
                    let fresh11 = format;
                    format = format.offset(1);
                    ch = *fresh11;
                    state = 5 as i32;
                } else {
                    state = 5 as i32;
                }
            }
            5 => {
                match ch as i32 {
                    104 => {
                        cflags = 1 as i32;
                        let fresh12 = format;
                        format = format.offset(1);
                        ch = *fresh12;
                    }
                    108 => {
                        if *format as i32 == 'l' as i32 {
                            cflags = 4 as i32;
                            format = format.offset(1);
                            format;
                        } else {
                            cflags = 2 as i32;
                        }
                        let fresh13 = format;
                        format = format.offset(1);
                        ch = *fresh13;
                    }
                    113 => {
                        cflags = 4 as i32;
                        let fresh14 = format;
                        format = format.offset(1);
                        ch = *fresh14;
                    }
                    76 => {
                        cflags = 3 as i32;
                        let fresh15 = format;
                        format = format.offset(1);
                        ch = *fresh15;
                    }
                    _ => {}
                }
                state = 6 as i32;
            }
            6 => {
                let mut current_block_114: u64;
                match ch as i32 {
                    100 | 105 => {
                        match cflags {
                            1 => {
                                value = args.arg::<i32>() as libc::c_short
                                    as libc::c_longlong;
                            }
                            2 => {
                                value = args.arg::<i64>() as libc::c_longlong;
                            }
                            4 => {
                                value = args.arg::<libc::c_longlong>();
                            }
                            _ => {
                                value = args.arg::<i32>() as libc::c_longlong;
                            }
                        }
                        fmtint(
                            buffer,
                            &mut currlen,
                            maxlen,
                            value,
                            10 as i32,
                            min,
                            max,
                            flags,
                        );
                        current_block_114 = 16314074004867283505;
                    }
                    88 => {
                        flags |= (1 as i32) << 5 as i32;
                        current_block_114 = 12027190426053135769;
                    }
                    120 | 111 | 117 => {
                        current_block_114 = 12027190426053135769;
                    }
                    102 => {
                        if cflags == 3 as i32 {
                            fvalue = args.arg::<f128::f128>();
                        } else {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        }
                        fmtfp(buffer, &mut currlen, maxlen, fvalue, min, max, flags);
                        current_block_114 = 16314074004867283505;
                    }
                    69 => {
                        flags |= (1 as i32) << 5 as i32;
                        current_block_114 = 9541762626985535097;
                    }
                    101 => {
                        current_block_114 = 9541762626985535097;
                    }
                    71 => {
                        flags |= (1 as i32) << 5 as i32;
                        current_block_114 = 11743040019909382591;
                    }
                    103 => {
                        current_block_114 = 11743040019909382591;
                    }
                    99 => {
                        dopr_outch(buffer, &mut currlen, maxlen, args.arg::<i32>());
                        current_block_114 = 16314074004867283505;
                    }
                    115 => {
                        strvalue = args.arg::<*mut i8>();
                        if max < 0 as i32 {
                            max = maxlen as i32;
                        }
                        fmtstr(buffer, &mut currlen, maxlen, strvalue, flags, min, max);
                        current_block_114 = 16314074004867283505;
                    }
                    112 => {
                        value = args.arg::<*mut libc::c_void>() as i64
                            as libc::c_longlong;
                        fmtint(
                            buffer,
                            &mut currlen,
                            maxlen,
                            value,
                            16 as i32,
                            min,
                            max,
                            flags,
                        );
                        current_block_114 = 16314074004867283505;
                    }
                    110 => {
                        if cflags == 1 as i32 {
                            let mut num: *mut libc::c_short = 0 as *mut libc::c_short;
                            num = args.arg::<*mut libc::c_short>();
                            *num = currlen as libc::c_short;
                        } else if cflags == 2 as i32 {
                            let mut num_0: *mut i64 = 0 as *mut i64;
                            num_0 = args.arg::<*mut i64>();
                            *num_0 = currlen as i64;
                        } else if cflags == 4 as i32 {
                            let mut num_1: *mut libc::c_longlong = 0
                                as *mut libc::c_longlong;
                            num_1 = args.arg::<*mut libc::c_longlong>();
                            *num_1 = currlen as libc::c_longlong;
                        } else {
                            let mut num_2: *mut i32 = 0 as *mut i32;
                            num_2 = args.arg::<*mut i32>();
                            *num_2 = currlen as i32;
                        }
                        current_block_114 = 16314074004867283505;
                    }
                    37 => {
                        dopr_outch(buffer, &mut currlen, maxlen, ch as i32);
                        current_block_114 = 16314074004867283505;
                    }
                    119 => {
                        let fresh16 = format;
                        format = format.offset(1);
                        ch = *fresh16;
                        current_block_114 = 16314074004867283505;
                    }
                    _ => {
                        current_block_114 = 16314074004867283505;
                    }
                }
                match current_block_114 {
                    12027190426053135769 => {
                        flags |= (1 as i32) << 6 as i32;
                        match cflags {
                            1 => {
                                value = args.arg::<u32>() as libc::c_ushort
                                    as libc::c_longlong;
                            }
                            2 => {
                                value = args.arg::<u64>() as libc::c_longlong;
                            }
                            4 => {
                                value = args.arg::<libc::c_ulonglong>() as libc::c_longlong;
                            }
                            _ => {
                                value = args.arg::<u32>() as libc::c_longlong;
                            }
                        }
                        fmtint(
                            buffer,
                            &mut currlen,
                            maxlen,
                            value,
                            if ch as i32 == 'o' as i32 {
                                8 as i32
                            } else if ch as i32 == 'u' as i32 {
                                10 as i32
                            } else {
                                16 as i32
                            },
                            min,
                            max,
                            flags,
                        );
                    }
                    9541762626985535097 => {
                        if cflags == 3 as i32 {
                            fvalue = args.arg::<f128::f128>();
                        } else {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        }
                    }
                    11743040019909382591 => {
                        if cflags == 3 as i32 {
                            fvalue = args.arg::<f128::f128>();
                        } else {
                            fvalue = f128::f128::new(args.arg::<libc::c_double>());
                        }
                    }
                    _ => {}
                }
                let fresh17 = format;
                format = format.offset(1);
                ch = *fresh17;
                state = 0 as i32;
                min = 0 as i32;
                cflags = min;
                flags = cflags;
                max = -(1 as i32);
            }
            7 | _ => {}
        }
    }
    if currlen >= maxlen.wrapping_sub(1 as i32 as u64) {
        currlen = maxlen.wrapping_sub(1 as i32 as u64);
    }
    if !buffer.is_null() {
        *buffer.offset(currlen as isize) = '\0' as i32 as i8;
    }
    *retlen = currlen;
}
unsafe extern "C" fn fmtstr(
    mut buffer: *mut i8,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut value: *mut i8,
    mut flags: i32,
    mut min: i32,
    mut max: i32,
) {
    let mut padlen: i32 = 0;
    let mut strln: i32 = 0;
    let mut cnt: i32 = 0 as i32;
    if value.is_null() {
        value = b"<NULL>\0" as *const u8 as *const i8 as *mut i8;
    }
    strln = 0 as i32;
    while *value.offset(strln as isize) as i32 != '\0' as i32 {
        strln += 1;
        strln;
    }
    padlen = min - strln;
    if padlen < 0 as i32 {
        padlen = 0 as i32;
    }
    if flags & (1 as i32) << 0 as i32 != 0 {
        padlen = -padlen;
    }
    while padlen > 0 as i32 && cnt < max {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen -= 1;
        padlen;
        cnt += 1;
        cnt;
    }
    while *value as i32 != 0 && cnt < max {
        let fresh18 = value;
        value = value.offset(1);
        dopr_outch(buffer, currlen, maxlen, *fresh18 as i32);
        cnt += 1;
        cnt;
    }
    while padlen < 0 as i32 && cnt < max {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen += 1;
        padlen;
        cnt += 1;
        cnt;
    }
}
unsafe extern "C" fn fmtint(
    mut buffer: *mut i8,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut value: libc::c_longlong,
    mut base: i32,
    mut min: i32,
    mut max: i32,
    mut flags: i32,
) {
    let mut signvalue: i32 = 0 as i32;
    let mut uvalue: libc::c_ulonglong = 0;
    let mut convert: [i8; 20] = [0; 20];
    let mut place: i32 = 0 as i32;
    let mut spadlen: i32 = 0 as i32;
    let mut zpadlen: i32 = 0 as i32;
    let mut caps: i32 = 0 as i32;
    if max < 0 as i32 {
        max = 0 as i32;
    }
    uvalue = value as libc::c_ulonglong;
    if flags & (1 as i32) << 6 as i32 == 0 {
        if value < 0 as i32 as libc::c_longlong {
            signvalue = '-' as i32;
            uvalue = -value as libc::c_ulonglong;
        } else if flags & (1 as i32) << 1 as i32 != 0 {
            signvalue = '+' as i32;
        } else if flags & (1 as i32) << 2 as i32 != 0 {
            signvalue = ' ' as i32;
        }
    }
    if flags & (1 as i32) << 5 as i32 != 0 {
        caps = 1 as i32;
    }
    loop {
        let fresh19 = place;
        place = place + 1;
        convert[fresh19 as usize] = *if caps != 0 {
            b"0123456789ABCDEF\0" as *const u8 as *const i8
        } else {
            b"0123456789abcdef\0" as *const u8 as *const i8
        }
            .offset(uvalue.wrapping_rem(base as u32 as libc::c_ulonglong) as isize);
        uvalue = uvalue.wrapping_div(base as u32 as libc::c_ulonglong);
        if !(uvalue != 0 && place < 20 as i32) {
            break;
        }
    }
    if place == 20 as i32 {
        place -= 1;
        place;
    }
    convert[place as usize] = 0 as i32 as i8;
    zpadlen = max - place;
    spadlen = min - (if max >= place { max } else { place })
        - (if signvalue != 0 { 1 as i32 } else { 0 as i32 });
    if zpadlen < 0 as i32 {
        zpadlen = 0 as i32;
    }
    if spadlen < 0 as i32 {
        spadlen = 0 as i32;
    }
    if flags & (1 as i32) << 4 as i32 != 0 {
        zpadlen = if zpadlen >= spadlen { zpadlen } else { spadlen };
        spadlen = 0 as i32;
    }
    if flags & (1 as i32) << 0 as i32 != 0 {
        spadlen = -spadlen;
    }
    while spadlen > 0 as i32 {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        spadlen -= 1;
        spadlen;
    }
    if signvalue != 0 {
        dopr_outch(buffer, currlen, maxlen, signvalue);
    }
    if zpadlen > 0 as i32 {
        while zpadlen > 0 as i32 {
            dopr_outch(buffer, currlen, maxlen, '0' as i32);
            zpadlen -= 1;
            zpadlen;
        }
    }
    while place > 0 as i32 {
        place -= 1;
        dopr_outch(buffer, currlen, maxlen, convert[place as usize] as i32);
    }
    while spadlen < 0 as i32 {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        spadlen += 1;
        spadlen;
    }
}
unsafe extern "C" fn math_abs(mut value: f128::f128) -> f128::f128 {
    let mut result: f128::f128 = value;
    if value < f128::f128::new(0 as i32) {
        result = -value;
    }
    return result;
}
unsafe extern "C" fn math_pow10(mut exponent: i32) -> f128::f128 {
    let mut result: f128::f128 = f128::f128::new(1 as i32);
    while exponent > 0 as i32 {
        result *= f128::f128::new(10 as i32);
        exponent -= 1;
        exponent;
    }
    return result;
}
unsafe extern "C" fn math_round(mut value: f128::f128) -> i64 {
    let mut intpart: i64 = 0;
    intpart = value.to_i64().unwrap();
    value = value - f128::f128::new(intpart);
    if value >= f128::f128::new(0.5f64) {
        intpart += 1;
        intpart;
    }
    return intpart;
}
unsafe extern "C" fn fmtfp(
    mut buffer: *mut i8,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut fvalue: f128::f128,
    mut min: i32,
    mut max: i32,
    mut flags: i32,
) {
    let mut signvalue: i32 = 0 as i32;
    let mut ufvalue: f128::f128 = f128::f128::ZERO;
    let mut iconvert: [i8; 20] = [0; 20];
    let mut fconvert: [i8; 20] = [0; 20];
    let mut iplace: i32 = 0 as i32;
    let mut fplace: i32 = 0 as i32;
    let mut padlen: i32 = 0 as i32;
    let mut zpadlen: i32 = 0 as i32;
    let mut caps: i32 = 0 as i32;
    let mut intpart: i64 = 0;
    let mut fracpart: i64 = 0;
    if max < 0 as i32 {
        max = 6 as i32;
    }
    ufvalue = math_abs(fvalue);
    if fvalue < f128::f128::new(0 as i32) {
        signvalue = '-' as i32;
    } else if flags & (1 as i32) << 1 as i32 != 0 {
        signvalue = '+' as i32;
    } else if flags & (1 as i32) << 2 as i32 != 0 {
        signvalue = ' ' as i32;
    }
    intpart = ufvalue.to_i64().unwrap();
    if max > 9 as i32 {
        max = 9 as i32;
    }
    fracpart = math_round(math_pow10(max) * (ufvalue - f128::f128::new(intpart)));
    if f128::f128::new(fracpart) >= math_pow10(max) {
        intpart += 1;
        intpart;
        fracpart = (f128::f128::from(fracpart) - math_pow10(max)).to_i64().unwrap();
    }
    loop {
        let fresh20 = iplace;
        iplace = iplace + 1;
        iconvert[fresh20 as usize] = *if caps != 0 {
            b"0123456789ABCDEF\0" as *const u8 as *const i8
        } else {
            b"0123456789abcdef\0" as *const u8 as *const i8
        }
            .offset((intpart % 10 as i32 as i64) as isize);
        intpart = intpart / 10 as i32 as i64;
        if !(intpart != 0 && iplace < 20 as i32) {
            break;
        }
    }
    if iplace == 20 as i32 {
        iplace -= 1;
        iplace;
    }
    iconvert[iplace as usize] = 0 as i32 as i8;
    loop {
        let fresh21 = fplace;
        fplace = fplace + 1;
        fconvert[fresh21 as usize] = *if caps != 0 {
            b"0123456789ABCDEF\0" as *const u8 as *const i8
        } else {
            b"0123456789abcdef\0" as *const u8 as *const i8
        }
            .offset((fracpart % 10 as i32 as i64) as isize);
        fracpart = fracpart / 10 as i32 as i64;
        if !(fracpart != 0 && fplace < 20 as i32) {
            break;
        }
    }
    if fplace == 20 as i32 {
        fplace -= 1;
        fplace;
    }
    fconvert[fplace as usize] = 0 as i32 as i8;
    padlen = min - iplace - max - 1 as i32
        - (if signvalue != 0 { 1 as i32 } else { 0 as i32 });
    zpadlen = max - fplace;
    if zpadlen < 0 as i32 {
        zpadlen = 0 as i32;
    }
    if padlen < 0 as i32 {
        padlen = 0 as i32;
    }
    if flags & (1 as i32) << 0 as i32 != 0 {
        padlen = -padlen;
    }
    if flags & (1 as i32) << 4 as i32 != 0 && padlen > 0 as i32 {
        if signvalue != 0 {
            dopr_outch(buffer, currlen, maxlen, signvalue);
            padlen -= 1;
            padlen;
            signvalue = 0 as i32;
        }
        while padlen > 0 as i32 {
            dopr_outch(buffer, currlen, maxlen, '0' as i32);
            padlen -= 1;
            padlen;
        }
    }
    while padlen > 0 as i32 {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen -= 1;
        padlen;
    }
    if signvalue != 0 {
        dopr_outch(buffer, currlen, maxlen, signvalue);
    }
    while iplace > 0 as i32 {
        iplace -= 1;
        dopr_outch(buffer, currlen, maxlen, iconvert[iplace as usize] as i32);
    }
    if max > 0 as i32 {
        dopr_outch(buffer, currlen, maxlen, '.' as i32);
        while fplace > 0 as i32 {
            fplace -= 1;
            dopr_outch(buffer, currlen, maxlen, fconvert[fplace as usize] as i32);
        }
    }
    while zpadlen > 0 as i32 {
        dopr_outch(buffer, currlen, maxlen, '0' as i32);
        zpadlen -= 1;
        zpadlen;
    }
    while padlen < 0 as i32 {
        dopr_outch(buffer, currlen, maxlen, ' ' as i32);
        padlen += 1;
        padlen;
    }
}
unsafe extern "C" fn dopr_outch(
    mut buffer: *mut i8,
    mut currlen: *mut size_t,
    mut maxlen: size_t,
    mut c: i32,
) {
    if *currlen < maxlen {
        if !buffer.is_null() {
            *buffer.offset(*currlen as isize) = c as i8;
        }
        *currlen = (*currlen).wrapping_add(1);
        *currlen;
    }
}
#[no_mangle]
pub unsafe extern "C" fn __pth_vsnprintf(
    mut str: *mut i8,
    mut count: size_t,
    mut fmt: *const i8,
    mut args: ::core::ffi::VaList,
) -> i32 {
    let mut retlen: size_t = 0;
    if !str.is_null() {
        *str.offset(0 as i32 as isize) = '\0' as i32 as i8;
    }
    dopr(str, count, &mut retlen, fmt, args.as_va_list());
    return retlen as i32;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_snprintf(
    mut str: *mut i8,
    mut count: size_t,
    mut fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rv: i32 = 0;
    ap = args.clone();
    rv = __pth_vsnprintf(str, count, fmt, ap.as_va_list());
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_vasprintf(
    mut fmt: *const i8,
    mut ap: ::core::ffi::VaList,
) -> *mut i8 {
    let mut rv: *mut i8 = 0 as *mut i8;
    let mut n: i32 = 0;
    n = __pth_vsnprintf(0 as *mut i8, -(1 as i32) as size_t, fmt, ap.as_va_list());
    rv = malloc((n + 1 as i32) as u64) as *mut i8;
    if rv.is_null() {
        return 0 as *mut i8;
    }
    __pth_vsnprintf(rv, (n + 1 as i32) as size_t, fmt, ap.as_va_list());
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_asprintf(mut fmt: *const i8, mut args: ...) -> *mut i8 {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rv: *mut i8 = 0 as *mut i8;
    ap = args.clone();
    rv = __pth_vasprintf(fmt, ap.as_va_list());
    return rv;
}