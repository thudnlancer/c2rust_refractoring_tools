#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn xalloc_die() -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub const HAVE_GNU_CALLOC: C2RustUnnamed_0 = 1;
pub const DEFAULT_MXFAST: C2RustUnnamed = 64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    DEFAULT_MXFAST,
}  // end of enum

pub type C2RustUnnamed = libc::c_uint;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    HAVE_GNU_CALLOC = 1,
}  // end of enum
named {
    DEFAULT_MXFAST,
}  // end of enum

pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
    } else {
        if (-(1 as libc::c_int) as size_t)
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut n: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(n);
    if p.is_null() && n != 0 as libc::c_int as libc::c_ulong {
        xalloc_die();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    p = realloc(p, n);
    if p.is_null() && n != 0 as libc::c_int as libc::c_ulong {
        xalloc_die();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn x2realloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
) -> *mut libc::c_void {
    return x2nrealloc(p, pn, 1 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn xzalloc(mut s: size_t) -> *mut libc::c_void {
    return memset(xmalloc(s), 0 as libc::c_int, s);
}
#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if HAVE_GNU_CALLOC as libc::c_int == 0
        && ((if ::core::mem::size_of::<ptrdiff_t>() as libc::c_ulong
            <= ::core::mem::size_of::<size_t>() as libc::c_ulong
        {
            -(1 as libc::c_int)
        } else {
            -(2 as libc::c_int)
        }) as size_t)
            .wrapping_div(s) < n
        || {
            p = calloc(n, s);
            p.is_null()
                && (HAVE_GNU_CALLOC as libc::c_int != 0
                    || n != 0 as libc::c_int as libc::c_ulong)
        }
    {
        xalloc_die();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xmemdup(
    mut p: *const libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    return memcpy(xmalloc(s), p, s);
}
#[no_mangle]
pub unsafe extern "C" fn xstrdup(mut string: *const libc::c_char) -> *mut libc::c_char {
    return xmemdup(
        string as *const libc::c_void,
        (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
}
