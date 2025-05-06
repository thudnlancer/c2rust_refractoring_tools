#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strndup(_: *const i8, _: u64) -> *mut i8;
    fn _nc_free(ptr: *mut libc::c_void, name: *const i8, line: i32);
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
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type rstatus_t = i32;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn string_init(mut str: *mut string) {
    (*str).len = 0 as i32 as uint32_t;
    (*str).data = 0 as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn string_deinit(mut str: *mut string) {
    if !((*str).data).is_null() {
        _nc_free(
            (*str).data as *mut libc::c_void,
            b"nc_string.c\0" as *const u8 as *const i8,
            54 as i32,
        );
        (*str).data = 0 as *mut uint8_t;
        string_init(str);
    }
}
#[no_mangle]
pub unsafe extern "C" fn string_empty(mut str: *const string) -> bool {
    return (*str).len == 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn string_duplicate(
    mut dst: *mut string,
    mut src: *const string,
) -> rstatus_t {
    (*dst).data = strndup(
        (*src).data as *mut i8,
        ((*src).len).wrapping_add(1 as i32 as u32) as size_t,
    ) as *mut uint8_t;
    if ((*dst).data).is_null() {
        return -(3 as i32);
    }
    (*dst).len = (*src).len;
    *((*dst).data).offset((*dst).len as isize) = '\0' as i32 as uint8_t;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn string_copy(
    mut dst: *mut string,
    mut src: *const uint8_t,
    mut srclen: uint32_t,
) -> rstatus_t {
    (*dst).data = strndup(src as *mut i8, srclen.wrapping_add(1 as i32 as u32) as size_t)
        as *mut uint8_t;
    if ((*dst).data).is_null() {
        return -(3 as i32);
    }
    (*dst).len = srclen;
    *((*dst).data).offset((*dst).len as isize) = '\0' as i32 as uint8_t;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn string_compare(
    mut s1: *const string,
    mut s2: *const string,
) -> i32 {
    if (*s1).len != (*s2).len {
        return if (*s1).len > (*s2).len { 1 as i32 } else { -(1 as i32) };
    }
    return strncmp((*s1).data as *mut i8, (*s2).data as *mut i8, (*s1).len as size_t);
}
static mut hex: *const i8 = b"0123456789abcdef\0" as *const u8 as *const i8;
unsafe extern "C" fn _safe_utoa(
    mut _base: i32,
    mut val: uint64_t,
    mut buf: *mut i8,
) -> *mut i8 {
    let mut base: uint32_t = _base as uint32_t;
    let fresh0 = buf;
    buf = buf.offset(-1);
    *fresh0 = 0 as i32 as i8;
    loop {
        let fresh1 = buf;
        buf = buf.offset(-1);
        *fresh1 = *hex.offset(val.wrapping_rem(base as u64) as isize);
        val = (val as u64).wrapping_div(base as u64) as uint64_t as uint64_t;
        if !(val != 0 as i32 as u64) {
            break;
        }
    }
    return buf.offset(1 as i32 as isize);
}
unsafe extern "C" fn _safe_itoa(
    mut base: i32,
    mut val: int64_t,
    mut buf: *mut i8,
) -> *mut i8 {
    let mut orig_buf: *mut i8 = buf;
    let is_neg: int32_t = (val < 0 as i32 as i64) as i32;
    let fresh2 = buf;
    buf = buf.offset(-1);
    *fresh2 = 0 as i32 as i8;
    if is_neg != 0 {
        val = -val;
    }
    if is_neg != 0 && base == 16 as i32 {
        let mut ix: i32 = 0;
        val -= 1 as i32 as i64;
        ix = 0 as i32;
        while ix < 16 as i32 {
            *buf.offset(-ix as isize) = '0' as i32 as i8;
            ix += 1;
            ix;
        }
    }
    loop {
        let fresh3 = buf;
        buf = buf.offset(-1);
        *fresh3 = *hex.offset((val % base as i64) as isize);
        val /= base as i64;
        if !(val != 0 as i32 as i64) {
            break;
        }
    }
    if is_neg != 0 && base == 10 as i32 {
        let fresh4 = buf;
        buf = buf.offset(-1);
        *fresh4 = '-' as i32 as i8;
    }
    if is_neg != 0 && base == 16 as i32 {
        let mut ix_0: i32 = 0;
        buf = orig_buf.offset(-(1 as i32 as isize));
        ix_0 = 0 as i32;
        while ix_0 < 16 as i32 {
            match *buf as i32 {
                48 => {
                    *buf = 'f' as i32 as i8;
                }
                49 => {
                    *buf = 'e' as i32 as i8;
                }
                50 => {
                    *buf = 'd' as i32 as i8;
                }
                51 => {
                    *buf = 'c' as i32 as i8;
                }
                52 => {
                    *buf = 'b' as i32 as i8;
                }
                53 => {
                    *buf = 'a' as i32 as i8;
                }
                54 => {
                    *buf = '9' as i32 as i8;
                }
                55 => {
                    *buf = '8' as i32 as i8;
                }
                56 => {
                    *buf = '7' as i32 as i8;
                }
                57 => {
                    *buf = '6' as i32 as i8;
                }
                97 => {
                    *buf = '5' as i32 as i8;
                }
                98 => {
                    *buf = '4' as i32 as i8;
                }
                99 => {
                    *buf = '3' as i32 as i8;
                }
                100 => {
                    *buf = '2' as i32 as i8;
                }
                101 => {
                    *buf = '1' as i32 as i8;
                }
                102 => {
                    *buf = '0' as i32 as i8;
                }
                _ => {}
            }
            ix_0 += 1;
            ix_0;
            buf = buf.offset(-1);
            buf;
        }
    }
    return buf.offset(1 as i32 as isize);
}
unsafe extern "C" fn _safe_check_longlong(
    mut fmt: *const i8,
    mut have_longlong: *mut int32_t,
) -> *const i8 {
    *have_longlong = 0 as i32;
    if *fmt as i32 == 'l' as i32 {
        fmt = fmt.offset(1);
        fmt;
        if *fmt as i32 != 'l' as i32 {
            *have_longlong = (::core::mem::size_of::<i64>() as u64
                == ::core::mem::size_of::<int64_t>() as u64) as i32;
        } else {
            fmt = fmt.offset(1);
            fmt;
            *have_longlong = 1 as i32;
        }
    }
    return fmt;
}
#[no_mangle]
pub unsafe extern "C" fn _safe_vsnprintf(
    mut to: *mut i8,
    mut size: size_t,
    mut format: *const i8,
    mut ap: ::core::ffi::VaList,
) -> i32 {
    let mut start: *mut i8 = to;
    let mut end: *mut i8 = start.offset(size as isize).offset(-(1 as i32 as isize));
    let mut current_block_29: u64;
    while *format != 0 {
        let mut have_longlong: int32_t = 0 as i32;
        if *format as i32 != '%' as i32 {
            if to == end {
                break;
            }
            let fresh5 = to;
            to = to.offset(1);
            *fresh5 = *format;
        } else {
            format = format.offset(1);
            format;
            format = _safe_check_longlong(format, &mut have_longlong);
            match *format as i32 {
                100 | 105 | 117 | 120 | 112 => {
                    current_block_29 = 7815301370352969686;
                    match current_block_29 {
                        4775909272756257391 => {
                            let mut val: *const i8 = ap.arg::<*mut i8>();
                            if val.is_null() {
                                val = b"(null)\0" as *const u8 as *const i8;
                            }
                            while *val as i32 != 0 && to < end {
                                let fresh8 = val;
                                val = val.offset(1);
                                let fresh9 = to;
                                to = to.offset(1);
                                *fresh9 = *fresh8;
                            }
                        }
                        _ => {
                            let mut ival: int64_t = 0 as i32 as int64_t;
                            let mut uval: uint64_t = 0 as i32 as uint64_t;
                            if *format as i32 == 'p' as i32 {
                                have_longlong = (::core::mem::size_of::<*mut libc::c_void>()
                                    as u64 == ::core::mem::size_of::<uint64_t>() as u64) as i32;
                            }
                            if have_longlong != 0 {
                                if *format as i32 == 'u' as i32 {
                                    uval = ap.arg::<uint64_t>();
                                } else {
                                    ival = ap.arg::<int64_t>();
                                }
                            } else if *format as i32 == 'u' as i32 {
                                uval = ap.arg::<uint32_t>() as uint64_t;
                            } else {
                                ival = ap.arg::<int32_t>() as int64_t;
                            }
                            let mut buff: [i8; 22] = [0; 22];
                            let base: i32 = if *format as i32 == 'x' as i32
                                || *format as i32 == 'p' as i32
                            {
                                16 as i32
                            } else {
                                10 as i32
                            };
                            let mut val_as_str: *mut i8 = if *format as i32 == 'u' as i32
                            {
                                _safe_utoa(
                                    base,
                                    uval,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::core::mem::size_of::<[i8; 22]>() as u64)
                                                .wrapping_sub(1 as i32 as u64) as isize,
                                        ),
                                )
                            } else {
                                _safe_itoa(
                                    base,
                                    ival,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::core::mem::size_of::<[i8; 22]>() as u64)
                                                .wrapping_sub(1 as i32 as u64) as isize,
                                        ),
                                )
                            };
                            if *format as i32 == 'x' as i32 && have_longlong == 0
                                && ival < 0 as i32 as i64
                            {
                                val_as_str = val_as_str.offset(8 as i32 as isize);
                            }
                            while *val_as_str as i32 != 0 && to < end {
                                let fresh6 = val_as_str;
                                val_as_str = val_as_str.offset(1);
                                let fresh7 = to;
                                to = to.offset(1);
                                *fresh7 = *fresh6;
                            }
                        }
                    }
                }
                115 => {
                    current_block_29 = 4775909272756257391;
                    match current_block_29 {
                        4775909272756257391 => {
                            let mut val: *const i8 = ap.arg::<*mut i8>();
                            if val.is_null() {
                                val = b"(null)\0" as *const u8 as *const i8;
                            }
                            while *val as i32 != 0 && to < end {
                                let fresh8 = val;
                                val = val.offset(1);
                                let fresh9 = to;
                                to = to.offset(1);
                                *fresh9 = *fresh8;
                            }
                        }
                        _ => {
                            let mut ival: int64_t = 0 as i32 as int64_t;
                            let mut uval: uint64_t = 0 as i32 as uint64_t;
                            if *format as i32 == 'p' as i32 {
                                have_longlong = (::core::mem::size_of::<*mut libc::c_void>()
                                    as u64 == ::core::mem::size_of::<uint64_t>() as u64) as i32;
                            }
                            if have_longlong != 0 {
                                if *format as i32 == 'u' as i32 {
                                    uval = ap.arg::<uint64_t>();
                                } else {
                                    ival = ap.arg::<int64_t>();
                                }
                            } else if *format as i32 == 'u' as i32 {
                                uval = ap.arg::<uint32_t>() as uint64_t;
                            } else {
                                ival = ap.arg::<int32_t>() as int64_t;
                            }
                            let mut buff: [i8; 22] = [0; 22];
                            let base: i32 = if *format as i32 == 'x' as i32
                                || *format as i32 == 'p' as i32
                            {
                                16 as i32
                            } else {
                                10 as i32
                            };
                            let mut val_as_str: *mut i8 = if *format as i32 == 'u' as i32
                            {
                                _safe_utoa(
                                    base,
                                    uval,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::core::mem::size_of::<[i8; 22]>() as u64)
                                                .wrapping_sub(1 as i32 as u64) as isize,
                                        ),
                                )
                            } else {
                                _safe_itoa(
                                    base,
                                    ival,
                                    &mut *buff
                                        .as_mut_ptr()
                                        .offset(
                                            (::core::mem::size_of::<[i8; 22]>() as u64)
                                                .wrapping_sub(1 as i32 as u64) as isize,
                                        ),
                                )
                            };
                            if *format as i32 == 'x' as i32 && have_longlong == 0
                                && ival < 0 as i32 as i64
                            {
                                val_as_str = val_as_str.offset(8 as i32 as isize);
                            }
                            while *val_as_str as i32 != 0 && to < end {
                                let fresh6 = val_as_str;
                                val_as_str = val_as_str.offset(1);
                                let fresh7 = to;
                                to = to.offset(1);
                                *fresh7 = *fresh6;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        format = format.offset(1);
        format;
    }
    *to = 0 as i32 as i8;
    return to.offset_from(start) as i64 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _safe_snprintf(
    mut to: *mut i8,
    mut n: size_t,
    mut fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut result: i32 = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    result = _safe_vsnprintf(to, n, fmt, args_0.as_va_list());
    return result;
}