#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    fn mktime_z(__tz: timezone_t, __result: *mut tm) -> time_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
pub type __int32_t = i32;
pub type __time_t = i64;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm_zone {
    pub next: *mut tm_zone,
    pub tz_is_set: i8,
    pub abbrs: [i8; 0],
}
pub type timezone_t = *mut tm_zone;
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn memcpy_lowcase(
    mut dest: *mut i8,
    mut src: *const i8,
    mut len: size_t,
) -> *mut i8 {
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 > 0 as i32 as u64) {
            break;
        }
        *dest.offset(len as isize) = ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = *src.offset(len as isize) as u8 as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(*src.offset(len as isize) as u8 as i32);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(*src.offset(len as isize) as u8 as i32 as isize);
            }
            __res
        }) as i8;
    }
    return dest;
}
unsafe extern "C" fn memcpy_uppcase(
    mut dest: *mut i8,
    mut src: *const i8,
    mut len: size_t,
) -> *mut i8 {
    loop {
        let fresh1 = len;
        len = len.wrapping_sub(1);
        if !(fresh1 > 0 as i32 as u64) {
            break;
        }
        *dest.offset(len as isize) = ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = *src.offset(len as isize) as u8 as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    };
                } else {
                    __res = toupper(*src.offset(len as isize) as u8 as i32);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*src.offset(len as isize) as u8 as i32 as isize);
            }
            __res
        }) as i8;
    }
    return dest;
}
#[inline]
unsafe extern "C" fn iso_week_days(mut yday: i32, mut wday: i32) -> i32 {
    let mut big_enough_multiple_of_7: i32 = (--(366 as i32) / 7 as i32 + 2 as i32)
        * 7 as i32;
    return yday - (yday - wday + 4 as i32 + big_enough_multiple_of_7) % 7 as i32
        + 4 as i32 - 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn nstrftime(
    mut s: *mut i8,
    mut maxsize: size_t,
    mut format: *const i8,
    mut tp: *const tm,
    mut tz: timezone_t,
    mut ns: i32,
) -> size_t {
    let mut tzset_called: bool = 0 as i32 != 0;
    return __strftime_internal(
        s,
        maxsize,
        format,
        tp,
        0 as i32 != 0,
        0 as i32,
        -(1 as i32),
        &mut tzset_called,
        tz,
        ns,
    );
}
unsafe extern "C" fn __strftime_internal(
    mut s: *mut i8,
    mut maxsize: size_t,
    mut format: *const i8,
    mut tp: *const tm,
    mut upcase: bool,
    mut yr_spec: i32,
    mut width: i32,
    mut tzset_called: *mut bool,
    mut tz: timezone_t,
    mut ns: i32,
) -> size_t {
    let mut current_block: u64;
    let mut saved_errno: i32 = *__errno_location();
    let mut hour12: i32 = (*tp).tm_hour;
    let mut zone: *const i8 = 0 as *const i8;
    let mut i: size_t = 0 as i32 as size_t;
    let mut p: *mut i8 = s;
    let mut f: *const i8 = 0 as *const i8;
    zone = 0 as *const i8;
    zone = (*tp).tm_zone;
    if zone.is_null() {
        zone = b"\0" as *const u8 as *const i8;
    }
    if hour12 > 12 as i32 {
        hour12 -= 12 as i32;
    } else if hour12 == 0 as i32 {
        hour12 = 12 as i32;
    }
    f = format;
    while *f as i32 != '\0' as i32 {
        let mut pad: i32 = 0 as i32;
        let mut modifier: i32 = 0;
        let mut digits: i32 = 0 as i32;
        let mut number_value: i32 = 0;
        let mut u_number_value: u32 = 0;
        let mut negative_number: bool = false;
        let mut always_output_a_sign: bool = false;
        let mut tz_colon_mask: i32 = 0;
        let mut subfmt: *const i8 = 0 as *const i8;
        let mut bufp: *mut i8 = 0 as *mut i8;
        let mut buf: [i8; 23] = [0; 23];
        let mut to_lowcase: bool = 0 as i32 != 0;
        let mut to_uppcase: bool = upcase;
        let mut colons: size_t = 0;
        let mut change_case: bool = 0 as i32 != 0;
        let mut format_char: i32 = 0;
        let mut subwidth: i32 = 0;
        if *f as i32 != '%' as i32 {
            let mut _n: size_t = 1 as i32 as size_t;
            let mut _w: size_t = (if pad == '-' as i32 || width < 0 as i32 {
                0 as i32
            } else {
                width
            }) as size_t;
            let mut _incr: size_t = if _n < _w { _w } else { _n };
            if _incr >= maxsize.wrapping_sub(i) {
                *__errno_location() = 34 as i32;
                return 0 as i32 as size_t;
            }
            if !p.is_null() {
                if _n < _w {
                    let mut _delta: size_t = _w.wrapping_sub(_n);
                    if pad == '0' as i32 || pad == '+' as i32 {
                        memset(p as *mut libc::c_void, '0' as i32, _delta);
                        p = p.offset(_delta as isize);
                    } else {
                        memset(p as *mut libc::c_void, ' ' as i32, _delta);
                        p = p.offset(_delta as isize);
                    }
                }
                *p = *f;
                p = p.offset(_n as isize);
            }
            i = (i as u64).wrapping_add(_incr) as size_t as size_t;
        } else {
            loop {
                f = f.offset(1);
                match *f as i32 {
                    95 | 45 | 43 | 48 => {
                        pad = *f as i32;
                    }
                    94 => {
                        to_uppcase = 1 as i32 != 0;
                    }
                    35 => {
                        change_case = 1 as i32 != 0;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if (*f as u32).wrapping_sub('0' as i32 as u32) <= 9 as i32 as u32 {
                width = 0 as i32;
                loop {
                    if (if ::core::mem::size_of::<i32>() as u64
                        == ::core::mem::size_of::<libc::c_schar>() as u64
                    {
                        (if !((0 as i32) < -(1 as i32)) {
                            (if (if (10 as i32) < 0 as i32 {
                                (if width < 0 as i32 {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 127 as i32 })
                                            + 10 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        (width < 127 as i32 / 10 as i32) as i32
                                    } else {
                                        ((if (if (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            10 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32)
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 0 as i32
                                        }) < 0 as i32
                                        {
                                            ((10 as i32)
                                                < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    - 1 as i32) < 0 as i32
                                                {
                                                    ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        - 1 as i32
                                                })) as i32
                                        } else {
                                            ((0 as i32) < 10 as i32) as i32
                                        }) != 0
                                        {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 127 as i32
                                                >> (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(1 as i32 as u64)
                                        } else {
                                            127 as i32 / -(10 as i32)
                                        }) <= -(1 as i32) - width) as i32
                                    })
                                } else {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                            + (-(127 as i32) - 1 as i32)
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + (-(127 as i32) - 1 as i32)
                                        }) + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                            + (-(127 as i32) - 1 as i32)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + (-(127 as i32) - 1 as i32)
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + (-(127 as i32) - 1 as i32)
                                                }) - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32)
                                            < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + (-(127 as i32) - 1 as i32)) as i32
                                    }) != 0 && 10 as i32 == -(1 as i32)
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32) < width + (-(127 as i32) - 1 as i32)) as i32
                                        } else {
                                            ((0 as i32) < width
                                                && -(1 as i32) - (-(127 as i32) - 1 as i32)
                                                    < width - 1 as i32) as i32
                                        })
                                    } else {
                                        (((-(127 as i32) - 1 as i32) / 10 as i32) < width) as i32
                                    })
                                })
                            } else {
                                (if 10 as i32 == 0 as i32 {
                                    0 as i32
                                } else {
                                    (if width < 0 as i32 {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                + (-(127 as i32) - 1 as i32)
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { width })
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
                                                (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                    + (-(127 as i32) - 1 as i32)
                                            }) + 0 as i32
                                        }) < 0 as i32
                                        {
                                            ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                + (-(127 as i32) - 1 as i32)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        + (-(127 as i32) - 1 as i32)
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width })
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
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                            + (-(127 as i32) - 1 as i32)
                                                    }) - 1 as i32
                                                })) as i32
                                        } else {
                                            ((0 as i32)
                                                < (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                    + (-(127 as i32) - 1 as i32)) as i32
                                        }) != 0 && width == -(1 as i32)
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((0 as i32) < 10 as i32 + (-(127 as i32) - 1 as i32)) as i32
                                            } else {
                                                (-(1 as i32) - (-(127 as i32) - 1 as i32)
                                                    < 10 as i32 - 1 as i32) as i32
                                            })
                                        } else {
                                            ((-(127 as i32) - 1 as i32) / width < 10 as i32) as i32
                                        })
                                    } else {
                                        ((127 as i32 / 10 as i32) < width) as i32
                                    })
                                })
                            }) != 0
                            {
                                width = (width as u32).wrapping_mul(10 as i32 as u32)
                                    as libc::c_schar as i32;
                                1 as i32
                            } else {
                                width = (width as u32).wrapping_mul(10 as i32 as u32)
                                    as libc::c_schar as i32;
                                0 as i32
                            })
                        } else {
                            (if (if (10 as i32) < 0 as i32 {
                                (if width < 0 as i32 {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            127 as i32 * 2 as i32 + 1 as i32
                                        }) + 10 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        (width < (127 as i32 * 2 as i32 + 1 as i32) / 10 as i32)
                                            as i32
                                    } else {
                                        ((if (if (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            10 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32)
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 0 as i32
                                        }) < 0 as i32
                                        {
                                            ((10 as i32)
                                                < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    - 1 as i32) < 0 as i32
                                                {
                                                    ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        - 1 as i32
                                                })) as i32
                                        } else {
                                            ((0 as i32) < 10 as i32) as i32
                                        }) != 0
                                        {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + (127 as i32 * 2 as i32 + 1 as i32)
                                                >> (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(1 as i32 as u64)
                                        } else {
                                            (127 as i32 * 2 as i32 + 1 as i32) / -(10 as i32)
                                        }) <= -(1 as i32) - width) as i32
                                    })
                                } else {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                            + 0 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 0 as i32
                                        }) + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 0 as i32
                                        }) + 0 as i32
                                    }) < 0 as i32
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                            + 0 as i32)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32)
                                            < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 0 as i32) as i32
                                    }) != 0 && 10 as i32 == -(1 as i32)
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32) < width + 0 as i32) as i32
                                        } else {
                                            ((0 as i32) < width
                                                && (-(1 as i32) - 0 as i32) < width - 1 as i32) as i32
                                        })
                                    } else {
                                        ((0 as i32 / 10 as i32) < width) as i32
                                    })
                                })
                            } else {
                                (if 10 as i32 == 0 as i32 {
                                    0 as i32
                                } else {
                                    (if width < 0 as i32 {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                            }) + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32)
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                            }) + 0 as i32
                                        }) < 0 as i32
                                        {
                                            (((if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                    }) + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                    }) - 1 as i32
                                                })) as i32
                                        } else {
                                            ((0 as i32)
                                                < (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                as i32
                                        }) != 0 && width == -(1 as i32)
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((0 as i32) < 10 as i32 + 0 as i32) as i32
                                            } else {
                                                ((-(1 as i32) - 0 as i32) < 10 as i32 - 1 as i32) as i32
                                            })
                                        } else {
                                            (0 as i32 / width < 10 as i32) as i32
                                        })
                                    } else {
                                        (((127 as i32 * 2 as i32 + 1 as i32) / 10 as i32) < width)
                                            as i32
                                    })
                                })
                            }) != 0
                            {
                                width = (width as u32).wrapping_mul(10 as i32 as u32) as u8
                                    as i32;
                                1 as i32
                            } else {
                                width = (width as u32).wrapping_mul(10 as i32 as u32) as u8
                                    as i32;
                                0 as i32
                            })
                        })
                    } else {
                        (if ::core::mem::size_of::<i32>() as u64
                            == ::core::mem::size_of::<libc::c_short>() as u64
                        {
                            (if !((0 as i32) < -(1 as i32)) {
                                (if (if (10 as i32) < 0 as i32 {
                                    (if width < 0 as i32 {
                                        (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 32767 as i32 })
                                                + 10 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            (width < 32767 as i32 / 10 as i32) as i32
                                        } else {
                                            ((if (if (if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                10 as i32
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32)
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32
                                            }) < 0 as i32
                                            {
                                                ((10 as i32)
                                                    < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32
                                                    })) as i32
                                            } else {
                                                ((0 as i32) < 10 as i32) as i32
                                            }) != 0
                                            {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 32767 as i32
                                                    >> (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64)
                                            } else {
                                                32767 as i32 / -(10 as i32)
                                            }) <= -(1 as i32) - width) as i32
                                        })
                                    } else {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + (-(32767 as i32) - 1 as i32)
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + (-(32767 as i32) - 1 as i32)
                                            }) + 0 as i32
                                        }) < 0 as i32
                                        {
                                            ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + (-(32767 as i32) - 1 as i32)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + (-(32767 as i32) - 1 as i32)
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + (-(32767 as i32) - 1 as i32)
                                                    }) - 1 as i32
                                                })) as i32
                                        } else {
                                            ((0 as i32)
                                                < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + (-(32767 as i32) - 1 as i32)) as i32
                                        }) != 0 && 10 as i32 == -(1 as i32)
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((0 as i32) < width + (-(32767 as i32) - 1 as i32)) as i32
                                            } else {
                                                ((0 as i32) < width
                                                    && -(1 as i32) - (-(32767 as i32) - 1 as i32)
                                                        < width - 1 as i32) as i32
                                            })
                                        } else {
                                            (((-(32767 as i32) - 1 as i32) / 10 as i32) < width) as i32
                                        })
                                    })
                                } else {
                                    (if 10 as i32 == 0 as i32 {
                                        0 as i32
                                    } else {
                                        (if width < 0 as i32 {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                    + (-(32767 as i32) - 1 as i32)
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width })
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
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        + (-(32767 as i32) - 1 as i32)
                                                }) + 0 as i32
                                            }) < 0 as i32
                                            {
                                                ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                    + (-(32767 as i32) - 1 as i32)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                            + (-(32767 as i32) - 1 as i32)
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width })
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
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                + (-(32767 as i32) - 1 as i32)
                                                        }) - 1 as i32
                                                    })) as i32
                                            } else {
                                                ((0 as i32)
                                                    < (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        + (-(32767 as i32) - 1 as i32)) as i32
                                            }) != 0 && width == -(1 as i32)
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    - 1 as i32) < 0 as i32
                                                {
                                                    ((0 as i32) < 10 as i32 + (-(32767 as i32) - 1 as i32))
                                                        as i32
                                                } else {
                                                    (-(1 as i32) - (-(32767 as i32) - 1 as i32)
                                                        < 10 as i32 - 1 as i32) as i32
                                                })
                                            } else {
                                                ((-(32767 as i32) - 1 as i32) / width < 10 as i32) as i32
                                            })
                                        } else {
                                            ((32767 as i32 / 10 as i32) < width) as i32
                                        })
                                    })
                                }) != 0
                                {
                                    width = (width as u32).wrapping_mul(10 as i32 as u32)
                                        as libc::c_short as i32;
                                    1 as i32
                                } else {
                                    width = (width as u32).wrapping_mul(10 as i32 as u32)
                                        as libc::c_short as i32;
                                    0 as i32
                                })
                            } else {
                                (if (if (10 as i32) < 0 as i32 {
                                    (if width < 0 as i32 {
                                        (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                32767 as i32 * 2 as i32 + 1 as i32
                                            }) + 10 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            (width < (32767 as i32 * 2 as i32 + 1 as i32) / 10 as i32)
                                                as i32
                                        } else {
                                            ((if (if (if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                10 as i32
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32)
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32
                                            }) < 0 as i32
                                            {
                                                ((10 as i32)
                                                    < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32
                                                    })) as i32
                                            } else {
                                                ((0 as i32) < 10 as i32) as i32
                                            }) != 0
                                            {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + (32767 as i32 * 2 as i32 + 1 as i32)
                                                    >> (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64)
                                            } else {
                                                (32767 as i32 * 2 as i32 + 1 as i32) / -(10 as i32)
                                            }) <= -(1 as i32) - width) as i32
                                        })
                                    } else {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 0 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32
                                            }) + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32)
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32
                                            }) + 0 as i32
                                        }) < 0 as i32
                                        {
                                            (((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                + 0 as i32)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) - 1 as i32
                                                })) as i32
                                        } else {
                                            ((0 as i32)
                                                < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32) as i32
                                        }) != 0 && 10 as i32 == -(1 as i32)
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((0 as i32) < width + 0 as i32) as i32
                                            } else {
                                                ((0 as i32) < width
                                                    && (-(1 as i32) - 0 as i32) < width - 1 as i32) as i32
                                            })
                                        } else {
                                            ((0 as i32 / 10 as i32) < width) as i32
                                        })
                                    })
                                } else {
                                    (if 10 as i32 == 0 as i32 {
                                        0 as i32
                                    } else {
                                        (if width < 0 as i32 {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                }) + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32)
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                }) + 0 as i32
                                            }) < 0 as i32
                                            {
                                                (((if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                        }) + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                        }) - 1 as i32
                                                    })) as i32
                                            } else {
                                                ((0 as i32)
                                                    < (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                    as i32
                                            }) != 0 && width == -(1 as i32)
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    - 1 as i32) < 0 as i32
                                                {
                                                    ((0 as i32) < 10 as i32 + 0 as i32) as i32
                                                } else {
                                                    ((-(1 as i32) - 0 as i32) < 10 as i32 - 1 as i32) as i32
                                                })
                                            } else {
                                                (0 as i32 / width < 10 as i32) as i32
                                            })
                                        } else {
                                            (((32767 as i32 * 2 as i32 + 1 as i32) / 10 as i32) < width)
                                                as i32
                                        })
                                    })
                                }) != 0
                                {
                                    width = (width as u32).wrapping_mul(10 as i32 as u32)
                                        as libc::c_ushort as i32;
                                    1 as i32
                                } else {
                                    width = (width as u32).wrapping_mul(10 as i32 as u32)
                                        as libc::c_ushort as i32;
                                    0 as i32
                                })
                            })
                        } else {
                            (if ::core::mem::size_of::<i32>() as u64
                                == ::core::mem::size_of::<i32>() as u64
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                    - 1 as i32) < 0 as i32
                                {
                                    (if (if (10 as i32) < 0 as i32 {
                                        (if width < 0 as i32 {
                                            (if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 2147483647 as i32 })
                                                    + 10 as i32
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                (width < 2147483647 as i32 / 10 as i32) as i32
                                            } else {
                                                ((if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    10 as i32
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32)
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) < 0 as i32
                                                {
                                                    ((10 as i32)
                                                        < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32) < 0 as i32
                                                        {
                                                            ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                + 1 as i32)
                                                                << (::core::mem::size_of::<i32>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                + 1 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                - 1 as i32
                                                        })) as i32
                                                } else {
                                                    ((0 as i32) < 10 as i32) as i32
                                                }) != 0
                                                {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 2147483647 as i32
                                                        >> (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(1 as i32 as u64)
                                                } else {
                                                    2147483647 as i32 / -(10 as i32)
                                                }) <= -(1 as i32) - width) as i32
                                            })
                                        } else {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + (-(2147483647 as i32) - 1 as i32)
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + (-(2147483647 as i32) - 1 as i32)
                                                }) + 0 as i32
                                            }) < 0 as i32
                                            {
                                                ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + (-(2147483647 as i32) - 1 as i32)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + (-(2147483647 as i32) - 1 as i32)
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                + (-(2147483647 as i32) - 1 as i32)
                                                        }) - 1 as i32
                                                    })) as i32
                                            } else {
                                                ((0 as i32)
                                                    < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + (-(2147483647 as i32) - 1 as i32)) as i32
                                            }) != 0 && 10 as i32 == -(1 as i32)
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                    - 1 as i32) < 0 as i32
                                                {
                                                    ((0 as i32) < width + (-(2147483647 as i32) - 1 as i32))
                                                        as i32
                                                } else {
                                                    ((0 as i32) < width
                                                        && -(1 as i32) - (-(2147483647 as i32) - 1 as i32)
                                                            < width - 1 as i32) as i32
                                                })
                                            } else {
                                                (((-(2147483647 as i32) - 1 as i32) / 10 as i32) < width)
                                                    as i32
                                            })
                                        })
                                    } else {
                                        (if 10 as i32 == 0 as i32 {
                                            0 as i32
                                        } else {
                                            (if width < 0 as i32 {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        + (-(2147483647 as i32) - 1 as i32)
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width })
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
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                            + (-(2147483647 as i32) - 1 as i32)
                                                    }) + 0 as i32
                                                }) < 0 as i32
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        + (-(2147483647 as i32) - 1 as i32)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                + (-(2147483647 as i32) - 1 as i32)
                                                        }) - 1 as i32) < 0 as i32
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width })
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
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                    + (-(2147483647 as i32) - 1 as i32)
                                                            }) - 1 as i32
                                                        })) as i32
                                                } else {
                                                    ((0 as i32)
                                                        < (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                            + (-(2147483647 as i32) - 1 as i32)) as i32
                                                }) != 0 && width == -(1 as i32)
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((0 as i32) < 10 as i32 + (-(2147483647 as i32) - 1 as i32))
                                                            as i32
                                                    } else {
                                                        (-(1 as i32) - (-(2147483647 as i32) - 1 as i32)
                                                            < 10 as i32 - 1 as i32) as i32
                                                    })
                                                } else {
                                                    ((-(2147483647 as i32) - 1 as i32) / width < 10 as i32)
                                                        as i32
                                                })
                                            } else {
                                                ((2147483647 as i32 / 10 as i32) < width) as i32
                                            })
                                        })
                                    }) != 0
                                    {
                                        width = (width as u32).wrapping_mul(10 as i32 as u32)
                                            as i32;
                                        1 as i32
                                    } else {
                                        width = (width as u32).wrapping_mul(10 as i32 as u32)
                                            as i32;
                                        0 as i32
                                    })
                                } else {
                                    (if (if (10 as i32) < 0 as i32 {
                                        (if width < 0 as i32 {
                                            (if (if 1 as i32 != 0 {
                                                0 as i32 as u32
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as u32
                                                } else {
                                                    (2147483647 as i32 as u32)
                                                        .wrapping_mul(2 as u32)
                                                        .wrapping_add(1 as u32)
                                                })
                                                    .wrapping_add(10 as i32 as u32)
                                            })
                                                .wrapping_sub(1 as i32 as u32) < 0 as i32 as u32
                                            {
                                                ((width as u32)
                                                    < (2147483647 as i32 as u32)
                                                        .wrapping_mul(2 as u32)
                                                        .wrapping_add(1 as u32)
                                                        .wrapping_div(10 as i32 as u32)) as i32
                                            } else {
                                                ((if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    10 as i32
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32)
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) < 0 as i32
                                                {
                                                    ((10 as i32)
                                                        < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32) < 0 as i32
                                                        {
                                                            ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                + 1 as i32)
                                                                << (::core::mem::size_of::<i32>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                + 1 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                - 1 as i32
                                                        })) as i32
                                                } else {
                                                    ((0 as i32) < 10 as i32) as i32
                                                }) != 0
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as u32)
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
                                                        .wrapping_div(-(10 as i32) as u32)
                                                }) <= (-(1 as i32) - width) as u32) as i32
                                            })
                                        } else {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32
                                            }) - 1 as i32) < 0 as i32
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32)
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) + 0 as i32
                                            }) < 0 as i32
                                            {
                                                (((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                    + 0 as i32)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                + 0 as i32
                                                        }) - 1 as i32
                                                    })) as i32
                                            } else {
                                                ((0 as i32)
                                                    < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32) as i32
                                            }) != 0 && 10 as i32 == -(1 as i32)
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                    - 1 as i32) < 0 as i32
                                                {
                                                    ((0 as i32) < width + 0 as i32) as i32
                                                } else {
                                                    ((0 as i32) < width
                                                        && (-(1 as i32) - 0 as i32) < width - 1 as i32) as i32
                                                })
                                            } else {
                                                ((0 as i32 / 10 as i32) < width) as i32
                                            })
                                        })
                                    } else {
                                        (if 10 as i32 == 0 as i32 {
                                            0 as i32
                                        } else {
                                            (if width < 0 as i32 {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                    }) + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32)
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                    }) + 0 as i32
                                                }) < 0 as i32
                                                {
                                                    (((if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                        }) - 1 as i32) < 0 as i32
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                            }) + 1 as i32)
                                                                << (::core::mem::size_of::<i32>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                + 1 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 {
                                                                0 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                            }) - 1 as i32
                                                        })) as i32
                                                } else {
                                                    ((0 as i32)
                                                        < (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                        as i32
                                                }) != 0 && width == -(1 as i32)
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((0 as i32) < 10 as i32 + 0 as i32) as i32
                                                    } else {
                                                        ((-(1 as i32) - 0 as i32) < 10 as i32 - 1 as i32) as i32
                                                    })
                                                } else {
                                                    (0 as i32 / width < 10 as i32) as i32
                                                })
                                            } else {
                                                ((2147483647 as i32 as u32)
                                                    .wrapping_mul(2 as u32)
                                                    .wrapping_add(1 as u32)
                                                    .wrapping_div(10 as i32 as u32) < width as u32) as i32
                                            })
                                        })
                                    }) != 0
                                    {
                                        width = (width as u32).wrapping_mul(10 as i32 as u32)
                                            as i32;
                                        1 as i32
                                    } else {
                                        width = (width as u32).wrapping_mul(10 as i32 as u32)
                                            as i32;
                                        0 as i32
                                    })
                                })
                            } else {
                                (if ::core::mem::size_of::<i32>() as u64
                                    == ::core::mem::size_of::<i64>() as u64
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                        - 1 as i32) < 0 as i32
                                    {
                                        (if (if (10 as i32) < 0 as i32 {
                                            (if width < 0 as i32 {
                                                (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        9223372036854775807 as i64
                                                    }) + 10 as i32 as i64
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((width as i64)
                                                        < 9223372036854775807 as i64 / 10 as i32 as i64) as i32
                                                } else {
                                                    ((if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        10 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32)
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) < 0 as i32
                                                    {
                                                        ((10 as i32)
                                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                - 1 as i32) < 0 as i32
                                                            {
                                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    + 1 as i32)
                                                                    << (::core::mem::size_of::<i32>() as u64)
                                                                        .wrapping_mul(8 as i32 as u64)
                                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                    + 1 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    - 1 as i32
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32) < 10 as i32) as i32
                                                    }) != 0
                                                    {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
                                                            + 9223372036854775807 as i64
                                                            >> (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(1 as i32 as u64)
                                                    } else {
                                                        9223372036854775807 as i64 / -(10 as i32) as i64
                                                    }) <= (-(1 as i32) - width) as i64) as i32
                                                })
                                            } else {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
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
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
                                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                                    }) + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
                                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32 as i64
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
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
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
                                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                                            }) - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64)
                                                        < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as i64
                                                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                                }) != 0 && 10 as i32 == -(1 as i32)
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((0 as i32 as i64)
                                                            < width as i64 + (-(9223372036854775807 as i64) - 1 as i64))
                                                            as i32
                                                    } else {
                                                        ((0 as i32) < width
                                                            && -(1 as i32) as i64
                                                                - (-(9223372036854775807 as i64) - 1 as i64)
                                                                < (width - 1 as i32) as i64) as i32
                                                    })
                                                } else {
                                                    (((-(9223372036854775807 as i64) - 1 as i64)
                                                        / 10 as i32 as i64) < width as i64) as i32
                                                })
                                            })
                                        } else {
                                            (if 10 as i32 == 0 as i32 {
                                                0 as i32
                                            } else {
                                                (if width < 0 as i32 {
                                                    (if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
                                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        !(((((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
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
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
                                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                                        }) + 0 as i32 as i64
                                                    }) < 0 as i32 as i64
                                                    {
                                                        ((if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
                                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                                            < -(if ((if 1 as i32 != 0 {
                                                                0 as i32 as i64
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
                                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                                            {
                                                                ((((if 1 as i32 != 0 {
                                                                    0 as i32 as i64
                                                                } else {
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
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
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
                                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                                }) - 1 as i32 as i64
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32 as i64)
                                                            < (if 1 as i32 != 0 { 0 as i32 } else { width }) as i64
                                                                + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                                    }) != 0 && width == -(1 as i32)
                                                    {
                                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32) < 0 as i32
                                                        {
                                                            ((0 as i32 as i64)
                                                                < 10 as i32 as i64
                                                                    + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                                        } else {
                                                            (-(1 as i32) as i64
                                                                - (-(9223372036854775807 as i64) - 1 as i64)
                                                                < (10 as i32 - 1 as i32) as i64) as i32
                                                        })
                                                    } else {
                                                        (((-(9223372036854775807 as i64) - 1 as i64) / width as i64)
                                                            < 10 as i32 as i64) as i32
                                                    })
                                                } else {
                                                    ((9223372036854775807 as i64 / 10 as i32 as i64)
                                                        < width as i64) as i32
                                                })
                                            })
                                        }) != 0
                                        {
                                            width = (width as u64).wrapping_mul(10 as i32 as u64) as i64
                                                as i32;
                                            1 as i32
                                        } else {
                                            width = (width as u64).wrapping_mul(10 as i32 as u64) as i64
                                                as i32;
                                            0 as i32
                                        })
                                    } else {
                                        (if (if (10 as i32) < 0 as i32 {
                                            (if width < 0 as i32 {
                                                (if (if 1 as i32 != 0 {
                                                    0 as i32 as u64
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32 as u64
                                                    } else {
                                                        (9223372036854775807 as i64 as u64)
                                                            .wrapping_mul(2 as u64)
                                                            .wrapping_add(1 as u64)
                                                    })
                                                        .wrapping_add(10 as i32 as u64)
                                                })
                                                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                                {
                                                    ((width as u64)
                                                        < (9223372036854775807 as i64 as u64)
                                                            .wrapping_mul(2 as u64)
                                                            .wrapping_add(1 as u64)
                                                            .wrapping_div(10 as i32 as u64)) as i32
                                                } else {
                                                    ((if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        10 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32)
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) < 0 as i32
                                                    {
                                                        ((10 as i32)
                                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                - 1 as i32) < 0 as i32
                                                            {
                                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    + 1 as i32)
                                                                    << (::core::mem::size_of::<i32>() as u64)
                                                                        .wrapping_mul(8 as i32 as u64)
                                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                    + 1 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    - 1 as i32
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32) < 10 as i32) as i32
                                                    }) != 0
                                                    {
                                                        ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 }) as u64)
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
                                                            .wrapping_div(-(10 as i32) as u64)
                                                    }) <= (-(1 as i32) - width) as u64) as i32
                                                })
                                            } else {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32)
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) + 0 as i32
                                                }) < 0 as i32
                                                {
                                                    (((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                + 0 as i32
                                                        }) - 1 as i32) < 0 as i32
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    + 0 as i32
                                                            }) - 1 as i32
                                                        })) as i32
                                                } else {
                                                    ((0 as i32)
                                                        < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32) as i32
                                                }) != 0 && 10 as i32 == -(1 as i32)
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((0 as i32) < width + 0 as i32) as i32
                                                    } else {
                                                        ((0 as i32) < width
                                                            && (-(1 as i32) - 0 as i32) < width - 1 as i32) as i32
                                                    })
                                                } else {
                                                    ((0 as i32 / 10 as i32) < width) as i32
                                                })
                                            })
                                        } else {
                                            (if 10 as i32 == 0 as i32 {
                                                0 as i32
                                            } else {
                                                (if width < 0 as i32 {
                                                    (if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        !(((((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                        }) + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32)
                                                    } else {
                                                        (if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                        }) + 0 as i32
                                                    }) < 0 as i32
                                                    {
                                                        (((if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                            < -(if ((if 1 as i32 != 0 {
                                                                0 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                            }) - 1 as i32) < 0 as i32
                                                            {
                                                                ((((if 1 as i32 != 0 {
                                                                    0 as i32
                                                                } else {
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                                }) + 1 as i32)
                                                                    << (::core::mem::size_of::<i32>() as u64)
                                                                        .wrapping_mul(8 as i32 as u64)
                                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                    + 1 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 {
                                                                    0 as i32
                                                                } else {
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                                }) - 1 as i32
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32)
                                                            < (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                            as i32
                                                    }) != 0 && width == -(1 as i32)
                                                    {
                                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32) < 0 as i32
                                                        {
                                                            ((0 as i32) < 10 as i32 + 0 as i32) as i32
                                                        } else {
                                                            ((-(1 as i32) - 0 as i32) < 10 as i32 - 1 as i32) as i32
                                                        })
                                                    } else {
                                                        (0 as i32 / width < 10 as i32) as i32
                                                    })
                                                } else {
                                                    ((9223372036854775807 as i64 as u64)
                                                        .wrapping_mul(2 as u64)
                                                        .wrapping_add(1 as u64)
                                                        .wrapping_div(10 as i32 as u64) < width as u64) as i32
                                                })
                                            })
                                        }) != 0
                                        {
                                            width = (width as u64).wrapping_mul(10 as i32 as u64)
                                                as i32;
                                            1 as i32
                                        } else {
                                            width = (width as u64).wrapping_mul(10 as i32 as u64)
                                                as i32;
                                            0 as i32
                                        })
                                    })
                                } else {
                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                        - 1 as i32) < 0 as i32
                                    {
                                        (if (if (10 as i32) < 0 as i32 {
                                            (if width < 0 as i32 {
                                                (if ((if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32 as libc::c_longlong
                                                    } else {
                                                        9223372036854775807 as libc::c_longlong
                                                    }) + 10 as i32 as libc::c_longlong
                                                }) - 1 as i32 as libc::c_longlong)
                                                    < 0 as i32 as libc::c_longlong
                                                {
                                                    ((width as libc::c_longlong)
                                                        < 9223372036854775807 as libc::c_longlong
                                                            / 10 as i32 as libc::c_longlong) as i32
                                                } else {
                                                    ((if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        10 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32)
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) < 0 as i32
                                                    {
                                                        ((10 as i32)
                                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                - 1 as i32) < 0 as i32
                                                            {
                                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    + 1 as i32)
                                                                    << (::core::mem::size_of::<i32>() as u64)
                                                                        .wrapping_mul(8 as i32 as u64)
                                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                    + 1 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    - 1 as i32
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32) < 10 as i32) as i32
                                                    }) != 0
                                                    {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            as libc::c_longlong
                                                            + 9223372036854775807 as libc::c_longlong
                                                            >> (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(1 as i32 as u64)
                                                    } else {
                                                        9223372036854775807 as libc::c_longlong
                                                            / -(10 as i32) as libc::c_longlong
                                                    }) <= (-(1 as i32) - width) as libc::c_longlong) as i32
                                                })
                                            } else {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) - 1 as i32 as libc::c_longlong)
                                                    < 0 as i32 as libc::c_longlong
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32 as libc::c_longlong
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) + 1 as i32 as libc::c_longlong)
                                                        << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64))
                                                        - 1 as i32 as libc::c_longlong)
                                                        * 2 as i32 as libc::c_longlong
                                                        + 1 as i32 as libc::c_longlong)
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32 as libc::c_longlong
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) + 0 as i32 as libc::c_longlong
                                                }) < 0 as i32 as libc::c_longlong
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32 as libc::c_longlong
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) - 1 as i32 as libc::c_longlong)
                                                            < 0 as i32 as libc::c_longlong
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32 as libc::c_longlong
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) + 1 as i32 as libc::c_longlong)
                                                                << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64))
                                                                - 1 as i32 as libc::c_longlong)
                                                                * 2 as i32 as libc::c_longlong
                                                                + 1 as i32 as libc::c_longlong
                                                        } else {
                                                            (if 1 as i32 != 0 {
                                                                0 as i32 as libc::c_longlong
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) - 1 as i32 as libc::c_longlong
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as libc::c_longlong)
                                                        < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)) as i32
                                                }) != 0 && 10 as i32 == -(1 as i32)
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((0 as i32 as libc::c_longlong)
                                                            < width as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)) as i32
                                                    } else {
                                                        ((0 as i32) < width
                                                            && -(1 as i32) as libc::c_longlong
                                                                - (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                                < (width - 1 as i32) as libc::c_longlong) as i32
                                                    })
                                                } else {
                                                    (((-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) / 10 as i32 as libc::c_longlong)
                                                        < width as libc::c_longlong) as i32
                                                })
                                            })
                                        } else {
                                            (if 10 as i32 == 0 as i32 {
                                                0 as i32
                                            } else {
                                                (if width < 0 as i32 {
                                                    (if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32 as libc::c_longlong
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) - 1 as i32 as libc::c_longlong)
                                                        < 0 as i32 as libc::c_longlong
                                                    {
                                                        !(((((if 1 as i32 != 0 {
                                                            0 as i32 as libc::c_longlong
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) + 1 as i32 as libc::c_longlong)
                                                            << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64))
                                                            - 1 as i32 as libc::c_longlong)
                                                            * 2 as i32 as libc::c_longlong
                                                            + 1 as i32 as libc::c_longlong)
                                                    } else {
                                                        (if 1 as i32 != 0 {
                                                            0 as i32 as libc::c_longlong
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) + 0 as i32 as libc::c_longlong
                                                    }) < 0 as i32 as libc::c_longlong
                                                    {
                                                        ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                            < -(if ((if 1 as i32 != 0 {
                                                                0 as i32 as libc::c_longlong
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                    as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) - 1 as i32 as libc::c_longlong)
                                                                < 0 as i32 as libc::c_longlong
                                                            {
                                                                ((((if 1 as i32 != 0 {
                                                                    0 as i32 as libc::c_longlong
                                                                } else {
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                        as libc::c_longlong
                                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                                            - 1 as libc::c_longlong)
                                                                }) + 1 as i32 as libc::c_longlong)
                                                                    << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                                        .wrapping_mul(8 as i32 as u64)
                                                                        .wrapping_sub(2 as i32 as u64))
                                                                    - 1 as i32 as libc::c_longlong)
                                                                    * 2 as i32 as libc::c_longlong
                                                                    + 1 as i32 as libc::c_longlong
                                                            } else {
                                                                (if 1 as i32 != 0 {
                                                                    0 as i32 as libc::c_longlong
                                                                } else {
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                        as libc::c_longlong
                                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                                            - 1 as libc::c_longlong)
                                                                }) - 1 as i32 as libc::c_longlong
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32 as libc::c_longlong)
                                                            < (if 1 as i32 != 0 { 0 as i32 } else { width })
                                                                as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)) as i32
                                                    }) != 0 && width == -(1 as i32)
                                                    {
                                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32) < 0 as i32
                                                        {
                                                            ((0 as i32 as libc::c_longlong)
                                                                < 10 as i32 as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)) as i32
                                                        } else {
                                                            (-(1 as i32) as libc::c_longlong
                                                                - (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                                < (10 as i32 - 1 as i32) as libc::c_longlong) as i32
                                                        })
                                                    } else {
                                                        (((-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong) / width as libc::c_longlong)
                                                            < 10 as i32 as libc::c_longlong) as i32
                                                    })
                                                } else {
                                                    ((9223372036854775807 as libc::c_longlong
                                                        / 10 as i32 as libc::c_longlong)
                                                        < width as libc::c_longlong) as i32
                                                })
                                            })
                                        }) != 0
                                        {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as i32 as libc::c_ulonglong)
                                                as libc::c_longlong as i32;
                                            1 as i32
                                        } else {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as i32 as libc::c_ulonglong)
                                                as libc::c_longlong as i32;
                                            0 as i32
                                        })
                                    } else {
                                        (if (if (10 as i32) < 0 as i32 {
                                            (if width < 0 as i32 {
                                                (if (if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_ulonglong
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32 as libc::c_ulonglong
                                                    } else {
                                                        (9223372036854775807 as libc::c_longlong
                                                            as libc::c_ulonglong)
                                                            .wrapping_mul(2 as libc::c_ulonglong)
                                                            .wrapping_add(1 as libc::c_ulonglong)
                                                    })
                                                        .wrapping_add(10 as i32 as libc::c_ulonglong)
                                                })
                                                    .wrapping_sub(1 as i32 as libc::c_ulonglong)
                                                    < 0 as i32 as libc::c_ulonglong
                                                {
                                                    ((width as libc::c_ulonglong)
                                                        < (9223372036854775807 as libc::c_longlong
                                                            as libc::c_ulonglong)
                                                            .wrapping_mul(2 as libc::c_ulonglong)
                                                            .wrapping_add(1 as libc::c_ulonglong)
                                                            .wrapping_div(10 as i32 as libc::c_ulonglong)) as i32
                                                } else {
                                                    ((if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        10 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32)
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) < 0 as i32
                                                    {
                                                        ((10 as i32)
                                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                - 1 as i32) < 0 as i32
                                                            {
                                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    + 1 as i32)
                                                                    << (::core::mem::size_of::<i32>() as u64)
                                                                        .wrapping_mul(8 as i32 as u64)
                                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                    + 1 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    - 1 as i32
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32) < 10 as i32) as i32
                                                    }) != 0
                                                    {
                                                        ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                        (9223372036854775807 as libc::c_longlong
                                                            as libc::c_ulonglong)
                                                            .wrapping_mul(2 as libc::c_ulonglong)
                                                            .wrapping_add(1 as libc::c_ulonglong)
                                                            .wrapping_div(-(10 as i32) as libc::c_ulonglong)
                                                    }) <= (-(1 as i32) - width) as libc::c_ulonglong) as i32
                                                })
                                            } else {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32
                                                }) - 1 as i32) < 0 as i32
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) + 1 as i32)
                                                        << (::core::mem::size_of::<i32>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                        + 1 as i32)
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32
                                                    }) + 0 as i32
                                                }) < 0 as i32
                                                {
                                                    (((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                        + 0 as i32)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                + 0 as i32
                                                        }) - 1 as i32) < 0 as i32
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
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
                                                                (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                                    + 0 as i32
                                                            }) - 1 as i32
                                                        })) as i32
                                                } else {
                                                    ((0 as i32)
                                                        < (if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            + 0 as i32) as i32
                                                }) != 0 && 10 as i32 == -(1 as i32)
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { width })
                                                        - 1 as i32) < 0 as i32
                                                    {
                                                        ((0 as i32) < width + 0 as i32) as i32
                                                    } else {
                                                        ((0 as i32) < width
                                                            && (-(1 as i32) - 0 as i32) < width - 1 as i32) as i32
                                                    })
                                                } else {
                                                    ((0 as i32 / 10 as i32) < width) as i32
                                                })
                                            })
                                        } else {
                                            (if 10 as i32 == 0 as i32 {
                                                0 as i32
                                            } else {
                                                (if width < 0 as i32 {
                                                    (if (if (if ((if 1 as i32 != 0 {
                                                        0 as i32
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                    }) - 1 as i32) < 0 as i32
                                                    {
                                                        !(((((if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                        }) + 1 as i32)
                                                            << (::core::mem::size_of::<i32>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                            + 1 as i32)
                                                    } else {
                                                        (if 1 as i32 != 0 {
                                                            0 as i32
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                        }) + 0 as i32
                                                    }) < 0 as i32
                                                    {
                                                        (((if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                            < -(if ((if 1 as i32 != 0 {
                                                                0 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                            }) - 1 as i32) < 0 as i32
                                                            {
                                                                ((((if 1 as i32 != 0 {
                                                                    0 as i32
                                                                } else {
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                                }) + 1 as i32)
                                                                    << (::core::mem::size_of::<i32>() as u64)
                                                                        .wrapping_mul(8 as i32 as u64)
                                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                                    + 1 as i32
                                                            } else {
                                                                (if 1 as i32 != 0 {
                                                                    0 as i32
                                                                } else {
                                                                    (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32
                                                                }) - 1 as i32
                                                            })) as i32
                                                    } else {
                                                        ((0 as i32)
                                                            < (if 1 as i32 != 0 { 0 as i32 } else { width }) + 0 as i32)
                                                            as i32
                                                    }) != 0 && width == -(1 as i32)
                                                    {
                                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 10 as i32 })
                                                            - 1 as i32) < 0 as i32
                                                        {
                                                            ((0 as i32) < 10 as i32 + 0 as i32) as i32
                                                        } else {
                                                            ((-(1 as i32) - 0 as i32) < 10 as i32 - 1 as i32) as i32
                                                        })
                                                    } else {
                                                        (0 as i32 / width < 10 as i32) as i32
                                                    })
                                                } else {
                                                    ((9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(10 as i32 as libc::c_ulonglong)
                                                        < width as libc::c_ulonglong) as i32
                                                })
                                            })
                                        }) != 0
                                        {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as i32 as libc::c_ulonglong) as i32;
                                            1 as i32
                                        } else {
                                            width = (width as libc::c_ulonglong)
                                                .wrapping_mul(10 as i32 as libc::c_ulonglong) as i32;
                                            0 as i32
                                        })
                                    })
                                })
                            })
                        })
                    }) != 0
                        || {
                            let (fresh2, fresh3) = width
                                .overflowing_add(*f as i32 - '0' as i32);
                            *(&mut width as *mut i32) = fresh2;
                            fresh3 as i32 != 0
                        }
                    {
                        width = 2147483647 as i32;
                    }
                    f = f.offset(1);
                    f;
                    if !((*f as u32).wrapping_sub('0' as i32 as u32) <= 9 as i32 as u32)
                    {
                        break;
                    }
                }
            }
            match *f as i32 {
                69 | 79 => {
                    let fresh4 = f;
                    f = f.offset(1);
                    modifier = *fresh4 as i32;
                }
                _ => {
                    modifier = 0 as i32;
                }
            }
            format_char = *f as i32;
            match format_char {
                37 => {
                    if modifier != 0 as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        let mut _n_0: size_t = 1 as i32 as size_t;
                        let mut _w_0: size_t = (if pad == '-' as i32 || width < 0 as i32
                        {
                            0 as i32
                        } else {
                            width
                        }) as size_t;
                        let mut _incr_0: size_t = if _n_0 < _w_0 { _w_0 } else { _n_0 };
                        if _incr_0 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as i32;
                            return 0 as i32 as size_t;
                        }
                        if !p.is_null() {
                            if _n_0 < _w_0 {
                                let mut _delta_0: size_t = _w_0.wrapping_sub(_n_0);
                                if pad == '0' as i32 || pad == '+' as i32 {
                                    memset(p as *mut libc::c_void, '0' as i32, _delta_0);
                                    p = p.offset(_delta_0 as isize);
                                } else {
                                    memset(p as *mut libc::c_void, ' ' as i32, _delta_0);
                                    p = p.offset(_delta_0 as isize);
                                }
                            }
                            *p = *f;
                            p = p.offset(_n_0 as isize);
                        }
                        i = (i as u64).wrapping_add(_incr_0) as size_t as size_t;
                        current_block = 3276175668257526147;
                    }
                }
                97 => {
                    if modifier != 0 as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        if change_case {
                            to_uppcase = 1 as i32 != 0;
                            to_lowcase = 0 as i32 != 0;
                        }
                        current_block = 12902579523164801321;
                    }
                }
                65 => {
                    if modifier != 0 as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        if change_case {
                            to_uppcase = 1 as i32 != 0;
                            to_lowcase = 0 as i32 != 0;
                        }
                        current_block = 12902579523164801321;
                    }
                }
                98 | 104 => {
                    if change_case {
                        to_uppcase = 1 as i32 != 0;
                        to_lowcase = 0 as i32 != 0;
                    }
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        current_block = 12902579523164801321;
                    }
                }
                66 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        if change_case {
                            to_uppcase = 1 as i32 != 0;
                            to_lowcase = 0 as i32 != 0;
                        }
                        current_block = 12902579523164801321;
                    }
                }
                99 => {
                    if modifier == 'O' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        current_block = 12902579523164801321;
                    }
                }
                67 => {
                    if modifier == 'E' as i32 {
                        current_block = 12902579523164801321;
                    } else {
                        let mut negative_year: bool = (*tp).tm_year < -(1900 as i32);
                        let mut zero_thru_1899: bool = !negative_year as i32
                            & ((*tp).tm_year < 0 as i32) as i32 != 0;
                        let mut century: i32 = ((*tp).tm_year
                            - 99 as i32 * zero_thru_1899 as i32) / 100 as i32
                            + 1900 as i32 / 100 as i32;
                        digits = 2 as i32;
                        negative_number = negative_year;
                        u_number_value = century as u32;
                        current_block = 18346233773820361581;
                    }
                }
                120 => {
                    if modifier == 'O' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        current_block = 12902579523164801321;
                    }
                }
                68 => {
                    if modifier != 0 as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        subfmt = b"%m/%d/%y\0" as *const u8 as *const i8;
                        current_block = 7030165609490500138;
                    }
                }
                100 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_mday;
                        current_block = 13485003182018958242;
                    }
                }
                101 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_mday;
                        current_block = 3043273946251187989;
                    }
                }
                70 => {
                    if modifier != 0 as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        if pad == 0 as i32 && width < 0 as i32 {
                            pad = '+' as i32;
                            subwidth = 4 as i32;
                        } else {
                            subwidth = width - 6 as i32;
                            if subwidth < 0 as i32 {
                                subwidth = 0 as i32;
                            }
                        }
                        subfmt = b"%Y-%m-%d\0" as *const u8 as *const i8;
                        current_block = 447645734570482032;
                    }
                }
                72 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_hour;
                        current_block = 13485003182018958242;
                    }
                }
                73 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = hour12;
                        current_block = 13485003182018958242;
                    }
                }
                107 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_hour;
                        current_block = 3043273946251187989;
                    }
                }
                108 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = hour12;
                        current_block = 3043273946251187989;
                    }
                }
                106 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 3 as i32;
                        negative_number = (*tp).tm_yday < -(1 as i32);
                        u_number_value = ((*tp).tm_yday as u32).wrapping_add(1 as u32);
                        current_block = 14619535680852412511;
                    }
                }
                77 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_min;
                        current_block = 13485003182018958242;
                    }
                }
                109 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        negative_number = (*tp).tm_mon < -(1 as i32);
                        u_number_value = ((*tp).tm_mon as u32).wrapping_add(1 as u32);
                        current_block = 14619535680852412511;
                    }
                }
                78 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        let mut n: i32 = ns;
                        let mut ns_digits: i32 = 9 as i32;
                        if width <= 0 as i32 {
                            width = ns_digits;
                        }
                        let mut ndigs: i32 = ns_digits;
                        while width < ndigs
                            || (1 as i32) < ndigs && n % 10 as i32 == 0 as i32
                        {
                            ndigs -= 1;
                            ndigs;
                            n /= 10 as i32;
                        }
                        let mut j: i32 = ndigs;
                        while (0 as i32) < j {
                            buf[(j - 1 as i32) as usize] = (n % 10 as i32 + '0' as i32)
                                as i8;
                            n /= 10 as i32;
                            j -= 1;
                            j;
                        }
                        if pad == 0 {
                            pad = '0' as i32;
                        }
                        let mut _n_5: size_t = ndigs as size_t;
                        let mut _w_5: size_t = (if pad == '-' as i32
                            || (0 as i32) < 0 as i32
                        {
                            0 as i32
                        } else {
                            0 as i32
                        }) as size_t;
                        let mut _incr_5: size_t = if _n_5 < _w_5 { _w_5 } else { _n_5 };
                        if _incr_5 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as i32;
                            return 0 as i32 as size_t;
                        }
                        if !p.is_null() {
                            if _n_5 < _w_5 {
                                let mut _delta_5: size_t = _w_5.wrapping_sub(_n_5);
                                if pad == '0' as i32 || pad == '+' as i32 {
                                    memset(p as *mut libc::c_void, '0' as i32, _delta_5);
                                    p = p.offset(_delta_5 as isize);
                                } else {
                                    memset(p as *mut libc::c_void, ' ' as i32, _delta_5);
                                    p = p.offset(_delta_5 as isize);
                                }
                            }
                            if to_lowcase {
                                memcpy_lowcase(p, buf.as_mut_ptr(), _n_5);
                            } else if to_uppcase {
                                memcpy_uppcase(p, buf.as_mut_ptr(), _n_5);
                            } else {
                                memcpy(
                                    p as *mut libc::c_void,
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    _n_5,
                                );
                            }
                            p = p.offset(_n_5 as isize);
                        }
                        i = (i as u64).wrapping_add(_incr_5) as size_t as size_t;
                        let mut _n_6: size_t = 0 as i32 as size_t;
                        let mut _w_6: size_t = (if pad == '-' as i32
                            || width - ndigs < 0 as i32
                        {
                            0 as i32
                        } else {
                            width - ndigs
                        }) as size_t;
                        let mut _incr_6: size_t = if _n_6 < _w_6 { _w_6 } else { _n_6 };
                        if _incr_6 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as i32;
                            return 0 as i32 as size_t;
                        }
                        if !p.is_null() {
                            if _n_6 < _w_6 {
                                let mut _delta_6: size_t = _w_6.wrapping_sub(_n_6);
                                if pad == '0' as i32 || pad == '+' as i32 {
                                    memset(p as *mut libc::c_void, '0' as i32, _delta_6);
                                    p = p.offset(_delta_6 as isize);
                                } else {
                                    memset(p as *mut libc::c_void, ' ' as i32, _delta_6);
                                    p = p.offset(_delta_6 as isize);
                                }
                            }
                            p = p.offset(_n_6 as isize);
                        }
                        i = (i as u64).wrapping_add(_incr_6) as size_t as size_t;
                        current_block = 3276175668257526147;
                    }
                }
                110 => {
                    let mut _n_7: size_t = 1 as i32 as size_t;
                    let mut _w_7: size_t = (if pad == '-' as i32 || width < 0 as i32 {
                        0 as i32
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_7: size_t = if _n_7 < _w_7 { _w_7 } else { _n_7 };
                    if _incr_7 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as i32;
                        return 0 as i32 as size_t;
                    }
                    if !p.is_null() {
                        if _n_7 < _w_7 {
                            let mut _delta_7: size_t = _w_7.wrapping_sub(_n_7);
                            if pad == '0' as i32 || pad == '+' as i32 {
                                memset(p as *mut libc::c_void, '0' as i32, _delta_7);
                                p = p.offset(_delta_7 as isize);
                            } else {
                                memset(p as *mut libc::c_void, ' ' as i32, _delta_7);
                                p = p.offset(_delta_7 as isize);
                            }
                        }
                        *p = '\n' as i32 as i8;
                        p = p.offset(_n_7 as isize);
                    }
                    i = (i as u64).wrapping_add(_incr_7) as size_t as size_t;
                    current_block = 3276175668257526147;
                }
                80 => {
                    to_lowcase = 1 as i32 != 0;
                    format_char = 'p' as i32;
                    current_block = 12851094283928535317;
                }
                112 => {
                    current_block = 12851094283928535317;
                }
                113 => {
                    digits = 1 as i32;
                    negative_number = 0 as i32 != 0;
                    u_number_value = (((*tp).tm_mon * 11 as i32 >> 5 as i32) + 1 as i32)
                        as u32;
                    current_block = 14619535680852412511;
                }
                82 => {
                    subfmt = b"%H:%M\0" as *const u8 as *const i8;
                    current_block = 7030165609490500138;
                }
                114 => {
                    current_block = 12902579523164801321;
                }
                83 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_sec;
                        current_block = 13485003182018958242;
                    }
                }
                115 => {
                    let mut ltm: tm = tm {
                        tm_sec: 0,
                        tm_min: 0,
                        tm_hour: 0,
                        tm_mday: 0,
                        tm_mon: 0,
                        tm_year: 0,
                        tm_wday: 0,
                        tm_yday: 0,
                        tm_isdst: 0,
                        tm_gmtoff: 0,
                        tm_zone: 0 as *const i8,
                    };
                    let mut t: time_t = 0;
                    ltm = *tp;
                    ltm.tm_yday = -(1 as i32);
                    t = mktime_z(tz, &mut ltm);
                    if ltm.tm_yday < 0 as i32 {
                        *__errno_location() = 75 as i32;
                        return 0 as i32 as size_t;
                    }
                    bufp = buf
                        .as_mut_ptr()
                        .offset(
                            (::core::mem::size_of::<[i8; 23]>() as u64)
                                .wrapping_div(::core::mem::size_of::<i8>() as u64) as isize,
                        );
                    negative_number = t < 0 as i32 as i64;
                    loop {
                        let mut d: i32 = (t % 10 as i32 as i64) as i32;
                        t /= 10 as i32 as i64;
                        bufp = bufp.offset(-1);
                        *bufp = ((if negative_number as i32 != 0 { -d } else { d })
                            + '0' as i32) as i8;
                        if !(t != 0 as i32 as i64) {
                            break;
                        }
                    }
                    digits = 1 as i32;
                    always_output_a_sign = 0 as i32 != 0;
                    current_block = 6370517008983650009;
                }
                88 => {
                    if modifier == 'O' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        current_block = 12902579523164801321;
                    }
                }
                84 => {
                    subfmt = b"%H:%M:%S\0" as *const u8 as *const i8;
                    current_block = 7030165609490500138;
                }
                116 => {
                    let mut _n_8: size_t = 1 as i32 as size_t;
                    let mut _w_8: size_t = (if pad == '-' as i32 || width < 0 as i32 {
                        0 as i32
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_8: size_t = if _n_8 < _w_8 { _w_8 } else { _n_8 };
                    if _incr_8 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as i32;
                        return 0 as i32 as size_t;
                    }
                    if !p.is_null() {
                        if _n_8 < _w_8 {
                            let mut _delta_8: size_t = _w_8.wrapping_sub(_n_8);
                            if pad == '0' as i32 || pad == '+' as i32 {
                                memset(p as *mut libc::c_void, '0' as i32, _delta_8);
                                p = p.offset(_delta_8 as isize);
                            } else {
                                memset(p as *mut libc::c_void, ' ' as i32, _delta_8);
                                p = p.offset(_delta_8 as isize);
                            }
                        }
                        *p = '\t' as i32 as i8;
                        p = p.offset(_n_8 as isize);
                    }
                    i = (i as u64).wrapping_add(_incr_8) as size_t as size_t;
                    current_block = 3276175668257526147;
                }
                117 => {
                    digits = 1 as i32;
                    number_value = ((*tp).tm_wday - 1 as i32 + 7 as i32) % 7 as i32
                        + 1 as i32;
                    current_block = 13485003182018958242;
                }
                85 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = ((*tp).tm_yday - (*tp).tm_wday + 7 as i32)
                            / 7 as i32;
                        current_block = 13485003182018958242;
                    }
                }
                86 | 103 | 71 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        let mut year: i32 = (*tp).tm_year
                            + (if (*tp).tm_year < 0 as i32 {
                                1900 as i32 % 400 as i32
                            } else {
                                1900 as i32 % 400 as i32 - 400 as i32
                            });
                        let mut year_adjust: i32 = 0 as i32;
                        let mut days: i32 = iso_week_days((*tp).tm_yday, (*tp).tm_wday);
                        if days < 0 as i32 {
                            year_adjust = -(1 as i32);
                            days = iso_week_days(
                                (*tp).tm_yday
                                    + (365 as i32
                                        + ((year - 1 as i32) % 4 as i32 == 0 as i32
                                            && ((year - 1 as i32) % 100 as i32 != 0 as i32
                                                || (year - 1 as i32) % 400 as i32 == 0 as i32)) as i32),
                                (*tp).tm_wday,
                            );
                        } else {
                            let mut d_0: i32 = iso_week_days(
                                (*tp).tm_yday
                                    - (365 as i32
                                        + (year % 4 as i32 == 0 as i32
                                            && (year % 100 as i32 != 0 as i32
                                                || year % 400 as i32 == 0 as i32)) as i32),
                                (*tp).tm_wday,
                            );
                            if 0 as i32 <= d_0 {
                                year_adjust = 1 as i32;
                                days = d_0;
                            }
                        }
                        match *f as i32 {
                            103 => {
                                let mut yy: i32 = ((*tp).tm_year % 100 as i32 + year_adjust)
                                    % 100 as i32;
                                digits = 2 as i32;
                                negative_number = 0 as i32 != 0;
                                u_number_value = (if 0 as i32 <= yy {
                                    yy
                                } else if (*tp).tm_year < -(1900 as i32) - year_adjust {
                                    -yy
                                } else {
                                    yy + 100 as i32
                                }) as u32;
                                current_block = 18346233773820361581;
                            }
                            71 => {
                                digits = 4 as i32;
                                negative_number = (*tp).tm_year
                                    < -(1900 as i32) - year_adjust;
                                u_number_value = ((*tp).tm_year as u32)
                                    .wrapping_add(1900 as i32 as u32)
                                    .wrapping_add(year_adjust as u32);
                                current_block = 18346233773820361581;
                            }
                            _ => {
                                digits = 2 as i32;
                                number_value = days / 7 as i32 + 1 as i32;
                                current_block = 13485003182018958242;
                            }
                        }
                    }
                }
                87 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 2 as i32;
                        number_value = ((*tp).tm_yday
                            - ((*tp).tm_wday - 1 as i32 + 7 as i32) % 7 as i32
                            + 7 as i32) / 7 as i32;
                        current_block = 13485003182018958242;
                    }
                }
                119 => {
                    if modifier == 'E' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 1 as i32;
                        number_value = (*tp).tm_wday;
                        current_block = 13485003182018958242;
                    }
                }
                89 => {
                    if modifier == 'E' as i32 {
                        current_block = 12902579523164801321;
                    } else if modifier == 'O' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        digits = 4 as i32;
                        negative_number = (*tp).tm_year < -(1900 as i32);
                        u_number_value = ((*tp).tm_year as u32)
                            .wrapping_add(1900 as i32 as u32);
                        current_block = 18346233773820361581;
                    }
                }
                121 => {
                    if modifier == 'E' as i32 {
                        current_block = 12902579523164801321;
                    } else {
                        let mut yy_0: i32 = (*tp).tm_year % 100 as i32;
                        if yy_0 < 0 as i32 {
                            yy_0 = if (*tp).tm_year < -(1900 as i32) {
                                -yy_0
                            } else {
                                yy_0 + 100 as i32
                            };
                        }
                        digits = 2 as i32;
                        negative_number = 0 as i32 != 0;
                        u_number_value = yy_0 as u32;
                        current_block = 18346233773820361581;
                    }
                }
                90 => {
                    if change_case {
                        to_uppcase = 0 as i32 != 0;
                        to_lowcase = 1 as i32 != 0;
                    }
                    let mut _n_9: size_t = strlen(zone);
                    let mut _w_9: size_t = (if pad == '-' as i32 || width < 0 as i32 {
                        0 as i32
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_9: size_t = if _n_9 < _w_9 { _w_9 } else { _n_9 };
                    if _incr_9 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as i32;
                        return 0 as i32 as size_t;
                    }
                    if !p.is_null() {
                        if _n_9 < _w_9 {
                            let mut _delta_9: size_t = _w_9.wrapping_sub(_n_9);
                            if pad == '0' as i32 || pad == '+' as i32 {
                                memset(p as *mut libc::c_void, '0' as i32, _delta_9);
                                p = p.offset(_delta_9 as isize);
                            } else {
                                memset(p as *mut libc::c_void, ' ' as i32, _delta_9);
                                p = p.offset(_delta_9 as isize);
                            }
                        }
                        if to_lowcase {
                            memcpy_lowcase(p, zone, _n_9);
                        } else if to_uppcase {
                            memcpy_uppcase(p, zone, _n_9);
                        } else {
                            memcpy(
                                p as *mut libc::c_void,
                                zone as *const libc::c_void,
                                _n_9,
                            );
                        }
                        p = p.offset(_n_9 as isize);
                    }
                    i = (i as u64).wrapping_add(_incr_9) as size_t as size_t;
                    current_block = 3276175668257526147;
                }
                58 => {
                    colons = 1 as i32 as size_t;
                    while *f.offset(colons as isize) as i32 == ':' as i32 {
                        colons = colons.wrapping_add(1);
                        colons;
                    }
                    if *f.offset(colons as isize) as i32 != 'z' as i32 {
                        current_block = 15513122595741660954;
                    } else {
                        f = f.offset(colons as isize);
                        current_block = 1048559300856241607;
                    }
                }
                122 => {
                    colons = 0 as i32 as size_t;
                    current_block = 1048559300856241607;
                }
                0 => {
                    f = f.offset(-1);
                    f;
                    current_block = 15513122595741660954;
                }
                _ => {
                    current_block = 15513122595741660954;
                }
            }
            match current_block {
                3276175668257526147 => {}
                _ => {
                    match current_block {
                        18346233773820361581 => {
                            if pad == 0 as i32 {
                                pad = yr_spec;
                            }
                            always_output_a_sign = pad == '+' as i32
                                && (((if digits == 2 as i32 {
                                    99 as i32
                                } else {
                                    9999 as i32
                                }) as u32) < u_number_value || digits < width);
                            current_block = 16035518928380119276;
                        }
                        1048559300856241607 => {
                            if (*tp).tm_isdst < 0 as i32 {
                                current_block = 3276175668257526147;
                            } else {
                                let mut diff: i32 = 0;
                                let mut hour_diff: i32 = 0;
                                let mut min_diff: i32 = 0;
                                let mut sec_diff: i32 = 0;
                                diff = (*tp).tm_gmtoff as i32;
                                negative_number = diff < 0 as i32
                                    || diff == 0 as i32 && *zone as i32 == '-' as i32;
                                hour_diff = diff / 60 as i32 / 60 as i32;
                                min_diff = diff / 60 as i32 % 60 as i32;
                                sec_diff = diff % 60 as i32;
                                match colons {
                                    0 => {
                                        current_block = 14795592282877320006;
                                        match current_block {
                                            14795592282877320006 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 8787174868952634065;
                                            }
                                            17715011718894633313 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 15770254314302528993;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 6225579869370739012;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 8787174868952634065;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            15770254314302528993 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 18027821804064647803;
                                    }
                                    1 => {
                                        current_block = 6225579869370739012;
                                        match current_block {
                                            14795592282877320006 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 8787174868952634065;
                                            }
                                            17715011718894633313 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 15770254314302528993;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 6225579869370739012;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 8787174868952634065;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            15770254314302528993 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 18027821804064647803;
                                    }
                                    2 => {
                                        current_block = 15770254314302528993;
                                        match current_block {
                                            14795592282877320006 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 8787174868952634065;
                                            }
                                            17715011718894633313 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 15770254314302528993;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 6225579869370739012;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 8787174868952634065;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            15770254314302528993 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 18027821804064647803;
                                    }
                                    3 => {
                                        current_block = 17715011718894633313;
                                        match current_block {
                                            14795592282877320006 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 8787174868952634065;
                                            }
                                            17715011718894633313 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 15770254314302528993;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 6225579869370739012;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 8787174868952634065;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            15770254314302528993 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 18027821804064647803;
                                    }
                                    _ => {
                                        current_block = 15513122595741660954;
                                    }
                                }
                            }
                        }
                        12851094283928535317 => {
                            if change_case {
                                to_uppcase = 0 as i32 != 0;
                                to_lowcase = 1 as i32 != 0;
                            }
                            current_block = 12902579523164801321;
                        }
                        3043273946251187989 => {
                            if pad == 0 as i32 {
                                pad = '_' as i32;
                            }
                            current_block = 13485003182018958242;
                        }
                        7030165609490500138 => {
                            subwidth = -(1 as i32);
                            current_block = 447645734570482032;
                        }
                        _ => {}
                    }
                    match current_block {
                        3276175668257526147 => {}
                        _ => {
                            match current_block {
                                13485003182018958242 => {
                                    negative_number = number_value < 0 as i32;
                                    u_number_value = number_value as u32;
                                    current_block = 14619535680852412511;
                                }
                                15513122595741660954 => {
                                    let mut flen: i32 = 0;
                                    flen = 1 as i32;
                                    while *f.offset((1 as i32 - flen) as isize) as i32
                                        != '%' as i32
                                    {
                                        flen += 1;
                                        flen;
                                    }
                                    let mut _n_10: size_t = flen as size_t;
                                    let mut _w_10: size_t = (if pad == '-' as i32
                                        || width < 0 as i32
                                    {
                                        0 as i32
                                    } else {
                                        width
                                    }) as size_t;
                                    let mut _incr_10: size_t = if _n_10 < _w_10 {
                                        _w_10
                                    } else {
                                        _n_10
                                    };
                                    if _incr_10 >= maxsize.wrapping_sub(i) {
                                        *__errno_location() = 34 as i32;
                                        return 0 as i32 as size_t;
                                    }
                                    if !p.is_null() {
                                        if _n_10 < _w_10 {
                                            let mut _delta_10: size_t = _w_10.wrapping_sub(_n_10);
                                            if pad == '0' as i32 || pad == '+' as i32 {
                                                memset(p as *mut libc::c_void, '0' as i32, _delta_10);
                                                p = p.offset(_delta_10 as isize);
                                            } else {
                                                memset(p as *mut libc::c_void, ' ' as i32, _delta_10);
                                                p = p.offset(_delta_10 as isize);
                                            }
                                        }
                                        if to_lowcase {
                                            memcpy_lowcase(
                                                p,
                                                &*f.offset((1 as i32 - flen) as isize),
                                                _n_10,
                                            );
                                        } else if to_uppcase {
                                            memcpy_uppcase(
                                                p,
                                                &*f.offset((1 as i32 - flen) as isize),
                                                _n_10,
                                            );
                                        } else {
                                            memcpy(
                                                p as *mut libc::c_void,
                                                &*f.offset((1 as i32 - flen) as isize) as *const i8
                                                    as *const libc::c_void,
                                                _n_10,
                                            );
                                        }
                                        p = p.offset(_n_10 as isize);
                                    }
                                    i = (i as u64).wrapping_add(_incr_10) as size_t as size_t;
                                    current_block = 3276175668257526147;
                                }
                                447645734570482032 => {
                                    let mut len: size_t = __strftime_internal(
                                        0 as *mut i8,
                                        -(1 as i32) as size_t,
                                        subfmt,
                                        tp,
                                        to_uppcase,
                                        pad,
                                        subwidth,
                                        tzset_called,
                                        tz,
                                        ns,
                                    );
                                    let mut _n_1: size_t = len;
                                    let mut _w_1: size_t = (if pad == '-' as i32
                                        || width < 0 as i32
                                    {
                                        0 as i32
                                    } else {
                                        width
                                    }) as size_t;
                                    let mut _incr_1: size_t = if _n_1 < _w_1 {
                                        _w_1
                                    } else {
                                        _n_1
                                    };
                                    if _incr_1 >= maxsize.wrapping_sub(i) {
                                        *__errno_location() = 34 as i32;
                                        return 0 as i32 as size_t;
                                    }
                                    if !p.is_null() {
                                        if _n_1 < _w_1 {
                                            let mut _delta_1: size_t = _w_1.wrapping_sub(_n_1);
                                            if pad == '0' as i32 || pad == '+' as i32 {
                                                memset(p as *mut libc::c_void, '0' as i32, _delta_1);
                                                p = p.offset(_delta_1 as isize);
                                            } else {
                                                memset(p as *mut libc::c_void, ' ' as i32, _delta_1);
                                                p = p.offset(_delta_1 as isize);
                                            }
                                        }
                                        __strftime_internal(
                                            p,
                                            maxsize.wrapping_sub(i),
                                            subfmt,
                                            tp,
                                            to_uppcase,
                                            pad,
                                            subwidth,
                                            tzset_called,
                                            tz,
                                            ns,
                                        );
                                        p = p.offset(_n_1 as isize);
                                    }
                                    i = (i as u64).wrapping_add(_incr_1) as size_t as size_t;
                                    current_block = 3276175668257526147;
                                }
                                _ => {}
                            }
                            match current_block {
                                3276175668257526147 => {}
                                _ => {
                                    match current_block {
                                        14619535680852412511 => {
                                            always_output_a_sign = 0 as i32 != 0;
                                            current_block = 16035518928380119276;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        16035518928380119276 => {
                                            tz_colon_mask = 0 as i32;
                                            current_block = 18027821804064647803;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        18027821804064647803 => {
                                            if modifier == 'O' as i32 && !negative_number {
                                                current_block = 12902579523164801321;
                                            } else {
                                                bufp = buf
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (::core::mem::size_of::<[i8; 23]>() as u64)
                                                            .wrapping_div(::core::mem::size_of::<i8>() as u64) as isize,
                                                    );
                                                if negative_number {
                                                    u_number_value = u_number_value.wrapping_neg();
                                                }
                                                loop {
                                                    if tz_colon_mask & 1 as i32 != 0 {
                                                        bufp = bufp.offset(-1);
                                                        *bufp = ':' as i32 as i8;
                                                    }
                                                    tz_colon_mask >>= 1 as i32;
                                                    bufp = bufp.offset(-1);
                                                    *bufp = u_number_value
                                                        .wrapping_rem(10 as i32 as u32)
                                                        .wrapping_add('0' as i32 as u32) as i8;
                                                    u_number_value = u_number_value
                                                        .wrapping_div(10 as i32 as u32);
                                                    if !(u_number_value != 0 as i32 as u32
                                                        || tz_colon_mask != 0 as i32)
                                                    {
                                                        break;
                                                    }
                                                }
                                                current_block = 6370517008983650009;
                                            }
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        12902579523164801321 => {
                                            let mut ufmt: [i8; 5] = [0; 5];
                                            let mut u: *mut i8 = ufmt.as_mut_ptr();
                                            let mut ubuf: [i8; 1024] = [0; 1024];
                                            let mut len_0: size_t = 0;
                                            let fresh5 = u;
                                            u = u.offset(1);
                                            *fresh5 = ' ' as i32 as i8;
                                            let fresh6 = u;
                                            u = u.offset(1);
                                            *fresh6 = '%' as i32 as i8;
                                            if modifier != 0 as i32 {
                                                let fresh7 = u;
                                                u = u.offset(1);
                                                *fresh7 = modifier as i8;
                                            }
                                            let fresh8 = u;
                                            u = u.offset(1);
                                            *fresh8 = format_char as i8;
                                            *u = '\0' as i32 as i8;
                                            len_0 = strftime(
                                                ubuf.as_mut_ptr(),
                                                ::core::mem::size_of::<[i8; 1024]>() as u64,
                                                ufmt.as_mut_ptr(),
                                                tp,
                                            );
                                            if len_0 != 0 as i32 as u64 {
                                                let mut _n_2: size_t = len_0.wrapping_sub(1 as i32 as u64);
                                                let mut _w_2: size_t = (if pad == '-' as i32
                                                    || width < 0 as i32
                                                {
                                                    0 as i32
                                                } else {
                                                    width
                                                }) as size_t;
                                                let mut _incr_2: size_t = if _n_2 < _w_2 {
                                                    _w_2
                                                } else {
                                                    _n_2
                                                };
                                                if _incr_2 >= maxsize.wrapping_sub(i) {
                                                    *__errno_location() = 34 as i32;
                                                    return 0 as i32 as size_t;
                                                }
                                                if !p.is_null() {
                                                    if _n_2 < _w_2 {
                                                        let mut _delta_2: size_t = _w_2.wrapping_sub(_n_2);
                                                        if pad == '0' as i32 || pad == '+' as i32 {
                                                            memset(p as *mut libc::c_void, '0' as i32, _delta_2);
                                                            p = p.offset(_delta_2 as isize);
                                                        } else {
                                                            memset(p as *mut libc::c_void, ' ' as i32, _delta_2);
                                                            p = p.offset(_delta_2 as isize);
                                                        }
                                                    }
                                                    if to_lowcase {
                                                        memcpy_lowcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as i32 as isize),
                                                            _n_2,
                                                        );
                                                    } else if to_uppcase {
                                                        memcpy_uppcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as i32 as isize),
                                                            _n_2,
                                                        );
                                                    } else {
                                                        memcpy(
                                                            p as *mut libc::c_void,
                                                            ubuf.as_mut_ptr().offset(1 as i32 as isize)
                                                                as *const libc::c_void,
                                                            _n_2,
                                                        );
                                                    }
                                                    p = p.offset(_n_2 as isize);
                                                }
                                                i = (i as u64).wrapping_add(_incr_2) as size_t as size_t;
                                            }
                                        }
                                        _ => {
                                            if pad == 0 as i32 {
                                                pad = '0' as i32;
                                            }
                                            if width < 0 as i32 {
                                                width = digits;
                                            }
                                            let mut sign_char: i8 = (if negative_number as i32 != 0 {
                                                '-' as i32
                                            } else if always_output_a_sign as i32 != 0 {
                                                '+' as i32
                                            } else {
                                                0 as i32
                                            }) as i8;
                                            let mut numlen: i32 = buf
                                                .as_mut_ptr()
                                                .offset(
                                                    (::core::mem::size_of::<[i8; 23]>() as u64)
                                                        .wrapping_div(::core::mem::size_of::<i8>() as u64) as isize,
                                                )
                                                .offset_from(bufp) as i64 as i32;
                                            let mut shortage: i32 = width - (sign_char != 0) as i32
                                                - numlen;
                                            let mut padding: i32 = if pad == '-' as i32
                                                || shortage <= 0 as i32
                                            {
                                                0 as i32
                                            } else {
                                                shortage
                                            };
                                            if sign_char != 0 {
                                                if pad == '_' as i32 {
                                                    if !p.is_null() {
                                                        memset(p as *mut libc::c_void, ' ' as i32, padding as u64);
                                                        p = p.offset(padding as isize);
                                                    }
                                                    i = (i as u64).wrapping_add(padding as u64) as size_t
                                                        as size_t;
                                                    width -= padding;
                                                }
                                                let mut _n_3: size_t = 1 as i32 as size_t;
                                                let mut _w_3: size_t = (if pad == '-' as i32
                                                    || (0 as i32) < 0 as i32
                                                {
                                                    0 as i32
                                                } else {
                                                    0 as i32
                                                }) as size_t;
                                                let mut _incr_3: size_t = if _n_3 < _w_3 {
                                                    _w_3
                                                } else {
                                                    _n_3
                                                };
                                                if _incr_3 >= maxsize.wrapping_sub(i) {
                                                    *__errno_location() = 34 as i32;
                                                    return 0 as i32 as size_t;
                                                }
                                                if !p.is_null() {
                                                    if _n_3 < _w_3 {
                                                        let mut _delta_3: size_t = _w_3.wrapping_sub(_n_3);
                                                        if pad == '0' as i32 || pad == '+' as i32 {
                                                            memset(p as *mut libc::c_void, '0' as i32, _delta_3);
                                                            p = p.offset(_delta_3 as isize);
                                                        } else {
                                                            memset(p as *mut libc::c_void, ' ' as i32, _delta_3);
                                                            p = p.offset(_delta_3 as isize);
                                                        }
                                                    }
                                                    *p = sign_char;
                                                    p = p.offset(_n_3 as isize);
                                                }
                                                i = (i as u64).wrapping_add(_incr_3) as size_t as size_t;
                                                width -= 1;
                                                width;
                                            }
                                            let mut _n_4: size_t = numlen as size_t;
                                            let mut _w_4: size_t = (if pad == '-' as i32
                                                || width < 0 as i32
                                            {
                                                0 as i32
                                            } else {
                                                width
                                            }) as size_t;
                                            let mut _incr_4: size_t = if _n_4 < _w_4 {
                                                _w_4
                                            } else {
                                                _n_4
                                            };
                                            if _incr_4 >= maxsize.wrapping_sub(i) {
                                                *__errno_location() = 34 as i32;
                                                return 0 as i32 as size_t;
                                            }
                                            if !p.is_null() {
                                                if _n_4 < _w_4 {
                                                    let mut _delta_4: size_t = _w_4.wrapping_sub(_n_4);
                                                    if pad == '0' as i32 || pad == '+' as i32 {
                                                        memset(p as *mut libc::c_void, '0' as i32, _delta_4);
                                                        p = p.offset(_delta_4 as isize);
                                                    } else {
                                                        memset(p as *mut libc::c_void, ' ' as i32, _delta_4);
                                                        p = p.offset(_delta_4 as isize);
                                                    }
                                                }
                                                if to_lowcase {
                                                    memcpy_lowcase(p, bufp, _n_4);
                                                } else if to_uppcase {
                                                    memcpy_uppcase(p, bufp, _n_4);
                                                } else {
                                                    memcpy(
                                                        p as *mut libc::c_void,
                                                        bufp as *const libc::c_void,
                                                        _n_4,
                                                    );
                                                }
                                                p = p.offset(_n_4 as isize);
                                            }
                                            i = (i as u64).wrapping_add(_incr_4) as size_t as size_t;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        width = -(1 as i32);
        f = f.offset(1);
        f;
    }
    if !p.is_null() && maxsize != 0 as i32 as u64 {
        *p = '\0' as i32 as i8;
    }
    *__errno_location() = saved_errno;
    return i;
}