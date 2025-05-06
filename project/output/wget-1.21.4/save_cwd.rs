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
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn close(__fd: i32) -> i32;
    fn fchdir(__fd: i32) -> i32;
    fn getcwd(__buf: *mut i8, __size: size_t) -> *mut i8;
    fn chdir_long(dir: *mut i8) -> i32;
    fn fd_safer_flag(_: i32, _: i32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: i32,
    pub name: *mut i8,
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn save_cwd(mut cwd: *mut saved_cwd) -> i32 {
    (*cwd).name = 0 as *mut i8;
    (*cwd).desc = open(b".\0" as *const u8 as *const i8, 0 as i32 | 0o2000000 as i32);
    if 0 as i32 == 0 {
        (*cwd).desc = fd_safer_flag((*cwd).desc, 0o2000000 as i32);
    }
    if (*cwd).desc < 0 as i32 {
        (*cwd).name = getcwd(0 as *mut i8, 0 as i32 as size_t);
        return if !((*cwd).name).is_null() { 0 as i32 } else { -(1 as i32) };
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn restore_cwd(mut cwd: *const saved_cwd) -> i32 {
    if 0 as i32 <= (*cwd).desc {
        return fchdir((*cwd).desc)
    } else {
        return chdir_long((*cwd).name)
    };
}
#[no_mangle]
pub unsafe extern "C" fn free_cwd(mut cwd: *mut saved_cwd) {
    if (*cwd).desc >= 0 as i32 {
        close((*cwd).desc);
    }
    rpl_free((*cwd).name as *mut libc::c_void);
}