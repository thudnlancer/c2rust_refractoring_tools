#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cd_buf {
    pub fd: libc::c_int,
}
unsafe extern "C" fn cdb_init(mut cdb: *mut cd_buf) {
    (*cdb).fd = -(100 as libc::c_int);
}
unsafe extern "C" fn cdb_fchdir(mut cdb: *const cd_buf) -> libc::c_int {
    return fchdir((*cdb).fd);
}
unsafe extern "C" fn cdb_free(mut cdb: *const cd_buf) {
    if 0 as libc::c_int <= (*cdb).fd {
        let mut close_fail: bool = close((*cdb).fd) != 0;
        (0 as libc::c_int != 0 && !close_fail) as libc::c_int;
    }
}
unsafe extern "C" fn cdb_advance_fd(
    mut cdb: *mut cd_buf,
    mut dir: *const libc::c_char,
) -> libc::c_int {
    let mut new_fd: libc::c_int = openat(
        (*cdb).fd,
        dir,
        0 as libc::c_int | 0o200000 as libc::c_int | 0o400 as libc::c_int
            | 0o4000 as libc::c_int,
    );
    if new_fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    cdb_free(cdb);
    (*cdb).fd = new_fd;
    return 0 as libc::c_int;
}
unsafe extern "C" fn find_non_slash(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut n_slash: size_t = strspn(s, b"/\0" as *const u8 as *const libc::c_char);
    return (s as *mut libc::c_char).offset(n_slash as isize);
}
#[no_mangle]
pub unsafe extern "C" fn chdir_long(mut dir: *mut libc::c_char) -> libc::c_int {
    let mut current_block: u64;
    let mut e: libc::c_int = chdir(dir);
    if e == 0 as libc::c_int || *__errno_location() != 36 as libc::c_int {
        return e;
    }
    let mut len: size_t = strlen(dir);
    let mut dir_end: *mut libc::c_char = dir.offset(len as isize);
    let mut cdb: cd_buf = cd_buf { fd: 0 };
    let mut n_leading_slash: size_t = 0;
    cdb_init(&mut cdb);
    (0 as libc::c_int != 0 && (0 as libc::c_int as libc::c_ulong) < len) as libc::c_int;
    (0 as libc::c_int != 0 && 4096 as libc::c_int as libc::c_ulong <= len)
        as libc::c_int;
    n_leading_slash = strspn(dir, b"/\0" as *const u8 as *const libc::c_char);
    if n_leading_slash == 2 as libc::c_int as libc::c_ulong {
        let mut err: libc::c_int = 0;
        let mut slash: *mut libc::c_char = memchr(
            dir.offset(3 as libc::c_int as isize) as *const libc::c_void,
            '/' as i32,
            dir_end.offset_from(dir.offset(3 as libc::c_int as isize)) as libc::c_long
                as libc::c_ulong,
        ) as *mut libc::c_char;
        if slash.is_null() {
            *__errno_location() = 36 as libc::c_int;
            return -(1 as libc::c_int);
        }
        *slash = '\0' as i32 as libc::c_char;
        err = cdb_advance_fd(&mut cdb, dir);
        *slash = '/' as i32 as libc::c_char;
        if err != 0 as libc::c_int {
            current_block = 9424354481868438207;
        } else {
            dir = find_non_slash(slash.offset(1 as libc::c_int as isize));
            current_block = 4956146061682418353;
        }
    } else if n_leading_slash != 0 {
        if cdb_advance_fd(&mut cdb, b"/\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
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
            (0 as libc::c_int != 0 && *dir as libc::c_int != '/' as i32) as libc::c_int;
            (0 as libc::c_int != 0 && dir <= dir_end) as libc::c_int;
            loop {
                if !(4096 as libc::c_int as libc::c_long
                    <= dir_end.offset_from(dir) as libc::c_long)
                {
                    current_block = 14763689060501151050;
                    break;
                }
                let mut err_0: libc::c_int = 0;
                let mut slash_0: *mut libc::c_char = memrchr(
                    dir as *const libc::c_void,
                    '/' as i32,
                    4096 as libc::c_int as size_t,
                ) as *mut libc::c_char;
                if slash_0.is_null() {
                    *__errno_location() = 36 as libc::c_int;
                    return -(1 as libc::c_int);
                }
                *slash_0 = '\0' as i32 as libc::c_char;
                (0 as libc::c_int != 0
                    && (slash_0.offset_from(dir) as libc::c_long)
                        < 4096 as libc::c_int as libc::c_long) as libc::c_int;
                err_0 = cdb_advance_fd(&mut cdb, dir);
                *slash_0 = '/' as i32 as libc::c_char;
                if err_0 != 0 as libc::c_int {
                    current_block = 9424354481868438207;
                    break;
                }
                dir = find_non_slash(slash_0.offset(1 as libc::c_int as isize));
            }
            match current_block {
                9424354481868438207 => {}
                _ => {
                    if dir < dir_end {
                        if cdb_advance_fd(&mut cdb, dir) != 0 as libc::c_int {
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
                            if !(cdb_fchdir(&mut cdb) != 0 as libc::c_int) {
                                cdb_free(&mut cdb);
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    let mut saved_errno: libc::c_int = *__errno_location();
    cdb_free(&mut cdb);
    *__errno_location() = saved_errno;
    return -(1 as libc::c_int);
}
