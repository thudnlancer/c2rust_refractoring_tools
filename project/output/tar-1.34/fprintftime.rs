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
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn mktime_z(__tz: timezone_t, __result: *mut tm) -> time_t;
    fn __errno_location() -> *mut i32;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
pub type __int32_t = i32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
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
#[no_mangle]
pub unsafe extern "C" fn fprintftime(
    mut s: *mut FILE,
    mut format: *const i8,
    mut tp: *const tm,
    mut tz: timezone_t,
    mut ns: i32,
) -> size_t {
    let mut tzset_called: bool = 0 as i32 != 0;
    return __strftime_internal(
        s,
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
    mut s: *mut FILE,
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
    let mut maxsize: size_t = -(1 as i32) as size_t;
    let mut saved_errno: i32 = *__errno_location();
    let mut hour12: i32 = (*tp).tm_hour;
    let mut zone: *const i8 = 0 as *const i8;
    let mut i: size_t = 0 as i32 as size_t;
    let mut p: *mut FILE = s;
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
                        let mut _i: size_t = 0;
                        _i = 0 as i32 as size_t;
                        while _i < _delta {
                            fputc('0' as i32, p);
                            _i = _i.wrapping_add(1);
                            _i;
                        }
                    } else {
                        let mut _i_0: size_t = 0;
                        _i_0 = 0 as i32 as size_t;
                        while _i_0 < _delta {
                            fputc(' ' as i32, p);
                            _i_0 = _i_0.wrapping_add(1);
                            _i_0;
                        }
                    }
                }
                fputc(*f as i32, p);
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
                            let (fresh0, fresh1) = width
                                .overflowing_add(*f as i32 - '0' as i32);
                            *(&mut width as *mut i32) = fresh0;
                            fresh1 as i32 != 0
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
                    let fresh2 = f;
                    f = f.offset(1);
                    modifier = *fresh2 as i32;
                }
                _ => {
                    modifier = 0 as i32;
                }
            }
            format_char = *f as i32;
            match format_char {
                37 => {
                    if modifier != 0 as i32 {
                        current_block = 8403858818449876884;
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
                                    let mut _i_1: size_t = 0;
                                    _i_1 = 0 as i32 as size_t;
                                    while _i_1 < _delta_0 {
                                        fputc('0' as i32, p);
                                        _i_1 = _i_1.wrapping_add(1);
                                        _i_1;
                                    }
                                } else {
                                    let mut _i_2: size_t = 0;
                                    _i_2 = 0 as i32 as size_t;
                                    while _i_2 < _delta_0 {
                                        fputc(' ' as i32, p);
                                        _i_2 = _i_2.wrapping_add(1);
                                        _i_2;
                                    }
                                }
                            }
                            fputc(*f as i32, p);
                        }
                        i = (i as u64).wrapping_add(_incr_0) as size_t as size_t;
                        current_block = 1394248824506584008;
                    }
                }
                97 => {
                    if modifier != 0 as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        if change_case {
                            to_uppcase = 1 as i32 != 0;
                            to_lowcase = 0 as i32 != 0;
                        }
                        current_block = 3958843843407804156;
                    }
                }
                65 => {
                    if modifier != 0 as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        if change_case {
                            to_uppcase = 1 as i32 != 0;
                            to_lowcase = 0 as i32 != 0;
                        }
                        current_block = 3958843843407804156;
                    }
                }
                98 | 104 => {
                    if change_case {
                        to_uppcase = 1 as i32 != 0;
                        to_lowcase = 0 as i32 != 0;
                    }
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        current_block = 3958843843407804156;
                    }
                }
                66 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        if change_case {
                            to_uppcase = 1 as i32 != 0;
                            to_lowcase = 0 as i32 != 0;
                        }
                        current_block = 3958843843407804156;
                    }
                }
                99 => {
                    if modifier == 'O' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        current_block = 3958843843407804156;
                    }
                }
                67 => {
                    if modifier == 'E' as i32 {
                        current_block = 3958843843407804156;
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
                        current_block = 14058894172337169469;
                    }
                }
                120 => {
                    if modifier == 'O' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        current_block = 3958843843407804156;
                    }
                }
                68 => {
                    if modifier != 0 as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        subfmt = b"%m/%d/%y\0" as *const u8 as *const i8;
                        current_block = 3196985732147973637;
                    }
                }
                100 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_mday;
                        current_block = 15074307635129609231;
                    }
                }
                101 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_mday;
                        current_block = 8425457366631381623;
                    }
                }
                70 => {
                    if modifier != 0 as i32 {
                        current_block = 8403858818449876884;
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
                        current_block = 5218137901593810980;
                    }
                }
                72 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_hour;
                        current_block = 15074307635129609231;
                    }
                }
                73 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = hour12;
                        current_block = 15074307635129609231;
                    }
                }
                107 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_hour;
                        current_block = 8425457366631381623;
                    }
                }
                108 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = hour12;
                        current_block = 8425457366631381623;
                    }
                }
                106 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 3 as i32;
                        negative_number = (*tp).tm_yday < -(1 as i32);
                        u_number_value = ((*tp).tm_yday as u32).wrapping_add(1 as u32);
                        current_block = 16632252788676351511;
                    }
                }
                77 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_min;
                        current_block = 15074307635129609231;
                    }
                }
                109 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        negative_number = (*tp).tm_mon < -(1 as i32);
                        u_number_value = ((*tp).tm_mon as u32).wrapping_add(1 as u32);
                        current_block = 16632252788676351511;
                    }
                }
                78 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
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
                                    let mut _i_12: size_t = 0;
                                    _i_12 = 0 as i32 as size_t;
                                    while _i_12 < _delta_5 {
                                        fputc('0' as i32, p);
                                        _i_12 = _i_12.wrapping_add(1);
                                        _i_12;
                                    }
                                } else {
                                    let mut _i_13: size_t = 0;
                                    _i_13 = 0 as i32 as size_t;
                                    while _i_13 < _delta_5 {
                                        fputc(' ' as i32, p);
                                        _i_13 = _i_13.wrapping_add(1);
                                        _i_13;
                                    }
                                }
                            }
                            if to_lowcase {
                                fwrite_lowcase(p, buf.as_mut_ptr(), _n_5);
                            } else if to_uppcase {
                                fwrite_uppcase(p, buf.as_mut_ptr(), _n_5);
                            } else {
                                fwrite(
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    _n_5,
                                    1 as i32 as size_t,
                                    p,
                                );
                            }
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
                                    let mut _i_14: size_t = 0;
                                    _i_14 = 0 as i32 as size_t;
                                    while _i_14 < _delta_6 {
                                        fputc('0' as i32, p);
                                        _i_14 = _i_14.wrapping_add(1);
                                        _i_14;
                                    }
                                } else {
                                    let mut _i_15: size_t = 0;
                                    _i_15 = 0 as i32 as size_t;
                                    while _i_15 < _delta_6 {
                                        fputc(' ' as i32, p);
                                        _i_15 = _i_15.wrapping_add(1);
                                        _i_15;
                                    }
                                }
                            }
                        }
                        i = (i as u64).wrapping_add(_incr_6) as size_t as size_t;
                        current_block = 1394248824506584008;
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
                                let mut _i_16: size_t = 0;
                                _i_16 = 0 as i32 as size_t;
                                while _i_16 < _delta_7 {
                                    fputc('0' as i32, p);
                                    _i_16 = _i_16.wrapping_add(1);
                                    _i_16;
                                }
                            } else {
                                let mut _i_17: size_t = 0;
                                _i_17 = 0 as i32 as size_t;
                                while _i_17 < _delta_7 {
                                    fputc(' ' as i32, p);
                                    _i_17 = _i_17.wrapping_add(1);
                                    _i_17;
                                }
                            }
                        }
                        fputc('\n' as i32, p);
                    }
                    i = (i as u64).wrapping_add(_incr_7) as size_t as size_t;
                    current_block = 1394248824506584008;
                }
                80 => {
                    to_lowcase = 1 as i32 != 0;
                    format_char = 'p' as i32;
                    current_block = 17529221511509422402;
                }
                112 => {
                    current_block = 17529221511509422402;
                }
                113 => {
                    digits = 1 as i32;
                    negative_number = 0 as i32 != 0;
                    u_number_value = (((*tp).tm_mon * 11 as i32 >> 5 as i32) + 1 as i32)
                        as u32;
                    current_block = 16632252788676351511;
                }
                82 => {
                    subfmt = b"%H:%M\0" as *const u8 as *const i8;
                    current_block = 3196985732147973637;
                }
                114 => {
                    current_block = 3958843843407804156;
                }
                83 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = (*tp).tm_sec;
                        current_block = 15074307635129609231;
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
                    current_block = 7649788183169827419;
                }
                88 => {
                    if modifier == 'O' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        current_block = 3958843843407804156;
                    }
                }
                84 => {
                    subfmt = b"%H:%M:%S\0" as *const u8 as *const i8;
                    current_block = 3196985732147973637;
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
                                let mut _i_18: size_t = 0;
                                _i_18 = 0 as i32 as size_t;
                                while _i_18 < _delta_8 {
                                    fputc('0' as i32, p);
                                    _i_18 = _i_18.wrapping_add(1);
                                    _i_18;
                                }
                            } else {
                                let mut _i_19: size_t = 0;
                                _i_19 = 0 as i32 as size_t;
                                while _i_19 < _delta_8 {
                                    fputc(' ' as i32, p);
                                    _i_19 = _i_19.wrapping_add(1);
                                    _i_19;
                                }
                            }
                        }
                        fputc('\t' as i32, p);
                    }
                    i = (i as u64).wrapping_add(_incr_8) as size_t as size_t;
                    current_block = 1394248824506584008;
                }
                117 => {
                    digits = 1 as i32;
                    number_value = ((*tp).tm_wday - 1 as i32 + 7 as i32) % 7 as i32
                        + 1 as i32;
                    current_block = 15074307635129609231;
                }
                85 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = ((*tp).tm_yday - (*tp).tm_wday + 7 as i32)
                            / 7 as i32;
                        current_block = 15074307635129609231;
                    }
                }
                86 | 103 | 71 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
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
                                current_block = 14058894172337169469;
                            }
                            71 => {
                                digits = 4 as i32;
                                negative_number = (*tp).tm_year
                                    < -(1900 as i32) - year_adjust;
                                u_number_value = ((*tp).tm_year as u32)
                                    .wrapping_add(1900 as i32 as u32)
                                    .wrapping_add(year_adjust as u32);
                                current_block = 14058894172337169469;
                            }
                            _ => {
                                digits = 2 as i32;
                                number_value = days / 7 as i32 + 1 as i32;
                                current_block = 15074307635129609231;
                            }
                        }
                    }
                }
                87 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 2 as i32;
                        number_value = ((*tp).tm_yday
                            - ((*tp).tm_wday - 1 as i32 + 7 as i32) % 7 as i32
                            + 7 as i32) / 7 as i32;
                        current_block = 15074307635129609231;
                    }
                }
                119 => {
                    if modifier == 'E' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 1 as i32;
                        number_value = (*tp).tm_wday;
                        current_block = 15074307635129609231;
                    }
                }
                89 => {
                    if modifier == 'E' as i32 {
                        current_block = 3958843843407804156;
                    } else if modifier == 'O' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        digits = 4 as i32;
                        negative_number = (*tp).tm_year < -(1900 as i32);
                        u_number_value = ((*tp).tm_year as u32)
                            .wrapping_add(1900 as i32 as u32);
                        current_block = 14058894172337169469;
                    }
                }
                121 => {
                    if modifier == 'E' as i32 {
                        current_block = 3958843843407804156;
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
                        current_block = 14058894172337169469;
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
                                let mut _i_20: size_t = 0;
                                _i_20 = 0 as i32 as size_t;
                                while _i_20 < _delta_9 {
                                    fputc('0' as i32, p);
                                    _i_20 = _i_20.wrapping_add(1);
                                    _i_20;
                                }
                            } else {
                                let mut _i_21: size_t = 0;
                                _i_21 = 0 as i32 as size_t;
                                while _i_21 < _delta_9 {
                                    fputc(' ' as i32, p);
                                    _i_21 = _i_21.wrapping_add(1);
                                    _i_21;
                                }
                            }
                        }
                        if to_lowcase {
                            fwrite_lowcase(p, zone, _n_9);
                        } else if to_uppcase {
                            fwrite_uppcase(p, zone, _n_9);
                        } else {
                            fwrite(
                                zone as *const libc::c_void,
                                _n_9,
                                1 as i32 as size_t,
                                p,
                            );
                        }
                    }
                    i = (i as u64).wrapping_add(_incr_9) as size_t as size_t;
                    current_block = 1394248824506584008;
                }
                58 => {
                    colons = 1 as i32 as size_t;
                    while *f.offset(colons as isize) as i32 == ':' as i32 {
                        colons = colons.wrapping_add(1);
                        colons;
                    }
                    if *f.offset(colons as isize) as i32 != 'z' as i32 {
                        current_block = 8403858818449876884;
                    } else {
                        f = f.offset(colons as isize);
                        current_block = 5482672315040824105;
                    }
                }
                122 => {
                    colons = 0 as i32 as size_t;
                    current_block = 5482672315040824105;
                }
                0 => {
                    f = f.offset(-1);
                    f;
                    current_block = 8403858818449876884;
                }
                _ => {
                    current_block = 8403858818449876884;
                }
            }
            match current_block {
                1394248824506584008 => {}
                _ => {
                    match current_block {
                        14058894172337169469 => {
                            if pad == 0 as i32 {
                                pad = yr_spec;
                            }
                            always_output_a_sign = pad == '+' as i32
                                && (((if digits == 2 as i32 {
                                    99 as i32
                                } else {
                                    9999 as i32
                                }) as u32) < u_number_value || digits < width);
                            current_block = 18305941293582774070;
                        }
                        5482672315040824105 => {
                            if (*tp).tm_isdst < 0 as i32 {
                                current_block = 1394248824506584008;
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
                                        current_block = 11256224404817644654;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 1563745629155145216;
                                            }
                                            14185338118334136158 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 1563745629155145216;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 17630876937447939046;
                                    }
                                    1 => {
                                        current_block = 9136364834649731412;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 1563745629155145216;
                                            }
                                            14185338118334136158 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 1563745629155145216;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 17630876937447939046;
                                    }
                                    2 => {
                                        current_block = 13364777937596457114;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 1563745629155145216;
                                            }
                                            14185338118334136158 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 1563745629155145216;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 17630876937447939046;
                                    }
                                    3 => {
                                        current_block = 14185338118334136158;
                                        match current_block {
                                            11256224404817644654 => {
                                                digits = 5 as i32;
                                                tz_colon_mask = 0 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                                current_block = 1563745629155145216;
                                            }
                                            14185338118334136158 => {
                                                if sec_diff != 0 as i32 {
                                                    current_block = 13364777937596457114;
                                                } else if min_diff != 0 as i32 {
                                                    current_block = 9136364834649731412;
                                                } else {
                                                    digits = 3 as i32;
                                                    tz_colon_mask = 0 as i32;
                                                    u_number_value = hour_diff as u32;
                                                    current_block = 1563745629155145216;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            9136364834649731412 => {
                                                digits = 6 as i32;
                                                tz_colon_mask = 0o4 as i32;
                                                u_number_value = (hour_diff * 100 as i32 + min_diff) as u32;
                                            }
                                            13364777937596457114 => {
                                                digits = 9 as i32;
                                                tz_colon_mask = 0o24 as i32;
                                                u_number_value = (hour_diff * 10000 as i32
                                                    + min_diff * 100 as i32 + sec_diff) as u32;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as i32 != 0;
                                        current_block = 17630876937447939046;
                                    }
                                    _ => {
                                        current_block = 8403858818449876884;
                                    }
                                }
                            }
                        }
                        17529221511509422402 => {
                            if change_case {
                                to_uppcase = 0 as i32 != 0;
                                to_lowcase = 1 as i32 != 0;
                            }
                            current_block = 3958843843407804156;
                        }
                        8425457366631381623 => {
                            if pad == 0 as i32 {
                                pad = '_' as i32;
                            }
                            current_block = 15074307635129609231;
                        }
                        3196985732147973637 => {
                            subwidth = -(1 as i32);
                            current_block = 5218137901593810980;
                        }
                        _ => {}
                    }
                    match current_block {
                        1394248824506584008 => {}
                        _ => {
                            match current_block {
                                15074307635129609231 => {
                                    negative_number = number_value < 0 as i32;
                                    u_number_value = number_value as u32;
                                    current_block = 16632252788676351511;
                                }
                                8403858818449876884 => {
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
                                                let mut _i_22: size_t = 0;
                                                _i_22 = 0 as i32 as size_t;
                                                while _i_22 < _delta_10 {
                                                    fputc('0' as i32, p);
                                                    _i_22 = _i_22.wrapping_add(1);
                                                    _i_22;
                                                }
                                            } else {
                                                let mut _i_23: size_t = 0;
                                                _i_23 = 0 as i32 as size_t;
                                                while _i_23 < _delta_10 {
                                                    fputc(' ' as i32, p);
                                                    _i_23 = _i_23.wrapping_add(1);
                                                    _i_23;
                                                }
                                            }
                                        }
                                        if to_lowcase {
                                            fwrite_lowcase(
                                                p,
                                                &*f.offset((1 as i32 - flen) as isize),
                                                _n_10,
                                            );
                                        } else if to_uppcase {
                                            fwrite_uppcase(
                                                p,
                                                &*f.offset((1 as i32 - flen) as isize),
                                                _n_10,
                                            );
                                        } else {
                                            fwrite(
                                                &*f.offset((1 as i32 - flen) as isize) as *const i8
                                                    as *const libc::c_void,
                                                _n_10,
                                                1 as i32 as size_t,
                                                p,
                                            );
                                        }
                                    }
                                    i = (i as u64).wrapping_add(_incr_10) as size_t as size_t;
                                    current_block = 1394248824506584008;
                                }
                                5218137901593810980 => {
                                    let mut len: size_t = __strftime_internal(
                                        0 as *mut FILE,
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
                                                let mut _i_3: size_t = 0;
                                                _i_3 = 0 as i32 as size_t;
                                                while _i_3 < _delta_1 {
                                                    fputc('0' as i32, p);
                                                    _i_3 = _i_3.wrapping_add(1);
                                                    _i_3;
                                                }
                                            } else {
                                                let mut _i_4: size_t = 0;
                                                _i_4 = 0 as i32 as size_t;
                                                while _i_4 < _delta_1 {
                                                    fputc(' ' as i32, p);
                                                    _i_4 = _i_4.wrapping_add(1);
                                                    _i_4;
                                                }
                                            }
                                        }
                                        __strftime_internal(
                                            p,
                                            subfmt,
                                            tp,
                                            to_uppcase,
                                            pad,
                                            subwidth,
                                            tzset_called,
                                            tz,
                                            ns,
                                        );
                                    }
                                    i = (i as u64).wrapping_add(_incr_1) as size_t as size_t;
                                    current_block = 1394248824506584008;
                                }
                                _ => {}
                            }
                            match current_block {
                                1394248824506584008 => {}
                                _ => {
                                    match current_block {
                                        16632252788676351511 => {
                                            always_output_a_sign = 0 as i32 != 0;
                                            current_block = 18305941293582774070;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        18305941293582774070 => {
                                            tz_colon_mask = 0 as i32;
                                            current_block = 17630876937447939046;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        17630876937447939046 => {
                                            if modifier == 'O' as i32 && !negative_number {
                                                current_block = 3958843843407804156;
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
                                                current_block = 7649788183169827419;
                                            }
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        3958843843407804156 => {
                                            let mut ufmt: [i8; 5] = [0; 5];
                                            let mut u: *mut i8 = ufmt.as_mut_ptr();
                                            let mut ubuf: [i8; 1024] = [0; 1024];
                                            let mut len_0: size_t = 0;
                                            let fresh3 = u;
                                            u = u.offset(1);
                                            *fresh3 = ' ' as i32 as i8;
                                            let fresh4 = u;
                                            u = u.offset(1);
                                            *fresh4 = '%' as i32 as i8;
                                            if modifier != 0 as i32 {
                                                let fresh5 = u;
                                                u = u.offset(1);
                                                *fresh5 = modifier as i8;
                                            }
                                            let fresh6 = u;
                                            u = u.offset(1);
                                            *fresh6 = format_char as i8;
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
                                                            let mut _i_5: size_t = 0;
                                                            _i_5 = 0 as i32 as size_t;
                                                            while _i_5 < _delta_2 {
                                                                fputc('0' as i32, p);
                                                                _i_5 = _i_5.wrapping_add(1);
                                                                _i_5;
                                                            }
                                                        } else {
                                                            let mut _i_6: size_t = 0;
                                                            _i_6 = 0 as i32 as size_t;
                                                            while _i_6 < _delta_2 {
                                                                fputc(' ' as i32, p);
                                                                _i_6 = _i_6.wrapping_add(1);
                                                                _i_6;
                                                            }
                                                        }
                                                    }
                                                    if to_lowcase {
                                                        fwrite_lowcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as i32 as isize),
                                                            _n_2,
                                                        );
                                                    } else if to_uppcase {
                                                        fwrite_uppcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as i32 as isize),
                                                            _n_2,
                                                        );
                                                    } else {
                                                        fwrite(
                                                            ubuf.as_mut_ptr().offset(1 as i32 as isize)
                                                                as *const libc::c_void,
                                                            _n_2,
                                                            1 as i32 as size_t,
                                                            p,
                                                        );
                                                    }
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
                                                        let mut _i_7: size_t = 0;
                                                        _i_7 = 0 as i32 as size_t;
                                                        while _i_7 < padding as u64 {
                                                            fputc(' ' as i32, p);
                                                            _i_7 = _i_7.wrapping_add(1);
                                                            _i_7;
                                                        }
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
                                                            let mut _i_8: size_t = 0;
                                                            _i_8 = 0 as i32 as size_t;
                                                            while _i_8 < _delta_3 {
                                                                fputc('0' as i32, p);
                                                                _i_8 = _i_8.wrapping_add(1);
                                                                _i_8;
                                                            }
                                                        } else {
                                                            let mut _i_9: size_t = 0;
                                                            _i_9 = 0 as i32 as size_t;
                                                            while _i_9 < _delta_3 {
                                                                fputc(' ' as i32, p);
                                                                _i_9 = _i_9.wrapping_add(1);
                                                                _i_9;
                                                            }
                                                        }
                                                    }
                                                    fputc(sign_char as i32, p);
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
                                                        let mut _i_10: size_t = 0;
                                                        _i_10 = 0 as i32 as size_t;
                                                        while _i_10 < _delta_4 {
                                                            fputc('0' as i32, p);
                                                            _i_10 = _i_10.wrapping_add(1);
                                                            _i_10;
                                                        }
                                                    } else {
                                                        let mut _i_11: size_t = 0;
                                                        _i_11 = 0 as i32 as size_t;
                                                        while _i_11 < _delta_4 {
                                                            fputc(' ' as i32, p);
                                                            _i_11 = _i_11.wrapping_add(1);
                                                            _i_11;
                                                        }
                                                    }
                                                }
                                                if to_lowcase {
                                                    fwrite_lowcase(p, bufp, _n_4);
                                                } else if to_uppcase {
                                                    fwrite_uppcase(p, bufp, _n_4);
                                                } else {
                                                    fwrite(
                                                        bufp as *const libc::c_void,
                                                        _n_4,
                                                        1 as i32 as size_t,
                                                        p,
                                                    );
                                                }
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
    *__errno_location() = saved_errno;
    return i;
}
unsafe extern "C" fn fwrite_uppcase(
    mut fp: *mut FILE,
    mut src: *const i8,
    mut len: size_t,
) {
    loop {
        let fresh7 = len;
        len = len.wrapping_sub(1);
        if !(fresh7 > 0 as i32 as u64) {
            break;
        }
        fputc(
            ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *src as u8 as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = toupper(*src as u8 as i32);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(*src as u8 as i32 as isize);
                }
                __res
            }),
            fp,
        );
        src = src.offset(1);
        src;
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
unsafe extern "C" fn fwrite_lowcase(
    mut fp: *mut FILE,
    mut src: *const i8,
    mut len: size_t,
) {
    loop {
        let fresh8 = len;
        len = len.wrapping_sub(1);
        if !(fresh8 > 0 as i32 as u64) {
            break;
        }
        fputc(
            ({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *src as u8 as i32;
                        __res = if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        __res = tolower(*src as u8 as i32);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc()).offset(*src as u8 as i32 as isize);
                }
                __res
            }),
            fp,
        );
        src = src.offset(1);
        src;
    };
}
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn iso_week_days(mut yday: i32, mut wday: i32) -> i32 {
    let mut big_enough_multiple_of_7: i32 = (--(366 as i32) / 7 as i32 + 2 as i32)
        * 7 as i32;
    return yday - (yday - wday + 4 as i32 + big_enough_multiple_of_7) % 7 as i32
        + 4 as i32 - 1 as i32;
}