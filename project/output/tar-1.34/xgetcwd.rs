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
    fn __errno_location() -> *mut i32;
    fn getcwd(__buf: *mut i8, __size: size_t) -> *mut i8;
    fn xalloc_die();
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn xgetcwd() -> *mut i8 {
    let mut cwd: *mut i8 = getcwd(0 as *mut i8, 0 as i32 as size_t);
    if cwd.is_null() && *__errno_location() == 12 as i32 {
        xalloc_die();
    }
    return cwd;
}