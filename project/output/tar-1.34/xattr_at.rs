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
    fn setxattr(
        __path: *const i8,
        __name: *const i8,
        __value: *const libc::c_void,
        __size: size_t,
        __flags: i32,
    ) -> i32;
    fn lsetxattr(
        __path: *const i8,
        __name: *const i8,
        __value: *const libc::c_void,
        __size: size_t,
        __flags: i32,
    ) -> i32;
    fn getxattr(
        __path: *const i8,
        __name: *const i8,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn lgetxattr(
        __path: *const i8,
        __name: *const i8,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn listxattr(__path: *const i8, __list: *mut i8, __size: size_t) -> ssize_t;
    fn llistxattr(__path: *const i8, __list: *mut i8, __size: size_t) -> ssize_t;
    fn openat_restore_fail(_: i32);
    fn openat_save_fail(_: i32);
    fn fchdir(__fd: i32) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn save_cwd(cwd: *mut saved_cwd) -> i32;
    fn restore_cwd(cwd: *const saved_cwd) -> i32;
    fn free_cwd(cwd: *mut saved_cwd);
    fn openat_proc_name(buf: *mut i8, fd: i32, file: *const i8) -> *mut i8;
}
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: i32,
    pub name: *mut i8,
}
#[no_mangle]
pub unsafe extern "C" fn llistxattrat(
    mut fd: i32,
    mut file: *const i8,
    mut list: *mut i8,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return llistxattr(file, list, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = llistxattr(proc_file, list, size);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) as i64 != proc_result {
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
        return -(1 as i32) as ssize_t;
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32) as ssize_t;
    }
    err = llistxattr(file, list, size);
    saved_errno = if err == -(1 as i32) as i64 { *__errno_location() } else { 0 as i32 };
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
pub unsafe extern "C" fn setxattrat(
    mut fd: i32,
    mut file: *const i8,
    mut name: *const i8,
    mut value: *const libc::c_void,
    mut size: size_t,
    mut flags: i32,
) -> i32 {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return setxattr(file, name, value, size, flags);
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
        let mut proc_result: i32 = setxattr(proc_file, name, value, size, flags);
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
    err = setxattr(file, name, value, size, flags);
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
pub unsafe extern "C" fn lgetxattrat(
    mut fd: i32,
    mut file: *const i8,
    mut name: *const i8,
    mut value: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return lgetxattr(file, name, value, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = lgetxattr(proc_file, name, value, size);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) as i64 != proc_result {
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
        return -(1 as i32) as ssize_t;
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32) as ssize_t;
    }
    err = lgetxattr(file, name, value, size);
    saved_errno = if err == -(1 as i32) as i64 { *__errno_location() } else { 0 as i32 };
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
pub unsafe extern "C" fn getxattrat(
    mut fd: i32,
    mut file: *const i8,
    mut name: *const i8,
    mut value: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return getxattr(file, name, value, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = getxattr(proc_file, name, value, size);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) as i64 != proc_result {
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
        return -(1 as i32) as ssize_t;
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32) as ssize_t;
    }
    err = getxattr(file, name, value, size);
    saved_errno = if err == -(1 as i32) as i64 { *__errno_location() } else { 0 as i32 };
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
pub unsafe extern "C" fn listxattrat(
    mut fd: i32,
    mut file: *const i8,
    mut list: *mut i8,
    mut size: size_t,
) -> ssize_t {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return listxattr(file, list, size);
    }
    let mut saved_cwd: saved_cwd = saved_cwd {
        desc: 0,
        name: 0 as *mut i8,
    };
    let mut saved_errno: i32 = 0;
    let mut err: ssize_t = 0;
    let mut proc_buf: [i8; 4032] = [0; 4032];
    let mut proc_file: *mut i8 = openat_proc_name(proc_buf.as_mut_ptr(), fd, file);
    if !proc_file.is_null() {
        let mut proc_result: ssize_t = listxattr(proc_file, list, size);
        let mut proc_errno: i32 = *__errno_location();
        if proc_file != proc_buf.as_mut_ptr() {
            rpl_free(proc_file as *mut libc::c_void);
        }
        if -(1 as i32) as i64 != proc_result {
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
        return -(1 as i32) as ssize_t;
    }
    if fchdir(fd) != 0 as i32 {
        saved_errno = *__errno_location();
        free_cwd(&mut saved_cwd);
        *__errno_location() = saved_errno;
        return -(1 as i32) as ssize_t;
    }
    err = listxattr(file, list, size);
    saved_errno = if err == -(1 as i32) as i64 { *__errno_location() } else { 0 as i32 };
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
pub unsafe extern "C" fn lsetxattrat(
    mut fd: i32,
    mut file: *const i8,
    mut name: *const i8,
    mut value: *const libc::c_void,
    mut size: size_t,
    mut flags: i32,
) -> i32 {
    if fd == -(100 as i32) || *file.offset(0 as i32 as isize) as i32 == '/' as i32 {
        return lsetxattr(file, name, value, size, flags);
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
        let mut proc_result: i32 = lsetxattr(proc_file, name, value, size, flags);
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
    err = lsetxattr(file, name, value, size, flags);
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