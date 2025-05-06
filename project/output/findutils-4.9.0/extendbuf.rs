#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xalloc_die();
}
pub type size_t = u64;
unsafe extern "C" fn decide_size(mut current: size_t, mut wanted: size_t) -> size_t {
    let mut newsize: size_t = 0;
    if 0 as i32 as u64 == current {
        newsize = 16 as i32 as size_t;
    } else {
        newsize = current;
    }
    while newsize < wanted {
        if (2 as i32 as u64).wrapping_mul(newsize) < newsize {
            return wanted;
        }
        newsize = (newsize as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
    }
    return newsize;
}
#[no_mangle]
pub unsafe extern "C" fn extendbuf(
    mut existing: *mut libc::c_void,
    mut wanted: size_t,
    mut allocated: *mut size_t,
) -> *mut libc::c_void {
    let mut saved_errno: i32 = 0;
    let mut newsize: size_t = 0;
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    saved_errno = *__errno_location();
    if wanted > 0 as u32 as u64 {} else {
        __assert_fail(
            b"wanted > 0u\0" as *const u8 as *const i8,
            b"extendbuf.c\0" as *const u8 as *const i8,
            71 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[i8; 42],
            >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1713: {
        if wanted > 0 as u32 as u64 {} else {
            __assert_fail(
                b"wanted > 0u\0" as *const u8 as *const i8,
                b"extendbuf.c\0" as *const u8 as *const i8,
                71 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[i8; 42],
                >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    newsize = decide_size(*allocated, wanted);
    if *allocated == 0 as i32 as u64 {
        if existing.is_null() {} else {
            __assert_fail(
                b"NULL == existing\0" as *const u8 as *const i8,
                b"extendbuf.c\0" as *const u8 as *const i8,
                79 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[i8; 42],
                >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                    .as_ptr(),
            );
        }
        'c_1598: {
            if existing.is_null() {} else {
                __assert_fail(
                    b"NULL == existing\0" as *const u8 as *const i8,
                    b"extendbuf.c\0" as *const u8 as *const i8,
                    79 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 42],
                        &[i8; 42],
                    >(b"void *extendbuf(void *, size_t, size_t *)\0"))
                        .as_ptr(),
                );
            }
        };
        *allocated = newsize;
        result = malloc(newsize);
    } else if newsize != *allocated {
        *allocated = newsize;
        result = realloc(existing, newsize);
        if result.is_null() {
            saved_errno = *__errno_location();
        }
    } else {
        result = existing;
    }
    if !result.is_null() {
        *__errno_location() = saved_errno;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn xextendbuf(
    mut existing: *mut libc::c_void,
    mut wanted: size_t,
    mut allocated: *mut size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = extendbuf(existing, wanted, allocated);
    if p.is_null() {
        rpl_free(existing);
        xalloc_die();
    }
    return p;
}