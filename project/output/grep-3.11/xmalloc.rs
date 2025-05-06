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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
    fn xalloc_die();
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type idx_t = ptrdiff_t;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = u32;
pub const DEFAULT_MXFAST_0: C2RustUnnamed_0 = 128;
pub type C2RustUnnamed_0 = u32;
#[cold]
#[inline]
unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as i32;
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn imalloc(mut s: idx_t) -> *mut libc::c_void {
    return if s as u64 <= 18446744073709551615 as u64 {
        malloc(s as u64)
    } else {
        _gl_alloc_nomem()
    };
}
#[inline]
unsafe extern "C" fn irealloc(
    mut p: *mut libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return if s as u64 <= 18446744073709551615 as u64 {
        realloc(p, (s | (s == 0) as i32 as i64) as u64)
    } else {
        _gl_alloc_nomem()
    };
}
#[inline]
unsafe extern "C" fn icalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    if (18446744073709551615 as u64) < n as u64 {
        if s != 0 as i32 as i64 {
            return _gl_alloc_nomem();
        }
        n = 0 as i32 as idx_t;
    }
    if (18446744073709551615 as u64) < s as u64 {
        if n != 0 as i32 as i64 {
            return _gl_alloc_nomem();
        }
        s = 0 as i32 as idx_t;
    }
    return calloc(n as u64, s as u64);
}
#[inline]
unsafe extern "C" fn ireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    if n == 0 as i32 as i64 || s == 0 as i32 as i64 {
        s = 1 as i32 as idx_t;
        n = s;
    }
    return if n as u64 <= 18446744073709551615 as u64
        && s as u64 <= 18446744073709551615 as u64
    {
        reallocarray(p, n as size_t, s as size_t)
    } else {
        _gl_alloc_nomem()
    };
}
unsafe extern "C" fn nonnull(mut p: *mut libc::c_void) -> *mut libc::c_void {
    if p.is_null() {
        xalloc_die();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut s: size_t) -> *mut libc::c_void {
    return nonnull(malloc(s));
}
#[no_mangle]
pub unsafe extern "C" fn ximalloc(mut s: idx_t) -> *mut libc::c_void {
    return nonnull(imalloc(s));
}
#[no_mangle]
pub unsafe extern "C" fn xcharalloc(mut n: size_t) -> *mut i8 {
    return (if ::core::mem::size_of::<i8>() as u64 == 1 as i32 as u64 {
        xmalloc(n)
    } else {
        xnmalloc(n, ::core::mem::size_of::<i8>() as u64)
    }) as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut p: *mut libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = realloc(p, s);
    if r.is_null() && (p.is_null() || s != 0) {
        xalloc_die();
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn xirealloc(
    mut p: *mut libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return nonnull(irealloc(p, s));
}
#[no_mangle]
pub unsafe extern "C" fn xreallocarray(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = reallocarray(p, n, s);
    if r.is_null() && (p.is_null() || n != 0 && s != 0) {
        xalloc_die();
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn xireallocarray(
    mut p: *mut libc::c_void,
    mut n: idx_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    return nonnull(ireallocarray(p, n, s));
}
#[no_mangle]
pub unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    return xreallocarray(0 as *mut libc::c_void, n, s);
}
#[no_mangle]
pub unsafe extern "C" fn xinmalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    return xireallocarray(0 as *mut libc::c_void, n, s);
}
#[no_mangle]
pub unsafe extern "C" fn x2realloc(
    mut p: *mut libc::c_void,
    mut ps: *mut size_t,
) -> *mut libc::c_void {
    return x2nrealloc(p, ps, 1 as i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as i32 as u64).wrapping_div(s);
            n = (n as u64).wrapping_add((n == 0) as i32 as u64) as size_t as size_t;
        }
    } else {
        let (fresh0, fresh1) = n
            .overflowing_add((n >> 1 as i32).wrapping_add(1 as i32 as u64));
        *(&mut n as *mut size_t) = fresh0;
        if fresh1 {
            xalloc_die();
        }
    }
    p = xreallocarray(p, n, s);
    *pn = n;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xpalloc(
    mut pa: *mut libc::c_void,
    mut pn: *mut idx_t,
    mut n_incr_min: idx_t,
    mut n_max: ptrdiff_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    let mut n0: idx_t = *pn;
    let mut n: idx_t = 0;
    let (fresh2, fresh3) = n0.overflowing_add(n0 >> 1 as i32);
    *(&mut n as *mut idx_t) = fresh2;
    if fresh3 {
        n = 9223372036854775807 as i64;
    }
    if 0 as i32 as i64 <= n_max && n_max < n {
        n = n_max;
    }
    let mut nbytes: idx_t = 0;
    let mut adjusted_nbytes: idx_t = (if if ::core::mem::size_of::<idx_t>() as u64
        == ::core::mem::size_of::<libc::c_schar>() as u64
    {
        if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
            if if s < 0 as i32 as i64 {
                if n < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 127 as i32 }) as i64 + s
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (n < 127 as i32 as i64 / s) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 1 as i32 as i64)
                                << (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (s
                                < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + 1 as i32 as i64)
                                        << (::core::mem::size_of::<idx_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64) < s) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 127 as i32 as i64
                                >> (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            127 as i32 as i64 / -s
                        }) <= -(1 as i32) as i64 - n) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(127 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(127 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(127 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + (-(127 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(127 as i32) - 1 as i32) as i64) as i32
                }) != 0 && s == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64) < n + (-(127 as i32) - 1 as i32) as i64)
                            as i32
                    } else {
                        ((0 as i32 as i64) < n
                            && ((-(1 as i32) - (-(127 as i32) - 1 as i32)) as i64)
                                < n - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(127 as i32) - 1 as i32) as i64 / s < n) as i32
                }
            } else if s == 0 as i32 as i64 {
                0 as i32
            } else if n < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(127 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(127 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(127 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + (-(127 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(127 as i32) - 1 as i32) as i64) as i32
                }) != 0 && n == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64) < s + (-(127 as i32) - 1 as i32) as i64)
                            as i32
                    } else {
                        (((-(1 as i32) - (-(127 as i32) - 1 as i32)) as i64)
                            < s - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(127 as i32) - 1 as i32) as i64 / n < s) as i32
                }
            } else {
                (127 as i32 as i64 / s < n) as i32
            } != 0
            {
                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_schar as idx_t;
                1 as i32
            } else {
                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_schar as idx_t;
                0 as i32
            }
        } else if if s < 0 as i32 as i64 {
            if n < 0 as i32 as i64 {
                if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        127 as i32 * 2 as i32 + 1 as i32
                    }) as i64 + s
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    (n < (127 as i32 * 2 as i32 + 1 as i32) as i64 / s) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 1 as i32 as i64)
                            << (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) < 0 as i32 as i64
                    {
                        (s
                            < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + 1 as i32 as i64)
                                    << (::core::mem::size_of::<idx_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64
                            })) as i32
                    } else {
                        ((0 as i32 as i64) < s) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (127 as i32 * 2 as i32 + 1 as i32) as i64
                            >> (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (127 as i32 * 2 as i32 + 1 as i32) as i64 / -s
                    }) <= -(1 as i32) as i64 - n) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + 0 as i32 as i64) as i32
            }) != 0 && s == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < n
                        && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                        as i32
                }
            } else {
                (0 as i32 as i64 / s < n) as i32
            }
        } else if s == 0 as i32 as i64 {
            0 as i32
        } else if n < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + 0 as i32 as i64) as i32
            }) != 0 && n == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64) as i32
                }
            } else {
                (0 as i32 as i64 / n < s) as i32
            }
        } else {
            ((127 as i32 * 2 as i32 + 1 as i32) as i64 / s < n) as i32
        } != 0
        {
            nbytes = (n as u32).wrapping_mul(s as u32) as u8 as idx_t;
            1 as i32
        } else {
            nbytes = (n as u32).wrapping_mul(s as u32) as u8 as idx_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<idx_t>() as u64
        == ::core::mem::size_of::<libc::c_short>() as u64
    {
        if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
            if if s < 0 as i32 as i64 {
                if n < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 32767 as i32 }) as i64 + s
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (n < 32767 as i32 as i64 / s) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 1 as i32 as i64)
                                << (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (s
                                < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + 1 as i32 as i64)
                                        << (::core::mem::size_of::<idx_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64) < s) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 32767 as i32 as i64
                                >> (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            32767 as i32 as i64 / -s
                        }) <= -(1 as i32) as i64 - n) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(32767 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(32767 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(32767 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + (-(32767 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + (-(32767 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(32767 as i32) - 1 as i32) as i64) as i32
                }) != 0 && s == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64) < n + (-(32767 as i32) - 1 as i32) as i64)
                            as i32
                    } else {
                        ((0 as i32 as i64) < n
                            && ((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as i64)
                                < n - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(32767 as i32) - 1 as i32) as i64 / s < n) as i32
                }
            } else if s == 0 as i32 as i64 {
                0 as i32
            } else if n < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(32767 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(32767 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(32767 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + (-(32767 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                    + (-(32767 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(32767 as i32) - 1 as i32) as i64) as i32
                }) != 0 && n == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64) < s + (-(32767 as i32) - 1 as i32) as i64)
                            as i32
                    } else {
                        (((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as i64)
                            < s - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(32767 as i32) - 1 as i32) as i64 / n < s) as i32
                }
            } else {
                (32767 as i32 as i64 / s < n) as i32
            } != 0
            {
                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_short as idx_t;
                1 as i32
            } else {
                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_short as idx_t;
                0 as i32
            }
        } else if if s < 0 as i32 as i64 {
            if n < 0 as i32 as i64 {
                if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        32767 as i32 * 2 as i32 + 1 as i32
                    }) as i64 + s
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    (n < (32767 as i32 * 2 as i32 + 1 as i32) as i64 / s) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 1 as i32 as i64)
                            << (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) < 0 as i32 as i64
                    {
                        (s
                            < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + 1 as i32 as i64)
                                    << (::core::mem::size_of::<idx_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64
                            })) as i32
                    } else {
                        ((0 as i32 as i64) < s) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (32767 as i32 * 2 as i32 + 1 as i32) as i64
                            >> (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (32767 as i32 * 2 as i32 + 1 as i32) as i64 / -s
                    }) <= -(1 as i32) as i64 - n) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + 0 as i32 as i64) as i32
            }) != 0 && s == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < n
                        && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                        as i32
                }
            } else {
                (0 as i32 as i64 / s < n) as i32
            }
        } else if s == 0 as i32 as i64 {
            0 as i32
        } else if n < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + 0 as i32 as i64) as i32
            }) != 0 && n == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64) as i32
                }
            } else {
                (0 as i32 as i64 / n < s) as i32
            }
        } else {
            ((32767 as i32 * 2 as i32 + 1 as i32) as i64 / s < n) as i32
        } != 0
        {
            nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_ushort as idx_t;
            1 as i32
        } else {
            nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_ushort as idx_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<idx_t>() as u64
        == ::core::mem::size_of::<i32>() as u64
    {
        if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { nbytes }) - 1 as i32 as i64)
            < 0 as i32 as i64
        {
            if if s < 0 as i32 as i64 {
                if n < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 2147483647 as i32 }) as i64
                            + s
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (n < 2147483647 as i32 as i64 / s) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 1 as i32 as i64)
                                << (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (s
                                < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + 1 as i32 as i64)
                                        << (::core::mem::size_of::<idx_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64) < s) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 2147483647 as i32 as i64
                                >> (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            2147483647 as i32 as i64 / -s
                        }) <= -(1 as i32) as i64 - n) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(2147483647 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(2147483647 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(2147483647 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + (-(2147483647 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + (-(2147483647 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                }) != 0 && s == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64)
                            < n + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                    } else {
                        ((0 as i32 as i64) < n
                            && ((-(1 as i32) - (-(2147483647 as i32) - 1 as i32)) as i64)
                                < n - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(2147483647 as i32) - 1 as i32) as i64 / s < n) as i32
                }
            } else if s == 0 as i32 as i64 {
                0 as i32
            } else if n < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(2147483647 as i32) - 1 as i32) as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(2147483647 as i32) - 1 as i32) as i64
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(2147483647 as i32) - 1 as i32) as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + (-(2147483647 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                    + (-(2147483647 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                }) != 0 && n == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64)
                            < s + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                    } else {
                        (((-(1 as i32) - (-(2147483647 as i32) - 1 as i32)) as i64)
                            < s - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(2147483647 as i32) - 1 as i32) as i64 / n < s) as i32
                }
            } else {
                (2147483647 as i32 as i64 / s < n) as i32
            } != 0
            {
                nbytes = (n as u32).wrapping_mul(s as u32) as i32 as idx_t;
                1 as i32
            } else {
                nbytes = (n as u32).wrapping_mul(s as u32) as i32 as idx_t;
                0 as i32
            }
        } else if if s < 0 as i32 as i64 {
            if n < 0 as i32 as i64 {
                if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u32
                    } else {
                        (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32)
                    }) as i64 + s
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    (n
                        < (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as i64 / s) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 1 as i32 as i64)
                            << (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) < 0 as i32 as i64
                    {
                        (s
                            < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + 1 as i32 as i64)
                                    << (::core::mem::size_of::<idx_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64
                            })) as i32
                    } else {
                        ((0 as i32 as i64) < s) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (2147483647 as i32 as u32)
                                .wrapping_mul(2 as u32)
                                .wrapping_add(1 as u32) as i64
                            >> (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as i64 / -s
                    }) <= -(1 as i32) as i64 - n) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + 0 as i32 as i64) as i32
            }) != 0 && s == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < n
                        && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                        as i32
                }
            } else {
                (0 as i32 as i64 / s < n) as i32
            }
        } else if s == 0 as i32 as i64 {
            0 as i32
        } else if n < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + 0 as i32 as i64) as i32
            }) != 0 && n == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64) as i32
                }
            } else {
                (0 as i32 as i64 / n < s) as i32
            }
        } else {
            ((2147483647 as i32 as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32)
                as i64 / s < n) as i32
        } != 0
        {
            nbytes = (n as u32).wrapping_mul(s as u32) as idx_t;
            1 as i32
        } else {
            nbytes = (n as u32).wrapping_mul(s as u32) as idx_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<idx_t>() as u64
        == ::core::mem::size_of::<i64>() as u64
    {
        if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { nbytes }) - 1 as i32 as i64)
            < 0 as i32 as i64
        {
            if if s < 0 as i32 as i64 {
                if n < 0 as i32 as i64 {
                    if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            9223372036854775807 as i64
                        }) + s
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (n < 9223372036854775807 as i64 / s) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 1 as i32 as i64)
                                << (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (s
                                < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + 1 as i32 as i64)
                                        << (::core::mem::size_of::<idx_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64) < s) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 9223372036854775807 as i64
                                >> (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            9223372036854775807 as i64 / -s
                        }) <= -(1 as i32) as i64 - n) as i32
                    }
                } else if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(9223372036854775807 as i64) - 1 as i64)
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(9223372036854775807 as i64) - 1 as i64)
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + (-(9223372036854775807 as i64) - 1 as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + (-(9223372036854775807 as i64) - 1 as i64)
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + (-(9223372036854775807 as i64) - 1 as i64)
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                }) != 0 && s == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64)
                            < n + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                    } else {
                        ((0 as i32 as i64) < n
                            && -(1 as i32) as i64
                                - (-(9223372036854775807 as i64) - 1 as i64)
                                < n - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(9223372036854775807 as i64) - 1 as i64) / s < n) as i32
                }
            } else if s == 0 as i32 as i64 {
                0 as i32
            } else if n < 0 as i32 as i64 {
                if (if (if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(9223372036854775807 as i64) - 1 as i64)
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(9223372036854775807 as i64) - 1 as i64)
                    }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + (-(9223372036854775807 as i64) - 1 as i64)
                        < -(if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + (-(9223372036854775807 as i64) - 1 as i64)
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                    + (-(9223372036854775807 as i64) - 1 as i64)
                            }) - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64)
                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                }) != 0 && n == -(1 as i32) as i64
                {
                    if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((0 as i32 as i64)
                            < s + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                    } else {
                        (-(1 as i32) as i64 - (-(9223372036854775807 as i64) - 1 as i64)
                            < s - 1 as i32 as i64) as i32
                    }
                } else {
                    ((-(9223372036854775807 as i64) - 1 as i64) / n < s) as i32
                }
            } else {
                (9223372036854775807 as i64 / s < n) as i32
            } != 0
            {
                nbytes = (n as u64).wrapping_mul(s as u64) as i64;
                1 as i32
            } else {
                nbytes = (n as u64).wrapping_mul(s as u64) as i64;
                0 as i32
            }
        } else if if s < 0 as i32 as i64 {
            if n < 0 as i32 as i64 {
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
                        .wrapping_add(s as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((n as u64)
                        < (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                            .wrapping_div(s as u64)) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 1 as i32 as i64)
                            << (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) < 0 as i32 as i64
                    {
                        (s
                            < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + 1 as i32 as i64)
                                    << (::core::mem::size_of::<idx_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64
                            })) as i32
                    } else {
                        ((0 as i32 as i64) < s) as i32
                    }) != 0
                    {
                        ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) as u64)
                            .wrapping_add(
                                (9223372036854775807 as i64 as u64)
                                    .wrapping_mul(2 as u64)
                                    .wrapping_add(1 as u64),
                            )
                            >> (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                            .wrapping_div(-s as u64)
                    }) <= (-(1 as i32) as i64 - n) as u64) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + 0 as i32 as i64) as i32
            }) != 0 && s == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                } else {
                    ((0 as i32 as i64) < n
                        && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                        as i32
                }
            } else {
                (0 as i32 as i64 / s < n) as i32
            }
        } else if s == 0 as i32 as i64 {
            0 as i32
        } else if n < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
            }) - 1 as i32 as i64) < 0 as i32 as i64
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 1 as i32 as i64)
                    << (::core::mem::size_of::<i64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) + 0 as i32 as i64
            }) < 0 as i32 as i64
            {
                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64
                    })) as i32
            } else {
                ((0 as i32 as i64)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        + 0 as i32 as i64) as i32
            }) != 0 && n == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64) as i32
                }
            } else {
                (0 as i32 as i64 / n < s) as i32
            }
        } else {
            ((9223372036854775807 as i64 as u64)
                .wrapping_mul(2 as u64)
                .wrapping_add(1 as u64)
                .wrapping_div(s as u64) < n as u64) as i32
        } != 0
        {
            nbytes = (n as u64).wrapping_mul(s as u64) as idx_t;
            1 as i32
        } else {
            nbytes = (n as u64).wrapping_mul(s as u64) as idx_t;
            0 as i32
        }
    } else if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { nbytes }) - 1 as i32 as i64)
        < 0 as i32 as i64
    {
        if if s < 0 as i32 as i64 {
            if n < 0 as i32 as i64 {
                if ((if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as libc::c_longlong
                    } else {
                        9223372036854775807 as libc::c_longlong
                    }) + s as libc::c_longlong
                }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
                {
                    ((n as libc::c_longlong)
                        < 9223372036854775807 as libc::c_longlong
                            / s as libc::c_longlong) as i32
                } else {
                    ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 1 as i32 as i64)
                            << (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) < 0 as i32 as i64
                    {
                        (s
                            < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    + 1 as i32 as i64)
                                    << (::core::mem::size_of::<idx_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                    - 1 as i32 as i64
                            })) as i32
                    } else {
                        ((0 as i32 as i64) < s) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            as libc::c_longlong + 9223372036854775807 as libc::c_longlong
                            >> (::core::mem::size_of::<idx_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        9223372036854775807 as libc::c_longlong / -s as libc::c_longlong
                    }) <= (-(1 as i32) as i64 - n) as libc::c_longlong) as i32
                }
            } else if (if (if ((if 1 as i32 != 0 {
                0 as i32 as libc::c_longlong
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
            }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) as libc::c_longlong
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
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as i32 as libc::c_longlong
            }) < 0 as i32 as libc::c_longlong
            {
                ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as libc::c_longlong
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as libc::c_longlong
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as i32 as libc::c_longlong
                    })) as i32
            } else {
                ((0 as i32 as libc::c_longlong)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as i32
            }) != 0 && s == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as libc::c_longlong)
                        < n as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as i32
                } else {
                    ((0 as i32 as i64) < n
                        && -(1 as i32) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                            < (n - 1 as i32 as i64) as libc::c_longlong) as i32
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / s as libc::c_longlong) < n as libc::c_longlong) as i32
            }
        } else if s == 0 as i32 as i64 {
            0 as i32
        } else if n < 0 as i32 as i64 {
            if (if (if ((if 1 as i32 != 0 {
                0 as i32 as libc::c_longlong
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
            }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
            {
                !(((((if 1 as i32 != 0 {
                    0 as i32 as libc::c_longlong
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) as libc::c_longlong
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
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                }) + 0 as i32 as libc::c_longlong
            }) < 0 as i32 as libc::c_longlong
            {
                ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) as libc::c_longlong
                    + (-(9223372036854775807 as libc::c_longlong)
                        - 1 as libc::c_longlong)
                    < -(if ((if 1 as i32 != 0 {
                        0 as i32 as libc::c_longlong
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)
                    }) - 1 as i32 as libc::c_longlong) < 0 as i32 as libc::c_longlong
                    {
                        ((((if 1 as i32 != 0 {
                            0 as i32 as libc::c_longlong
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                as libc::c_longlong
                                + (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong)
                        }) - 1 as i32 as libc::c_longlong
                    })) as i32
            } else {
                ((0 as i32 as libc::c_longlong)
                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                        as libc::c_longlong
                        + (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as i32
            }) != 0 && n == -(1 as i32) as i64
            {
                if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) - 1 as i32 as i64)
                    < 0 as i32 as i64
                {
                    ((0 as i32 as libc::c_longlong)
                        < s as libc::c_longlong
                            + (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as i32
                } else {
                    (-(1 as i32) as libc::c_longlong
                        - (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)
                        < (s - 1 as i32 as i64) as libc::c_longlong) as i32
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    / n as libc::c_longlong) < s as libc::c_longlong) as i32
            }
        } else {
            ((9223372036854775807 as libc::c_longlong / s as libc::c_longlong)
                < n as libc::c_longlong) as i32
        } != 0
        {
            nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong)
                as libc::c_longlong as idx_t;
            1 as i32
        } else {
            nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong)
                as libc::c_longlong as idx_t;
            0 as i32
        }
    } else if if s < 0 as i32 as i64 {
        if n < 0 as i32 as i64 {
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
                    .wrapping_add(s as libc::c_ulonglong)
            })
                .wrapping_sub(1 as i32 as libc::c_ulonglong)
                < 0 as i32 as libc::c_ulonglong
            {
                ((n as libc::c_ulonglong)
                    < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(s as libc::c_ulonglong)) as i32
            } else {
                ((if (if (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                    - 1 as i32 as i64) < 0 as i32 as i64
                {
                    !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        + 1 as i32 as i64)
                        << (::core::mem::size_of::<idx_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64)
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) < 0 as i32 as i64
                {
                    (s
                        < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                + 1 as i32 as i64)
                                << (::core::mem::size_of::<idx_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                - 1 as i32 as i64
                        })) as i32
                } else {
                    ((0 as i32 as i64) < s) as i32
                }) != 0
                {
                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong),
                        )
                        >> (::core::mem::size_of::<idx_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(1 as i32 as u64)
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(-s as libc::c_ulonglong)
                }) <= (-(1 as i32) as i64 - n) as libc::c_ulonglong) as i32
            }
        } else if (if (if ((if 1 as i32 != 0 {
            0 as i32 as i64
        } else {
            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
        }) - 1 as i32 as i64) < 0 as i32 as i64
        {
            !(((((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
            }) + 1 as i32 as i64)
                << (::core::mem::size_of::<i64>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64)
        } else {
            (if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
            }) + 0 as i32 as i64
        }) < 0 as i32 as i64
        {
            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64)
                < -(if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    ((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64
                })) as i32
        } else {
            ((0 as i32 as i64)
                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) + 0 as i32 as i64)
                as i32
        }) != 0 && s == -(1 as i32) as i64
        {
            if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) - 1 as i32 as i64)
                < 0 as i32 as i64
            {
                ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
            } else {
                ((0 as i32 as i64) < n
                    && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64) as i32
            }
        } else {
            (0 as i32 as i64 / s < n) as i32
        }
    } else if s == 0 as i32 as i64 {
        0 as i32
    } else if n < 0 as i32 as i64 {
        if (if (if ((if 1 as i32 != 0 {
            0 as i32 as i64
        } else {
            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
        }) - 1 as i32 as i64) < 0 as i32 as i64
        {
            !(((((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
            }) + 1 as i32 as i64)
                << (::core::mem::size_of::<i64>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64)
        } else {
            (if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
            }) + 0 as i32 as i64
        }) < 0 as i32 as i64
        {
            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64)
                < -(if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    ((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                            + 0 as i32 as i64
                    }) - 1 as i32 as i64
                })) as i32
        } else {
            ((0 as i32 as i64)
                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n }) + 0 as i32 as i64)
                as i32
        }) != 0 && n == -(1 as i32) as i64
        {
            if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) - 1 as i32 as i64)
                < 0 as i32 as i64
            {
                ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
            } else {
                (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64) as i32
            }
        } else {
            (0 as i32 as i64 / n < s) as i32
        }
    } else {
        ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(s as libc::c_ulonglong) < n as libc::c_ulonglong) as i32
    } != 0
    {
        nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong) as idx_t;
        1 as i32
    } else {
        nbytes = (n as libc::c_ulonglong).wrapping_mul(s as libc::c_ulonglong) as idx_t;
        0 as i32
    } != 0
    {
        if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
            9223372036854775807 as i64 as u64
        } else {
            18446744073709551615 as u64
        }
    } else {
        (if nbytes < DEFAULT_MXFAST_0 as i32 as i64 {
            DEFAULT_MXFAST_0 as i32
        } else {
            0 as i32
        }) as u64
    }) as idx_t;
    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }
    if pa.is_null() {
        *pn = 0 as i32 as idx_t;
    }
    if n - n0 < n_incr_min
        && {
            let (fresh4, fresh5) = n0.overflowing_add(n_incr_min);
            *(&mut n as *mut idx_t) = fresh4;
            fresh5 as i32 != 0 || 0 as i32 as i64 <= n_max && n_max < n
                || (if ::core::mem::size_of::<idx_t>() as u64
                    == ::core::mem::size_of::<libc::c_schar>() as u64
                {
                    (if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
                        (if (if s < 0 as i32 as i64 {
                            (if n < 0 as i32 as i64 {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 127 as i32 }) as i64
                                        + s
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    (n < 127 as i32 as i64 / s) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        s
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 1 as i32 as i64)
                                            << (::core::mem::size_of::<idx_t>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (s
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<idx_t>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64) < s) as i32
                                    }) != 0
                                    {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 127 as i32 as i64
                                            >> (::core::mem::size_of::<idx_t>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        127 as i32 as i64 / -s
                                    }) <= -(1 as i32) as i64 - n) as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + (-(127 as i32) - 1 as i32) as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + (-(127 as i32) - 1 as i32) as i64
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + (-(127 as i32) - 1 as i32) as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + (-(127 as i32) - 1 as i32) as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + (-(127 as i32) - 1 as i32) as i64
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + (-(127 as i32) - 1 as i32) as i64) as i32
                                }) != 0 && s == -(1 as i32) as i64
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                        - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64) < n + (-(127 as i32) - 1 as i32) as i64)
                                            as i32
                                    } else {
                                        ((0 as i32 as i64) < n
                                            && ((-(1 as i32) - (-(127 as i32) - 1 as i32)) as i64)
                                                < n - 1 as i32 as i64) as i32
                                    })
                                } else {
                                    ((-(127 as i32) - 1 as i32) as i64 / s < n) as i32
                                })
                            })
                        } else {
                            (if s == 0 as i32 as i64 {
                                0 as i32
                            } else {
                                (if n < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                            + (-(127 as i32) - 1 as i32) as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + (-(127 as i32) - 1 as i32) as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                            + (-(127 as i32) - 1 as i32) as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + (-(127 as i32) - 1 as i32) as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + (-(127 as i32) - 1 as i32) as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + (-(127 as i32) - 1 as i32) as i64) as i32
                                    }) != 0 && n == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((0 as i32 as i64) < s + (-(127 as i32) - 1 as i32) as i64)
                                                as i32
                                        } else {
                                            (((-(1 as i32) - (-(127 as i32) - 1 as i32)) as i64)
                                                < s - 1 as i32 as i64) as i32
                                        })
                                    } else {
                                        ((-(127 as i32) - 1 as i32) as i64 / n < s) as i32
                                    })
                                } else {
                                    (127 as i32 as i64 / s < n) as i32
                                })
                            })
                        }) != 0
                        {
                            nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_schar
                                as idx_t;
                            1 as i32
                        } else {
                            nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_schar
                                as idx_t;
                            0 as i32
                        })
                    } else {
                        (if (if s < 0 as i32 as i64 {
                            (if n < 0 as i32 as i64 {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        127 as i32 * 2 as i32 + 1 as i32
                                    }) as i64 + s
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    (n < (127 as i32 * 2 as i32 + 1 as i32) as i64 / s) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        s
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 1 as i32 as i64)
                                            << (::core::mem::size_of::<idx_t>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (s
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<idx_t>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64) < s) as i32
                                    }) != 0
                                    {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + (127 as i32 * 2 as i32 + 1 as i32) as i64
                                            >> (::core::mem::size_of::<idx_t>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        (127 as i32 * 2 as i32 + 1 as i32) as i64 / -s
                                    }) <= -(1 as i32) as i64 - n) as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + 0 as i32 as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 0 as i32 as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 0 as i32 as i64
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                        + 0 as i32 as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 0 as i32 as i64) as i32
                                }) != 0 && s == -(1 as i32) as i64
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                        - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                                    } else {
                                        ((0 as i32 as i64) < n
                                            && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                                            as i32
                                    })
                                } else {
                                    (0 as i32 as i64 / s < n) as i32
                                })
                            })
                        } else {
                            (if s == 0 as i32 as i64 {
                                0 as i32
                            } else {
                                (if n < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + 0 as i32 as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                            + 0 as i32 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + 0 as i32 as i64) as i32
                                    }) != 0 && n == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                                        } else {
                                            (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64)
                                                as i32
                                        })
                                    } else {
                                        (0 as i32 as i64 / n < s) as i32
                                    })
                                } else {
                                    ((127 as i32 * 2 as i32 + 1 as i32) as i64 / s < n) as i32
                                })
                            })
                        }) != 0
                        {
                            nbytes = (n as u32).wrapping_mul(s as u32) as u8 as idx_t;
                            1 as i32
                        } else {
                            nbytes = (n as u32).wrapping_mul(s as u32) as u8 as idx_t;
                            0 as i32
                        })
                    })
                } else {
                    (if ::core::mem::size_of::<idx_t>() as u64
                        == ::core::mem::size_of::<libc::c_short>() as u64
                    {
                        (if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
                            (if (if s < 0 as i32 as i64 {
                                (if n < 0 as i32 as i64 {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 32767 as i32 }) as i64
                                            + s
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        (n < 32767 as i32 as i64 / s) as i32
                                    } else {
                                        ((if (if (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            s
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 1 as i32 as i64)
                                                << (::core::mem::size_of::<idx_t>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64)
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64
                                        }) < 0 as i32 as i64
                                        {
                                            (s
                                                < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64
                                                })) as i32
                                        } else {
                                            ((0 as i32 as i64) < s) as i32
                                        }) != 0
                                        {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 32767 as i32 as i64
                                                >> (::core::mem::size_of::<idx_t>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(1 as i32 as u64)
                                        } else {
                                            32767 as i32 as i64 / -s
                                        }) <= -(1 as i32) as i64 - n) as i32
                                    })
                                } else {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + (-(32767 as i32) - 1 as i32) as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + (-(32767 as i32) - 1 as i32) as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + (-(32767 as i32) - 1 as i32) as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + (-(32767 as i32) - 1 as i32) as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + (-(32767 as i32) - 1 as i32) as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + (-(32767 as i32) - 1 as i32) as i64) as i32
                                    }) != 0 && s == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                            - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((0 as i32 as i64)
                                                < n + (-(32767 as i32) - 1 as i32) as i64) as i32
                                        } else {
                                            ((0 as i32 as i64) < n
                                                && ((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as i64)
                                                    < n - 1 as i32 as i64) as i32
                                        })
                                    } else {
                                        ((-(32767 as i32) - 1 as i32) as i64 / s < n) as i32
                                    })
                                })
                            } else {
                                (if s == 0 as i32 as i64 {
                                    0 as i32
                                } else {
                                    (if n < 0 as i32 as i64 {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + (-(32767 as i32) - 1 as i32) as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + (-(32767 as i32) - 1 as i32) as i64
                                            }) + 0 as i32 as i64
                                        }) < 0 as i32 as i64
                                        {
                                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + (-(32767 as i32) - 1 as i32) as i64)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + (-(32767 as i32) - 1 as i32) as i64
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + (-(32767 as i32) - 1 as i32) as i64
                                                    }) - 1 as i32 as i64
                                                })) as i32
                                        } else {
                                            ((0 as i32 as i64)
                                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + (-(32767 as i32) - 1 as i32) as i64) as i32
                                        }) != 0 && n == -(1 as i32) as i64
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((0 as i32 as i64)
                                                    < s + (-(32767 as i32) - 1 as i32) as i64) as i32
                                            } else {
                                                (((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as i64)
                                                    < s - 1 as i32 as i64) as i32
                                            })
                                        } else {
                                            ((-(32767 as i32) - 1 as i32) as i64 / n < s) as i32
                                        })
                                    } else {
                                        (32767 as i32 as i64 / s < n) as i32
                                    })
                                })
                            }) != 0
                            {
                                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_short
                                    as idx_t;
                                1 as i32
                            } else {
                                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_short
                                    as idx_t;
                                0 as i32
                            })
                        } else {
                            (if (if s < 0 as i32 as i64 {
                                (if n < 0 as i32 as i64 {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            32767 as i32 * 2 as i32 + 1 as i32
                                        }) as i64 + s
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        (n < (32767 as i32 * 2 as i32 + 1 as i32) as i64 / s) as i32
                                    } else {
                                        ((if (if (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            s
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 1 as i32 as i64)
                                                << (::core::mem::size_of::<idx_t>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64)
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64
                                        }) < 0 as i32 as i64
                                        {
                                            (s
                                                < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64
                                                })) as i32
                                        } else {
                                            ((0 as i32 as i64) < s) as i32
                                        }) != 0
                                        {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + (32767 as i32 * 2 as i32 + 1 as i32) as i64
                                                >> (::core::mem::size_of::<idx_t>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(1 as i32 as u64)
                                        } else {
                                            (32767 as i32 * 2 as i32 + 1 as i32) as i64 / -s
                                        }) <= -(1 as i32) as i64 - n) as i32
                                    })
                                } else {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                            + 0 as i32 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64) as i32
                                    }) != 0 && s == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                            - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                                        } else {
                                            ((0 as i32 as i64) < n
                                                && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                                                as i32
                                        })
                                    } else {
                                        (0 as i32 as i64 / s < n) as i32
                                    })
                                })
                            } else {
                                (if s == 0 as i32 as i64 {
                                    0 as i32
                                } else {
                                    (if n < 0 as i32 as i64 {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + 0 as i32 as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + 0 as i32 as i64
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64)
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + 0 as i32 as i64
                                            }) + 0 as i32 as i64
                                        }) < 0 as i32 as i64
                                        {
                                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                + 0 as i32 as i64)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64
                                                    }) - 1 as i32 as i64
                                                })) as i32
                                        } else {
                                            ((0 as i32 as i64)
                                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + 0 as i32 as i64) as i32
                                        }) != 0 && n == -(1 as i32) as i64
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                                            } else {
                                                (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64)
                                                    as i32
                                            })
                                        } else {
                                            (0 as i32 as i64 / n < s) as i32
                                        })
                                    } else {
                                        ((32767 as i32 * 2 as i32 + 1 as i32) as i64 / s < n) as i32
                                    })
                                })
                            }) != 0
                            {
                                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_ushort
                                    as idx_t;
                                1 as i32
                            } else {
                                nbytes = (n as u32).wrapping_mul(s as u32) as libc::c_ushort
                                    as idx_t;
                                0 as i32
                            })
                        })
                    } else {
                        (if ::core::mem::size_of::<idx_t>() as u64
                            == ::core::mem::size_of::<i32>() as u64
                        {
                            (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { nbytes })
                                - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                (if (if s < 0 as i32 as i64 {
                                    (if n < 0 as i32 as i64 {
                                        (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 2147483647 as i32 })
                                                as i64 + s
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            (n < 2147483647 as i32 as i64 / s) as i32
                                        } else {
                                            ((if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                s
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<idx_t>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64)
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) < 0 as i32 as i64
                                            {
                                                (s
                                                    < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            + 1 as i32 as i64)
                                                            << (::core::mem::size_of::<idx_t>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                            * 2 as i32 as i64 + 1 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            - 1 as i32 as i64
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as i64) < s) as i32
                                            }) != 0
                                            {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 2147483647 as i32 as i64
                                                    >> (::core::mem::size_of::<idx_t>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64)
                                            } else {
                                                2147483647 as i32 as i64 / -s
                                            }) <= -(1 as i32) as i64 - n) as i32
                                        })
                                    } else {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + (-(2147483647 as i32) - 1 as i32) as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64
                                            }) + 0 as i32 as i64
                                        }) < 0 as i32 as i64
                                        {
                                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + (-(2147483647 as i32) - 1 as i32) as i64)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + (-(2147483647 as i32) - 1 as i32) as i64
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            + (-(2147483647 as i32) - 1 as i32) as i64
                                                    }) - 1 as i32 as i64
                                                })) as i32
                                        } else {
                                            ((0 as i32 as i64)
                                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                                        }) != 0 && s == -(1 as i32) as i64
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((0 as i32 as i64)
                                                    < n + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                                            } else {
                                                ((0 as i32 as i64) < n
                                                    && ((-(1 as i32) - (-(2147483647 as i32) - 1 as i32))
                                                        as i64) < n - 1 as i32 as i64) as i32
                                            })
                                        } else {
                                            ((-(2147483647 as i32) - 1 as i32) as i64 / s < n) as i32
                                        })
                                    })
                                } else {
                                    (if s == 0 as i32 as i64 {
                                        0 as i32
                                    } else {
                                        (if n < 0 as i32 as i64 {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + (-(2147483647 as i32) - 1 as i32) as i64
                                                }) + 0 as i32 as i64
                                            }) < 0 as i32 as i64
                                            {
                                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + (-(2147483647 as i32) - 1 as i32) as i64
                                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                + (-(2147483647 as i32) - 1 as i32) as i64
                                                        }) - 1 as i32 as i64
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as i64)
                                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                                            }) != 0 && n == -(1 as i32) as i64
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((0 as i32 as i64)
                                                        < s + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                                                } else {
                                                    (((-(1 as i32) - (-(2147483647 as i32) - 1 as i32)) as i64)
                                                        < s - 1 as i32 as i64) as i32
                                                })
                                            } else {
                                                ((-(2147483647 as i32) - 1 as i32) as i64 / n < s) as i32
                                            })
                                        } else {
                                            (2147483647 as i32 as i64 / s < n) as i32
                                        })
                                    })
                                }) != 0
                                {
                                    nbytes = (n as u32).wrapping_mul(s as u32) as i32 as idx_t;
                                    1 as i32
                                } else {
                                    nbytes = (n as u32).wrapping_mul(s as u32) as i32 as idx_t;
                                    0 as i32
                                })
                            } else {
                                (if (if s < 0 as i32 as i64 {
                                    (if n < 0 as i32 as i64 {
                                        (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as u32
                                            } else {
                                                (2147483647 as i32 as u32)
                                                    .wrapping_mul(2 as u32)
                                                    .wrapping_add(1 as u32)
                                            }) as i64 + s
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            (n
                                                < (2147483647 as i32 as u32)
                                                    .wrapping_mul(2 as u32)
                                                    .wrapping_add(1 as u32) as i64 / s) as i32
                                        } else {
                                            ((if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                s
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<idx_t>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64)
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) < 0 as i32 as i64
                                            {
                                                (s
                                                    < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            + 1 as i32 as i64)
                                                            << (::core::mem::size_of::<idx_t>() as u64)
                                                                .wrapping_mul(8 as i32 as u64)
                                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                            * 2 as i32 as i64 + 1 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            - 1 as i32 as i64
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as i64) < s) as i32
                                            }) != 0
                                            {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + (2147483647 as i32 as u32)
                                                        .wrapping_mul(2 as u32)
                                                        .wrapping_add(1 as u32) as i64
                                                    >> (::core::mem::size_of::<idx_t>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(1 as i32 as u64)
                                            } else {
                                                (2147483647 as i32 as u32)
                                                    .wrapping_mul(2 as u32)
                                                    .wrapping_add(1 as u32) as i64 / -s
                                            }) <= -(1 as i32) as i64 - n) as i32
                                        })
                                    } else {
                                        (if (if (if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            !(((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64)
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) + 0 as i32 as i64
                                        }) < 0 as i32 as i64
                                        {
                                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                + 0 as i32 as i64)
                                                < -(if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            + 0 as i32 as i64
                                                    }) - 1 as i32 as i64
                                                })) as i32
                                        } else {
                                            ((0 as i32 as i64)
                                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64) as i32
                                        }) != 0 && s == -(1 as i32) as i64
                                        {
                                            (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                                            } else {
                                                ((0 as i32 as i64) < n
                                                    && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                                                    as i32
                                            })
                                        } else {
                                            (0 as i32 as i64 / s < n) as i32
                                        })
                                    })
                                } else {
                                    (if s == 0 as i32 as i64 {
                                        0 as i32
                                    } else {
                                        (if n < 0 as i32 as i64 {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64)
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64
                                                }) + 0 as i32 as i64
                                            }) < 0 as i32 as i64
                                            {
                                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    + 0 as i32 as i64)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64
                                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                + 0 as i32 as i64
                                                        }) - 1 as i32 as i64
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as i64)
                                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64) as i32
                                            }) != 0 && n == -(1 as i32) as i64
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                                                } else {
                                                    (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64)
                                                        as i32
                                                })
                                            } else {
                                                (0 as i32 as i64 / n < s) as i32
                                            })
                                        } else {
                                            ((2147483647 as i32 as u32)
                                                .wrapping_mul(2 as u32)
                                                .wrapping_add(1 as u32) as i64 / s < n) as i32
                                        })
                                    })
                                }) != 0
                                {
                                    nbytes = (n as u32).wrapping_mul(s as u32) as idx_t;
                                    1 as i32
                                } else {
                                    nbytes = (n as u32).wrapping_mul(s as u32) as idx_t;
                                    0 as i32
                                })
                            })
                        } else {
                            (if ::core::mem::size_of::<idx_t>() as u64
                                == ::core::mem::size_of::<i64>() as u64
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { nbytes })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    (if (if s < 0 as i32 as i64 {
                                        (if n < 0 as i32 as i64 {
                                            (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    9223372036854775807 as i64
                                                }) + s
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                (n < 9223372036854775807 as i64 / s) as i32
                                            } else {
                                                ((if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    s
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    (s
                                                        < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                + 1 as i32 as i64)
                                                                << (::core::mem::size_of::<idx_t>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                                * 2 as i32 as i64 + 1 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64) < s) as i32
                                                }) != 0
                                                {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 9223372036854775807 as i64
                                                        >> (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(1 as i32 as u64)
                                                } else {
                                                    9223372036854775807 as i64 / -s
                                                }) <= -(1 as i32) as i64 - n) as i32
                                            })
                                        } else {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                }) + 0 as i32 as i64
                                            }) < 0 as i32 as i64
                                            {
                                                ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                                        }) - 1 as i32 as i64
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as i64)
                                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                            }) != 0 && s == -(1 as i32) as i64
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((0 as i32 as i64)
                                                        < n + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                                } else {
                                                    ((0 as i32 as i64) < n
                                                        && -(1 as i32) as i64
                                                            - (-(9223372036854775807 as i64) - 1 as i64)
                                                            < n - 1 as i32 as i64) as i32
                                                })
                                            } else {
                                                ((-(9223372036854775807 as i64) - 1 as i64) / s < n) as i32
                                            })
                                        })
                                    } else {
                                        (if s == 0 as i32 as i64 {
                                            0 as i32
                                        } else {
                                            (if n < 0 as i32 as i64 {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                                    }) + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32 as i64
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                                            }) - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64)
                                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                                }) != 0 && n == -(1 as i32) as i64
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((0 as i32 as i64)
                                                            < s + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                                    } else {
                                                        (-(1 as i32) as i64
                                                            - (-(9223372036854775807 as i64) - 1 as i64)
                                                            < s - 1 as i32 as i64) as i32
                                                    })
                                                } else {
                                                    ((-(9223372036854775807 as i64) - 1 as i64) / n < s) as i32
                                                })
                                            } else {
                                                (9223372036854775807 as i64 / s < n) as i32
                                            })
                                        })
                                    }) != 0
                                    {
                                        nbytes = (n as u64).wrapping_mul(s as u64) as i64;
                                        1 as i32
                                    } else {
                                        nbytes = (n as u64).wrapping_mul(s as u64) as i64;
                                        0 as i32
                                    })
                                } else {
                                    (if (if s < 0 as i32 as i64 {
                                        (if n < 0 as i32 as i64 {
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
                                                    .wrapping_add(s as u64)
                                            })
                                                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                            {
                                                ((n as u64)
                                                    < (9223372036854775807 as i64 as u64)
                                                        .wrapping_mul(2 as u64)
                                                        .wrapping_add(1 as u64)
                                                        .wrapping_div(s as u64)) as i32
                                            } else {
                                                ((if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    s
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    (s
                                                        < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                + 1 as i32 as i64)
                                                                << (::core::mem::size_of::<idx_t>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                                * 2 as i32 as i64 + 1 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64) < s) as i32
                                                }) != 0
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s }) as u64)
                                                        .wrapping_add(
                                                            (9223372036854775807 as i64 as u64)
                                                                .wrapping_mul(2 as u64)
                                                                .wrapping_add(1 as u64),
                                                        )
                                                        >> (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(1 as i32 as u64)
                                                } else {
                                                    (9223372036854775807 as i64 as u64)
                                                        .wrapping_mul(2 as u64)
                                                        .wrapping_add(1 as u64)
                                                        .wrapping_div(-s as u64)
                                                }) <= (-(1 as i32) as i64 - n) as u64) as i32
                                            })
                                        } else {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64)
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) + 0 as i32 as i64
                                            }) < 0 as i32 as i64
                                            {
                                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            + 0 as i32 as i64
                                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                + 0 as i32 as i64
                                                        }) - 1 as i32 as i64
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as i64)
                                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64) as i32
                                            }) != 0 && s == -(1 as i32) as i64
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                                                } else {
                                                    ((0 as i32 as i64) < n
                                                        && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                                                        as i32
                                                })
                                            } else {
                                                (0 as i32 as i64 / s < n) as i32
                                            })
                                        })
                                    } else {
                                        (if s == 0 as i32 as i64 {
                                            0 as i32
                                        } else {
                                            (if n < 0 as i32 as i64 {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64
                                                    }) + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<i64>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64
                                                    }) + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                + 0 as i32 as i64
                                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32 as i64
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                    + 0 as i32 as i64
                                                            }) - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64)
                                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64) as i32
                                                }) != 0 && n == -(1 as i32) as i64
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                                                    } else {
                                                        (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64)
                                                            as i32
                                                    })
                                                } else {
                                                    (0 as i32 as i64 / n < s) as i32
                                                })
                                            } else {
                                                ((9223372036854775807 as i64 as u64)
                                                    .wrapping_mul(2 as u64)
                                                    .wrapping_add(1 as u64)
                                                    .wrapping_div(s as u64) < n as u64) as i32
                                            })
                                        })
                                    }) != 0
                                    {
                                        nbytes = (n as u64).wrapping_mul(s as u64) as idx_t;
                                        1 as i32
                                    } else {
                                        nbytes = (n as u64).wrapping_mul(s as u64) as idx_t;
                                        0 as i32
                                    })
                                })
                            } else {
                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { nbytes })
                                    - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    (if (if s < 0 as i32 as i64 {
                                        (if n < 0 as i32 as i64 {
                                            (if ((if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    9223372036854775807 as libc::c_longlong
                                                }) + s as libc::c_longlong
                                            }) - 1 as i32 as libc::c_longlong)
                                                < 0 as i32 as libc::c_longlong
                                            {
                                                ((n as libc::c_longlong)
                                                    < 9223372036854775807 as libc::c_longlong
                                                        / s as libc::c_longlong) as i32
                                            } else {
                                                ((if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    s
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    (s
                                                        < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                + 1 as i32 as i64)
                                                                << (::core::mem::size_of::<idx_t>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                                * 2 as i32 as i64 + 1 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64) < s) as i32
                                                }) != 0
                                                {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        as libc::c_longlong
                                                        + 9223372036854775807 as libc::c_longlong
                                                        >> (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(1 as i32 as u64)
                                                } else {
                                                    9223372036854775807 as libc::c_longlong
                                                        / -s as libc::c_longlong
                                                }) <= (-(1 as i32) as i64 - n) as libc::c_longlong) as i32
                                            })
                                        } else {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as i32 as libc::c_longlong)
                                                < 0 as i32 as libc::c_longlong
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 0 as i32 as libc::c_longlong
                                            }) < 0 as i32 as libc::c_longlong
                                            {
                                                ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32 as libc::c_longlong
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) - 1 as i32 as libc::c_longlong)
                                                        < 0 as i32 as libc::c_longlong
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32 as libc::c_longlong
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) - 1 as i32 as libc::c_longlong
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as libc::c_longlong)
                                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)) as i32
                                            }) != 0 && s == -(1 as i32) as i64
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((0 as i32 as libc::c_longlong)
                                                        < n as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)) as i32
                                                } else {
                                                    ((0 as i32 as i64) < n
                                                        && -(1 as i32) as libc::c_longlong
                                                            - (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                            < (n - 1 as i32 as i64) as libc::c_longlong) as i32
                                                })
                                            } else {
                                                (((-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong) / s as libc::c_longlong)
                                                    < n as libc::c_longlong) as i32
                                            })
                                        })
                                    } else {
                                        (if s == 0 as i32 as i64 {
                                            0 as i32
                                        } else {
                                            (if n < 0 as i32 as i64 {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) - 1 as i32 as libc::c_longlong)
                                                    < 0 as i32 as libc::c_longlong
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32 as libc::c_longlong
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                    }) + 0 as i32 as libc::c_longlong
                                                }) < 0 as i32 as libc::c_longlong
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32 as libc::c_longlong
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)
                                                        }) - 1 as i32 as libc::c_longlong)
                                                            < 0 as i32 as libc::c_longlong
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32 as libc::c_longlong
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                    as libc::c_longlong
                                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                                        - 1 as libc::c_longlong)
                                                            }) - 1 as i32 as libc::c_longlong
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as libc::c_longlong)
                                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            as libc::c_longlong
                                                            + (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)) as i32
                                                }) != 0 && n == -(1 as i32) as i64
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((0 as i32 as libc::c_longlong)
                                                            < s as libc::c_longlong
                                                                + (-(9223372036854775807 as libc::c_longlong)
                                                                    - 1 as libc::c_longlong)) as i32
                                                    } else {
                                                        (-(1 as i32) as libc::c_longlong
                                                            - (-(9223372036854775807 as libc::c_longlong)
                                                                - 1 as libc::c_longlong)
                                                            < (s - 1 as i32 as i64) as libc::c_longlong) as i32
                                                    })
                                                } else {
                                                    (((-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong) / n as libc::c_longlong)
                                                        < s as libc::c_longlong) as i32
                                                })
                                            } else {
                                                ((9223372036854775807 as libc::c_longlong
                                                    / s as libc::c_longlong) < n as libc::c_longlong) as i32
                                            })
                                        })
                                    }) != 0
                                    {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as libc::c_longlong
                                            as idx_t;
                                        1 as i32
                                    } else {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as libc::c_longlong
                                            as idx_t;
                                        0 as i32
                                    })
                                } else {
                                    (if (if s < 0 as i32 as i64 {
                                        (if n < 0 as i32 as i64 {
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
                                                    .wrapping_add(s as libc::c_ulonglong)
                                            })
                                                .wrapping_sub(1 as i32 as libc::c_ulonglong)
                                                < 0 as i32 as libc::c_ulonglong
                                            {
                                                ((n as libc::c_ulonglong)
                                                    < (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(s as libc::c_ulonglong)) as i32
                                            } else {
                                                ((if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    s
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    (s
                                                        < -(if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                + 1 as i32 as i64)
                                                                << (::core::mem::size_of::<idx_t>() as u64)
                                                                    .wrapping_mul(8 as i32 as u64)
                                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                                * 2 as i32 as i64 + 1 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64) < s) as i32
                                                }) != 0
                                                {
                                                    ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        as libc::c_ulonglong)
                                                        .wrapping_add(
                                                            (9223372036854775807 as libc::c_longlong
                                                                as libc::c_ulonglong)
                                                                .wrapping_mul(2 as libc::c_ulonglong)
                                                                .wrapping_add(1 as libc::c_ulonglong),
                                                        )
                                                        >> (::core::mem::size_of::<idx_t>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(1 as i32 as u64)
                                                } else {
                                                    (9223372036854775807 as libc::c_longlong
                                                        as libc::c_ulonglong)
                                                        .wrapping_mul(2 as libc::c_ulonglong)
                                                        .wrapping_add(1 as libc::c_ulonglong)
                                                        .wrapping_div(-s as libc::c_ulonglong)
                                                }) <= (-(1 as i32) as i64 - n) as libc::c_ulonglong) as i32
                                            })
                                        } else {
                                            (if (if (if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                !(((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64)
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64
                                                }) + 0 as i32 as i64
                                            }) < 0 as i32 as i64
                                            {
                                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                    + 0 as i32 as i64)
                                                    < -(if ((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                            + 0 as i32 as i64
                                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
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
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                                + 0 as i32 as i64
                                                        }) - 1 as i32 as i64
                                                    })) as i32
                                            } else {
                                                ((0 as i32 as i64)
                                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        + 0 as i32 as i64) as i32
                                            }) != 0 && s == -(1 as i32) as i64
                                            {
                                                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                    - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    ((0 as i32 as i64) < n + 0 as i32 as i64) as i32
                                                } else {
                                                    ((0 as i32 as i64) < n
                                                        && ((-(1 as i32) - 0 as i32) as i64) < n - 1 as i32 as i64)
                                                        as i32
                                                })
                                            } else {
                                                (0 as i32 as i64 / s < n) as i32
                                            })
                                        })
                                    } else {
                                        (if s == 0 as i32 as i64 {
                                            0 as i32
                                        } else {
                                            (if n < 0 as i32 as i64 {
                                                (if (if (if ((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                                {
                                                    !(((((if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64
                                                    }) + 1 as i32 as i64)
                                                        << (::core::mem::size_of::<i64>() as u64)
                                                            .wrapping_mul(8 as i32 as u64)
                                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                                } else {
                                                    (if 1 as i32 != 0 {
                                                        0 as i32 as i64
                                                    } else {
                                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64
                                                    }) + 0 as i32 as i64
                                                }) < 0 as i32 as i64
                                                {
                                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                        + 0 as i32 as i64)
                                                        < -(if ((if 1 as i32 != 0 {
                                                            0 as i32 as i64
                                                        } else {
                                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                + 0 as i32 as i64
                                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                                        {
                                                            ((((if 1 as i32 != 0 {
                                                                0 as i32 as i64
                                                            } else {
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
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
                                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                                    + 0 as i32 as i64
                                                            }) - 1 as i32 as i64
                                                        })) as i32
                                                } else {
                                                    ((0 as i32 as i64)
                                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { n })
                                                            + 0 as i32 as i64) as i32
                                                }) != 0 && n == -(1 as i32) as i64
                                                {
                                                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { s })
                                                        - 1 as i32 as i64) < 0 as i32 as i64
                                                    {
                                                        ((0 as i32 as i64) < s + 0 as i32 as i64) as i32
                                                    } else {
                                                        (((-(1 as i32) - 0 as i32) as i64) < s - 1 as i32 as i64)
                                                            as i32
                                                    })
                                                } else {
                                                    (0 as i32 as i64 / n < s) as i32
                                                })
                                            } else {
                                                ((9223372036854775807 as libc::c_longlong
                                                    as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_ulonglong)
                                                    .wrapping_div(s as libc::c_ulonglong)
                                                    < n as libc::c_ulonglong) as i32
                                            })
                                        })
                                    }) != 0
                                    {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as idx_t;
                                        1 as i32
                                    } else {
                                        nbytes = (n as libc::c_ulonglong)
                                            .wrapping_mul(s as libc::c_ulonglong) as idx_t;
                                        0 as i32
                                    })
                                })
                            })
                        })
                    })
                }) != 0
        }
    {
        xalloc_die();
    }
    pa = xrealloc(pa, nbytes as size_t);
    *pn = n;
    return pa;
}
#[no_mangle]
pub unsafe extern "C" fn xzalloc(mut s: size_t) -> *mut libc::c_void {
    return xcalloc(s, 1 as i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn xizalloc(mut s: idx_t) -> *mut libc::c_void {
    return xicalloc(s, 1 as i32 as idx_t);
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    return nonnull(calloc(n, s));
}
#[no_mangle]
pub unsafe extern "C" fn xicalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
    return nonnull(icalloc(n, s));
}
#[no_mangle]
pub unsafe extern "C" fn xmemdup(
    mut p: *const libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    return memcpy(xmalloc(s), p, s);
}
#[no_mangle]
pub unsafe extern "C" fn ximemdup(
    mut p: *const libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return memcpy(ximalloc(s), p, s as u64);
}
#[no_mangle]
pub unsafe extern "C" fn ximemdup0(mut p: *const libc::c_void, mut s: idx_t) -> *mut i8 {
    let mut result: *mut i8 = ximalloc(s + 1 as i32 as i64) as *mut i8;
    *result.offset(s as isize) = 0 as i32 as i8;
    return memcpy(result as *mut libc::c_void, p, s as u64) as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut string: *const i8) -> *mut i8 {
    return xmemdup(
        string as *const libc::c_void,
        (strlen(string)).wrapping_add(1 as i32 as u64),
    ) as *mut i8;
}