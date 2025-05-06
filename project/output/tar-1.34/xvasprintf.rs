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
    fn vasprintf(__ptr: *mut *mut i8, __f: *const i8, __arg: ::core::ffi::VaList) -> i32;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
pub type va_list = __builtin_va_list;
pub type size_t = u64;
#[inline]
unsafe extern "C" fn xnmalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    if (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
        9223372036854775807 as i64 as u64
    } else {
        (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
    })
        .wrapping_div(s) < n
    {
        xalloc_die();
    }
    return xmalloc(n.wrapping_mul(s));
}
#[inline]
unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as u64 };
}
unsafe extern "C" fn xstrcat(
    mut argcount: size_t,
    mut args: ::core::ffi::VaList,
) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut ap: ::core::ffi::VaListImpl;
    let mut totalsize: size_t = 0;
    let mut i: size_t = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    totalsize = 0 as i32 as size_t;
    ap = args.clone();
    i = argcount;
    while i > 0 as i32 as u64 {
        let mut next: *const i8 = ap.arg::<*const i8>();
        totalsize = xsum(totalsize, strlen(next));
        i = i.wrapping_sub(1);
        i;
    }
    if totalsize == 18446744073709551615 as u64 || totalsize > 2147483647 as i32 as u64 {
        *__errno_location() = 75 as i32;
        return 0 as *mut i8;
    }
    result = (if ::core::mem::size_of::<i8>() as u64 == 1 as i32 as u64 {
        xmalloc(totalsize.wrapping_add(1 as i32 as u64))
    } else {
        xnmalloc(
            totalsize.wrapping_add(1 as i32 as u64),
            ::core::mem::size_of::<i8>() as u64,
        )
    }) as *mut i8;
    p = result;
    i = argcount;
    while i > 0 as i32 as u64 {
        let mut next_0: *const i8 = args.arg::<*const i8>();
        let mut len: size_t = strlen(next_0);
        memcpy(p as *mut libc::c_void, next_0 as *const libc::c_void, len);
        p = p.offset(len as isize);
        i = i.wrapping_sub(1);
        i;
    }
    *p = '\0' as i32 as i8;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xvasprintf(
    mut format: *const i8,
    mut args: ::core::ffi::VaList,
) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut argcount: size_t = 0 as i32 as size_t;
    let mut f: *const i8 = 0 as *const i8;
    f = format;
    loop {
        if *f as i32 == '\0' as i32 {
            return xstrcat(argcount, args.as_va_list());
        }
        if *f as i32 != '%' as i32 {
            break;
        }
        f = f.offset(1);
        f;
        if *f as i32 != 's' as i32 {
            break;
        }
        f = f.offset(1);
        f;
        argcount = argcount.wrapping_add(1);
        argcount;
    }
    if vasprintf(&mut result, format, args.as_va_list()) < 0 as i32 {
        if *__errno_location() == 12 as i32 {
            xalloc_die();
        }
        return 0 as *mut i8;
    }
    return result;
}