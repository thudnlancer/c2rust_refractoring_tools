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
    fn fchdir(__fd: i32) -> i32;
    fn openat_restore_fail(_: i32);
    fn openat_save_fail(_: i32);
    fn save_cwd(cwd: *mut saved_cwd) -> i32;
    fn restore_cwd(cwd: *const saved_cwd) -> i32;
    fn free_cwd(cwd: *mut saved_cwd);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: i32,
    pub name: *mut i8,
}
#[no_mangle]
pub unsafe extern "C" fn run_in_dir(
    mut there: *const saved_cwd,
    mut callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    mut usercontext: *mut libc::c_void,
) -> i32 {
    let mut err: i32 = -(1 as i32);
    let mut saved_errno: i32 = 0 as i32;
    let mut here: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    if 0 as i32 == save_cwd(&mut here) {
        if 0 as i32 == restore_cwd(there) {
            err = callback.expect("non-null function pointer")(usercontext);
            saved_errno = if err < 0 as i32 { *__errno_location() } else { 0 as i32 };
        } else {
            openat_restore_fail(*__errno_location());
        }
        if restore_cwd(&mut here) != 0 as i32 {
            openat_restore_fail(*__errno_location());
        }
        free_cwd(&mut here);
    } else {
        openat_save_fail(*__errno_location());
    }
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn run_in_dirfd(
    mut dir_fd: i32,
    mut callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    mut usercontext: *mut libc::c_void,
) -> i32 {
    if dir_fd == -(100 as i32) {
        return (Some(callback.expect("non-null function pointer")))
            .expect("non-null function pointer")(usercontext)
    } else {
        let mut saved_cwd: saved_cwd = saved_cwd {
            desc: 0,
            name: 0 as *mut i8,
        };
        let mut saved_errno: i32 = 0;
        let mut err: i32 = 0;
        if save_cwd(&mut saved_cwd) != 0 as i32 {
            openat_save_fail(*__errno_location());
        }
        if fchdir(dir_fd) != 0 as i32 {
            saved_errno = *__errno_location();
            free_cwd(&mut saved_cwd);
            *__errno_location() = saved_errno;
            return -(1 as i32);
        }
        err = (Some(callback.expect("non-null function pointer")))
            .expect("non-null function pointer")(usercontext);
        saved_errno = if err < 0 as i32 { *__errno_location() } else { 0 as i32 };
        if restore_cwd(&mut saved_cwd) != 0 as i32 {
            openat_restore_fail(*__errno_location());
        }
        free_cwd(&mut saved_cwd);
        if saved_errno != 0 {
            *__errno_location() = saved_errno;
        }
        return err;
    };
}