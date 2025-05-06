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
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type dict;
    pub type sockadr;
    pub type pool;
    pub type acl;
    fn http_parser_init(parser: *mut http_parser, type_0: http_parser_type);
    fn http_parser_execute(
        parser: *mut http_parser,
        settings: *const http_parser_settings,
        data: *const i8,
        len: size_t,
    ) -> size_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn redisAsyncDisconnect(ac: *mut redisAsyncContext);
    fn worker_process_client(c: *mut http_client);
    fn cmd_free(c: *mut cmd);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(__s: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strncasecmp(__s1: *const i8, __s2: *const i8, __n: size_t) -> i32;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
pub type int64_t = __int64_t;
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_4,
    pub ev_fd: i32,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ev_io: C2RustUnnamed_2,
    pub ev_signal: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub ev_signal_next: C2RustUnnamed_1,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ev_io_next: C2RustUnnamed_3,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ev_next_with_common_timeout: C2RustUnnamed_5,
    pub min_heap_idx: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_7,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_6,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
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
pub type http_data_cb = Option<
    unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum http_parser_type {
    HTTP_REQUEST,
    HTTP_RESPONSE,
    HTTP_BOTH,
}
impl http_parser_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            http_parser_type::HTTP_REQUEST => 0,
            http_parser_type::HTTP_RESPONSE => 1,
            http_parser_type::HTTP_BOTH => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> http_parser_type {
        match value {
            0 => http_parser_type::HTTP_REQUEST,
            1 => http_parser_type::HTTP_RESPONSE,
            2 => http_parser_type::HTTP_BOTH,
            _ => panic!("Invalid value for http_parser_type: {}", value),
        }
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
pub struct redisAsyncContext {
    pub c: redisContext,
    pub err: i32,
    pub errstr: *mut i8,
    pub data: *mut libc::c_void,
    pub dataCleanup: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ev: C2RustUnnamed_9,
    pub onDisconnect: Option<redisDisconnectCallback>,
    pub onConnect: Option<redisConnectCallback>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed_8,
    pub push_cb: Option<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub struct C2RustUnnamed_9 {
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
    pub tcp: C2RustUnnamed_11,
    pub unix_sock: C2RustUnnamed_10,
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
pub struct C2RustUnnamed_10 {
    pub path: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
pub struct server {
    pub fd: i32,
    pub ev: event,
    pub base: *mut event_base,
    pub cfg: *mut conf,
    pub w: *mut *mut worker,
    pub next_worker: i32,
    pub log: C2RustUnnamed_12,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub self_0: pid_t,
    pub fd: i32,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
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
    pub log_fsync: C2RustUnnamed_14,
    pub hiredis_opts: C2RustUnnamed_13,
    pub default_root: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub keep_alive_sec: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct auth {
    pub use_legacy_auth: i32,
    pub username: *mut i8,
    pub password: *mut i8,
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
pub type C2RustUnnamed_15 = i32;
pub const CLIENT_OOM: C2RustUnnamed_15 = -2;
pub const CLIENT_DISCONNECTED: C2RustUnnamed_15 = -1;
unsafe extern "C" fn http_client_on_url(
    mut p: *mut http_parser,
    mut at: *const i8,
    mut sz: size_t,
) -> i32 {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    (*c).path = realloc(
        (*c).path as *mut libc::c_void,
        ((*c).path_sz).wrapping_add(sz).wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    if ((*c).path).is_null() {
        (*c).failed_alloc = 1 as i32 as i8;
        return -(1 as i32);
    }
    memcpy(
        ((*c).path).offset((*c).path_sz as isize) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    (*c).path_sz = ((*c).path_sz as u64).wrapping_add(sz) as size_t as size_t;
    *((*c).path).offset((*c).path_sz as isize) = 0 as i32 as i8;
    return 0 as i32;
}
unsafe extern "C" fn http_client_on_body(
    mut p: *mut http_parser,
    mut at: *const i8,
    mut sz: size_t,
) -> i32 {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    return http_client_add_to_body(c, at, sz);
}
#[no_mangle]
pub unsafe extern "C" fn http_client_add_to_body(
    mut c: *mut http_client,
    mut at: *const i8,
    mut sz: size_t,
) -> i32 {
    (*c).body = realloc(
        (*c).body as *mut libc::c_void,
        ((*c).body_sz).wrapping_add(sz).wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    if ((*c).body).is_null() {
        (*c).failed_alloc = 1 as i32 as i8;
        return -(1 as i32);
    }
    memcpy(
        ((*c).body).offset((*c).body_sz as isize) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    (*c).body_sz = ((*c).body_sz as u64).wrapping_add(sz) as size_t as size_t;
    *((*c).body).offset((*c).body_sz as isize) = 0 as i32 as i8;
    return 0 as i32;
}
unsafe extern "C" fn http_client_on_header_name(
    mut p: *mut http_parser,
    mut at: *const i8,
    mut sz: size_t,
) -> i32 {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    let mut n: size_t = (*c).header_count as size_t;
    if (*c).last_cb as u32 != last_cb_t::LAST_CB_KEY as i32 as u32 {
        (*c).header_count += 1;
        n = (*c).header_count as size_t;
        (*c).headers = realloc(
            (*c).headers as *mut libc::c_void,
            n.wrapping_mul(::core::mem::size_of::<http_header>() as u64),
        ) as *mut http_header;
        if ((*c).headers).is_null() {
            (*c).failed_alloc = 1 as i32 as i8;
            return -(1 as i32);
        }
        memset(
            &mut *((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)
                as *mut http_header as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<http_header>() as u64,
        );
    }
    let ref mut fresh0 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as i32 as u64) as isize))
        .key;
    *fresh0 = realloc(
        (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key
            as *mut libc::c_void,
        ((*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key_sz)
            .wrapping_add(sz)
            .wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    if (*fresh0).is_null() {
        (*c).failed_alloc = 1 as i32 as i8;
        return -(1 as i32);
    }
    memcpy(
        ((*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key)
            .offset(
                (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key_sz
                    as isize,
            ) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    let ref mut fresh1 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as i32 as u64) as isize))
        .key_sz;
    *fresh1 = (*fresh1 as u64).wrapping_add(sz) as size_t as size_t;
    *((*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key)
        .offset(
            (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key_sz
                as isize,
        ) = 0 as i32 as i8;
    (*c).last_cb = last_cb_t::LAST_CB_KEY;
    return 0 as i32;
}
unsafe extern "C" fn wrap_filename(mut val: *const i8, mut val_len: size_t) -> *mut i8 {
    let mut format: [i8; 23] = *::core::mem::transmute::<
        &[u8; 23],
        &mut [i8; 23],
    >(b"attachment; filename=\"\0");
    let mut sz: size_t = (::core::mem::size_of::<[i8; 23]>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_add(val_len)
        .wrapping_add(1 as i32 as u64);
    let mut p: *mut i8 = calloc(sz.wrapping_add(1 as i32 as u64), 1 as i32 as u64)
        as *mut i8;
    memcpy(
        p as *mut libc::c_void,
        format.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[i8; 23]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    memcpy(
        p
            .offset(::core::mem::size_of::<[i8; 23]>() as u64 as isize)
            .offset(-(1 as i32 as isize)) as *mut libc::c_void,
        val as *const libc::c_void,
        val_len,
    );
    *p.offset(sz.wrapping_sub(1 as i32 as u64) as isize) = '"' as i32 as i8;
    return p;
}
unsafe extern "C" fn http_client_on_query_string(
    mut parser: *mut http_parser,
    mut at: *const i8,
    mut sz: size_t,
) -> i32 {
    let mut c: *mut http_client = (*parser).data as *mut http_client;
    let mut p: *const i8 = at;
    while p < at.offset(sz as isize) {
        let mut key: *const i8 = p;
        let mut val: *const i8 = 0 as *const i8;
        let mut key_len: i32 = 0;
        let mut val_len: i32 = 0;
        let mut eq: *mut i8 = memchr(
            key as *const libc::c_void,
            '=' as i32,
            sz.wrapping_sub(p.offset_from(at) as i64 as u64),
        ) as *mut i8;
        if eq.is_null() || eq > at.offset(sz as isize) as *mut i8 {
            break;
        } else {
            let mut amp: *mut i8 = 0 as *mut i8;
            val = eq.offset(1 as i32 as isize);
            key_len = eq.offset_from(key) as i64 as i32;
            p = eq.offset(1 as i32 as isize);
            amp = memchr(
                p as *const libc::c_void,
                '&' as i32,
                sz.wrapping_sub(p.offset_from(at) as i64 as u64),
            ) as *mut i8;
            if amp.is_null() || amp > at.offset(sz as isize) as *mut i8 {
                val_len = at.offset(sz as isize).offset_from(p) as i64 as i32;
            } else {
                val_len = amp.offset_from(val) as i64 as i32;
                p = amp.offset(1 as i32 as isize);
            }
            if key_len == 4 as i32
                && strncmp(key, b"type\0" as *const u8 as *const i8, 4 as i32 as u64)
                    == 0 as i32
            {
                (*c).type_0 = calloc((1 as i32 + val_len) as u64, 1 as i32 as u64)
                    as *mut i8;
                memcpy(
                    (*c).type_0 as *mut libc::c_void,
                    val as *const libc::c_void,
                    val_len as u64,
                );
            } else if key_len == 5 as i32
                && strncmp(key, b"jsonp\0" as *const u8 as *const i8, 5 as i32 as u64)
                    == 0 as i32
                || key_len == 8 as i32
                    && strncmp(
                        key,
                        b"callback\0" as *const u8 as *const i8,
                        8 as i32 as u64,
                    ) == 0 as i32
            {
                (*c).jsonp = calloc((1 as i32 + val_len) as u64, 1 as i32 as u64)
                    as *mut i8;
                memcpy(
                    (*c).jsonp as *mut libc::c_void,
                    val as *const libc::c_void,
                    val_len as u64,
                );
            } else if key_len == 3 as i32
                && strncmp(key, b"sep\0" as *const u8 as *const i8, 3 as i32 as u64)
                    == 0 as i32
            {
                (*c).separator = calloc((1 as i32 + val_len) as u64, 1 as i32 as u64)
                    as *mut i8;
                memcpy(
                    (*c).separator as *mut libc::c_void,
                    val as *const libc::c_void,
                    val_len as u64,
                );
            } else if key_len == 8 as i32
                && strncmp(key, b"filename\0" as *const u8 as *const i8, 8 as i32 as u64)
                    == 0 as i32
            {
                (*c).filename = wrap_filename(val, val_len as size_t);
            }
            if amp.is_null() {
                break;
            }
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn http_client_on_header_value(
    mut p: *mut http_parser,
    mut at: *const i8,
    mut sz: size_t,
) -> i32 {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    let mut n: size_t = (*c).header_count as size_t;
    let ref mut fresh2 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as i32 as u64) as isize))
        .val;
    *fresh2 = realloc(
        (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).val
            as *mut libc::c_void,
        ((*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).val_sz)
            .wrapping_add(sz)
            .wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    if (*fresh2).is_null() {
        (*c).failed_alloc = 1 as i32 as i8;
        return -(1 as i32);
    }
    memcpy(
        ((*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).val)
            .offset(
                (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).val_sz
                    as isize,
            ) as *mut libc::c_void,
        at as *const libc::c_void,
        sz,
    );
    let ref mut fresh3 = (*((*c).headers)
        .offset(n.wrapping_sub(1 as i32 as u64) as isize))
        .val_sz;
    *fresh3 = (*fresh3 as u64).wrapping_add(sz) as size_t as size_t;
    *((*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).val)
        .offset(
            (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).val_sz
                as isize,
        ) = 0 as i32 as i8;
    (*c).last_cb = last_cb_t::LAST_CB_VAL;
    if strncmp(
        b"Expect\0" as *const u8 as *const i8,
        (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key,
        (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key_sz,
    ) == 0 as i32
    {
        if sz == 12 as i32 as u64
            && strncasecmp(at, b"100-continue\0" as *const u8 as *const i8, sz)
                == 0 as i32
        {
            let mut http100: [i8; 26] = *::core::mem::transmute::<
                &[u8; 26],
                &mut [i8; 26],
            >(b"HTTP/1.1 100 Continue\r\n\r\n\0");
            let mut ret: i32 = write(
                (*c).fd,
                http100.as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[i8; 26]>() as u64).wrapping_sub(1 as i32 as u64),
            ) as i32;
        }
    } else if strncasecmp(
        b"Connection\0" as *const u8 as *const i8,
        (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key,
        (*((*c).headers).offset(n.wrapping_sub(1 as i32 as u64) as isize)).key_sz,
    ) == 0 as i32
    {
        if sz == 10 as i32 as u64
            && strncasecmp(at, b"Keep-Alive\0" as *const u8 as *const i8, sz) == 0 as i32
        {
            (*c).keep_alive = 1 as i32 as i8;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn http_client_on_message_complete(mut p: *mut http_parser) -> i32 {
    let mut c: *mut http_client = (*p).data as *mut http_client;
    if ((*c).parser).flags() as i32 & flags::F_CONNECTION_CLOSE as i32 != 0 {
        (*c).keep_alive = 0 as i32 as i8;
        (*c).fully_read = 1 as i32 as i8;
    } else if (*c).parser.http_major as i32 == 1 as i32
        && (*c).parser.http_minor as i32 == 1 as i32
    {
        (*c).keep_alive = 1 as i32 as i8;
    }
    (*c).http_version = (*c).parser.http_minor as i8;
    if (*p).upgrade as i32 != 0 && (*(*(*(*c).w).s).cfg).websockets != 0 {
        (*c).is_websocket = 1 as i32 as i8;
        return 0 as i32;
    }
    if (*c).path_sz == 1 as i32 as u64 && *(*c).path as i32 == '/' as i32
        && !((*(*(*(*c).w).s).cfg).default_root).is_null()
    {
        free((*c).path as *mut libc::c_void);
        (*c).path = strdup((*(*(*(*c).w).s).cfg).default_root);
        (*c).path_sz = strlen((*c).path);
    }
    worker_process_client(c);
    http_client_reset(c);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn http_client_new(
    mut w: *mut worker,
    mut fd: i32,
    mut addr: in_addr_t,
) -> *mut http_client {
    let mut c: *mut http_client = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<http_client>() as u64,
    ) as *mut http_client;
    (*c).fd = fd;
    (*c).w = w;
    (*c).addr = addr;
    (*c).s = (*w).s;
    http_parser_init(&mut (*c).parser, http_parser_type::HTTP_REQUEST);
    (*c).parser.data = c as *mut libc::c_void;
    (*c).settings.on_url = Some(
        http_client_on_url
            as unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
    );
    (*c).settings.on_query_string = Some(
        http_client_on_query_string
            as unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
    );
    (*c).settings.on_body = Some(
        http_client_on_body
            as unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
    );
    (*c).settings.on_message_complete = Some(
        http_client_on_message_complete as unsafe extern "C" fn(*mut http_parser) -> i32,
    );
    (*c).settings.on_header_field = Some(
        http_client_on_header_name
            as unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
    );
    (*c).settings.on_header_value = Some(
        http_client_on_header_value
            as unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
    );
    (*c).last_cb = last_cb_t::LAST_CB_NONE;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn http_client_reset(mut c: *mut http_client) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*c).header_count {
        free((*((*c).headers).offset(i as isize)).key as *mut libc::c_void);
        free((*((*c).headers).offset(i as isize)).val as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*c).headers as *mut libc::c_void);
    (*c).headers = 0 as *mut http_header;
    (*c).header_count = 0 as i32;
    free((*c).body as *mut libc::c_void);
    (*c).body = 0 as *mut i8;
    (*c).body_sz = 0 as i32 as size_t;
    free((*c).path as *mut libc::c_void);
    (*c).path = 0 as *mut i8;
    (*c).path_sz = 0 as i32 as size_t;
    free((*c).type_0 as *mut libc::c_void);
    (*c).type_0 = 0 as *mut i8;
    free((*c).jsonp as *mut libc::c_void);
    (*c).jsonp = 0 as *mut i8;
    free((*c).filename as *mut libc::c_void);
    (*c).filename = 0 as *mut i8;
    (*c).request_sz = 0 as i32 as size_t;
    (*c).last_cb = last_cb_t::LAST_CB_NONE;
    if (*c).keep_alive as i32 == 0 as i32 {
        (*c).broken = 1 as i32 as i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_client_free(mut c: *mut http_client) {
    http_client_reset(c);
    free((*c).buffer as *mut libc::c_void);
    free(c as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn http_client_read(mut c: *mut http_client) -> i32 {
    let mut buffer: [i8; 4096] = [0; 4096];
    let mut ret: i32 = 0;
    ret = read(
        (*c).fd,
        buffer.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[i8; 4096]>() as u64,
    ) as i32;
    if ret <= 0 as i32 {
        if !((*c).reused_cmd).is_null() && !((*(*c).reused_cmd).ac).is_null() {
            let mut cmd: *mut cmd = (*c).reused_cmd;
            redisAsyncDisconnect((*(*c).reused_cmd).ac);
            (*cmd).ac = 0 as *mut redisAsyncContext;
            (*c).reused_cmd = 0 as *mut cmd;
            cmd_free(cmd);
        }
        if !((*c).last_cmd).is_null() {
            (*(*c).last_cmd).fd = -(1 as i32);
            (*(*c).last_cmd).http_client = 0 as *mut http_client;
            (*c).last_cmd = 0 as *mut cmd;
        }
        close((*c).fd);
        http_client_free(c);
        return CLIENT_DISCONNECTED as i32;
    }
    (*c).buffer = realloc(
        (*c).buffer as *mut libc::c_void,
        ((*c).sz).wrapping_add(ret as u64),
    ) as *mut i8;
    if ((*c).buffer).is_null() {
        return CLIENT_OOM as i32;
    }
    memcpy(
        ((*c).buffer).offset((*c).sz as isize) as *mut libc::c_void,
        buffer.as_mut_ptr() as *const libc::c_void,
        ret as u64,
    );
    (*c).sz = ((*c).sz as u64).wrapping_add(ret as u64) as size_t as size_t;
    (*c).request_sz = ((*c).request_sz as u64).wrapping_add(ret as u64) as size_t
        as size_t;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn http_client_remove_data(
    mut c: *mut http_client,
    mut sz: size_t,
) -> i32 {
    let mut buffer: *mut i8 = 0 as *mut i8;
    if (*c).sz < sz {
        return -(1 as i32);
    }
    buffer = malloc(((*c).sz).wrapping_sub(sz)) as *mut i8;
    if buffer.is_null() {
        (*c).failed_alloc = 1 as i32 as i8;
        return -(1 as i32);
    }
    memcpy(
        buffer as *mut libc::c_void,
        ((*c).buffer).offset(sz as isize) as *const libc::c_void,
        ((*c).sz).wrapping_sub(sz),
    );
    free((*c).buffer as *mut libc::c_void);
    (*c).buffer = buffer;
    (*c).sz = ((*c).sz as u64).wrapping_sub(sz) as size_t as size_t;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn http_client_execute(mut c: *mut http_client) -> i32 {
    let mut nparsed: i32 = http_parser_execute(
        &mut (*c).parser,
        &mut (*c).settings,
        (*c).buffer,
        (*c).sz,
    ) as i32;
    if (*c).is_websocket == 0 {
        free((*c).buffer as *mut libc::c_void);
        (*c).buffer = 0 as *mut i8;
        (*c).sz = 0 as i32 as size_t;
    }
    return nparsed;
}
#[no_mangle]
pub unsafe extern "C" fn client_get_header(
    mut c: *mut http_client,
    mut key: *const i8,
) -> *const i8 {
    let mut i: i32 = 0;
    let mut sz: size_t = strlen(key);
    i = 0 as i32;
    while i < (*c).header_count {
        if sz == (*((*c).headers).offset(i as isize)).key_sz
            && strncasecmp(key, (*((*c).headers).offset(i as isize)).key, sz) == 0 as i32
        {
            return (*((*c).headers).offset(i as isize)).val;
        }
        i += 1;
        i;
    }
    return 0 as *const i8;
}