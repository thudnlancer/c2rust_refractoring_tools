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
    fn abort() -> !;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
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
}
pub type size_t = u64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
unsafe extern "C" fn careadlinkatcwd(
    mut fd: i32,
    mut filename: *const i8,
    mut buffer: *mut i8,
    mut buffer_size: size_t,
) -> ssize_t {
    if fd != -(100 as i32) {
        abort();
    }
    return readlink(filename, buffer, buffer_size);
}
#[no_mangle]
pub unsafe extern "C" fn areadlink(mut filename: *const i8) -> *mut i8 {
    return careadlinkat(
        -(100 as i32),
        filename,
        0 as *mut i8,
        0 as i32 as size_t,
        0 as *const allocator,
        Some(
            careadlinkatcwd
                as unsafe extern "C" fn(i32, *const i8, *mut i8, size_t) -> ssize_t,
        ),
    );
}