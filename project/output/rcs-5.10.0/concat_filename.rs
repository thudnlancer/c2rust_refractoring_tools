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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn stpcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn concatenated_filename(
    mut directory: *const i8,
    mut filename: *const i8,
    mut suffix: *const i8,
) -> *mut i8 {
    let mut result: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    if strcmp(directory, b".\0" as *const u8 as *const i8) == 0 as i32 {
        result = malloc(
            (strlen(filename))
                .wrapping_add(
                    (if !suffix.is_null() { strlen(suffix) } else { 0 as i32 as u64 }),
                )
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        if result.is_null() {
            return 0 as *mut i8;
        }
        p = result;
    } else {
        let mut directory_len: size_t = strlen(directory);
        let mut need_slash: i32 = (directory_len > 0 as i32 as u64
            && !(*directory.offset(directory_len.wrapping_sub(1 as i32 as u64) as isize)
                as i32 == '/' as i32)) as i32;
        result = malloc(
            directory_len
                .wrapping_add(need_slash as u64)
                .wrapping_add(strlen(filename))
                .wrapping_add(
                    (if !suffix.is_null() { strlen(suffix) } else { 0 as i32 as u64 }),
                )
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        if result.is_null() {
            return 0 as *mut i8;
        }
        memcpy(
            result as *mut libc::c_void,
            directory as *const libc::c_void,
            directory_len,
        );
        p = result.offset(directory_len as isize);
        if need_slash != 0 {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '/' as i32 as i8;
        }
    }
    p = stpcpy(p, filename);
    if !suffix.is_null() {
        stpcpy(p, suffix);
    }
    return result;
}