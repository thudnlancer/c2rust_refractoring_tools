#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
extern "C" {
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type idx_t = ptrdiff_t;
#[no_mangle]
#[cold]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as i32;
    return 0 as *mut libc::c_void;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn imalloc(mut s: idx_t) -> *mut libc::c_void {
    return if s as u64 <= 18446744073709551615 as u64 {
        malloc(s as u64)
    } else {
        _gl_alloc_nomem()
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn irealloc(
    mut p: *mut libc::c_void,
    mut s: idx_t,
) -> *mut libc::c_void {
    return if s as u64 <= 18446744073709551615 as u64 {
        realloc(p, (s | (s == 0) as i32 as i64) as u64)
    } else {
        _gl_alloc_nomem()
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn icalloc(mut n: idx_t, mut s: idx_t) -> *mut libc::c_void {
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn ireallocarray(
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