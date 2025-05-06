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
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    fn execl(__path: *const i8, __arg: *const i8, _: ...) -> i32;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> i32;
    fn setgid(__gid: __gid_t) -> i32;
    fn fork() -> __pid_t;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn abort() -> !;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn last_component(filename: *const i8) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn initgroups(__user: *const i8, __group: __gid_t) -> i32;
    fn safe_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn full_write(fd: i32, buf: *const libc::c_void, count: size_t) -> size_t;
    fn gethostbyname(__name: *const i8) -> *mut hostent;
}
pub type __intmax_t = i64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __off_t = i64;
pub type __pid_t = i32;
pub type __daddr_t = i32;
pub type __ssize_t = i64;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
pub type intmax_t = __intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtget {
    pub mt_type: i64,
    pub mt_resid: i64,
    pub mt_dsreg: i64,
    pub mt_gstat: i64,
    pub mt_erreg: i64,
    pub mt_fileno: __daddr_t,
    pub mt_blkno: __daddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut i8,
    pub h_aliases: *mut *mut i8,
    pub h_addrtype: i32,
    pub h_length: i32,
    pub h_addr_list: *mut *mut i8,
}
pub type C2RustUnnamed = u32;
pub const countlen: C2RustUnnamed = 11;
pub type C2RustUnnamed_0 = u32;
pub const oplen: C2RustUnnamed_0 = 6;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const i8) -> i64 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32);
}
static mut from_remote: [[i32; 2]; 4] = [
    [-(1 as i32), -(1 as i32)],
    [-(1 as i32), -(1 as i32)],
    [-(1 as i32), -(1 as i32)],
    [-(1 as i32), -(1 as i32)],
];
static mut to_remote: [[i32; 2]; 4] = [
    [-(1 as i32), -(1 as i32)],
    [-(1 as i32), -(1 as i32)],
    [-(1 as i32), -(1 as i32)],
    [-(1 as i32), -(1 as i32)],
];
#[no_mangle]
pub static mut rmt_command: *const i8 = b"/usr/local/libexec/rmt\0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut rmt_dev_name__: *const i8 = 0 as *const i8;
#[no_mangle]
pub static mut force_local_option: bool = false;
unsafe extern "C" fn _rmt_shutdown(mut handle: i32, mut errno_value: i32) {
    close(from_remote[handle as usize][0 as i32 as usize]);
    close(to_remote[handle as usize][1 as i32 as usize]);
    from_remote[handle as usize][0 as i32 as usize] = -(1 as i32);
    to_remote[handle as usize][1 as i32 as usize] = -(1 as i32);
    *__errno_location() = errno_value;
}
unsafe extern "C" fn do_command(mut handle: i32, mut buffer: *const i8) -> i32 {
    let mut length: size_t = strlen(buffer);
    let mut pipe_handler: Option<unsafe extern "C" fn(i32) -> ()> = signal(
        13 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    let mut written: ssize_t = full_write(
        to_remote[handle as usize][1 as i32 as usize],
        buffer as *const libc::c_void,
        length,
    ) as ssize_t;
    signal(13 as i32, pipe_handler);
    if written as u64 == length {
        return 0 as i32;
    }
    _rmt_shutdown(handle, 5 as i32);
    return -(1 as i32);
}
unsafe extern "C" fn get_status_string(
    mut handle: i32,
    mut command_buffer: *mut i8,
) -> *mut i8 {
    let mut cursor: *mut i8 = 0 as *mut i8;
    let mut counter: i32 = 0;
    counter = 0 as i32;
    cursor = command_buffer;
    while counter < 64 as i32 {
        if safe_read(
            from_remote[handle as usize][0 as i32 as usize],
            cursor as *mut libc::c_void,
            1 as i32 as size_t,
        ) != 1 as i32 as u64
        {
            _rmt_shutdown(handle, 5 as i32);
            return 0 as *mut i8;
        }
        if *cursor as i32 == '\n' as i32 {
            *cursor = '\0' as i32 as i8;
            break;
        } else {
            counter += 1;
            counter;
            cursor = cursor.offset(1);
            cursor;
        }
    }
    if counter == 64 as i32 {
        _rmt_shutdown(handle, 5 as i32);
        return 0 as *mut i8;
    }
    cursor = command_buffer;
    while *cursor != 0 {
        if *cursor as i32 != ' ' as i32 {
            break;
        }
        cursor = cursor.offset(1);
        cursor;
    }
    if *cursor as i32 == 'E' as i32 || *cursor as i32 == 'F' as i32 {
        let mut character: i8 = 0;
        while safe_read(
            from_remote[handle as usize][0 as i32 as usize],
            &mut character as *mut i8 as *mut libc::c_void,
            1 as i32 as size_t,
        ) == 1 as i32 as u64
        {
            if character as i32 == '\n' as i32 {
                break;
            }
        }
        *__errno_location() = atoi(cursor.offset(1 as i32 as isize));
        if *cursor as i32 == 'F' as i32 {
            _rmt_shutdown(handle, *__errno_location());
        }
        return 0 as *mut i8;
    }
    if *cursor as i32 != 'A' as i32 {
        _rmt_shutdown(handle, 5 as i32);
        return 0 as *mut i8;
    }
    return cursor.offset(1 as i32 as isize);
}
unsafe extern "C" fn get_status(mut handle: i32) -> i64 {
    let mut command_buffer: [i8; 64] = [0; 64];
    let mut status: *const i8 = get_status_string(handle, command_buffer.as_mut_ptr());
    if !status.is_null() {
        let mut result: i64 = atol(status);
        if 0 as i32 as i64 <= result {
            return result;
        }
        *__errno_location() = 5 as i32;
    }
    return -(1 as i32) as i64;
}
unsafe extern "C" fn get_status_off(mut handle: i32) -> off_t {
    let mut command_buffer: [i8; 64] = [0; 64];
    let mut status: *const i8 = get_status_string(handle, command_buffer.as_mut_ptr());
    if status.is_null() {
        return -(1 as i32) as off_t
    } else {
        let mut count: off_t = 0 as i32 as off_t;
        let mut negative: i32 = 0;
        while *status as i32 == ' ' as i32 || *status as i32 == '\t' as i32 {
            status = status.offset(1);
            status;
        }
        negative = (*status as i32 == '-' as i32) as i32;
        status = status
            .offset((negative != 0 || *status as i32 == '+' as i32) as i32 as isize);
        loop {
            let fresh0 = status;
            status = status.offset(1);
            let mut digit: i32 = *fresh0 as i32 - '0' as i32;
            if (9 as i32 as u32) < digit as u32 {
                break;
            }
            let mut c10: off_t = 10 as i32 as i64 * count;
            let mut nc: off_t = if negative != 0 {
                c10 - digit as i64
            } else {
                c10 + digit as i64
            };
            if c10 / 10 as i32 as i64 != count
                || (if negative != 0 { (c10 < nc) as i32 } else { (nc < c10) as i32 })
                    != 0
            {
                return -(1 as i32) as off_t;
            }
            count = nc;
        }
        return count;
    };
}
unsafe extern "C" fn encode_oflag(mut buf: *mut i8, mut oflag: i32) {
    sprintf(buf, b"%d \0" as *const u8 as *const i8, oflag);
    match oflag & 0o3 as i32 {
        0 => {
            strcat(buf, b"O_RDONLY\0" as *const u8 as *const i8);
        }
        2 => {
            strcat(buf, b"O_RDWR\0" as *const u8 as *const i8);
        }
        1 => {
            strcat(buf, b"O_WRONLY\0" as *const u8 as *const i8);
        }
        _ => {
            abort();
        }
    }
    if oflag & 0o2000 as i32 != 0 {
        strcat(buf, b"|O_APPEND\0" as *const u8 as *const i8);
    }
    if oflag & 0o100 as i32 != 0 {
        strcat(buf, b"|O_CREAT\0" as *const u8 as *const i8);
    }
    if oflag & 0o10000 as i32 != 0 {
        strcat(buf, b"|O_DSYNC\0" as *const u8 as *const i8);
    }
    if oflag & 0o200 as i32 != 0 {
        strcat(buf, b"|O_EXCL\0" as *const u8 as *const i8);
    }
    if oflag & 0 as i32 != 0 {
        strcat(buf, b"|O_LARGEFILE\0" as *const u8 as *const i8);
    }
    if oflag & 0o400 as i32 != 0 {
        strcat(buf, b"|O_NOCTTY\0" as *const u8 as *const i8);
    }
    if oflag & 0o4000 as i32 != 0 {
        strcat(buf, b"|O_NONBLOCK\0" as *const u8 as *const i8);
    }
    if oflag & 0o4010000 as i32 != 0 {
        strcat(buf, b"|O_RSYNC\0" as *const u8 as *const i8);
    }
    if oflag & 0o4010000 as i32 != 0 {
        strcat(buf, b"|O_SYNC\0" as *const u8 as *const i8);
    }
    if oflag & 0o1000 as i32 != 0 {
        strcat(buf, b"|O_TRUNC\0" as *const u8 as *const i8);
    }
}
unsafe extern "C" fn sys_reset_uid_gid() -> *const i8 {
    let mut uid: uid_t = getuid();
    let mut gid: gid_t = getgid();
    let mut pw: *mut passwd = getpwuid(uid);
    if pw.is_null() {
        return b"getpwuid\0" as *const u8 as *const i8;
    }
    if initgroups((*pw).pw_name, gid) != 0 as i32 {
        return b"initgroups\0" as *const u8 as *const i8;
    }
    if gid != getegid() && setgid(gid) != 0 as i32 && *__errno_location() != 1 as i32 {
        return b"setgid\0" as *const u8 as *const i8;
    }
    if uid != geteuid() && setuid(uid) != 0 as i32 && *__errno_location() != 1 as i32 {
        return b"setuid\0" as *const u8 as *const i8;
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn rmt_open__(
    mut file_name: *const i8,
    mut open_mode: i32,
    mut bias: i32,
    mut remote_shell: *const i8,
) -> i32 {
    let mut remote_pipe_number: i32 = 0;
    let mut file_name_copy: *mut i8 = 0 as *mut i8;
    let mut remote_host: *mut i8 = 0 as *mut i8;
    let mut remote_file: *mut i8 = 0 as *mut i8;
    let mut remote_user: *mut i8 = 0 as *mut i8;
    remote_pipe_number = 0 as i32;
    while remote_pipe_number < 4 as i32 {
        if from_remote[remote_pipe_number as usize][0 as i32 as usize] == -(1 as i32)
            && to_remote[remote_pipe_number as usize][1 as i32 as usize] == -(1 as i32)
        {
            break;
        }
        remote_pipe_number += 1;
        remote_pipe_number;
    }
    if remote_pipe_number == 4 as i32 {
        *__errno_location() = 24 as i32;
        return -(1 as i32);
    }
    let mut cursor: *mut i8 = 0 as *mut i8;
    file_name_copy = xstrdup(file_name);
    remote_host = file_name_copy;
    remote_user = 0 as *mut i8;
    remote_file = 0 as *mut i8;
    cursor = file_name_copy;
    while *cursor != 0 {
        match *cursor as i32 {
            10 => {
                rpl_free(file_name_copy as *mut libc::c_void);
                *__errno_location() = 2 as i32;
                return -(1 as i32);
            }
            64 => {
                if remote_user.is_null() {
                    remote_user = remote_host;
                    *cursor = '\0' as i32 as i8;
                    remote_host = cursor.offset(1 as i32 as isize);
                }
            }
            58 => {
                if remote_file.is_null() {
                    *cursor = '\0' as i32 as i8;
                    remote_file = cursor.offset(1 as i32 as isize);
                }
            }
            _ => {}
        }
        cursor = cursor.offset(1);
        cursor;
    }
    if !remote_file.is_null() {} else {
        unreachable!();
    };
    if (gethostbyname(remote_host)).is_null() {
        error(
            128 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Cannot connect to %s: resolve failed\0" as *const u8 as *const i8,
                5 as i32,
            ),
            remote_host,
        );
    }
    if !remote_user.is_null() && *remote_user as i32 == '\0' as i32 {
        remote_user = 0 as *mut i8;
    }
    let mut remote_shell_basename: *const i8 = 0 as *const i8;
    let mut status: pid_t = 0;
    if remote_shell.is_null() {
        rpl_free(file_name_copy as *mut libc::c_void);
        *__errno_location() = 5 as i32;
        return -(1 as i32);
    }
    remote_shell_basename = last_component(remote_shell);
    if pipe((to_remote[remote_pipe_number as usize]).as_mut_ptr()) == -(1 as i32)
        || pipe((from_remote[remote_pipe_number as usize]).as_mut_ptr()) == -(1 as i32)
    {
        let mut e: i32 = *__errno_location();
        rpl_free(file_name_copy as *mut libc::c_void);
        *__errno_location() = e;
        return -(1 as i32);
    }
    status = fork();
    if status == -(1 as i32) {
        let mut e_0: i32 = *__errno_location();
        rpl_free(file_name_copy as *mut libc::c_void);
        *__errno_location() = e_0;
        return -(1 as i32);
    }
    if status == 0 as i32 {
        if dup2(to_remote[remote_pipe_number as usize][0 as i32 as usize], 0 as i32)
            < 0 as i32
            || to_remote[remote_pipe_number as usize][0 as i32 as usize] != 0 as i32
                && close(to_remote[remote_pipe_number as usize][0 as i32 as usize])
                    != 0 as i32
            || to_remote[remote_pipe_number as usize][1 as i32 as usize] != 0 as i32
                && close(to_remote[remote_pipe_number as usize][1 as i32 as usize])
                    != 0 as i32
            || dup2(
                from_remote[remote_pipe_number as usize][1 as i32 as usize],
                1 as i32,
            ) < 0 as i32
            || close(from_remote[remote_pipe_number as usize][0 as i32 as usize])
                != 0 as i32
            || close(from_remote[remote_pipe_number as usize][1 as i32 as usize])
                != 0 as i32
        {
            error(
                128 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Cannot redirect files for remote shell\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        let mut reseterr: *const i8 = sys_reset_uid_gid();
        if !reseterr.is_null() {
            error(
                128 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"Cannot reset uid and gid: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                reseterr,
            );
        }
        if !remote_user.is_null() {
            execl(
                remote_shell,
                remote_shell_basename,
                remote_host,
                b"-l\0" as *const u8 as *const i8,
                remote_user,
                rmt_command,
                0 as *mut i8,
            );
        } else {
            execl(
                remote_shell,
                remote_shell_basename,
                remote_host,
                rmt_command,
                0 as *mut i8,
            );
        }
        error(
            128 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"Cannot execute remote shell\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    close(from_remote[remote_pipe_number as usize][1 as i32 as usize]);
    close(to_remote[remote_pipe_number as usize][0 as i32 as usize]);
    let mut remote_file_len: size_t = strlen(remote_file);
    let mut command_buffer: *mut i8 = xmalloc(
        remote_file_len.wrapping_add(1000 as i32 as u64),
    ) as *mut i8;
    sprintf(command_buffer, b"O%s\n\0" as *const u8 as *const i8, remote_file);
    encode_oflag(
        command_buffer.offset(remote_file_len as isize).offset(2 as i32 as isize),
        open_mode,
    );
    strcat(command_buffer, b"\n\0" as *const u8 as *const i8);
    if do_command(remote_pipe_number, command_buffer) == -(1 as i32)
        || get_status(remote_pipe_number) == -(1 as i32) as i64
    {
        let mut e_1: i32 = *__errno_location();
        rpl_free(command_buffer as *mut libc::c_void);
        rpl_free(file_name_copy as *mut libc::c_void);
        _rmt_shutdown(remote_pipe_number, e_1);
        return -(1 as i32);
    }
    rpl_free(command_buffer as *mut libc::c_void);
    rpl_free(file_name_copy as *mut libc::c_void);
    return remote_pipe_number + bias;
}
#[no_mangle]
pub unsafe extern "C" fn rmt_close__(mut handle: i32) -> i32 {
    let mut status: i64 = 0;
    if do_command(handle, b"C\n\0" as *const u8 as *const i8) == -(1 as i32) {
        return -(1 as i32);
    }
    status = get_status(handle);
    _rmt_shutdown(handle, *__errno_location());
    return status as i32;
}
#[no_mangle]
pub unsafe extern "C" fn rmt_read__(
    mut handle: i32,
    mut buffer: *mut i8,
    mut length: size_t,
) -> size_t {
    let mut command_buffer: [i8; 23] = [0; 23];
    let mut status: size_t = 0;
    let mut rlen: size_t = 0;
    let mut counter: size_t = 0;
    sprintf(command_buffer.as_mut_ptr(), b"R%zu\n\0" as *const u8 as *const i8, length);
    if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as i32)
        || {
            status = get_status(handle) as size_t;
            status == -(1 as i32) as size_t
        } || status > length
    {
        return -(1 as i32) as size_t;
    }
    counter = 0 as i32 as size_t;
    while counter < status {
        rlen = safe_read(
            from_remote[handle as usize][0 as i32 as usize],
            buffer as *mut libc::c_void,
            status.wrapping_sub(counter),
        );
        if rlen == -(1 as i32) as size_t || rlen == 0 as i32 as u64 {
            _rmt_shutdown(handle, 5 as i32);
            return -(1 as i32) as size_t;
        }
        counter = (counter as u64).wrapping_add(rlen) as size_t as size_t;
        buffer = buffer.offset(rlen as isize);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn rmt_write__(
    mut handle: i32,
    mut buffer: *mut i8,
    mut length: size_t,
) -> size_t {
    let mut command_buffer: [i8; 23] = [0; 23];
    let mut pipe_handler: Option<unsafe extern "C" fn(i32) -> ()> = None;
    let mut written: size_t = 0;
    sprintf(command_buffer.as_mut_ptr(), b"W%zu\n\0" as *const u8 as *const i8, length);
    if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as i32) {
        return 0 as i32 as size_t;
    }
    pipe_handler = signal(
        13 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    written = full_write(
        to_remote[handle as usize][1 as i32 as usize],
        buffer as *const libc::c_void,
        length,
    );
    signal(13 as i32, pipe_handler);
    if written == length {
        let mut r: i64 = get_status(handle);
        if r < 0 as i32 as i64 {
            return 0 as i32 as size_t;
        }
        if r as u64 == length {
            return length;
        }
        written = r as size_t;
    }
    _rmt_shutdown(handle, 5 as i32);
    return written;
}
#[no_mangle]
pub unsafe extern "C" fn rmt_lseek__(
    mut handle: i32,
    mut offset: off_t,
    mut whence: i32,
) -> off_t {
    let mut command_buffer: [i8; 25] = [0; 25];
    match whence {
        0 => {
            whence = 0 as i32;
        }
        1 => {
            whence = 1 as i32;
        }
        2 => {
            whence = 2 as i32;
        }
        _ => {
            abort();
        }
    }
    let mut off: intmax_t = offset;
    sprintf(
        command_buffer.as_mut_ptr(),
        b"L%jd\n%d\n\0" as *const u8 as *const i8,
        off,
        whence,
    );
    if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as i32) {
        return -(1 as i32) as off_t;
    }
    return get_status_off(handle);
}
#[no_mangle]
pub unsafe extern "C" fn rmt_ioctl__(
    mut handle: i32,
    mut operation: u64,
    mut argument: *mut libc::c_void,
) -> i32 {
    match operation {
        1074294017 => {
            let mut mtop: *mut mtop = argument as *mut mtop;
            let mut command_buffer: [i8; 21] = [0; 21];
            let mut count: intmax_t = (*mtop).mt_count as intmax_t;
            sprintf(
                command_buffer.as_mut_ptr(),
                b"I%d\n%jd\n\0" as *const u8 as *const i8,
                (*mtop).mt_op as i32,
                count,
            );
            if do_command(handle, command_buffer.as_mut_ptr()) == -(1 as i32) {
                return -(1 as i32);
            }
            return get_status(handle) as i32;
        }
        2150657282 => {
            let mut status: ssize_t = 0;
            let mut counter: size_t = 0;
            if do_command(handle, b"S\0" as *const u8 as *const i8) == -(1 as i32)
                || {
                    status = get_status(handle);
                    status == -(1 as i32) as i64
                }
            {
                return -(1 as i32);
            }
            if status as u64 > ::core::mem::size_of::<mtop>() as u64 {
                *__errno_location() = 75 as i32;
                return -(1 as i32);
            }
            let mut p: *mut i8 = argument as *mut i8;
            while status > 0 as i32 as i64 {
                counter = safe_read(
                    from_remote[handle as usize][0 as i32 as usize],
                    p as *mut libc::c_void,
                    status as size_t,
                );
                if counter == -(1 as i32) as size_t || counter == 0 as i32 as u64 {
                    _rmt_shutdown(handle, 5 as i32);
                    return -(1 as i32);
                }
                status = (status as u64).wrapping_sub(counter) as ssize_t as ssize_t;
                p = p.offset(counter as isize);
            }
            let mut mtget: *mut mtget = argument as *mut mtget;
            if (*mtget).mt_type < 256 as i32 as i64 {
                return 0 as i32;
            }
            let mut buf: *mut i8 = argument as *mut i8;
            counter = 0 as i32 as size_t;
            while counter < status as u64 {
                let mut copy: i8 = *buf.offset(counter as isize);
                *buf.offset(counter as isize) = *buf
                    .offset(counter.wrapping_add(1 as i32 as u64) as isize);
                *buf.offset(counter.wrapping_add(1 as i32 as u64) as isize) = copy;
                counter = (counter as u64).wrapping_add(2 as i32 as u64) as size_t
                    as size_t;
            }
            return 0 as i32;
        }
        _ => {
            *__errno_location() = 95 as i32;
            return -(1 as i32);
        }
    };
}