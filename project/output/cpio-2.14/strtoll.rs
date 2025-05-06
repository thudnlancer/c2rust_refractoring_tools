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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __errno_location() -> *mut i32;
}
pub type __int32_t = i32;
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn rpl_strtoll(
    mut nptr: *const i8,
    mut endptr: *mut *mut i8,
    mut base: i32,
) -> libc::c_longlong {
    let mut negative: i32 = 0;
    let mut cutoff: libc::c_ulonglong = 0;
    let mut cutlim: u32 = 0;
    let mut i: libc::c_ulonglong = 0;
    let mut s: *const i8 = 0 as *const i8;
    let mut c: u8 = 0;
    let mut save: *const i8 = 0 as *const i8;
    let mut end: *const i8 = 0 as *const i8;
    let mut overflow: i32 = 0;
    if base < 0 as i32 || base == 1 as i32 || base > 36 as i32 {
        *__errno_location() = 22 as i32;
        return 0 as i32 as libc::c_longlong;
    }
    s = nptr;
    save = s;
    while *(*__ctype_b_loc()).offset(*s as u8 as i32 as isize) as i32
        & _ISspace as i32 as libc::c_ushort as i32 != 0
    {
        s = s.offset(1);
        s;
    }
    if !(*s as i32 == '\0' as i32) {
        if *s as i32 == '-' as i32 {
            negative = 1 as i32;
            s = s.offset(1);
            s;
        } else if *s as i32 == '+' as i32 {
            negative = 0 as i32;
            s = s.offset(1);
            s;
        } else {
            negative = 0 as i32;
        }
        if *s as i32 == '0' as i32 {
            if (base == 0 as i32 || base == 16 as i32)
                && ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *s.offset(1 as i32 as isize) as u8 as i32;
                            __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(*s.offset(1 as i32 as isize) as u8 as i32);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize);
                    }
                    __res
                }) == 'X' as i32
            {
                s = s.offset(2 as i32 as isize);
                base = 16 as i32;
            } else if (base == 0 as i32 || base == 2 as i32)
                && ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *s.offset(1 as i32 as isize) as u8 as i32;
                            __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(*s.offset(1 as i32 as isize) as u8 as i32);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*s.offset(1 as i32 as isize) as u8 as i32 as isize);
                    }
                    __res
                }) == 'B' as i32
            {
                s = s.offset(2 as i32 as isize);
                base = 2 as i32;
            } else if base == 0 as i32 {
                base = 8 as i32;
            }
        } else if base == 0 as i32 {
            base = 10 as i32;
        }
        save = s;
        end = 0 as *const i8;
        cutoff = (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(base as libc::c_ulonglong);
        cutlim = (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_rem(base as libc::c_ulonglong) as u32;
        overflow = 0 as i32;
        i = 0 as i32 as libc::c_ulonglong;
        c = *s as u8;
        while c as i32 != '\0' as i32 {
            if s == end {
                break;
            }
            if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
                c = (c as i32 - '0' as i32) as u8;
            } else {
                if !(*(*__ctype_b_loc()).offset(c as i32 as isize) as i32
                    & _ISalpha as i32 as libc::c_ushort as i32 != 0)
                {
                    break;
                }
                c = (({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = c as i32;
                            __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(c as i32);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc()).offset(c as i32 as isize);
                    }
                    __res
                }) - 'A' as i32 + 10 as i32) as u8;
            }
            if c as i32 >= base {
                break;
            }
            if i > cutoff || i == cutoff && c as u32 > cutlim {
                overflow = 1 as i32;
            } else {
                i = i.wrapping_mul(base as libc::c_ulonglong);
                i = i.wrapping_add(c as libc::c_ulonglong);
            }
            s = s.offset(1);
            c = *s as u8;
        }
        if !(s == save) {
            if !endptr.is_null() {
                *endptr = s as *mut i8;
            }
            if overflow == 0 as i32
                && i
                    > (if negative != 0 {
                        ((-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong + 1 as i32 as libc::c_longlong)
                            as libc::c_ulonglong)
                            .wrapping_neg()
                            .wrapping_add(1 as i32 as libc::c_ulonglong)
                    } else {
                        9223372036854775807 as libc::c_longlong as libc::c_ulonglong
                    })
            {
                overflow = 1 as i32;
            }
            if overflow != 0 {
                *__errno_location() = 34 as i32;
                return if negative != 0 {
                    -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                } else {
                    9223372036854775807 as libc::c_longlong
                };
            }
            return (if negative != 0 { i.wrapping_neg() } else { i })
                as libc::c_longlong;
        }
    }
    if !endptr.is_null() {
        if save.offset_from(nptr) as i64 >= 2 as i32 as i64
            && (({
                let mut __res: i32 = 0;
                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                    if 0 != 0 {
                        let mut __c: i32 = *save.offset(-(1 as i32) as isize) as u8
                            as i32;
                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(*save.offset(-(1 as i32) as isize) as u8 as i32);
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(
                            *save.offset(-(1 as i32) as isize) as u8 as i32 as isize,
                        );
                }
                __res
            }) == 'X' as i32
                || ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *save.offset(-(1 as i32) as isize) as u8
                                as i32;
                            __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(
                                *save.offset(-(1 as i32) as isize) as u8 as i32,
                            );
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(
                                *save.offset(-(1 as i32) as isize) as u8 as i32 as isize,
                            );
                    }
                    __res
                }) == 'B' as i32)
            && *save.offset(-(2 as i32) as isize) as i32 == '0' as i32
        {
            *endptr = &*save.offset(-(1 as i32) as isize) as *const i8 as *mut i8;
        } else {
            *endptr = nptr as *mut i8;
        }
    }
    return 0 as i64 as libc::c_longlong;
}