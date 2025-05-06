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
    fn openat_restore_fail(_: i32);
    fn openat_save_fail(_: i32);
    fn fchdir(__fd: i32) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn save_cwd(cwd: *mut saved_cwd) -> i32;
    fn restore_cwd(cwd: *const saved_cwd) -> i32;
    fn free_cwd(cwd: *mut saved_cwd);
    fn openat_proc_name(buf: *mut i8, fd: i32, file: *const i8) -> *mut i8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: i32,
    pub name: *mut i8,
}
#[inline]
unsafe extern "C" fn getfilecon(mut file: *const i8, mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[inline]
unsafe extern "C" fn lgetfilecon(mut file: *const i8, mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[inline]
unsafe extern "C" fn setfilecon(mut file: *const i8, mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[inline]
unsafe extern "C" fn lsetfilecon(mut file: *const i8, mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn getfileconat(
    mut fd: i32,
    mut file: *const i8,
    mut con: *mut *mut i8,
) -> i32 {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return getfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: i32 = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: i32 = getfilecon(proc_file, con);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as i32 || proc_errno == 2 as i32 || proc_errno == 1 as i32
            || proc_errno == 13 as i32 || proc_errno == 38 as i32
            || proc_errno == 95 as i32)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as i32 {
        openat_save_fail(*__errno_location());
    }
    if 0 as i32 <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32);
    }
    err = getfilecon(file, con);
    saved_errno = if err == -(1 as i32) { *__errno_location() } else { 0 as i32 };
    if restore_cwd(&mut saved_cwd) != 0 as i32 {
        openat_restore_fail(*__errno_location());
    }
    free_cwd(&mut saved_cwd);
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn lsetfileconat(
    mut fd: i32,
    mut file: *const i8,
    mut con: *const i8,
) -> i32 {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return lsetfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: i32 = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: i32 = lsetfilecon(proc_file, con);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as i32 || proc_errno == 2 as i32 || proc_errno == 1 as i32
            || proc_errno == 13 as i32 || proc_errno == 38 as i32
            || proc_errno == 95 as i32)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as i32 {
        openat_save_fail(*__errno_location());
    }
    if 0 as i32 <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32);
    }
    err = lsetfilecon(file, con);
    saved_errno = if err == -(1 as i32) { *__errno_location() } else { 0 as i32 };
    if restore_cwd(&mut saved_cwd) != 0 as i32 {
        openat_restore_fail(*__errno_location());
    }
    free_cwd(&mut saved_cwd);
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn setfileconat(
    mut fd: i32,
    mut file: *const i8,
    mut con: *const i8,
) -> i32 {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return setfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: i32 = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: i32 = setfilecon(proc_file, con);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as i32 || proc_errno == 2 as i32 || proc_errno == 1 as i32
            || proc_errno == 13 as i32 || proc_errno == 38 as i32
            || proc_errno == 95 as i32)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as i32 {
        openat_save_fail(*__errno_location());
    }
    if 0 as i32 <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32);
    }
    err = setfilecon(file, con);
    saved_errno = if err == -(1 as i32) { *__errno_location() } else { 0 as i32 };
    if restore_cwd(&mut saved_cwd) != 0 as i32 {
        openat_restore_fail(*__errno_location());
    }
    free_cwd(&mut saved_cwd);
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
    }
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn lgetfileconat(
    mut fd: i32,
    mut file: *const i8,
    mut con: *mut *mut i8,
) -> i32 {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return lgetfilecon(file, con);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: i32 = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: i32 = lgetfilecon(proc_file, con);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) != proc_result {
            return proc_result;
        }
        if !(proc_errno == 20 as i32 || proc_errno == 2 as i32 || proc_errno == 1 as i32
            || proc_errno == 13 as i32 || proc_errno == 38 as i32
            || proc_errno == 95 as i32)
        {
            *__errno_location() = proc_errno;
            return proc_result;
        }
    }
    if save_cwd(&mut saved_cwd) != 0 as i32 {
        openat_save_fail(*__errno_location());
    }
    if 0 as i32 <= fd && fd == saved_cwd.desc {
        free_cwd(&mut saved_cwd);
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32);
    }
    err = lgetfilecon(file, con);
    saved_errno = if err == -(1 as i32) { *__errno_location() } else { 0 as i32 };
    if restore_cwd(&mut saved_cwd) != 0 as i32 {
        openat_restore_fail(*__errno_location());
    }
    free_cwd(&mut saved_cwd);
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
    }
    return err;
}