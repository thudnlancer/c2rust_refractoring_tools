#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum strtol_error {
    LONGINT_INVALID,
    LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW,
    LONGINT_INVALID_SUFFIX_CHAR,
    LONGINT_OVERFLOW,
    LONGINT_OK,
}  // end of enum

pub const _ISspace: C2RustUnnamed = 8192;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISspace,
    _ISalnum,
    _ISpunct,
    _IScntrl,
    _ISblank,
    _ISgraph,
    _ISprint,
    _ISxdigit,
    _ISdigit,
    _ISalpha,
    _ISlower,
    _ISupper,
}  // end of enum

#[no_mangle]
pub unsafe extern "C" fn xstrtoul(
    mut s: *const libc::c_char,
    mut ptr: *mut *mut libc::c_char,
    mut strtol_base: libc::c_int,
    mut val: *mut libc::c_ulong,
    mut valid_suffixes: *const libc::c_char,
) -> strtol_error {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp: libc::c_ulong = 0;
    let mut err: strtol_error = LONGINT_OK;
    if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
        __assert_fail(
            b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                as *const libc::c_char,
            b"./xstrtol.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 81],
                &[libc::c_char; 81],
            >(
                b"strtol_error xstrtoul(const char *, char **, int, unsigned long *, const char *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_970: {
        if 0 as libc::c_int <= strtol_base && strtol_base <= 36 as libc::c_int {} else {
            __assert_fail(
                b"0 <= strtol_base && strtol_base <= 36\0" as *const u8
                    as *const libc::c_char,
                b"./xstrtol.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 81],
                    &[libc::c_char; 81],
                >(
                    b"strtol_error xstrtoul(const char *, char **, int, unsigned long *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    p = if !ptr.is_null() { ptr } else { &mut t_ptr };
    *__errno_location() = 0 as libc::c_int;
    if (0 as libc::c_int as libc::c_ulong) < -(1 as libc::c_int) as libc::c_ulong {
        let mut q: *const libc::c_char = s;
        let mut ch: libc::c_uchar = *q as libc::c_uchar;
        while *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            q = q.offset(1);
            ch = *q as libc::c_uchar;
        }
        if ch as libc::c_int == '-' as i32 {
            return LONGINT_INVALID;
        }
    }
    tmp = strtoul(s, p, strtol_base);
    if *p == s as *mut libc::c_char {
        if !valid_suffixes.is_null() && **p as libc::c_int != 0
            && !(strchr(valid_suffixes, **p as libc::c_int)).is_null()
        {
            tmp = 1 as libc::c_int as libc::c_ulong;
        } else {
            return LONGINT_INVALID
        }
    } else if *__errno_location() != 0 as libc::c_int {
        if *__errno_location() != 34 as libc::c_int {
            return LONGINT_INVALID;
        }
        err = LONGINT_OVERFLOW;
    }
    if valid_suffixes.is_null() {
        *val = tmp;
        return err;
    }
    if **p as libc::c_int != '\0' as i32 {
        let mut base: libc::c_int = 1024 as libc::c_int;
        let mut suffixes: libc::c_int = 1 as libc::c_int;
        let mut overflow: strtol_error = LONGINT_OK;
        if (strchr(valid_suffixes, **p as libc::c_int)).is_null() {
            *val = tmp;
            return (err as libc::c_uint
                | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint)
                as strtol_error;
        }
        match **p as libc::c_int {
            69 | 71 | 103 | 107 | 75 | 77 | 109 | 80 | 84 | 116 | 89 | 90 => {
                if !(strchr(valid_suffixes, '0' as i32)).is_null() {
                    match *(*p.offset(0 as libc::c_int as isize))
                        .offset(1 as libc::c_int as isize) as libc::c_int
                    {
                        105 => {
                            if *(*p.offset(0 as libc::c_int as isize))
                                .offset(2 as libc::c_int as isize) as libc::c_int
                                == 'B' as i32
                            {
                                suffixes += 2 as libc::c_int;
                            }
                        }
                        66 | 68 => {
                            base = 1000 as libc::c_int;
                            suffixes += 1;
                            suffixes;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        match **p as libc::c_int {
            98 => {
                overflow = bkm_scale(&mut tmp, 512 as libc::c_int);
            }
            66 => {
                overflow = bkm_scale(&mut tmp, 1024 as libc::c_int);
            }
            99 => {
                overflow = LONGINT_OK;
            }
            69 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 6 as libc::c_int);
            }
            71 | 103 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 3 as libc::c_int);
            }
            107 | 75 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 1 as libc::c_int);
            }
            77 | 109 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 2 as libc::c_int);
            }
            80 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 5 as libc::c_int);
            }
            84 | 116 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 4 as libc::c_int);
            }
            119 => {
                overflow = bkm_scale(&mut tmp, 2 as libc::c_int);
            }
            89 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 8 as libc::c_int);
            }
            90 => {
                overflow = bkm_scale_by_power(&mut tmp, base, 7 as libc::c_int);
            }
            _ => {
                *val = tmp;
                return (err as libc::c_uint
                    | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint)
                    as strtol_error;
            }
        }
        err = ::core::mem::transmute::<
            libc::c_uint,
            strtol_error,
        >(err as libc::c_uint | overflow as libc::c_uint);
        *p = (*p).offset(suffixes as isize);
        if **p != 0 {
            err = ::core::mem::transmute::<
                libc::c_uint,
                strtol_error,
            >(
                err as libc::c_uint
                    | LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint,
            );
        }
    }
    *val = tmp;
    return err;
}
unsafe extern "C" fn bkm_scale_by_power(
    mut x: *mut libc::c_ulong,
    mut base: libc::c_int,
    mut power: libc::c_int,
) -> strtol_error {
    let mut err: strtol_error = LONGINT_OK;
    loop {
        let fresh0 = power;
        power = power - 1;
        if !(fresh0 != 0) {
            break;
        }
        err = ::core::mem::transmute::<
            libc::c_uint,
            strtol_error,
        >(err as libc::c_uint | bkm_scale(x, base) as libc::c_uint);
    }
    return err;
}
unsafe extern "C" fn bkm_scale(
    mut x: *mut libc::c_ulong,
    mut scale_factor: libc::c_int,
) -> strtol_error {
    if !((0 as libc::c_int as libc::c_ulong) < -(1 as libc::c_int) as libc::c_ulong)
        && *x < (0 as libc::c_int / scale_factor) as libc::c_ulong
    {
        *x = 0 as libc::c_int as libc::c_ulong;
        return LONGINT_OVERFLOW;
    }
    if (9223372036854775807 as libc::c_long as libc::c_ulong)
        .wrapping_mul(2 as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong)
        .wrapping_div(scale_factor as libc::c_ulong) < *x
    {
        *x = (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong);
        return LONGINT_OVERFLOW;
    }
    *x = (*x).wrapping_mul(scale_factor as libc::c_ulong);
    return LONGINT_OK;
}
