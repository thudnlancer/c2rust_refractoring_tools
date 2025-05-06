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
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
unsafe extern "C" fn setlocale_null_androidfix(mut category: i32) -> *const i8 {
    let mut result: *const i8 = setlocale(category, 0 as *const i8);
    return result;
}
unsafe extern "C" fn setlocale_null_unlocked(
    mut category: i32,
    mut buf: *mut i8,
    mut bufsize: size_t,
) -> i32 {
    let mut result: *const i8 = setlocale_null_androidfix(category);
    if result.is_null() {
        if bufsize > 0 as i32 as u64 {
            *buf.offset(0 as i32 as isize) = '\0' as i32 as i8;
        }
        return 22 as i32;
    } else {
        let mut length: size_t = strlen(result);
        if length < bufsize {
            memcpy(
                buf as *mut libc::c_void,
                result as *const libc::c_void,
                length.wrapping_add(1 as i32 as u64),
            );
            return 0 as i32;
        } else {
            if bufsize > 0 as i32 as u64 {
                memcpy(
                    buf as *mut libc::c_void,
                    result as *const libc::c_void,
                    bufsize.wrapping_sub(1 as i32 as u64),
                );
                *buf.offset(bufsize.wrapping_sub(1 as i32 as u64) as isize) = '\0' as i32
                    as i8;
            }
            return 34 as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn setlocale_null_r(
    mut category: i32,
    mut buf: *mut i8,
    mut bufsize: size_t,
) -> i32 {
    return setlocale_null_unlocked(category, buf, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn setlocale_null(mut category: i32) -> *const i8 {
    return setlocale_null_androidfix(category);
}