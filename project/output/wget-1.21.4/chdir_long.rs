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
    fn close(__fd: i32) -> i32;
    fn chdir(__path: *const i8) -> i32;
    fn fchdir(__fd: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn openat(__fd: i32, __file: *const i8, __oflag: i32, _: ...) -> i32;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memrchr(__s: *const libc::c_void, __c: i32, __n: size_t) -> *mut libc::c_void;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cd_buf {
    pub fd: i32,
}
unsafe extern "C" fn cdb_init(mut cdb: *mut cd_buf) {
    (*cdb).fd = -(100 as i32);
}
unsafe extern "C" fn cdb_fchdir(mut cdb: *const cd_buf) -> i32 {
    return fchdir((*cdb).fd);
}
unsafe extern "C" fn cdb_free(mut cdb: *const cd_buf) {
    if 0 as i32 <= (*cdb).fd {
        let mut close_fail: bool = close((*cdb).fd) != 0;
        (0 as i32 != 0 && !close_fail) as i32;
    }
}
unsafe extern "C" fn cdb_advance_fd(mut cdb: *mut cd_buf, mut dir: *const i8) -> i32 {
    let mut new_fd: i32 = openat(
        (*cdb).fd,
        dir,
        0 as i32 | 0o200000 as i32 | 0o400 as i32 | 0o4000 as i32,
    );
    if new_fd < 0 as i32 {
        return -(1 as i32);
    }
    cdb_free(cdb);
    (*cdb).fd = new_fd;
    return 0 as i32;
}
unsafe extern "C" fn find_non_slash(mut s: *const i8) -> *mut i8 {
    let mut n_slash: size_t = strspn(s, b"/\0" as *const u8 as *const i8);
    return (s as *mut i8).offset(n_slash as isize);
}
#[no_mangle]
pub unsafe extern "C" fn chdir_long(mut dir: *mut i8) -> i32 {
    let mut current_block: u64;
    let mut e: i32 = chdir(dir);
    if e == 0 as i32 || *__errno_location() != 36 as i32 {
        return e;
    }
    let mut len: size_t = strlen(dir);
    let mut dir_end: *mut i8 = dir.offset(len as isize);
    let mut cdb: cd_buf = cd_buf { fd: 0 };
    let mut n_leading_slash: size_t = 0;
    cdb_init(&mut cdb);
    (0 as i32 != 0 && (0 as i32 as u64) < len) as i32;
    (0 as i32 != 0 && 4096 as i32 as u64 <= len) as i32;
    n_leading_slash = strspn(dir, b"/\0" as *const u8 as *const i8);
    if n_leading_slash == 2 as i32 as u64 {
        let mut err: i32 = 0;
        let mut slash: *mut i8 = memchr(
            dir.offset(3 as i32 as isize) as *const libc::c_void,
            '/' as i32,
            dir_end.offset_from(dir.offset(3 as i32 as isize)) as i64 as u64,
        ) as *mut i8;
        if slash.is_null() {
            *__errno_location() = 36 as i32;
            return -(1 as i32);
        }
        *slash = '\0' as i32 as i8;
        err = cdb_advance_fd(&mut cdb, dir);
        *slash = '/' as i32 as i8;
        if err != 0 as i32 {
            current_block = 9424354481868438207;
        } else {
            dir = find_non_slash(slash.offset(1 as i32 as isize));
            current_block = 4956146061682418353;
        }
    } else if n_leading_slash != 0 {
        if cdb_advance_fd(&mut cdb, b"/\0" as *const u8 as *const i8) != 0 as i32 {
            current_block = 9424354481868438207;
        } else {
            dir = dir.offset(n_leading_slash as isize);
            current_block = 4956146061682418353;
        }
    } else {
        current_block = 4956146061682418353;
    }
    match current_block {
        4956146061682418353 => {
            (0 as i32 != 0 && *dir as i32 != '/' as i32) as i32;
            (0 as i32 != 0 && dir <= dir_end) as i32;
            loop {
                if !(4096 as i32 as i64 <= dir_end.offset_from(dir) as i64) {
                    current_block = 14763689060501151050;
                    break;
                }
                let mut err_0: i32 = 0;
                let mut slash_0: *mut i8 = memrchr(
                    dir as *const libc::c_void,
                    '/' as i32,
                    4096 as i32 as size_t,
                ) as *mut i8;
                if slash_0.is_null() {
                    *__errno_location() = 36 as i32;
                    return -(1 as i32);
                }
                *slash_0 = '\0' as i32 as i8;
                (0 as i32 != 0 && (slash_0.offset_from(dir) as i64) < 4096 as i32 as i64)
                    as i32;
                err_0 = cdb_advance_fd(&mut cdb, dir);
                *slash_0 = '/' as i32 as i8;
                if err_0 != 0 as i32 {
                    current_block = 9424354481868438207;
                    break;
                }
                dir = find_non_slash(slash_0.offset(1 as i32 as isize));
            }
            match current_block {
                9424354481868438207 => {}
                _ => {
                    if dir < dir_end {
                        if cdb_advance_fd(&mut cdb, dir) != 0 as i32 {
                            current_block = 9424354481868438207;
                        } else {
                            current_block = 5689316957504528238;
                        }
                    } else {
                        current_block = 5689316957504528238;
                    }
                    match current_block {
                        9424354481868438207 => {}
                        _ => {
                            if !(cdb_fchdir(&mut cdb) != 0 as i32) {
                                cdb_free(&mut cdb);
                                return 0 as i32;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    let mut saved_errno: i32 = *__errno_location();
    cdb_free(&mut cdb);
    *__errno_location() = saved_errno;
    return -(1 as i32);
}