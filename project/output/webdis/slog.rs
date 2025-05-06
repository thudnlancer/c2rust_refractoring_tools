#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type event_base;
    pub type worker;
    pub type acl;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut i8,
        __maxsize: size_t,
        __format: *const i8,
        __tp: *const tm,
    ) -> size_t;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn close(__fd: i32) -> i32;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn fsync(__fd: i32) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn event_add(ev: *mut event, timeout: *const timeval) -> i32;
    fn event_new(
        _: *mut event_base,
        _: i32,
        _: libc::c_short,
        _: event_callback_fn,
        _: *mut libc::c_void,
    ) -> *mut event;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub __tm_gmtoff: i64,
    pub __tm_zone: *const i8,
}
pub type pid_t = __pid_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_level {
    WEBDIS_ERROR = 0,
    WEBDIS_WARNING,
    WEBDIS_NOTICE,
    WEBDIS_INFO,
    WEBDIS_DEBUG,
    WEBDIS_TRACE = 8,
}
impl log_level {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_level::WEBDIS_ERROR => 0,
            log_level::WEBDIS_WARNING => 1,
            log_level::WEBDIS_NOTICE => 2,
            log_level::WEBDIS_INFO => 3,
            log_level::WEBDIS_DEBUG => 4,
            log_level::WEBDIS_TRACE => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_level {
        match value {
            0 => log_level::WEBDIS_ERROR,
            1 => log_level::WEBDIS_WARNING,
            2 => log_level::WEBDIS_NOTICE,
            3 => log_level::WEBDIS_INFO,
            4 => log_level::WEBDIS_DEBUG,
            8 => log_level::WEBDIS_TRACE,
            _ => panic!("Invalid value for log_level: {}", value),
        }
    }
}
impl AddAssign<u32> for log_level {
    fn add_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for log_level {
    fn sub_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for log_level {
    fn mul_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for log_level {
    fn div_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for log_level {
    fn rem_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for log_level {
    type Output = log_level;
    fn add(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for log_level {
    type Output = log_level;
    fn sub(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for log_level {
    type Output = log_level;
    fn mul(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for log_level {
    type Output = log_level;
    fn div(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for log_level {
    type Output = log_level;
    fn rem(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum log_fsync_mode {
    LOG_FSYNC_AUTO = 0,
    LOG_FSYNC_MILLIS,
    LOG_FSYNC_ALL,
}
impl log_fsync_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            log_fsync_mode::LOG_FSYNC_AUTO => 0,
            log_fsync_mode::LOG_FSYNC_MILLIS => 1,
            log_fsync_mode::LOG_FSYNC_ALL => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> log_fsync_mode {
        match value {
            0 => log_fsync_mode::LOG_FSYNC_AUTO,
            1 => log_fsync_mode::LOG_FSYNC_MILLIS,
            2 => log_fsync_mode::LOG_FSYNC_ALL,
            _ => panic!("Invalid value for log_fsync_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for log_fsync_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for log_fsync_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for log_fsync_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for log_fsync_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for log_fsync_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn add(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn sub(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn mul(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn div(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn rem(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub fd: i32,
    pub ev: event,
    pub base: *mut event_base,
    pub cfg: *mut conf,
    pub w: *mut *mut worker,
    pub next_worker: i32,
    pub log: C2RustUnnamed,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub self_0: pid_t,
    pub fd: i32,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: i32,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub evcb_callback: Option<
        unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
    >,
    pub evcb_selfcb: Option<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
    pub evcb_evfinalize: Option<
        unsafe extern "C" fn(*mut event, *mut libc::c_void) -> (),
    >,
    pub evcb_cbfinalize: Option<
        unsafe extern "C" fn(*mut event_callback, *mut libc::c_void) -> (),
    >,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf {
    pub redis_host: *mut i8,
    pub redis_port: i32,
    pub redis_auth: *mut auth,
    pub http_host: *mut i8,
    pub http_port: i32,
    pub http_threads: i32,
    pub http_max_request_size: size_t,
    pub pool_size_per_thread: i32,
    pub daemonize: i32,
    pub pidfile: *mut i8,
    pub websockets: i32,
    pub database: i32,
    pub perms: *mut acl,
    pub user: uid_t,
    pub group: gid_t,
    pub logfile: *mut i8,
    pub verbosity: log_level,
    pub log_fsync: C2RustUnnamed_10,
    pub hiredis_opts: C2RustUnnamed_9,
    pub default_root: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub keep_alive_sec: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub mode: log_fsync_mode,
    pub period_millis: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: i32,
    pub username: *mut i8,
    pub password: *mut i8,
}
pub type event_callback_fn = Option<
    unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
>;
#[no_mangle]
pub unsafe extern "C" fn slog_init(mut s: *mut server) {
    (*s).log.self_0 = getpid();
    if !((*(*s).cfg).logfile).is_null() {
        let mut old_fd: i32 = (*s).log.fd;
        (*s).log.fd = open(
            (*(*s).cfg).logfile,
            0o1 as i32 | 0o2000 as i32 | 0o100 as i32,
            0o400 as i32 | 0o200 as i32,
        );
        if old_fd != -(1 as i32) {
            close(old_fd);
        }
        if (*s).log.fd != -(1 as i32) {
            return;
        }
        fprintf(
            stderr,
            b"Could not open %s: %s\n\0" as *const u8 as *const i8,
            (*(*s).cfg).logfile,
            strerror(*__errno_location()),
        );
    }
    (*s).log.fd = 2 as i32;
}
unsafe extern "C" fn slog_fsync_tick(
    mut fd: i32,
    mut event: libc::c_short,
    mut data: *mut libc::c_void,
) {
    let mut s: *mut server = data as *mut server;
    let mut ret: i32 = fsync((*s).log.fd);
}
#[no_mangle]
pub unsafe extern "C" fn slog_fsync_init(mut s: *mut server) {
    if (*(*s).cfg).log_fsync.mode as u32
        != log_fsync_mode::LOG_FSYNC_MILLIS as i32 as u32
    {
        return;
    }
    (*s).log.fsync_ev = event_new(
        (*s).base,
        -(1 as i32),
        0x10 as i32 as libc::c_short,
        Some(
            slog_fsync_tick
                as unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
        ),
        s as *mut libc::c_void,
    );
    if ((*s).log.fsync_ev).is_null() {
        let evnew_error: [i8; 33] = *::core::mem::transmute::<
            &[u8; 33],
            &[i8; 33],
        >(b"fsync timer could not be created\0");
        slog(
            s,
            log_level::WEBDIS_ERROR,
            evnew_error.as_ptr(),
            (::core::mem::size_of::<[i8; 33]>() as u64).wrapping_sub(1 as i32 as u64),
        );
        return;
    }
    let mut period_usec: i32 = (*(*s).cfg).log_fsync.period_millis * 1000 as i32;
    (*s).log.fsync_tv.tv_sec = (period_usec / 1000000 as i32) as __time_t;
    (*s).log.fsync_tv.tv_usec = (period_usec % 1000000 as i32) as __suseconds_t;
    let mut ret_ta: i32 = event_add((*s).log.fsync_ev, &mut (*s).log.fsync_tv);
    if ret_ta != 0 as i32 {
        let reason: [i8; 35] = *::core::mem::transmute::<
            &[u8; 35],
            &[i8; 35],
        >(b"fsync timer could not be added: %d\0");
        let mut error_msg: [i8; 51] = [0; 51];
        let mut error_len: i32 = snprintf(
            error_msg.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 51]>() as u64,
            reason.as_ptr(),
            ret_ta,
        );
        slog(s, log_level::WEBDIS_ERROR, error_msg.as_mut_ptr(), error_len as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn slog_enabled(mut s: *mut server, mut level: log_level) -> i32 {
    return if level as u32 <= (*(*s).cfg).verbosity as u32 {
        1 as i32
    } else {
        0 as i32
    };
}
unsafe extern "C" fn slog_internal(
    mut s: *mut server,
    mut level: log_level,
    mut body: *const i8,
    mut sz: size_t,
) {
    let mut c: *const i8 = b"EWNIDT\0" as *const u8 as *const i8;
    let mut now: time_t = 0;
    let mut now_tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const i8,
    };
    let mut lt_ret: *mut tm = 0 as *mut tm;
    let mut time_buf: [i8; 64] = [0; 64];
    let mut msg: [i8; 125] = [0; 125];
    let mut line: [i8; 248] = [0; 248];
    let mut line_sz: i32 = 0;
    let mut ret: i32 = 0;
    if (*s).log.fd == 0 {
        return;
    }
    sz = if sz != 0 { sz } else { strlen(body) };
    snprintf(
        msg.as_mut_ptr(),
        if sz.wrapping_add(1 as i32 as u64) > ::core::mem::size_of::<[i8; 125]>() as u64
        {
            ::core::mem::size_of::<[i8; 125]>() as u64
        } else {
            sz.wrapping_add(1 as i32 as u64)
        },
        b"%s\0" as *const u8 as *const i8,
        body,
    );
    now = time(0 as *mut time_t);
    lt_ret = localtime_r(&mut now, &mut now_tm);
    if !lt_ret.is_null() {
        strftime(
            time_buf.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 64]>() as u64,
            b"%d %b %H:%M:%S\0" as *const u8 as *const i8,
            lt_ret,
        );
    } else {
        let err_msg: [i8; 20] = *::core::mem::transmute::<
            &[u8; 20],
            &[i8; 20],
        >(b"(NO TIME AVAILABLE)\0");
        memcpy(
            time_buf.as_mut_ptr() as *mut libc::c_void,
            err_msg.as_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[i8; 20]>() as u64,
        );
    }
    let mut letter: i8 = (if level as u32 == log_level::WEBDIS_TRACE as i32 as u32 {
        *c.offset(5 as i32 as isize) as i32
    } else {
        *c.offset(level as isize) as i32
    }) as i8;
    line_sz = snprintf(
        line.as_mut_ptr(),
        ::core::mem::size_of::<[i8; 248]>() as u64,
        b"[%d] %s %c %s\n\0" as *const u8 as *const i8,
        (*s).log.self_0,
        time_buf.as_mut_ptr(),
        letter as i32,
        msg.as_mut_ptr(),
    );
    ret = write((*s).log.fd, line.as_mut_ptr() as *const libc::c_void, line_sz as size_t)
        as i32;
    if (*(*s).cfg).log_fsync.mode as u32 == log_fsync_mode::LOG_FSYNC_ALL as i32 as u32 {
        ret = fsync((*s).log.fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn slog(
    mut s: *mut server,
    mut level: log_level,
    mut body: *const i8,
    mut sz: size_t,
) {
    if level as u32 <= (*(*s).cfg).verbosity as u32 {
        slog_internal(s, level, body, sz);
    }
}