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
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
}
static mut PATH_SEP: i8 = '/' as i32 as i8;
#[no_mangle]
pub unsafe extern "C" fn mt_basename(mut filename: *const i8) -> *const i8 {
    let mut ptr: *mut i8 = 0 as *mut i8;
    ptr = strrchr(filename, PATH_SEP as i32);
    if !ptr.is_null() {
        filename = ptr.offset(1 as i32 as isize);
    }
    return filename;
}
#[no_mangle]
pub unsafe extern "C" fn mt_stripexe(mut filename: *mut i8) {
    let mut ptr: *mut i8 = 0 as *mut i8;
    ptr = strrchr(filename, '.' as i32);
    if !ptr.is_null() && strcasecmp(ptr, b".exe\0" as *const u8 as *const i8) == 0 {
        *ptr = '\0' as i32 as i8;
    }
}