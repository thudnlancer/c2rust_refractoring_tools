#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn dup(__fd: i32) -> i32;
    fn pselect(
        __nfds: i32,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkfifo(__path: *const i8, __mode: __mode_t) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn tmpfile() -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn fileno(__stream: *mut FILE) -> i32;
    static mut handling_fatal_signal: sig_atomic_t;
    fn __errno_location() -> *mut i32;
    fn unlink(__name: *const i8) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn error(flocp: *const floc, length: size_t, fmt: *const i8, _: ...);
    fn fatal(flocp: *const floc, length: size_t, fmt: *const i8, _: ...) -> !;
    fn pfatal_with_name(_: *const i8) -> !;
    fn perror_with_name(_: *const i8, _: *const i8);
    fn make_pid() -> pid_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const i8) -> *mut i8;
    fn get_tmpdir() -> *const i8;
    fn get_tmpfd(_: *mut *mut i8) -> i32;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    static mut db_level: i32;
}
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type __sig_atomic_t = i32;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const i8,
    pub lineno: u64,
    pub offset: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum js_type {
    js_none = 0,
    js_pipe,
    js_fifo,
}
impl js_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            js_type::js_none => 0,
            js_type::js_pipe => 1,
            js_type::js_fifo => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> js_type {
        match value {
            0 => js_type::js_none,
            1 => js_type::js_pipe,
            2 => js_type::js_fifo,
            _ => panic!("Invalid value for js_type: {}", value),
        }
    }
}
impl AddAssign<u32> for js_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = js_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for js_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = js_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for js_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = js_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for js_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = js_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for js_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = js_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for js_type {
    type Output = js_type;
    fn add(self, rhs: u32) -> js_type {
        js_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for js_type {
    type Output = js_type;
    fn sub(self, rhs: u32) -> js_type {
        js_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for js_type {
    type Output = js_type;
    fn mul(self, rhs: u32) -> js_type {
        js_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for js_type {
    type Output = js_type;
    fn div(self, rhs: u32) -> js_type {
        js_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for js_type {
    type Output = js_type;
    fn rem(self, rhs: u32) -> js_type {
        js_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn check_io_state() -> u32 {
    static mut state: u32 = 0x1 as i32 as u32;
    if state != 0x1 as i32 as u32 {
        return state;
    }
    if fcntl(fileno(stdin), 1 as i32) != -(1 as i32) || *__errno_location() != 9 as i32 {
        state |= 0x4 as i32 as u32;
    }
    if fcntl(fileno(stdout), 1 as i32) != -(1 as i32) || *__errno_location() != 9 as i32
    {
        state |= 0x8 as i32 as u32;
    }
    if fcntl(fileno(stderr), 1 as i32) != -(1 as i32) || *__errno_location() != 9 as i32
    {
        state |= 0x10 as i32 as u32;
    }
    if state & (0x8 as i32 | 0x10 as i32) as u32 == (0x8 as i32 | 0x10 as i32) as u32 {
        let mut stbuf_o: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut stbuf_e: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if fstat(fileno(stdout), &mut stbuf_o) == 0 as i32
            && fstat(fileno(stderr), &mut stbuf_e) == 0 as i32
            && stbuf_o.st_dev == stbuf_e.st_dev && stbuf_o.st_ino == stbuf_e.st_ino
        {
            state |= 0x2 as i32 as u32;
        }
    }
    return state;
}
static mut job_root: u8 = 0 as i32 as u8;
static mut job_fds: [i32; 2] = [-(1 as i32), -(1 as i32)];
static mut job_rfd: i32 = -(1 as i32);
static mut token: i8 = '+' as i32 as i8;
static mut js_type: js_type = js_type::js_none;
static mut fifo_name: *mut i8 = 0 as *const i8 as *mut i8;
unsafe extern "C" fn make_job_rfd() -> i32 {
    return 0 as i32;
}
unsafe extern "C" fn set_blocking(mut fd: i32, mut blocking: i32) {
    let mut flags: i32 = 0;
    loop {
        flags = fcntl(fd, 3 as i32);
        if !(flags == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if flags >= 0 as i32 {
        let mut r: i32 = 0;
        flags = if blocking != 0 {
            flags & !(0o4000 as i32)
        } else {
            flags | 0o4000 as i32
        };
        loop {
            r = fcntl(fd, 4 as i32, flags);
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r < 0 as i32 {
            pfatal_with_name(b"fcntl(O_NONBLOCK)\0" as *const u8 as *const i8);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_setup(mut slots: i32, mut style: *const i8) -> u32 {
    let mut r: i32 = 0;
    if style.is_null() || strcmp(style, b"fifo\0" as *const u8 as *const i8) == 0 as i32
    {
        let mut tmpdir: *const i8 = get_tmpdir();
        fifo_name = xmalloc(
            (strlen(tmpdir))
                .wrapping_add(
                    (::core::mem::size_of::<[i8; 7]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
                .wrapping_add(
                    (53 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                        .wrapping_div(22 as i32 as u64)
                        .wrapping_add(3 as i32 as u64),
                )
                .wrapping_add(2 as i32 as u64),
        ) as *mut i8;
        sprintf(
            fifo_name,
            b"%s/GMfifo%lld\0" as *const u8 as *const i8,
            tmpdir,
            make_pid() as libc::c_longlong,
        );
        loop {
            r = mkfifo(fifo_name, 0o600 as i32 as __mode_t);
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r < 0 as i32 {
            perror_with_name(
                b"jobserver mkfifo: \0" as *const u8 as *const i8,
                fifo_name,
            );
            free(fifo_name as *mut libc::c_void);
            fifo_name = 0 as *mut i8;
        } else {
            loop {
                job_fds[0 as i32 as usize] = open(fifo_name, 0 as i32 | 0o4000 as i32);
                if !(job_fds[0 as i32 as usize] == -(1 as i32)
                    && *__errno_location() == 4 as i32)
                {
                    break;
                }
            }
            if job_fds[0 as i32 as usize] < 0 as i32 {
                fatal(
                    0 as *mut floc,
                    (strlen(fifo_name))
                        .wrapping_add(strlen(strerror(*__errno_location()))),
                    dcgettext(
                        0 as *const i8,
                        b"cannot open jobserver %s: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    fifo_name,
                    strerror(*__errno_location()),
                );
            }
            loop {
                job_fds[1 as i32 as usize] = open(fifo_name, 0o1 as i32);
                if !(job_fds[1 as i32 as usize] == -(1 as i32)
                    && *__errno_location() == 4 as i32)
                {
                    break;
                }
            }
            if job_fds[0 as i32 as usize] < 0 as i32 {
                fatal(
                    0 as *mut floc,
                    (strlen(fifo_name))
                        .wrapping_add(strlen(strerror(*__errno_location()))),
                    dcgettext(
                        0 as *const i8,
                        b"cannot open jobserver %s: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    fifo_name,
                    strerror(*__errno_location()),
                );
            }
            js_type = js_type::js_fifo;
        }
    }
    if js_type as u32 == js_type::js_none as i32 as u32 {
        if !style.is_null()
            && strcmp(style, b"pipe\0" as *const u8 as *const i8) != 0 as i32
        {
            fatal(
                0 as *mut floc,
                strlen(style),
                dcgettext(
                    0 as *const i8,
                    b"unknown jobserver auth style '%s'\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                style,
            );
        }
        loop {
            r = pipe(job_fds.as_mut_ptr());
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r < 0 as i32 {
            pfatal_with_name(
                dcgettext(
                    0 as *const i8,
                    b"creating jobs pipe\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        js_type = js_type::js_pipe;
    }
    fd_noinherit(job_fds[0 as i32 as usize]);
    fd_noinherit(job_fds[1 as i32 as usize]);
    if make_job_rfd() < 0 as i32 {
        pfatal_with_name(
            dcgettext(
                0 as *const i8,
                b"duping jobs pipe\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    loop {
        let fresh0 = slots;
        slots = slots - 1;
        if !(fresh0 != 0) {
            break;
        }
        loop {
            r = write(
                job_fds[1 as i32 as usize],
                &mut token as *mut i8 as *const libc::c_void,
                1 as i32 as size_t,
            ) as i32;
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r != 1 as i32 {
            pfatal_with_name(
                dcgettext(
                    0 as *const i8,
                    b"init jobserver pipe\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    set_blocking(job_fds[0 as i32 as usize], 0 as i32);
    job_root = 1 as i32 as u8;
    return 1 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_parse_auth(mut auth: *const i8) -> u32 {
    let mut rfd: i32 = 0;
    let mut wfd: i32 = 0;
    if strncmp(
        auth,
        b"fifo:\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64),
    ) == 0 as i32
    {
        fifo_name = xstrdup(
            auth
                .offset(
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_sub(1 as i32 as u64) as isize,
                ),
        );
        loop {
            job_fds[0 as i32 as usize] = open(fifo_name, 0 as i32);
            if !(job_fds[0 as i32 as usize] == -(1 as i32)
                && *__errno_location() == 4 as i32)
            {
                break;
            }
        }
        if job_fds[0 as i32 as usize] < 0 as i32 {
            error(
                0 as *mut floc,
                (strlen(fifo_name)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const i8,
                    b"cannot open jobserver %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                fifo_name,
                strerror(*__errno_location()),
            );
            return 0 as i32 as u32;
        }
        loop {
            job_fds[1 as i32 as usize] = open(fifo_name, 0o1 as i32);
            if !(job_fds[1 as i32 as usize] == -(1 as i32)
                && *__errno_location() == 4 as i32)
            {
                break;
            }
        }
        if job_fds[1 as i32 as usize] < 0 as i32 {
            error(
                0 as *mut floc,
                (strlen(fifo_name)).wrapping_add(strlen(strerror(*__errno_location()))),
                dcgettext(
                    0 as *const i8,
                    b"cannot open jobserver %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                fifo_name,
                strerror(*__errno_location()),
            );
            return 0 as i32 as u32;
        }
        js_type = js_type::js_fifo;
    } else if sscanf(
        auth,
        b"%d,%d\0" as *const u8 as *const i8,
        &mut rfd as *mut i32,
        &mut wfd as *mut i32,
    ) == 2 as i32
    {
        if rfd == -(2 as i32) || wfd == -(2 as i32) {
            return 0 as i32 as u32;
        }
        if !(fcntl(rfd, 1 as i32) != -(1 as i32))
            || !(fcntl(wfd, 1 as i32) != -(1 as i32))
        {
            return 0 as i32 as u32;
        }
        job_fds[0 as i32 as usize] = rfd;
        job_fds[1 as i32 as usize] = wfd;
        js_type = js_type::js_pipe;
    } else {
        error(
            0 as *mut floc,
            strlen(auth),
            dcgettext(
                0 as *const i8,
                b"invalid --jobserver-auth string '%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            auth,
        );
        return 0 as i32 as u32;
    }
    if make_job_rfd() < 0 as i32 {
        if *__errno_location() != 9 as i32 {
            pfatal_with_name(b"jobserver readfd\0" as *const u8 as *const i8);
        }
        jobserver_clear();
        return 0 as i32 as u32;
    }
    set_blocking(job_fds[0 as i32 as usize], 0 as i32);
    fd_noinherit(job_fds[0 as i32 as usize]);
    fd_noinherit(job_fds[1 as i32 as usize]);
    return 1 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_get_auth() -> *mut i8 {
    let mut auth: *mut i8 = 0 as *mut i8;
    if js_type as u32 == js_type::js_fifo as i32 as u32 {
        auth = xmalloc(
            (strlen(fifo_name))
                .wrapping_add(
                    (::core::mem::size_of::<[i8; 6]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        sprintf(auth, b"fifo:%s\0" as *const u8 as *const i8, fifo_name);
    } else {
        auth = xmalloc(
            (53 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64)
                .wrapping_div(22 as i32 as u64)
                .wrapping_add(3 as i32 as u64)
                .wrapping_mul(2 as i32 as u64)
                .wrapping_add(2 as i32 as u64),
        ) as *mut i8;
        sprintf(
            auth,
            b"%d,%d\0" as *const u8 as *const i8,
            job_fds[0 as i32 as usize],
            job_fds[1 as i32 as usize],
        );
    }
    return auth;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_get_invalid_auth() -> *const i8 {
    if js_type as u32 == js_type::js_fifo as i32 as u32 {
        return 0 as *const i8;
    }
    return b" --jobserver-auth=-2,-2\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_enabled() -> u32 {
    return (js_type as u32 != js_type::js_none as i32 as u32) as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_clear() {
    if job_fds[0 as i32 as usize] >= 0 as i32 {
        close(job_fds[0 as i32 as usize]);
    }
    if job_fds[1 as i32 as usize] >= 0 as i32 {
        close(job_fds[1 as i32 as usize]);
    }
    if job_rfd >= 0 as i32 {
        close(job_rfd);
    }
    job_rfd = -(1 as i32);
    job_fds[1 as i32 as usize] = job_rfd;
    job_fds[0 as i32 as usize] = job_fds[1 as i32 as usize];
    if !fifo_name.is_null() {
        if job_root != 0 {
            let mut r: i32 = 0;
            loop {
                r = unlink(fifo_name);
                if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
        }
        if handling_fatal_signal == 0 {
            free(fifo_name as *mut libc::c_void);
            fifo_name = 0 as *mut i8;
        }
    }
    js_type = js_type::js_none;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_release(mut is_fatal: i32) {
    let mut r: i32 = 0;
    loop {
        r = write(
            job_fds[1 as i32 as usize],
            &mut token as *mut i8 as *const libc::c_void,
            1 as i32 as size_t,
        ) as i32;
        if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if r != 1 as i32 {
        if is_fatal != 0 {
            pfatal_with_name(
                dcgettext(
                    0 as *const i8,
                    b"write jobserver\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        perror_with_name(
            b"write\0" as *const u8 as *const i8,
            b"\0" as *const u8 as *const i8,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_acquire_all() -> u32 {
    let mut r: i32 = 0;
    let mut tokens: u32 = 0 as i32 as u32;
    set_blocking(job_fds[0 as i32 as usize], 1 as i32);
    close(job_fds[1 as i32 as usize]);
    job_fds[1 as i32 as usize] = -(1 as i32);
    loop {
        let mut intake: i8 = 0;
        loop {
            r = read(
                job_fds[0 as i32 as usize],
                &mut intake as *mut i8 as *mut libc::c_void,
                1 as i32 as size_t,
            ) as i32;
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r != 1 as i32 {
            break;
        }
        tokens = tokens.wrapping_add(1);
        tokens;
    }
    if 0x4 as i32 & db_level != 0 {
        printf(
            b"Acquired all %u jobserver tokens.\n\0" as *const u8 as *const i8,
            tokens,
        );
        fflush(stdout);
    }
    jobserver_clear();
    return tokens;
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_pre_child(mut recursive: i32) {
    if recursive != 0 && js_type as u32 == js_type::js_pipe as i32 as u32 {
        fd_inherit(job_fds[0 as i32 as usize]);
        fd_inherit(job_fds[1 as i32 as usize]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_post_child(mut recursive: i32) {
    if recursive != 0 && js_type as u32 == js_type::js_pipe as i32 as u32 {
        fd_noinherit(job_fds[0 as i32 as usize]);
        fd_noinherit(job_fds[1 as i32 as usize]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_signal() {
    if job_rfd >= 0 as i32 {
        close(job_rfd);
        job_rfd = -(1 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_pre_acquire() {
    if job_rfd < 0 as i32 && job_fds[0 as i32 as usize] >= 0 as i32
        && make_job_rfd() < 0 as i32
    {
        pfatal_with_name(
            dcgettext(
                0 as *const i8,
                b"duping jobs pipe\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn jobserver_acquire(mut timeout: i32) -> u32 {
    let mut spec: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut specp: *mut timespec = 0 as *mut timespec;
    let mut empty: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut empty);
    if timeout != 0 {
        spec.tv_sec = 1 as i32 as __time_t;
        spec.tv_nsec = 0 as i32 as __syscall_slong_t;
        specp = &mut spec;
    }
    loop {
        let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
        let mut r: i32 = 0;
        let mut intake: i8 = 0;
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh1 = &mut __d0;
        let fresh2;
        let fresh3 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh4 = &mut __d1;
        let fresh5;
        let fresh6 = &mut *(readfds.fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh1,
            fresh3) => fresh2, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh4,
            fresh6) => fresh5, inlateout("ax") 0 as libc::c_int => _,
            options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh1, fresh3, fresh2);
        c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
        readfds
            .fds_bits[(job_fds[0 as i32 as usize]
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << job_fds[0 as i32 as usize]
                    % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        r = pselect(
            job_fds[0 as i32 as usize] + 1 as i32,
            &mut readfds,
            0 as *mut fd_set,
            0 as *mut fd_set,
            specp,
            &mut empty,
        );
        if r < 0 as i32 {
            match *__errno_location() {
                4 => return 0 as i32 as u32,
                9 => {
                    fatal(
                        0 as *mut floc,
                        0 as i32 as size_t,
                        dcgettext(
                            0 as *const i8,
                            b"job server shut down\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
                _ => {
                    pfatal_with_name(
                        dcgettext(
                            0 as *const i8,
                            b"pselect jobs pipe\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
        }
        if r == 0 as i32 {
            return 0 as i32 as u32;
        }
        loop {
            r = read(
                job_fds[0 as i32 as usize],
                &mut intake as *mut i8 as *mut libc::c_void,
                1 as i32 as size_t,
            ) as i32;
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if r < 0 as i32 {
            if *__errno_location() == 11 as i32 {
                continue;
            }
            pfatal_with_name(
                dcgettext(
                    0 as *const i8,
                    b"read jobs pipe\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else {
            return (r > 0 as i32) as i32 as u32
        }
    };
}
static mut osync_handle: i32 = -(1 as i32);
static mut osync_tmpfile: *mut i8 = 0 as *const i8 as *mut i8;
static mut sync_root: u32 = 0 as i32 as u32;
#[no_mangle]
pub unsafe extern "C" fn osync_enabled() -> u32 {
    return (osync_handle >= 0 as i32) as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn osync_setup() {
    osync_handle = get_tmpfd(&mut osync_tmpfile);
    fd_noinherit(osync_handle);
    sync_root = 1 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn osync_get_mutex() -> *mut i8 {
    let mut mutex: *mut i8 = 0 as *mut i8;
    if osync_enabled() != 0 {
        mutex = xmalloc(
            (strlen(osync_tmpfile))
                .wrapping_add(
                    (::core::mem::size_of::<[i8; 5]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                )
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        sprintf(mutex, b"fnm:%s\0" as *const u8 as *const i8, osync_tmpfile);
    }
    return mutex;
}
#[no_mangle]
pub unsafe extern "C" fn osync_parse_mutex(mut mutex: *const i8) -> u32 {
    if strncmp(
        mutex,
        b"fnm:\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64),
    ) != 0 as i32
    {
        error(
            0 as *mut floc,
            strlen(mutex),
            dcgettext(
                0 as *const i8,
                b"invalid --sync-mutex string '%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            mutex,
        );
        return 0 as i32 as u32;
    }
    free(osync_tmpfile as *mut libc::c_void);
    osync_tmpfile = xstrdup(
        mutex
            .offset(
                (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                    as isize,
            ),
    );
    loop {
        osync_handle = open(osync_tmpfile, 0o1 as i32);
        if !(osync_handle == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if osync_handle < 0 as i32 {
        fatal(
            0 as *mut floc,
            (strlen(osync_tmpfile)).wrapping_add(strlen(strerror(*__errno_location()))),
            dcgettext(
                0 as *const i8,
                b"cannot open output sync mutex %s: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            osync_tmpfile,
            strerror(*__errno_location()),
        );
    }
    fd_noinherit(osync_handle);
    return 1 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn osync_clear() {
    if osync_handle >= 0 as i32 {
        close(osync_handle);
        osync_handle = -(1 as i32);
    }
    if sync_root != 0 && !osync_tmpfile.is_null() {
        let mut r: i32 = 0;
        loop {
            r = unlink(osync_tmpfile);
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        free(osync_tmpfile as *mut libc::c_void);
        osync_tmpfile = 0 as *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn osync_acquire() -> u32 {
    if osync_enabled() != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        fl.l_type = 1 as i32 as libc::c_short;
        fl.l_whence = 0 as i32 as libc::c_short;
        fl.l_start = 0 as i32 as __off_t;
        fl.l_len = 1 as i32 as __off_t;
        if fcntl(osync_handle, 7 as i32, &mut fl as *mut flock) == -(1 as i32) {
            perror(b"fcntl()\0" as *const u8 as *const i8);
            return 0 as i32 as u32;
        }
    }
    return 1 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn osync_release() {
    if osync_enabled() != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        fl.l_type = 2 as i32 as libc::c_short;
        fl.l_whence = 0 as i32 as libc::c_short;
        fl.l_start = 0 as i32 as __off_t;
        fl.l_len = 1 as i32 as __off_t;
        if fcntl(osync_handle, 7 as i32, &mut fl as *mut flock) == -(1 as i32) {
            perror(b"fcntl()\0" as *const u8 as *const i8);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_bad_stdin() -> i32 {
    static mut bad_stdin: i32 = -(1 as i32);
    if bad_stdin == -(1 as i32) {
        let mut pd: [i32; 2] = [0; 2];
        if pipe(pd.as_mut_ptr()) == 0 as i32 {
            close(pd[1 as i32 as usize]);
            bad_stdin = pd[0 as i32 as usize];
            fd_noinherit(bad_stdin);
        }
    }
    return bad_stdin;
}
#[no_mangle]
pub unsafe extern "C" fn fd_inherit(mut fd: i32) {
    let mut flags: i32 = 0;
    loop {
        flags = fcntl(fd, 1 as i32);
        if !(flags == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if flags >= 0 as i32 {
        let mut r: i32 = 0;
        flags &= !(1 as i32);
        loop {
            r = fcntl(fd, 2 as i32, flags);
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fd_noinherit(mut fd: i32) {
    let mut flags: i32 = 0;
    loop {
        flags = fcntl(fd, 1 as i32);
        if !(flags == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if flags >= 0 as i32 {
        let mut r: i32 = 0;
        flags |= 1 as i32;
        loop {
            r = fcntl(fd, 2 as i32, flags);
            if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fd_set_append(mut fd: i32) {
    let mut stbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut flags: i32 = 0;
    if fstat(fd, &mut stbuf) == 0 as i32
        && stbuf.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32
    {
        flags = fcntl(fd, 3 as i32, 0 as i32);
        if flags >= 0 as i32 {
            let mut r: i32 = 0;
            loop {
                r = fcntl(fd, 4 as i32, flags | 0o2000 as i32);
                if !(r == -(1 as i32) && *__errno_location() == 4 as i32) {
                    break;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn os_anontmp() -> i32 {
    let mut tdir: *const i8 = get_tmpdir();
    let mut fd: i32 = -(1 as i32);
    static mut tmpfile_works: u32 = 1 as i32 as u32;
    if tmpfile_works != 0 {
        loop {
            fd = open(
                tdir,
                0o2 as i32 | (0o20000000 as i32 | 0o200000 as i32) | 0o200 as i32,
                0o600 as i32,
            );
            if !(fd == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if fd >= 0 as i32 {
            return fd;
        }
        if 0x1 as i32 & db_level != 0 {
            printf(
                dcgettext(
                    0 as *const i8,
                    b"Cannot open '%s' with O_TMPFILE: %s.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                tdir,
                strerror(*__errno_location()),
            );
            fflush(stdout);
        }
        tmpfile_works = 0 as i32 as u32;
    }
    if tdir == b"/tmp\0" as *const u8 as *const i8
        || *tdir as i32 == *(b"/tmp\0" as *const u8 as *const i8) as i32
            && (*tdir as i32 == '\0' as i32
                || strcmp(
                    tdir.offset(1 as i32 as isize),
                    (b"/tmp\0" as *const u8 as *const i8).offset(1 as i32 as isize),
                ) == 0)
    {
        let mut mask: mode_t = umask(0o77 as i32 as __mode_t);
        let mut tfile: *mut FILE = 0 as *mut FILE;
        loop {
            *__errno_location() = 0 as i32;
            tfile = tmpfile();
            if !(tfile.is_null() && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if tfile.is_null() {
            error(
                0 as *mut floc,
                strlen(strerror(*__errno_location())),
                b"tmpfile: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
            return -(1 as i32);
        }
        umask(mask);
        loop {
            fd = dup(fileno(tfile));
            if !(fd == -(1 as i32) && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if fd < 0 as i32 {
            error(
                0 as *mut floc,
                strlen(strerror(*__errno_location())),
                b"dup: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        fclose(tfile);
    }
    return fd;
}