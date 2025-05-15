use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type acl;
    pub type event_base;
    pub type evbuffer;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> i32;
    fn event_base_new() -> *mut event_base;
    fn event_base_dispatch(_: *mut event_base) -> i32;
    fn event_base_set(_: *mut event_base, _: *mut event) -> i32;
    fn event_add(ev: *mut event, timeout: *const timeval) -> i32;
    fn http_client_free(c: *mut http_client);
    fn free(__ptr: *mut libc::c_void);
    fn ws_process_read_data(ws: *mut ws_client, out_processed: *mut u32) -> ws_state;
    fn ws_close_if_able(ws: *mut ws_client);
    fn ws_handshake_reply(ws: *mut ws_client) -> i32;
    fn ws_client_new(http_client: *mut http_client) -> *mut ws_client;
    fn http_client_execute(c: *mut http_client) -> i32;
    fn http_client_read(c: *mut http_client) -> i32;
    fn event_set(
        _: *mut event,
        _: i32,
        _: libc::c_short,
        _: Option<unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    );
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn http_crossdomain(c: *mut http_client);
    fn http_send_error(c: *mut http_client, code: libc::c_short, msg: *const i8);
    fn http_send_options(c: *mut http_client);
    fn cmd_run(
        w: *mut worker,
        client: *mut http_client,
        uri: *const i8,
        uri_len: size_t,
        body: *const i8,
        body_len: size_t,
    ) -> cmd_response_t;
    fn pool_new(w: *mut worker, count: i32) -> *mut pool;
    fn pool_connect(p: *mut pool, db_num: i32, attach: i32) -> *mut redisAsyncContext;
    fn slog(s: *mut server, level: log_level, body: *const i8, sz: size_t);
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
pub type size_t = u64;
pub type pid_t = __pid_t;
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
pub type pthread_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_client {
    pub fd: i32,
    pub addr: in_addr_t,
    pub ev: event,
    pub w: *mut worker,
    pub s: *mut server,
    pub parser: http_parser,
    pub settings: http_parser_settings,
    pub buffer: *mut i8,
    pub sz: size_t,
    pub request_sz: size_t,
    pub last_cb: last_cb_t,
    pub keep_alive: i8,
    pub broken: i8,
    pub fully_read: i8,
    pub is_websocket: i8,
    pub http_version: i8,
    pub failed_alloc: i8,
    pub path: *mut i8,
    pub path_sz: size_t,
    pub headers: *mut http_header,
    pub header_count: i32,
    pub body: *mut i8,
    pub body_sz: size_t,
    pub type_0: *mut i8,
    pub jsonp: *mut i8,
    pub separator: *mut i8,
    pub filename: *mut i8,
    pub reused_cmd: *mut cmd,
    pub last_cmd: *mut cmd,
    pub ws: *mut ws_client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_client {
    pub http_client: *mut http_client,
    pub scheduled_read: i32,
    pub scheduled_write: i32,
    pub rbuf: *mut evbuffer,
    pub wbuf: *mut evbuffer,
    pub ac: *mut redisAsyncContext,
    pub cmd: *mut cmd,
    pub close_after_events: i32,
    pub ran_subscribe: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd {
    pub fd: i32,
    pub count: i32,
    pub argv: *mut *mut i8,
    pub argv_len: *mut size_t,
    pub mime: *mut i8,
    pub mime_free: i32,
    pub filename: *mut i8,
    pub if_none_match: *mut i8,
    pub jsonp: *mut i8,
    pub separator: *mut i8,
    pub keep_alive: i32,
    pub started_responding: i32,
    pub is_websocket: i32,
    pub http_version: i32,
    pub database: i32,
    pub http_client: *mut http_client,
    pub pub_sub_client: *mut http_client,
    pub ac: *mut redisAsyncContext,
    pub w: *mut worker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker {
    pub thread: pthread_t,
    pub base: *mut event_base,
    pub s: *mut server,
    pub link: [i32; 2],
    pub pool: *mut pool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool {
    pub w: *mut worker,
    pub cfg: *mut conf,
    pub ac: *mut *const redisAsyncContext,
    pub count: i32,
    pub cur: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisAsyncContext {
    pub c: redisContext,
    pub err: i32,
    pub errstr: *mut i8,
    pub data: *mut libc::c_void,
    pub dataCleanup: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ev: C2RustUnnamed_0,
    pub onDisconnect: Option<redisDisconnectCallback>,
    pub onConnect: Option<redisConnectCallback>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed,
    pub push_cb: Option<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub invalid: redisCallbackList,
    pub channels: *mut dict,
    pub patterns: *mut dict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallbackList {
    pub head: *mut redisCallback,
    pub tail: *mut redisCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallback {
    pub next: *mut redisCallback,
    pub fn_0: Option<redisCallbackFn>,
    pub pending_subs: i32,
    pub privdata: *mut libc::c_void,
}
pub type redisCallbackFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
pub type sa_family_t = libc::c_ushort;
pub type redisConnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    i32,
) -> ();
pub type redisDisconnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    i32,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub data: *mut libc::c_void,
    pub addRead: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delRead: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub addWrite: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delWrite: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cleanup: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scheduleTimer: Option<unsafe extern "C" fn(*mut libc::c_void, timeval) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContext {
    pub funcs: *const redisContextFuncs,
    pub err: i32,
    pub errstr: [i8; 128],
    pub fd: redisFD,
    pub flags: i32,
    pub obuf: *mut i8,
    pub reader: *mut redisReader,
    pub connection_type: redisConnectionType,
    pub connect_timeout: *mut timeval,
    pub command_timeout: *mut timeval,
    pub tcp: C2RustUnnamed_2,
    pub unix_sock: C2RustUnnamed_1,
    pub saddr: *mut sockadr,
    pub addrlen: size_t,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub privctx: *mut libc::c_void,
    pub push_cb: Option<redisPushFn>,
}
pub type redisPushFn = unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub path: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub host: *mut i8,
    pub source_addr: *mut i8,
    pub port: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum redisConnectionType {
    REDIS_CONN_TCP,
    REDIS_CONN_UNIX,
    REDIS_CONN_USERFD,
}
impl redisConnectionType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            redisConnectionType::REDIS_CONN_TCP => 0,
            redisConnectionType::REDIS_CONN_UNIX => 1,
            redisConnectionType::REDIS_CONN_USERFD => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> redisConnectionType {
        match value {
            0 => redisConnectionType::REDIS_CONN_TCP,
            1 => redisConnectionType::REDIS_CONN_UNIX,
            2 => redisConnectionType::REDIS_CONN_USERFD,
            _ => panic!("Invalid value for redisConnectionType: {}", value),
        }
    }
}
impl AddAssign<u32> for redisConnectionType {
    fn add_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for redisConnectionType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for redisConnectionType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for redisConnectionType {
    fn div_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for redisConnectionType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn add(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn sub(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn mul(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn div(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn rem(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReader {
    pub err: i32,
    pub errstr: [i8; 128],
    pub buf: *mut i8,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,
    pub maxelements: libc::c_longlong,
    pub task: *mut *mut redisReadTask,
    pub tasks: i32,
    pub ridx: i32,
    pub reply: *mut libc::c_void,
    pub fn_0: *mut redisReplyObjectFunctions,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option<
        unsafe extern "C" fn(*const redisReadTask, *mut i8, size_t) -> *mut libc::c_void,
    >,
    pub createArray: Option<
        unsafe extern "C" fn(*const redisReadTask, size_t) -> *mut libc::c_void,
    >,
    pub createInteger: Option<
        unsafe extern "C" fn(*const redisReadTask, libc::c_longlong) -> *mut libc::c_void,
    >,
    pub createDouble: Option<
        unsafe extern "C" fn(
            *const redisReadTask,
            libc::c_double,
            *mut i8,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createNil: Option<
        unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
    >,
    pub createBool: Option<
        unsafe extern "C" fn(*const redisReadTask, i32) -> *mut libc::c_void,
    >,
    pub freeObject: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReadTask {
    pub type_0: i32,
    pub elements: libc::c_longlong,
    pub idx: i32,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
pub type redisFD = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContextFuncs {
    pub free_privctx: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub async_read: Option<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub async_write: Option<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub read: Option<
        unsafe extern "C" fn(*mut redisContext, *mut i8, size_t) -> ssize_t,
    >,
    pub write: Option<unsafe extern "C" fn(*mut redisContext) -> ssize_t>,
}
pub type ssize_t = __ssize_t;
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
    pub log_fsync: C2RustUnnamed_4,
    pub hiredis_opts: C2RustUnnamed_3,
    pub default_root: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub keep_alive_sec: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub mode: log_fsync_mode,
    pub period_millis: i32,
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: i32,
    pub username: *mut i8,
    pub password: *mut i8,
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
    pub log: C2RustUnnamed_5,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub self_0: pid_t,
    pub fd: i32,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_11,
    pub ev_fd: i32,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_6,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ev_io: C2RustUnnamed_9,
    pub ev_signal: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub ev_signal_next: C2RustUnnamed_8,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub ev_io_next: C2RustUnnamed_10,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ev_next_with_common_timeout: C2RustUnnamed_12,
    pub min_heap_idx: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_14,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_13,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
pub struct C2RustUnnamed_14 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_header {
    pub key: *mut i8,
    pub key_sz: size_t,
    pub val: *mut i8,
    pub val_sz: size_t,
    pub copy: header_copy,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum header_copy {
    HEADER_COPY_NONE = 0,
    HEADER_COPY_KEY = 1,
    HEADER_COPY_VALUE = 2,
    HEADER_CHECK_DUPE = 4,
}
impl header_copy {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            header_copy::HEADER_COPY_NONE => 0,
            header_copy::HEADER_COPY_KEY => 1,
            header_copy::HEADER_COPY_VALUE => 2,
            header_copy::HEADER_CHECK_DUPE => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> header_copy {
        match value {
            0 => header_copy::HEADER_COPY_NONE,
            1 => header_copy::HEADER_COPY_KEY,
            2 => header_copy::HEADER_COPY_VALUE,
            4 => header_copy::HEADER_CHECK_DUPE,
            _ => panic!("Invalid value for header_copy: {}", value),
        }
    }
}
impl AddAssign<u32> for header_copy {
    fn add_assign(&mut self, rhs: u32) {
        *self = header_copy::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for header_copy {
    fn sub_assign(&mut self, rhs: u32) {
        *self = header_copy::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for header_copy {
    fn mul_assign(&mut self, rhs: u32) {
        *self = header_copy::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for header_copy {
    fn div_assign(&mut self, rhs: u32) {
        *self = header_copy::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for header_copy {
    fn rem_assign(&mut self, rhs: u32) {
        *self = header_copy::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for header_copy {
    type Output = header_copy;
    fn add(self, rhs: u32) -> header_copy {
        header_copy::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for header_copy {
    type Output = header_copy;
    fn sub(self, rhs: u32) -> header_copy {
        header_copy::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for header_copy {
    type Output = header_copy;
    fn mul(self, rhs: u32) -> header_copy {
        header_copy::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for header_copy {
    type Output = header_copy;
    fn div(self, rhs: u32) -> header_copy {
        header_copy::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for header_copy {
    type Output = header_copy;
    fn rem(self, rhs: u32) -> header_copy {
        header_copy::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum last_cb_t {
    LAST_CB_NONE = 0,
    LAST_CB_KEY = 1,
    LAST_CB_VAL = 2,
}
impl last_cb_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            last_cb_t::LAST_CB_NONE => 0,
            last_cb_t::LAST_CB_KEY => 1,
            last_cb_t::LAST_CB_VAL => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> last_cb_t {
        match value {
            0 => last_cb_t::LAST_CB_NONE,
            1 => last_cb_t::LAST_CB_KEY,
            2 => last_cb_t::LAST_CB_VAL,
            _ => panic!("Invalid value for last_cb_t: {}", value),
        }
    }
}
impl AddAssign<u32> for last_cb_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for last_cb_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for last_cb_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for last_cb_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for last_cb_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for last_cb_t {
    type Output = last_cb_t;
    fn add(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for last_cb_t {
    type Output = last_cb_t;
    fn sub(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for last_cb_t {
    type Output = last_cb_t;
    fn mul(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for last_cb_t {
    type Output = last_cb_t;
    fn div(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for last_cb_t {
    type Output = last_cb_t;
    fn rem(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_parser_settings {
    pub on_message_begin: http_cb,
    pub on_path: http_data_cb,
    pub on_query_string: http_data_cb,
    pub on_url: http_data_cb,
    pub on_fragment: http_data_cb,
    pub on_header_field: http_data_cb,
    pub on_header_value: http_data_cb,
    pub on_headers_complete: http_cb,
    pub on_body: http_data_cb,
    pub on_message_complete: http_cb,
}
pub type http_cb = Option<unsafe extern "C" fn(*mut http_parser) -> i32>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct http_parser {
    #[bitfield(name = "type_0", ty = "libc::c_uchar", bits = "0..=1")]
    #[bitfield(name = "flags", ty = "libc::c_uchar", bits = "2..=7")]
    pub type_0_flags: [u8; 1],
    pub state: u8,
    pub header_state: u8,
    pub index: u8,
    pub nread: uint32_t,
    pub content_length: int64_t,
    pub http_major: libc::c_ushort,
    pub http_minor: libc::c_ushort,
    pub status_code: libc::c_ushort,
    pub method: u8,
    pub upgrade: i8,
    pub data: *mut libc::c_void,
}
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type http_data_cb = Option<
    unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
>;
pub type in_addr_t = uint32_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ws_state {
    WS_ERROR,
    WS_READING,
    WS_MSG_COMPLETE,
}
impl ws_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ws_state::WS_ERROR => 0,
            ws_state::WS_READING => 1,
            ws_state::WS_MSG_COMPLETE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> ws_state {
        match value {
            0 => ws_state::WS_ERROR,
            1 => ws_state::WS_READING,
            2 => ws_state::WS_MSG_COMPLETE,
            _ => panic!("Invalid value for ws_state: {}", value),
        }
    }
}
impl AddAssign<u32> for ws_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = ws_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ws_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ws_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ws_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ws_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ws_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = ws_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ws_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ws_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ws_state {
    type Output = ws_state;
    fn add(self, rhs: u32) -> ws_state {
        ws_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ws_state {
    type Output = ws_state;
    fn sub(self, rhs: u32) -> ws_state {
        ws_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ws_state {
    type Output = ws_state;
    fn mul(self, rhs: u32) -> ws_state {
        ws_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ws_state {
    type Output = ws_state;
    fn div(self, rhs: u32) -> ws_state {
        ws_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ws_state {
    type Output = ws_state;
    fn rem(self, rhs: u32) -> ws_state {
        ws_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum flags {
    F_CHUNKED = 1,
    F_CONNECTION_KEEP_ALIVE = 2,
    F_CONNECTION_CLOSE = 4,
    F_TRAILING = 8,
    F_UPGRADE = 16,
    F_SKIPBODY = 32,
}
impl flags {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            flags::F_CHUNKED => 1,
            flags::F_CONNECTION_KEEP_ALIVE => 2,
            flags::F_CONNECTION_CLOSE => 4,
            flags::F_TRAILING => 8,
            flags::F_UPGRADE => 16,
            flags::F_SKIPBODY => 32,
        }
    }
    fn from_libc_c_uint(value: u32) -> flags {
        match value {
            1 => flags::F_CHUNKED,
            2 => flags::F_CONNECTION_KEEP_ALIVE,
            4 => flags::F_CONNECTION_CLOSE,
            8 => flags::F_TRAILING,
            16 => flags::F_UPGRADE,
            32 => flags::F_SKIPBODY,
            _ => panic!("Invalid value for flags: {}", value),
        }
    }
}
impl AddAssign<u32> for flags {
    fn add_assign(&mut self, rhs: u32) {
        *self = flags::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for flags {
    fn sub_assign(&mut self, rhs: u32) {
        *self = flags::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for flags {
    fn mul_assign(&mut self, rhs: u32) {
        *self = flags::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for flags {
    fn div_assign(&mut self, rhs: u32) {
        *self = flags::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for flags {
    fn rem_assign(&mut self, rhs: u32) {
        *self = flags::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for flags {
    type Output = flags;
    fn add(self, rhs: u32) -> flags {
        flags::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for flags {
    type Output = flags;
    fn sub(self, rhs: u32) -> flags {
        flags::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for flags {
    type Output = flags;
    fn mul(self, rhs: u32) -> flags {
        flags::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for flags {
    type Output = flags;
    fn div(self, rhs: u32) -> flags {
        flags::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for flags {
    type Output = flags;
    fn rem(self, rhs: u32) -> flags {
        flags::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub const CLIENT_OOM: client_error_t = -2;
pub type client_error_t = i32;
pub const CLIENT_DISCONNECTED: client_error_t = -1;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum cmd_response_t {
    CMD_SENT,
    CMD_PARAM_ERROR,
    CMD_ACL_FAIL,
    CMD_REDIS_UNAVAIL,
}
impl cmd_response_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            cmd_response_t::CMD_SENT => 0,
            cmd_response_t::CMD_PARAM_ERROR => 1,
            cmd_response_t::CMD_ACL_FAIL => 2,
            cmd_response_t::CMD_REDIS_UNAVAIL => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> cmd_response_t {
        match value {
            0 => cmd_response_t::CMD_SENT,
            1 => cmd_response_t::CMD_PARAM_ERROR,
            2 => cmd_response_t::CMD_ACL_FAIL,
            3 => cmd_response_t::CMD_REDIS_UNAVAIL,
            _ => panic!("Invalid value for cmd_response_t: {}", value),
        }
    }
}
impl AddAssign<u32> for cmd_response_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for cmd_response_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for cmd_response_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for cmd_response_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for cmd_response_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for cmd_response_t {
    type Output = cmd_response_t;
    fn add(self, rhs: u32) -> cmd_response_t {
        cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for cmd_response_t {
    type Output = cmd_response_t;
    fn sub(self, rhs: u32) -> cmd_response_t {
        cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for cmd_response_t {
    type Output = cmd_response_t;
    fn mul(self, rhs: u32) -> cmd_response_t {
        cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for cmd_response_t {
    type Output = cmd_response_t;
    fn div(self, rhs: u32) -> cmd_response_t {
        cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for cmd_response_t {
    type Output = cmd_response_t;
    fn rem(self, rhs: u32) -> cmd_response_t {
        cmd_response_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum http_method {
    HTTP_DELETE = 0,
    HTTP_GET,
    HTTP_HEAD,
    HTTP_POST,
    HTTP_PUT,
    HTTP_CONNECT,
    HTTP_OPTIONS,
    HTTP_TRACE,
    HTTP_COPY,
    HTTP_LOCK,
    HTTP_MKCOL,
    HTTP_MOVE,
    HTTP_PROPFIND,
    HTTP_PROPPATCH,
    HTTP_UNLOCK,
    HTTP_REPORT,
    HTTP_MKACTIVITY,
    HTTP_CHECKOUT,
    HTTP_MERGE,
    HTTP_MSEARCH,
    HTTP_NOTIFY,
    HTTP_SUBSCRIBE,
    HTTP_UNSUBSCRIBE,
}
impl http_method {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            http_method::HTTP_DELETE => 0,
            http_method::HTTP_GET => 1,
            http_method::HTTP_HEAD => 2,
            http_method::HTTP_POST => 3,
            http_method::HTTP_PUT => 4,
            http_method::HTTP_CONNECT => 5,
            http_method::HTTP_OPTIONS => 6,
            http_method::HTTP_TRACE => 7,
            http_method::HTTP_COPY => 8,
            http_method::HTTP_LOCK => 9,
            http_method::HTTP_MKCOL => 10,
            http_method::HTTP_MOVE => 11,
            http_method::HTTP_PROPFIND => 12,
            http_method::HTTP_PROPPATCH => 13,
            http_method::HTTP_UNLOCK => 14,
            http_method::HTTP_REPORT => 15,
            http_method::HTTP_MKACTIVITY => 16,
            http_method::HTTP_CHECKOUT => 17,
            http_method::HTTP_MERGE => 18,
            http_method::HTTP_MSEARCH => 19,
            http_method::HTTP_NOTIFY => 20,
            http_method::HTTP_SUBSCRIBE => 21,
            http_method::HTTP_UNSUBSCRIBE => 22,
        }
    }
    fn from_libc_c_uint(value: u32) -> http_method {
        match value {
            0 => http_method::HTTP_DELETE,
            1 => http_method::HTTP_GET,
            2 => http_method::HTTP_HEAD,
            3 => http_method::HTTP_POST,
            4 => http_method::HTTP_PUT,
            5 => http_method::HTTP_CONNECT,
            6 => http_method::HTTP_OPTIONS,
            7 => http_method::HTTP_TRACE,
            8 => http_method::HTTP_COPY,
            9 => http_method::HTTP_LOCK,
            10 => http_method::HTTP_MKCOL,
            11 => http_method::HTTP_MOVE,
            12 => http_method::HTTP_PROPFIND,
            13 => http_method::HTTP_PROPPATCH,
            14 => http_method::HTTP_UNLOCK,
            15 => http_method::HTTP_REPORT,
            16 => http_method::HTTP_MKACTIVITY,
            17 => http_method::HTTP_CHECKOUT,
            18 => http_method::HTTP_MERGE,
            19 => http_method::HTTP_MSEARCH,
            20 => http_method::HTTP_NOTIFY,
            21 => http_method::HTTP_SUBSCRIBE,
            22 => http_method::HTTP_UNSUBSCRIBE,
            _ => panic!("Invalid value for http_method: {}", value),
        }
    }
}
impl AddAssign<u32> for http_method {
    fn add_assign(&mut self, rhs: u32) {
        *self = http_method::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for http_method {
    fn sub_assign(&mut self, rhs: u32) {
        *self = http_method::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for http_method {
    fn mul_assign(&mut self, rhs: u32) {
        *self = http_method::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for http_method {
    fn div_assign(&mut self, rhs: u32) {
        *self = http_method::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for http_method {
    fn rem_assign(&mut self, rhs: u32) {
        *self = http_method::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for http_method {
    type Output = http_method;
    fn add(self, rhs: u32) -> http_method {
        http_method::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for http_method {
    type Output = http_method;
    fn sub(self, rhs: u32) -> http_method {
        http_method::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for http_method {
    type Output = http_method;
    fn mul(self, rhs: u32) -> http_method {
        http_method::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for http_method {
    type Output = http_method;
    fn div(self, rhs: u32) -> http_method {
        http_method::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for http_method {
    type Output = http_method;
    fn rem(self, rhs: u32) -> http_method {
        http_method::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub unsafe extern "C" fn worker_new(mut s: *mut server) -> *mut worker {
    let mut ret: i32 = 0;
    let mut w: *mut worker = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<worker>() as u64,
    ) as *mut worker;
    (*w).s = s;
    ret = pipe(((*w).link).as_mut_ptr());
    (*w).pool = pool_new(w, (*(*s).cfg).pool_size_per_thread);
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn worker_can_read(
    mut fd: i32,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut c: *mut http_client = p as *mut http_client;
    let mut ret: i32 = 0;
    let mut nparsed: i32 = 0;
    ret = http_client_read(c);
    if ret <= 0 as i32 {
        if ret as client_error_t as i32 == CLIENT_DISCONNECTED as i32 {
            return
        } else if (*c).failed_alloc as i32 != 0
            || ret as client_error_t as i32 == CLIENT_OOM as i32
        {
            slog(
                (*(*c).w).s,
                log_level::WEBDIS_DEBUG,
                b"503\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            );
            http_send_error(
                c,
                503 as i32 as libc::c_short,
                b"Service Unavailable\0" as *const u8 as *const i8,
            );
            return;
        }
    }
    if (*c).is_websocket == 0 {
        nparsed = http_client_execute(c);
        if (*c).failed_alloc != 0 {
            slog(
                (*(*c).w).s,
                log_level::WEBDIS_DEBUG,
                b"503\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            );
            http_send_error(
                c,
                503 as i32 as libc::c_short,
                b"Service Unavailable\0" as *const u8 as *const i8,
            );
        } else if ((*c).parser).flags() as i32 & flags::F_CONNECTION_CLOSE as i32 != 0
            && (*c).fully_read as i32 != 0
        {
            (*c).broken = 1 as i32 as i8;
        } else if (*c).is_websocket != 0 {
            (*c).ws = ws_client_new(c);
            if ((*c).ws).is_null() {
                (*c).broken = 1 as i32 as i8;
            } else {
                free((*c).buffer as *mut libc::c_void);
                (*c).buffer = 0 as *mut i8;
                (*c).sz = 0 as i32 as size_t;
                let mut reply_ret: i32 = ws_handshake_reply((*c).ws);
                if reply_ret < 0 as i32 {
                    (*(*c).ws).http_client = 0 as *mut http_client;
                    ws_close_if_able((*c).ws);
                    (*c).broken = 1 as i32 as i8;
                } else {
                    let mut processed: u32 = 0 as i32 as u32;
                    let mut process_ret: i32 = ws_process_read_data(
                        (*c).ws,
                        &mut processed,
                    ) as i32;
                    if process_ret == ws_state::WS_ERROR as i32 {
                        (*c).broken = 1 as i32 as i8;
                    }
                }
            }
            free((*c).buffer as *mut libc::c_void);
            (*c).buffer = 0 as *mut i8;
            (*c).sz = 0 as i32 as size_t;
        } else if nparsed != ret {
            slog(
                (*(*c).w).s,
                log_level::WEBDIS_DEBUG,
                b"400\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            );
            http_send_error(
                c,
                400 as i32 as libc::c_short,
                b"Bad Request\0" as *const u8 as *const i8,
            );
        } else if (*c).request_sz > (*(*(*c).s).cfg).http_max_request_size {
            slog(
                (*(*c).w).s,
                log_level::WEBDIS_DEBUG,
                b"413\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            );
            http_send_error(
                c,
                413 as i32 as libc::c_short,
                b"Request Entity Too Large\0" as *const u8 as *const i8,
            );
        }
    }
    if (*c).broken != 0 {
        if (*c).is_websocket != 0 {
            close((*c).fd);
        }
        http_client_free(c);
    } else if (*c).is_websocket == 0 {
        worker_monitor_input(c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn worker_monitor_input(mut c: *mut http_client) {
    event_set(
        &mut (*c).ev,
        (*c).fd,
        0x2 as i32 as libc::c_short,
        Some(
            worker_can_read
                as unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
        ),
        c as *mut libc::c_void,
    );
    event_base_set((*(*c).w).base, &mut (*c).ev);
    event_add(&mut (*c).ev, 0 as *const timeval);
}
unsafe extern "C" fn worker_on_new_client(
    mut pipefd: i32,
    mut event: libc::c_short,
    mut ptr: *mut libc::c_void,
) {
    let mut c: *mut http_client = 0 as *mut http_client;
    let mut addr: u64 = 0;
    let mut ret: i32 = read(
        pipefd,
        &mut addr as *mut u64 as *mut libc::c_void,
        ::core::mem::size_of::<u64>() as u64,
    ) as i32;
    if ret as u64 == ::core::mem::size_of::<u64>() as u64 {
        c = addr as *mut http_client;
        worker_monitor_input(c);
    }
}
unsafe extern "C" fn worker_pool_connect(mut w: *mut worker) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*(*w).pool).count {
        pool_connect((*w).pool, (*(*(*w).s).cfg).database, 1 as i32);
        i += 1;
        i;
    }
}
unsafe extern "C" fn worker_main(mut p: *mut libc::c_void) -> *mut libc::c_void {
    let mut w: *mut worker = p as *mut worker;
    let mut ev: event = event {
        ev_evcallback: event_callback {
            evcb_active_next: C2RustUnnamed_14 {
                tqe_next: 0 as *mut event_callback,
                tqe_prev: 0 as *mut *mut event_callback,
            },
            evcb_flags: 0,
            evcb_pri: 0,
            evcb_closure: 0,
            evcb_cb_union: C2RustUnnamed_13 {
                evcb_callback: None,
            },
            evcb_arg: 0 as *mut libc::c_void,
        },
        ev_timeout_pos: C2RustUnnamed_11 {
            ev_next_with_common_timeout: C2RustUnnamed_12 {
                tqe_next: 0 as *mut event,
                tqe_prev: 0 as *mut *mut event,
            },
        },
        ev_fd: 0,
        ev_base: 0 as *mut event_base,
        ev_: C2RustUnnamed_6 {
            ev_io: C2RustUnnamed_9 {
                ev_io_next: C2RustUnnamed_10 {
                    le_next: 0 as *mut event,
                    le_prev: 0 as *mut *mut event,
                },
                ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
            },
        },
        ev_events: 0,
        ev_res: 0,
        ev_timeout: timeval { tv_sec: 0, tv_usec: 0 },
    };
    (*w).base = event_base_new();
    event_set(
        &mut ev,
        (*w).link[0 as i32 as usize],
        (0x2 as i32 | 0x10 as i32) as libc::c_short,
        Some(
            worker_on_new_client
                as unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
        ),
        w as *mut libc::c_void,
    );
    event_base_set((*w).base, &mut ev);
    event_add(&mut ev, 0 as *const timeval);
    worker_pool_connect(w);
    event_base_dispatch((*w).base);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn worker_start(mut w: *mut worker) {
    pthread_create(
        &mut (*w).thread,
        0 as *const pthread_attr_t,
        Some(
            worker_main as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        w as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn worker_add_client(mut w: *mut worker, mut c: *mut http_client) {
    let mut addr: u64 = c as u64;
    let mut ret: i32 = write(
        (*w).link[1 as i32 as usize],
        &mut addr as *mut u64 as *const libc::c_void,
        ::core::mem::size_of::<u64>() as u64,
    ) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn worker_process_client(mut c: *mut http_client) {
    let mut w: *mut worker = (*c).w;
    let mut ret: cmd_response_t = cmd_response_t::CMD_PARAM_ERROR;
    match (*c).parser.method as i32 {
        1 => {
            if (*c).path_sz == 16 as i32 as u64
                && memcmp(
                    (*c).path as *const libc::c_void,
                    b"/crossdomain.xml\0" as *const u8 as *const i8
                        as *const libc::c_void,
                    16 as i32 as u64,
                ) == 0 as i32
            {
                http_crossdomain(c);
                return;
            }
            slog((*w).s, log_level::WEBDIS_DEBUG, (*c).path, (*c).path_sz);
            ret = cmd_run(
                (*c).w,
                c,
                ((*c).path).offset(1 as i32 as isize),
                ((*c).path_sz).wrapping_sub(1 as i32 as u64),
                0 as *const i8,
                0 as i32 as size_t,
            );
        }
        3 => {
            slog((*w).s, log_level::WEBDIS_DEBUG, (*c).path, (*c).path_sz);
            ret = cmd_run(
                (*c).w,
                c,
                (*c).body,
                (*c).body_sz,
                0 as *const i8,
                0 as i32 as size_t,
            );
        }
        4 => {
            slog((*w).s, log_level::WEBDIS_DEBUG, (*c).path, (*c).path_sz);
            ret = cmd_run(
                (*c).w,
                c,
                ((*c).path).offset(1 as i32 as isize),
                ((*c).path_sz).wrapping_sub(1 as i32 as u64),
                (*c).body,
                (*c).body_sz,
            );
        }
        6 => {
            http_send_options(c);
            return;
        }
        _ => {
            slog(
                (*w).s,
                log_level::WEBDIS_DEBUG,
                b"405\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            );
            http_send_error(
                c,
                405 as i32 as libc::c_short,
                b"Method Not Allowed\0" as *const u8 as *const i8,
            );
            return;
        }
    }
    match ret as u32 {
        2 | 1 => {
            slog(
                (*w).s,
                log_level::WEBDIS_DEBUG,
                b"403\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            );
            http_send_error(
                c,
                403 as i32 as libc::c_short,
                b"Forbidden\0" as *const u8 as *const i8,
            );
        }
        3 => {
            slog(
                (*w).s,
                log_level::WEBDIS_DEBUG,
                b"503\0" as *const u8 as *const i8,
                3 as i32 as size_t,
            );
            http_send_error(
                c,
                503 as i32 as libc::c_short,
                b"Service Unavailable\0" as *const u8 as *const i8,
            );
        }
        _ => {}
    };
}