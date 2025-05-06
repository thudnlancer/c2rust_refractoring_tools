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
    pub type cmd;
    pub type dict;
    pub type sockadr;
    pub type pool;
    pub type conf;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn event_base_set(_: *mut event_base, _: *mut event) -> i32;
    fn event_add(ev: *mut event, timeout: *const timeval) -> i32;
    fn event_set(
        _: *mut event,
        _: i32,
        _: libc::c_short,
        _: Option<unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    );
    fn http_client_reset(c: *mut http_client);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn slog(s: *mut server, level: log_level, body: *const i8, sz: size_t);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strncasecmp(__s1: *const i8, __s2: *const i8, __n: size_t) -> i32;
    fn close(__fd: i32) -> i32;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
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
pub type http_data_cb = Option<
    unsafe extern "C" fn(*mut http_parser, *const i8, size_t) -> i32,
>;
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
pub struct worker {
    pub thread: pthread_t,
    pub base: *mut event_base,
    pub s: *mut server,
    pub link: [i32; 2],
    pub pool: *mut pool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_response {
    pub ev: event,
    pub code: libc::c_short,
    pub msg: *const i8,
    pub headers: *mut http_header,
    pub header_count: i32,
    pub headers_array_size: i32,
    pub body: *const i8,
    pub body_len: size_t,
    pub out: *mut i8,
    pub out_sz: size_t,
    pub chunked: i32,
    pub http_version: i32,
    pub keep_alive: i32,
    pub sent: i32,
    pub w: *mut worker,
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
unsafe extern "C" fn http_response_allocate_headers(mut r: *mut http_response) {
    (*r).headers_array_size = 9 as i32;
    (*r).headers = calloc(
        (*r).headers_array_size as u64,
        ::core::mem::size_of::<http_header>() as u64,
    ) as *mut http_header;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_init(
    mut w: *mut worker,
    mut code: i32,
    mut msg: *const i8,
) -> *mut http_response {
    let mut r: *mut http_response = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<http_response>() as u64,
    ) as *mut http_response;
    if r.is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                log_level::WEBDIS_ERROR,
                b"Failed to allocate http_response\0" as *const u8 as *const i8,
                0 as i32 as size_t,
            );
        }
        return 0 as *mut http_response;
    }
    (*r).code = code as libc::c_short;
    (*r).msg = msg;
    (*r).w = w;
    (*r).keep_alive = 0 as i32;
    http_response_allocate_headers(r);
    if ((*r).headers).is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                log_level::WEBDIS_ERROR,
                b"Failed to allocate http_response headers\0" as *const u8 as *const i8,
                0 as i32 as size_t,
            );
        }
        free(r as *mut libc::c_void);
        return 0 as *mut http_response;
    }
    http_response_set_header(
        r,
        b"Server\0" as *const u8 as *const i8,
        b"Webdis\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Allow\0" as *const u8 as *const i8,
        b"GET,POST,PUT,OPTIONS\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Access-Control-Allow-Methods\0" as *const u8 as *const i8,
        b"GET,POST,PUT,OPTIONS\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Access-Control-Allow-Origin\0" as *const u8 as *const i8,
        b"*\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    http_response_set_header(
        r,
        b"Access-Control-Allow-Headers\0" as *const u8 as *const i8,
        b"X-Requested-With, Content-Type, Authorization\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_init_with_buffer(
    mut w: *mut worker,
    mut data: *mut i8,
    mut data_sz: size_t,
    mut keep_alive: i32,
) -> *mut http_response {
    let mut r: *mut http_response = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<http_response>() as u64,
    ) as *mut http_response;
    if r.is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                log_level::WEBDIS_ERROR,
                b"Failed to allocate http_response with buffer\0" as *const u8
                    as *const i8,
                0 as i32 as size_t,
            );
        }
        return 0 as *mut http_response;
    }
    (*r).w = w;
    http_response_allocate_headers(r);
    if ((*r).headers).is_null() {
        if !w.is_null() && !((*w).s).is_null() {
            slog(
                (*w).s,
                log_level::WEBDIS_ERROR,
                b"Failed to allocate http_response headers\0" as *const u8 as *const i8,
                0 as i32 as size_t,
            );
        }
        free(r as *mut libc::c_void);
        return 0 as *mut http_response;
    }
    (*r).out = data;
    (*r).out_sz = data_sz;
    (*r).sent = 0 as i32;
    (*r).keep_alive = keep_alive;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_set_header(
    mut r: *mut http_response,
    mut k: *const i8,
    mut v: *const i8,
    mut copy: header_copy,
) {
    let mut i: i32 = 0;
    let mut pos: i32 = (*r).header_count;
    let mut replaced: i32 = 0 as i32;
    let mut key_sz: size_t = strlen(k);
    let mut val_sz: size_t = strlen(v);
    if copy as u32 & header_copy::HEADER_CHECK_DUPE as i32 as u32 != 0 {
        i = 0 as i32;
        while i < (*r).header_count {
            if strncmp((*((*r).headers).offset(i as isize)).key, k, key_sz) == 0 as i32 {
                pos = i;
                if (*((*r).headers).offset(i as isize)).copy as u32
                    & header_copy::HEADER_COPY_KEY as i32 as u32 != 0
                {
                    free((*((*r).headers).offset(i as isize)).key as *mut libc::c_void);
                }
                if (*((*r).headers).offset(i as isize)).copy as u32
                    & header_copy::HEADER_COPY_VALUE as i32 as u32 != 0
                {
                    free((*((*r).headers).offset(i as isize)).val as *mut libc::c_void);
                }
                replaced = 1 as i32;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    if pos == (*r).headers_array_size {
        (*r).headers = realloc(
            (*r).headers as *mut libc::c_void,
            (::core::mem::size_of::<http_header>() as u64)
                .wrapping_mul(((*r).headers_array_size + 1 as i32) as u64),
        ) as *mut http_header;
        (*r).headers_array_size += 1;
        (*r).headers_array_size;
    }
    if replaced == 0 {
        (*r).header_count += 1;
        (*r).header_count;
    }
    if copy as u32 & header_copy::HEADER_COPY_KEY as i32 as u32 != 0 {
        let ref mut fresh0 = (*((*r).headers).offset(pos as isize)).key;
        *fresh0 = calloc(key_sz.wrapping_add(1 as i32 as u64), 1 as i32 as u64)
            as *mut i8;
        memcpy(
            (*((*r).headers).offset(pos as isize)).key as *mut libc::c_void,
            k as *const libc::c_void,
            key_sz,
        );
    } else {
        let ref mut fresh1 = (*((*r).headers).offset(pos as isize)).key;
        *fresh1 = k as *mut i8;
    }
    (*((*r).headers).offset(pos as isize)).key_sz = key_sz;
    if copy as u32 & header_copy::HEADER_COPY_VALUE as i32 as u32 != 0 {
        let ref mut fresh2 = (*((*r).headers).offset(pos as isize)).val;
        *fresh2 = calloc(val_sz.wrapping_add(1 as i32 as u64), 1 as i32 as u64)
            as *mut i8;
        memcpy(
            (*((*r).headers).offset(pos as isize)).val as *mut libc::c_void,
            v as *const libc::c_void,
            val_sz,
        );
    } else {
        let ref mut fresh3 = (*((*r).headers).offset(pos as isize)).val;
        *fresh3 = v as *mut i8;
    }
    (*((*r).headers).offset(pos as isize)).val_sz = val_sz;
    (*((*r).headers).offset(pos as isize)).copy = copy;
    if (*r).chunked == 0
        && strcmp(k, b"Transfer-Encoding\0" as *const u8 as *const i8) == 0
        && strcmp(v, b"chunked\0" as *const u8 as *const i8) == 0
    {
        (*r).chunked = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_response_set_body(
    mut r: *mut http_response,
    mut body: *const i8,
    mut body_len: size_t,
) {
    (*r).body = body;
    (*r).body_len = body_len;
}
unsafe extern "C" fn http_response_cleanup(
    mut r: *mut http_response,
    mut fd: i32,
    mut success: i32,
) {
    let mut i: i32 = 0;
    free((*r).out as *mut libc::c_void);
    if ((*r).keep_alive == 0 || success == 0) && fd > 0 as i32 {
        close(fd);
    }
    i = 0 as i32;
    while i < (*r).header_count {
        if (*((*r).headers).offset(i as isize)).copy as u32
            & header_copy::HEADER_COPY_KEY as i32 as u32 != 0
        {
            free((*((*r).headers).offset(i as isize)).key as *mut libc::c_void);
        }
        if (*((*r).headers).offset(i as isize)).copy as u32
            & header_copy::HEADER_COPY_VALUE as i32 as u32 != 0
        {
            free((*((*r).headers).offset(i as isize)).val as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    free((*r).headers as *mut libc::c_void);
    free(r as *mut libc::c_void);
}
unsafe extern "C" fn http_can_write(
    mut fd: i32,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut ret: i32 = 0;
    let mut r: *mut http_response = p as *mut http_response;
    ret = write(
        fd,
        ((*r).out).offset((*r).sent as isize) as *const libc::c_void,
        ((*r).out_sz).wrapping_sub((*r).sent as u64),
    ) as i32;
    if ret > 0 as i32 {
        (*r).sent += ret;
    }
    if ret <= 0 as i32 || ((*r).out_sz).wrapping_sub((*r).sent as u64) == 0 as i32 as u64
    {
        http_response_cleanup(
            r,
            fd,
            if (*r).out_sz as i32 == (*r).sent { 1 as i32 } else { 0 as i32 },
        );
    } else {
        http_schedule_write(fd, r);
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_schedule_write(mut fd: i32, mut r: *mut http_response) {
    if !((*r).w).is_null() {
        event_set(
            &mut (*r).ev,
            fd,
            0x4 as i32 as libc::c_short,
            Some(
                http_can_write
                    as unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
            ),
            r as *mut libc::c_void,
        );
        event_base_set((*(*r).w).base, &mut (*r).ev);
        let mut ret: i32 = event_add(&mut (*r).ev, 0 as *const timeval);
        if ret != 0 as i32 {
            slog(
                (*(*r).w).s,
                log_level::WEBDIS_ERROR,
                b"Could not schedule HTTP write\0" as *const u8 as *const i8,
                0 as i32 as size_t,
            );
            http_response_cleanup(r, fd, 0 as i32);
        }
    } else {
        http_can_write(fd, 0 as i32 as libc::c_short, r as *mut libc::c_void);
    };
}
unsafe extern "C" fn format_chunk(
    mut p: *const i8,
    mut sz: size_t,
    mut out_sz: *mut size_t,
) -> *mut i8 {
    let mut out: *mut i8 = 0 as *mut i8;
    let mut tmp: [i8; 64] = [0; 64];
    let mut chunk_size: i32 = 0;
    chunk_size = sprintf(
        tmp.as_mut_ptr(),
        b"%x\r\n\0" as *const u8 as *const i8,
        sz as i32,
    );
    *out_sz = (chunk_size as u64).wrapping_add(sz).wrapping_add(2 as i32 as u64);
    out = malloc(*out_sz) as *mut i8;
    memcpy(
        out as *mut libc::c_void,
        tmp.as_mut_ptr() as *const libc::c_void,
        chunk_size as u64,
    );
    memcpy(
        out.offset(chunk_size as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        sz,
    );
    memcpy(
        out.offset(chunk_size as isize).offset(sz as isize) as *mut libc::c_void,
        b"\r\n\0" as *const u8 as *const i8 as *const libc::c_void,
        2 as i32 as u64,
    );
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_write(mut r: *mut http_response, mut fd: i32) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    if fd < 0 as i32 {
        http_response_cleanup(r, fd, 0 as i32);
        return;
    }
    (*r).out_sz = (::core::mem::size_of::<[i8; 14]>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_add(strlen((*r).msg))
        .wrapping_add(2 as i32 as u64);
    (*r).out = calloc(((*r).out_sz).wrapping_add(1 as i32 as u64), 1 as i32 as u64)
        as *mut i8;
    ret = sprintf(
        (*r).out,
        b"HTTP/1.%d %d %s\r\n\0" as *const u8 as *const i8,
        if (*r).http_version != 0 { 1 as i32 } else { 0 as i32 },
        (*r).code as i32,
        (*r).msg,
    );
    p = (*r).out;
    if (*r).chunked == 0 {
        if (*r).code as i32 == 200 as i32 && !((*r).body).is_null() {
            let mut content_length: [i8; 22] = [0; 22];
            sprintf(
                content_length.as_mut_ptr(),
                b"%zd\0" as *const u8 as *const i8,
                (*r).body_len,
            );
            http_response_set_header(
                r,
                b"Content-Length\0" as *const u8 as *const i8,
                content_length.as_mut_ptr(),
                (header_copy::HEADER_COPY_VALUE as i32
                    | header_copy::HEADER_CHECK_DUPE as i32) as header_copy,
            );
        } else {
            http_response_set_header(
                r,
                b"Content-Length\0" as *const u8 as *const i8,
                b"0\0" as *const u8 as *const i8,
                (header_copy::HEADER_COPY_NONE as i32
                    | header_copy::HEADER_CHECK_DUPE as i32) as header_copy,
            );
        }
    }
    i = 0 as i32;
    while i < (*r).header_count {
        let mut header_sz: size_t = ((*((*r).headers).offset(i as isize)).key_sz)
            .wrapping_add(2 as i32 as u64)
            .wrapping_add((*((*r).headers).offset(i as isize)).val_sz)
            .wrapping_add(2 as i32 as u64);
        (*r).out = realloc(
            (*r).out as *mut libc::c_void,
            ((*r).out_sz).wrapping_add(header_sz),
        ) as *mut i8;
        p = ((*r).out).offset((*r).out_sz as isize);
        memcpy(
            p as *mut libc::c_void,
            (*((*r).headers).offset(i as isize)).key as *const libc::c_void,
            (*((*r).headers).offset(i as isize)).key_sz,
        );
        p = p.offset((*((*r).headers).offset(i as isize)).key_sz as isize);
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = ':' as i32 as i8;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = ' ' as i32 as i8;
        memcpy(
            p as *mut libc::c_void,
            (*((*r).headers).offset(i as isize)).val as *const libc::c_void,
            (*((*r).headers).offset(i as isize)).val_sz,
        );
        p = p.offset((*((*r).headers).offset(i as isize)).val_sz as isize);
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '\r' as i32 as i8;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = '\n' as i32 as i8;
        (*r).out_sz = ((*r).out_sz as u64).wrapping_add(header_sz) as size_t as size_t;
        if strncasecmp(
            b"Connection\0" as *const u8 as *const i8,
            (*((*r).headers).offset(i as isize)).key,
            (*((*r).headers).offset(i as isize)).key_sz,
        ) == 0 as i32
            && strncasecmp(
                b"Keep-Alive\0" as *const u8 as *const i8,
                (*((*r).headers).offset(i as isize)).val,
                (*((*r).headers).offset(i as isize)).val_sz,
            ) == 0 as i32
        {
            (*r).keep_alive = 1 as i32;
        }
        i += 1;
        i;
    }
    (*r).out = realloc(
        (*r).out as *mut libc::c_void,
        ((*r).out_sz).wrapping_add(2 as i32 as u64),
    ) as *mut i8;
    memcpy(
        ((*r).out).offset((*r).out_sz as isize) as *mut libc::c_void,
        b"\r\n\0" as *const u8 as *const i8 as *const libc::c_void,
        2 as i32 as u64,
    );
    (*r).out_sz = ((*r).out_sz as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
    if !((*r).body).is_null() && (*r).body_len != 0 {
        let mut tmp: *mut i8 = (*r).body as *mut i8;
        let mut tmp_len: size_t = (*r).body_len;
        if (*r).chunked != 0 {
            tmp = format_chunk((*r).body, (*r).body_len, &mut tmp_len);
        }
        (*r).out = realloc(
            (*r).out as *mut libc::c_void,
            ((*r).out_sz).wrapping_add(tmp_len),
        ) as *mut i8;
        memcpy(
            ((*r).out).offset((*r).out_sz as isize) as *mut libc::c_void,
            tmp as *const libc::c_void,
            tmp_len,
        );
        (*r).out_sz = ((*r).out_sz as u64).wrapping_add(tmp_len) as size_t as size_t;
        if (*r).chunked != 0 {
            free(tmp as *mut libc::c_void);
        }
    }
    (*r).sent = 0 as i32;
    http_schedule_write(fd, r);
}
unsafe extern "C" fn http_response_set_connection_header(
    mut c: *mut http_client,
    mut r: *mut http_response,
) {
    http_response_set_keep_alive(r, (*c).keep_alive as i32);
}
#[no_mangle]
pub unsafe extern "C" fn http_crossdomain(mut c: *mut http_client) {
    let mut resp: *mut http_response = http_response_init(
        0 as *mut worker,
        200 as i32,
        b"OK\0" as *const u8 as *const i8,
    );
    let mut out: [i8; 200] = *::core::mem::transmute::<
        &[u8; 200],
        &mut [i8; 200],
    >(
        b"<?xml version=\"1.0\"?>\n<!DOCTYPE cross-domain-policy SYSTEM \"http://www.macromedia.com/xml/dtds/cross-domain-policy.dtd\">\n<cross-domain-policy>\n<allow-access-from domain=\"*\" />\n</cross-domain-policy>\n\0",
    );
    (*resp).http_version = (*c).http_version as i32;
    http_response_set_connection_header(c, resp);
    http_response_set_header(
        resp,
        b"Content-Type\0" as *const u8 as *const i8,
        b"application/xml\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    http_response_set_body(
        resp,
        out.as_mut_ptr(),
        (::core::mem::size_of::<[i8; 200]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    http_response_write(resp, (*c).fd);
    http_client_reset(c);
}
#[no_mangle]
pub unsafe extern "C" fn http_send_error(
    mut c: *mut http_client,
    mut code: libc::c_short,
    mut msg: *const i8,
) {
    let mut resp: *mut http_response = http_response_init(
        0 as *mut worker,
        code as i32,
        msg,
    );
    (*resp).http_version = (*c).http_version as i32;
    http_response_set_connection_header(c, resp);
    http_response_set_body(resp, 0 as *const i8, 0 as i32 as size_t);
    http_response_write(resp, (*c).fd);
    http_client_reset(c);
}
#[no_mangle]
pub unsafe extern "C" fn http_response_set_keep_alive(
    mut r: *mut http_response,
    mut enabled: i32,
) {
    (*r).keep_alive = enabled;
    if enabled != 0 {
        http_response_set_header(
            r,
            b"Connection\0" as *const u8 as *const i8,
            b"Keep-Alive\0" as *const u8 as *const i8,
            header_copy::HEADER_COPY_NONE,
        );
    } else {
        http_response_set_header(
            r,
            b"Connection\0" as *const u8 as *const i8,
            b"Close\0" as *const u8 as *const i8,
            header_copy::HEADER_COPY_NONE,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_send_options(mut c: *mut http_client) {
    let mut resp: *mut http_response = http_response_init(
        0 as *mut worker,
        200 as i32,
        b"OK\0" as *const u8 as *const i8,
    );
    (*resp).http_version = (*c).http_version as i32;
    http_response_set_connection_header(c, resp);
    http_response_set_header(
        resp,
        b"Content-Type\0" as *const u8 as *const i8,
        b"text/html\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    http_response_set_header(
        resp,
        b"Content-Length\0" as *const u8 as *const i8,
        b"0\0" as *const u8 as *const i8,
        header_copy::HEADER_COPY_NONE,
    );
    http_response_write(resp, (*c).fd);
    http_client_reset(c);
}
#[no_mangle]
pub unsafe extern "C" fn http_response_write_chunk(
    mut fd: i32,
    mut w: *mut worker,
    mut p: *const i8,
    mut sz: size_t,
) {
    let mut r: *mut http_response = http_response_init(w, 0 as i32, 0 as *const i8);
    (*r).keep_alive = 1 as i32;
    (*r).out = format_chunk(p, sz, &mut (*r).out_sz);
    http_schedule_write(fd, r);
}