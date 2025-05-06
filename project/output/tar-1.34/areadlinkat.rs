#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type allocator;
    fn careadlinkat(
        fd: i32,
        filename: *const i8,
        buffer: *mut i8,
        buffer_size: size_t,
        alloc: *const allocator,
        preadlinkat: Option<
            unsafe extern "C" fn(i32, *const i8, *mut i8, size_t) -> ssize_t,
        >,
    ) -> *mut i8;
    fn readlinkat(
        __fd: i32,
        __path: *const i8,
        __buf: *mut i8,
        __len: size_t,
    ) -> ssize_t;
}
pub type size_t = u64;
pub type ssize_t = __ssize_t;
pub type __ssize_t = i64;
#[no_mangle]
pub unsafe extern "C" fn areadlinkat(mut fd: i32, mut filename: *const i8) -> *mut i8 {
    return careadlinkat(
        fd,
        filename,
        0 as *mut i8,
        0 as i32 as size_t,
        0 as *const allocator,
        Some(
            readlinkat
                as unsafe extern "C" fn(i32, *const i8, *mut i8, size_t) -> ssize_t,
        ),
    );
}