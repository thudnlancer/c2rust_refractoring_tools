#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, extern_types)]
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type dict;
    pub type sockadr;
    pub type event_base;
    pub type evbuffer;
    fn SHA1Reset(_: *mut SHA1Context);
    fn SHA1Result(_: *mut SHA1Context) -> i32;
    fn SHA1Input(_: *mut SHA1Context, _: *const u8, _: u32);
    fn base64_init_encodestate(state_in: *mut base64_encodestate);
    fn base64_encode_block(
        plaintext_in: *const i8,
        length_in: i32,
        code_out: *mut i8,
        state_in: *mut base64_encodestate,
    ) -> i32;
    fn base64_encode_blockend(
        code_out: *mut i8,
        state_in: *mut base64_encodestate,
    ) -> i32;
    fn acl_allow_command(cmd: *mut cmd, cfg: *mut conf, client: *mut http_client) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn event_base_once(
        _: *mut event_base,
        _: i32,
        _: libc::c_short,
        _: event_callback_fn,
        _: *mut libc::c_void,
        _: *const timeval,
    ) -> i32;
    fn evbuffer_new() -> *mut evbuffer;
    fn evbuffer_free(buf: *mut evbuffer);
    fn evbuffer_get_length(buf: *const evbuffer) -> size_t;
    fn evbuffer_add(
        buf: *mut evbuffer,
        data: *const libc::c_void,
        datlen: size_t,
    ) -> i32;
    fn evbuffer_copyout(
        buf: *mut evbuffer,
        data_out: *mut libc::c_void,
        datlen: size_t,
    ) -> ssize_t;
    fn evbuffer_drain(buf: *mut evbuffer, len: size_t) -> i32;
    fn evbuffer_write_atmost(buffer: *mut evbuffer, fd: i32, howmuch: ssize_t) -> i32;
    fn evbuffer_read(buffer: *mut evbuffer, fd: i32, howmuch: i32) -> i32;
    fn redisAsyncFree(ac: *mut redisAsyncContext);
    fn http_client_free(c: *mut http_client);
    fn client_get_header(c: *mut http_client, key: *const i8) -> *const i8;
    fn cmd_free_argv(c: *mut cmd);
    fn cmd_free(c: *mut cmd);
    fn cmd_is_subscribe_args(cmd: *mut cmd) -> i32;
    fn cmd_is_unsubscribe_args(cmd: *mut cmd) -> i32;
    fn cmd_send(cmd: *mut cmd, f_format: formatting_fun);
    fn cmd_setup(cmd: *mut cmd, client: *mut http_client);
    fn pool_free_context(ac: *mut redisAsyncContext);
    fn pool_connect(p: *mut pool, db_num: i32, attach: i32) -> *mut redisAsyncContext;
    fn slog_enabled(s: *mut server, level: log_level) -> i32;
    fn slog(s: *mut server, level: log_level, body: *const i8, sz: size_t);
    fn json_reply(
        c: *mut redisAsyncContext,
        r: *mut libc::c_void,
        privdata: *mut libc::c_void,
    );
    fn json_ws_extract(c: *mut http_client, p: *const i8, sz: size_t) -> *mut cmd;
    fn json_ws_error(
        http_status: i32,
        msg: *const i8,
        msg_sz: size_t,
        out_sz: *mut size_t,
    ) -> *mut i8;
    fn raw_reply(
        c: *mut redisAsyncContext,
        r: *mut libc::c_void,
        privdata: *mut libc::c_void,
    );
    fn raw_ws_extract(c: *mut http_client, p: *const i8, sz: size_t) -> *mut cmd;
    fn raw_ws_error(
        http_status: i32,
        msg: *const i8,
        msg_sz: size_t,
        out_sz: *mut size_t,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn close(__fd: i32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA1Context {
    pub Message_Digest: [u32; 5],
    pub Length_Low: u32,
    pub Length_High: u32,
    pub Message_Block: [u8; 64],
    pub Message_Block_Index: i32,
    pub Computed: i32,
    pub Corrupted: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum base64_encodestep {
    step_A,
    step_B,
    step_C,
}
impl base64_encodestep {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            base64_encodestep::step_A => 0,
            base64_encodestep::step_B => 1,
            base64_encodestep::step_C => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> base64_encodestep {
        match value {
            0 => base64_encodestep::step_A,
            1 => base64_encodestep::step_B,
            2 => base64_encodestep::step_C,
            _ => panic!("Invalid value for base64_encodestep: {}", value),
        }
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encodestate {
    pub step: base64_encodestep,
    pub result: i8,
    pub stepcount: i32,
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = u64;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
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
pub struct acl {
    pub cidr: C2RustUnnamed_5,
    pub http_basic_auth: *mut i8,
    pub enabled: acl_commands,
    pub disabled: acl_commands,
    pub next: *mut acl,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acl_commands {
    pub count: u32,
    pub commands: *mut *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub enabled: i32,
    pub subnet: in_addr_t,
    pub mask: in_addr_t,
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
pub struct server {
    pub fd: i32,
    pub ev: event,
    pub base: *mut event_base,
    pub cfg: *mut conf,
    pub w: *mut *mut worker,
    pub next_worker: i32,
    pub log: C2RustUnnamed_6,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub self_0: pid_t,
    pub fd: i32,
    pub fsync_tv: timeval,
    pub fsync_ev: *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_12,
    pub ev_fd: i32,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_7,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ev_io: C2RustUnnamed_10,
    pub ev_signal: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub ev_signal_next: C2RustUnnamed_9,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub ev_io_next: C2RustUnnamed_11,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ev_next_with_common_timeout: C2RustUnnamed_13,
    pub min_heap_idx: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_15,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_14,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
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
pub struct C2RustUnnamed_15 {
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
pub type event_callback_fn = Option<
    unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
>;
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
pub type ws_frame_type = i32;
pub const WS_UNKNOWN_FRAME: ws_frame_type = -1;
pub const WS_PONG: ws_frame_type = 10;
pub const WS_PING: ws_frame_type = 9;
pub const WS_CONNECTION_CLOSE: ws_frame_type = 8;
pub const WS_BINARY_FRAME: ws_frame_type = 1;
pub const WS_TEXT_FRAME: ws_frame_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ws_msg {
    pub type_0: ws_frame_type,
    pub payload: *mut i8,
    pub payload_sz: size_t,
    pub total_sz: size_t,
}
pub type formatting_fun = Option<
    unsafe extern "C" fn(
        *mut redisAsyncContext,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> (),
>;
pub type ws_error_fun = Option<
    unsafe extern "C" fn(i32, *const i8, size_t, *mut size_t) -> *mut i8,
>;
unsafe extern "C" fn ws_compute_handshake(
    mut c: *mut http_client,
    mut out: *mut i8,
    mut out_sz: *mut size_t,
) -> i32 {
    let mut buffer: *mut u8 = 0 as *mut u8;
    let mut sha1_output: [u8; 20] = [0; 20];
    let mut magic: [i8; 37] = *::core::mem::transmute::<
        &[u8; 37],
        &mut [i8; 37],
    >(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11\0");
    let mut ctx: SHA1Context = SHA1Context {
        Message_Digest: [0; 5],
        Length_Low: 0,
        Length_High: 0,
        Message_Block: [0; 64],
        Message_Block_Index: 0,
        Computed: 0,
        Corrupted: 0,
    };
    let mut b64_ctx: base64_encodestate = base64_encodestate {
        step: base64_encodestep::step_A,
        result: 0,
        stepcount: 0,
    };
    let mut pos: i32 = 0;
    let mut i: i32 = 0;
    let mut key: *const i8 = client_get_header(
        c,
        b"Sec-WebSocket-Key\0" as *const u8 as *const i8,
    );
    let mut key_sz: size_t = if !key.is_null() { strlen(key) } else { 0 as i32 as u64 };
    let mut buffer_sz: size_t = key_sz
        .wrapping_add(::core::mem::size_of::<[i8; 37]>() as u64)
        .wrapping_sub(1 as i32 as u64);
    if key.is_null() || key_sz < 16 as i32 as u64 || key_sz > 32 as i32 as u64 {
        slog(
            (*c).s,
            log_level::WEBDIS_WARNING,
            b"Invalid Sec-WebSocket-Key\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    buffer = calloc(buffer_sz, 1 as i32 as u64) as *mut u8;
    if buffer.is_null() {
        slog(
            (*c).s,
            log_level::WEBDIS_ERROR,
            b"Failed to allocate memory for WS header\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    memcpy(buffer as *mut libc::c_void, key as *const libc::c_void, key_sz);
    memcpy(
        buffer.offset(key_sz as isize) as *mut libc::c_void,
        magic.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[i8; 37]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    SHA1Reset(&mut ctx);
    SHA1Input(&mut ctx, buffer, buffer_sz as u32);
    SHA1Result(&mut ctx);
    i = 0 as i32;
    while i
        < (20 as i32 as u64).wrapping_div(::core::mem::size_of::<i32>() as u64) as i32
    {
        ctx.Message_Digest[i as usize] = ({
            let mut __v: u32 = 0;
            let mut __x: u32 = ctx.Message_Digest[i as usize];
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh0 = &mut __v;
                let fresh1;
                let fresh2 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            }
            __v
        });
        i += 1;
        i;
    }
    memcpy(
        sha1_output.as_mut_ptr() as *mut libc::c_void,
        (ctx.Message_Digest).as_mut_ptr() as *const libc::c_void,
        20 as i32 as u64,
    );
    base64_init_encodestate(&mut b64_ctx);
    pos = base64_encode_block(
        sha1_output.as_mut_ptr() as *const i8,
        20 as i32,
        out,
        &mut b64_ctx,
    );
    base64_encode_blockend(out.offset(pos as isize), &mut b64_ctx);
    *out_sz = strlen(out);
    if *out.offset((*out_sz).wrapping_sub(1 as i32 as u64) as isize) as i32
        == '\n' as i32
    {
        *out_sz = (*out_sz).wrapping_sub(1);
        *out_sz;
    }
    free(buffer as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn ws_client_new(
    mut http_client: *mut http_client,
) -> *mut ws_client {
    let mut db_num: i32 = (*(*(*(*http_client).w).s).cfg).database;
    let mut ws: *mut ws_client = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<ws_client>() as u64,
    ) as *mut ws_client;
    let mut rbuf: *mut evbuffer = evbuffer_new();
    let mut wbuf: *mut evbuffer = evbuffer_new();
    let mut ac: *mut redisAsyncContext = pool_connect(
        (*(*http_client).w).pool,
        db_num,
        0 as i32,
    );
    if ws.is_null() || rbuf.is_null() || wbuf.is_null() {
        slog(
            (*http_client).s,
            log_level::WEBDIS_ERROR,
            b"Failed to allocate memory for WS client\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        if !ws.is_null() {
            free(ws as *mut libc::c_void);
        }
        if !rbuf.is_null() {
            evbuffer_free(rbuf);
        }
        if !wbuf.is_null() {
            evbuffer_free(wbuf);
        }
        if !ac.is_null() {
            redisAsyncFree(ac);
        }
        return 0 as *mut ws_client;
    }
    (*http_client).ws = ws;
    (*ws).http_client = http_client;
    (*ws).rbuf = rbuf;
    (*ws).wbuf = wbuf;
    (*ws).ac = ac;
    return ws;
}
unsafe extern "C" fn ws_client_free(mut ws: *mut ws_client) {
    (*ws).close_after_events = 1 as i32;
    pool_free_context((*ws).ac);
    let mut c: *mut http_client = (*ws).http_client;
    if !c.is_null() {
        close((*c).fd);
        (*c).ws = 0 as *mut ws_client;
    }
    evbuffer_free((*ws).rbuf);
    evbuffer_free((*ws).wbuf);
    if !((*ws).cmd).is_null() {
        (*(*ws).cmd).ac = 0 as *mut redisAsyncContext;
        cmd_free((*ws).cmd);
    }
    free(ws as *mut libc::c_void);
    if !c.is_null() {
        http_client_free(c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ws_handshake_reply(mut ws: *mut ws_client) -> i32 {
    let mut c: *mut http_client = (*ws).http_client;
    let mut sha1_handshake: [i8; 40] = [0; 40];
    let mut buffer: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut origin: *const i8 = 0 as *const i8;
    let mut host: *const i8 = 0 as *const i8;
    let mut origin_sz: size_t = 0 as i32 as size_t;
    let mut host_sz: size_t = 0 as i32 as size_t;
    let mut handshake_sz: size_t = 0 as i32 as size_t;
    let mut sz: size_t = 0;
    let mut template_start: [i8; 74] = *::core::mem::transmute::<
        &[u8; 74],
        &mut [i8; 74],
    >(
        b"HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\0",
    );
    let mut template_accept: [i8; 25] = *::core::mem::transmute::<
        &[u8; 25],
        &mut [i8; 25],
    >(b"\r\nSec-WebSocket-Accept: \0");
    let mut template_sec_origin: [i8; 25] = *::core::mem::transmute::<
        &[u8; 25],
        &mut [i8; 25],
    >(b"\r\nSec-WebSocket-Origin: \0");
    let mut template_loc: [i8; 32] = *::core::mem::transmute::<
        &[u8; 32],
        &mut [i8; 32],
    >(b"\r\nSec-WebSocket-Location: ws://\0");
    let mut template_end: [i8; 5] = *::core::mem::transmute::<
        &[u8; 5],
        &mut [i8; 5],
    >(b"\r\n\r\n\0");
    origin = client_get_header(c, b"Origin\0" as *const u8 as *const i8);
    if !origin.is_null() {
        origin_sz = strlen(origin);
    } else {
        origin = client_get_header(
            c,
            b"Sec-WebSocket-Origin\0" as *const u8 as *const i8,
        );
        if !origin.is_null() {
            origin_sz = strlen(origin);
        }
    }
    host = client_get_header(c, b"Host\0" as *const u8 as *const i8);
    if !host.is_null() {
        host_sz = strlen(host);
    }
    if host.is_null() || host_sz == 0 || ((*c).path).is_null() || (*c).path_sz == 0 {
        slog(
            (*c).s,
            log_level::WEBDIS_WARNING,
            b"Missing headers for WS handshake\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    memset(
        sha1_handshake.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[i8; 40]>() as u64,
    );
    if ws_compute_handshake(
        c,
        &mut *sha1_handshake.as_mut_ptr().offset(0 as i32 as isize),
        &mut handshake_sz,
    ) != 0 as i32
    {
        slog(
            (*c).s,
            log_level::WEBDIS_WARNING,
            b"Failed to compute handshake\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    sz = (::core::mem::size_of::<[i8; 74]>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_add(::core::mem::size_of::<[i8; 25]>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_add(handshake_sz)
        .wrapping_add(
            (if !origin.is_null() && origin_sz != 0 {
                (::core::mem::size_of::<[i8; 25]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_add(origin_sz)
            } else {
                0 as i32 as u64
            }),
        )
        .wrapping_add(::core::mem::size_of::<[i8; 32]>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_add(host_sz)
        .wrapping_add((*c).path_sz)
        .wrapping_add(::core::mem::size_of::<[i8; 5]>() as u64)
        .wrapping_sub(1 as i32 as u64);
    buffer = malloc(sz) as *mut i8;
    p = buffer;
    if p.is_null() {
        slog(
            (*c).s,
            log_level::WEBDIS_ERROR,
            b"Failed to allocate buffer for WS handshake\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    memcpy(
        p as *mut libc::c_void,
        template_start.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[i8; 74]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    p = p
        .offset(
            (::core::mem::size_of::<[i8; 74]>() as u64).wrapping_sub(1 as i32 as u64)
                as isize,
        );
    memcpy(
        p as *mut libc::c_void,
        template_accept.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[i8; 25]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    p = p
        .offset(
            (::core::mem::size_of::<[i8; 25]>() as u64).wrapping_sub(1 as i32 as u64)
                as isize,
        );
    memcpy(
        p as *mut libc::c_void,
        &mut *sha1_handshake.as_mut_ptr().offset(0 as i32 as isize) as *mut i8
            as *const libc::c_void,
        handshake_sz,
    );
    p = p.offset(handshake_sz as isize);
    if !origin.is_null() && origin_sz != 0 {
        memcpy(
            p as *mut libc::c_void,
            template_sec_origin.as_mut_ptr() as *const libc::c_void,
            (::core::mem::size_of::<[i8; 25]>() as u64).wrapping_sub(1 as i32 as u64),
        );
        p = p
            .offset(
                (::core::mem::size_of::<[i8; 25]>() as u64).wrapping_sub(1 as i32 as u64)
                    as isize,
            );
        memcpy(p as *mut libc::c_void, origin as *const libc::c_void, origin_sz);
        p = p.offset(origin_sz as isize);
    }
    memcpy(
        p as *mut libc::c_void,
        template_loc.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[i8; 32]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    p = p
        .offset(
            (::core::mem::size_of::<[i8; 32]>() as u64).wrapping_sub(1 as i32 as u64)
                as isize,
        );
    memcpy(p as *mut libc::c_void, host as *const libc::c_void, host_sz);
    p = p.offset(host_sz as isize);
    memcpy(p as *mut libc::c_void, (*c).path as *const libc::c_void, (*c).path_sz);
    p = p.offset((*c).path_sz as isize);
    memcpy(
        p as *mut libc::c_void,
        template_end.as_mut_ptr() as *const libc::c_void,
        (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    p = p
        .offset(
            (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                as isize,
        );
    let mut add_ret: i32 = evbuffer_add((*ws).wbuf, buffer as *const libc::c_void, sz);
    free(buffer as *mut libc::c_void);
    if add_ret < 0 as i32 {
        slog(
            (*c).s,
            log_level::WEBDIS_ERROR,
            b"Failed to add response for WS handshake\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    return ws_schedule_write(ws);
}
unsafe extern "C" fn ws_log_cmd(mut ws: *mut ws_client, mut cmd: *mut cmd) {
    let mut log_msg: [i8; 124] = [0; 124];
    let mut p: *mut i8 = log_msg.as_mut_ptr();
    let mut eom: *mut i8 = log_msg
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[i8; 124]>() as u64 as isize)
        .offset(-(1 as i32 as isize));
    if slog_enabled((*(*ws).http_client).s, log_level::WEBDIS_DEBUG) == 0 {
        return;
    }
    memset(
        log_msg.as_mut_ptr() as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[i8; 124]>() as u64,
    );
    memcpy(
        p as *mut libc::c_void,
        b"WS: \0" as *const u8 as *const i8 as *const libc::c_void,
        4 as i32 as u64,
    );
    p = p.offset(4 as i32 as isize);
    let mut i: i32 = 0 as i32;
    while p < eom && i < (*cmd).count {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = '/' as i32 as i8;
        let mut arg: *mut i8 = *((*cmd).argv).offset(i as isize);
        let mut arg_sz: size_t = *((*cmd).argv_len).offset(i as isize);
        let mut copy_sz: size_t = if arg_sz < eom.offset_from(p) as i64 as size_t {
            arg_sz
        } else {
            eom.offset_from(p) as i64 as size_t
        };
        memcpy(p as *mut libc::c_void, arg as *const libc::c_void, copy_sz);
        p = p.offset(copy_sz as isize);
        i += 1;
        i;
    }
    slog(
        (*(*ws).http_client).s,
        log_level::WEBDIS_DEBUG,
        log_msg.as_mut_ptr(),
        p.offset_from(log_msg.as_mut_ptr()) as i64 as size_t,
    );
}
unsafe extern "C" fn ws_log_unauthorized(mut ws: *mut ws_client) {
    if slog_enabled((*(*ws).http_client).s, log_level::WEBDIS_DEBUG) == 0 {
        return;
    }
    let msg: [i8; 8] = *::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"WS: 403\0");
    slog(
        (*(*ws).http_client).s,
        log_level::WEBDIS_DEBUG,
        msg.as_ptr(),
        (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64),
    );
}
unsafe extern "C" fn ws_execute(mut ws: *mut ws_client, mut msg: *mut ws_msg) -> i32 {
    let mut c: *mut http_client = (*ws).http_client;
    let mut fun_extract: Option<
        unsafe extern "C" fn(*mut http_client, *const i8, size_t) -> *mut cmd,
    > = None;
    let mut fun_reply: formatting_fun = None;
    let mut fun_error: ws_error_fun = None;
    if (*c).path_sz == 1 as i32 as u64
        && strncmp((*c).path, b"/\0" as *const u8 as *const i8, 1 as i32 as u64)
            == 0 as i32
        || strncmp((*c).path, b"/.json\0" as *const u8 as *const i8, 6 as i32 as u64)
            == 0 as i32
    {
        fun_extract = Some(
            json_ws_extract
                as unsafe extern "C" fn(*mut http_client, *const i8, size_t) -> *mut cmd,
        );
        fun_reply = Some(
            json_reply
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        );
        fun_error = Some(
            json_ws_error
                as unsafe extern "C" fn(i32, *const i8, size_t, *mut size_t) -> *mut i8,
        );
    } else if strncmp((*c).path, b"/.raw\0" as *const u8 as *const i8, 5 as i32 as u64)
        == 0 as i32
    {
        fun_extract = Some(
            raw_ws_extract
                as unsafe extern "C" fn(*mut http_client, *const i8, size_t) -> *mut cmd,
        );
        fun_reply = Some(
            raw_reply
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        );
        fun_error = Some(
            raw_ws_error
                as unsafe extern "C" fn(i32, *const i8, size_t, *mut size_t) -> *mut i8,
        );
    }
    if fun_extract.is_some() {
        let mut cmd: *mut cmd = fun_extract
            .expect("non-null function pointer")(c, (*msg).payload, (*msg).payload_sz);
        if !cmd.is_null() {
            (*cmd).is_websocket = 1 as i32;
            if !((*ws).cmd).is_null() {
                cmd_free_argv((*ws).cmd);
                (*(*ws).cmd).count = (*cmd).count;
                (*(*ws).cmd).argv = (*cmd).argv;
                (*(*ws).cmd).argv_len = (*cmd).argv_len;
                (*(*ws).cmd).pub_sub_client = c;
                (*cmd).argv = 0 as *mut *mut i8;
                (*cmd).argv_len = 0 as *mut size_t;
                (*cmd).count = 0 as i32;
                cmd_free(cmd);
                cmd = (*ws).cmd;
            } else {
                cmd_setup(cmd, c);
                (*cmd).ac = (*ws).ac;
                (*ws).cmd = cmd;
                (*cmd).pub_sub_client = c;
            }
            let mut is_subscribe: i32 = cmd_is_subscribe_args(cmd);
            let mut is_unsubscribe: i32 = cmd_is_unsubscribe_args(cmd);
            if acl_allow_command(cmd, (*(*c).s).cfg, c) == 0 {
                let forbidden: [i8; 10] = *::core::mem::transmute::<
                    &[u8; 10],
                    &[i8; 10],
                >(b"Forbidden\0");
                let mut error_sz: size_t = 0;
                let mut error: *mut i8 = fun_error
                    .expect(
                        "non-null function pointer",
                    )(
                    403 as i32,
                    forbidden.as_ptr(),
                    (::core::mem::size_of::<[i8; 10]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                    &mut error_sz,
                );
                ws_frame_and_send_response(ws, WS_BINARY_FRAME, error, error_sz);
                free(error as *mut libc::c_void);
                ws_log_cmd(ws, cmd);
                ws_log_unauthorized(ws);
            } else if (*ws).ran_subscribe != 0 && is_subscribe == 0
                && is_unsubscribe == 0
            {
                let mut error_msg: [i8; 36] = *::core::mem::transmute::<
                    &[u8; 36],
                    &mut [i8; 36],
                >(b"Command not allowed after subscribe\0");
                ws_frame_and_send_response(
                    ws,
                    WS_BINARY_FRAME,
                    error_msg.as_mut_ptr(),
                    (::core::mem::size_of::<[i8; 36]>() as u64)
                        .wrapping_sub(1 as i32 as u64),
                );
            } else {
                ws_log_cmd(ws, cmd);
                cmd_send(cmd, fun_reply);
                (*ws).ran_subscribe = is_subscribe;
            }
            return 0 as i32;
        }
    }
    return -(1 as i32);
}
unsafe extern "C" fn ws_msg_new(mut frame_type: ws_frame_type) -> *mut ws_msg {
    let mut msg: *mut ws_msg = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<ws_msg>() as u64,
    ) as *mut ws_msg;
    if msg.is_null() {
        return 0 as *mut ws_msg;
    }
    (*msg).type_0 = frame_type;
    return msg;
}
unsafe extern "C" fn ws_msg_add(
    mut m: *mut ws_msg,
    mut p: *const i8,
    mut psz: size_t,
    mut mask: *const u8,
) -> i32 {
    let mut i: size_t = 0;
    (*m).payload = realloc(
        (*m).payload as *mut libc::c_void,
        ((*m).payload_sz).wrapping_add(psz),
    ) as *mut i8;
    if ((*m).payload).is_null() {
        return -(1 as i32);
    }
    memcpy(
        ((*m).payload).offset((*m).payload_sz as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        psz,
    );
    i = 0 as i32 as size_t;
    while i < psz && !mask.is_null() {
        *((*m).payload).offset(((*m).payload_sz).wrapping_add(i) as isize) = (*p
            .offset(i as isize) as u8 as i32
            ^ *mask.offset(i.wrapping_rem(4 as i32 as u64) as isize) as i32) as i8;
        i = i.wrapping_add(1);
        i;
    }
    (*m).payload_sz = ((*m).payload_sz as u64).wrapping_add(psz) as size_t as size_t;
    return 0 as i32;
}
unsafe extern "C" fn ws_msg_free(mut m: *mut ws_msg) {
    free((*m).payload as *mut libc::c_void);
    free(m as *mut libc::c_void);
}
unsafe extern "C" fn ws_peek_data(
    mut ws: *mut ws_client,
    mut out_msg: *mut *mut ws_msg,
) -> ws_state {
    let mut has_mask: i32 = 0;
    let mut len: uint64_t = 0;
    let mut p: *const i8 = 0 as *const i8;
    let mut frame: *mut i8 = 0 as *mut i8;
    let mut mask: [u8; 4] = [0; 4];
    let mut fin_bit_set: i8 = 0;
    let mut frame_type: ws_frame_type = WS_TEXT_FRAME;
    let mut sz: size_t = evbuffer_get_length((*ws).rbuf);
    if sz < 2 as i32 as u64 {
        return ws_state::WS_READING;
    }
    frame = malloc(sz) as *mut i8;
    if frame.is_null() {
        return ws_state::WS_ERROR;
    }
    let mut rem_ret: i32 = evbuffer_copyout((*ws).rbuf, frame as *mut libc::c_void, sz)
        as i32;
    if rem_ret < 0 as i32 {
        free(frame as *mut libc::c_void);
        return ws_state::WS_ERROR;
    }
    fin_bit_set = (if *frame.offset(0 as i32 as isize) as i32 & 0x80 as i32 != 0 {
        1 as i32
    } else {
        0 as i32
    }) as i8;
    frame_type = (*frame.offset(0 as i32 as isize) as i32 & 0xf as i32) as ws_frame_type;
    has_mask = if *frame.offset(1 as i32 as isize) as i32 & 0x80 as i32 != 0 {
        1 as i32
    } else {
        0 as i32
    };
    if has_mask == 0 {
        (*ws).close_after_events = 1 as i32;
        let close_code_reason: [i8; 85] = *::core::mem::transmute::<
            &[u8; 85],
            &[i8; 85],
        >(
            b"\x03\xEAReceived a frame without a mask from the client (violates RFC6455, 5.1. Overview).\0",
        );
        ws_frame_and_send_response(
            ws,
            WS_CONNECTION_CLOSE,
            close_code_reason.as_ptr(),
            (::core::mem::size_of::<[i8; 85]>() as u64).wrapping_sub(1 as i32 as u64),
        );
        free(frame as *mut libc::c_void);
        return ws_state::WS_ERROR;
    }
    len = (*frame.offset(1 as i32 as isize) as i32 & 0x7f as i32) as uint64_t;
    let mut min_sz: size_t = 6 as i32 as size_t;
    if len == 126 as i32 as u64 {
        min_sz = (min_sz as u64).wrapping_add(::core::mem::size_of::<uint16_t>() as u64)
            as size_t as size_t;
    } else if len == 127 as i32 as u64 {
        min_sz = (min_sz as u64).wrapping_add(::core::mem::size_of::<uint64_t>() as u64)
            as size_t as size_t;
    }
    if sz < min_sz {
        free(frame as *mut libc::c_void);
        return ws_state::WS_READING;
    }
    if len <= 125 as i32 as u64 {
        p = frame.offset(6 as i32 as isize);
        memcpy(
            &mut mask as *mut [u8; 4] as *mut libc::c_void,
            frame.offset(2 as i32 as isize) as *const libc::c_void,
            ::core::mem::size_of::<[u8; 4]>() as u64,
        );
    } else if len == 126 as i32 as u64 {
        let mut sz16: uint16_t = 0;
        memcpy(
            &mut sz16 as *mut uint16_t as *mut libc::c_void,
            frame.offset(2 as i32 as isize) as *const libc::c_void,
            ::core::mem::size_of::<uint16_t>() as u64,
        );
        len = ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = sz16;
            if 0 != 0 {
                __v = (__x as i32 >> 8 as i32 & 0xff as i32
                    | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
            } else {
                let fresh4 = &mut __v;
                let fresh5;
                let fresh6 = __x;
                asm!(
                    "rorw $8, {0:x}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh4, fresh6) => fresh5,
                    options(pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh4, fresh6, fresh5);
            }
            __v
        }) as uint64_t;
        p = frame
            .offset(6 as i32 as isize)
            .offset(::core::mem::size_of::<uint16_t>() as u64 as isize);
        memcpy(
            &mut mask as *mut [u8; 4] as *mut libc::c_void,
            frame.offset(4 as i32 as isize) as *const libc::c_void,
            ::core::mem::size_of::<[u8; 4]>() as u64,
        );
    } else if len == 127 as i32 as u64 {
        let mut sz64: uint64_t = *(frame.offset(2 as i32 as isize) as *mut uint64_t);
        len = (({
            let mut __v: u32 = 0;
            let mut __x: u32 = (sz64 & 0xffffffff as u32 as u64) as u32;
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh7 = &mut __v;
                let fresh8;
                let fresh9 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh7, fresh9) => fresh8,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh7, fresh9, fresh8);
            }
            __v
        }) as uint64_t) << 32 as i32
            | ({
                let mut __v: u32 = 0;
                let mut __x: u32 = (sz64 >> 32 as i32) as u32;
                if 0 != 0 {
                    __v = (__x & 0xff000000 as u32) >> 24 as i32
                        | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                        | (__x & 0xff00 as i32 as u32) << 8 as i32
                        | (__x & 0xff as i32 as u32) << 24 as i32;
                } else {
                    let fresh10 = &mut __v;
                    let fresh11;
                    let fresh12 = __x;
                    asm!(
                        "bswap {0}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh10, fresh12) => fresh11,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh10, fresh12, fresh11);
                }
                __v
            }) as u64;
        p = frame
            .offset(6 as i32 as isize)
            .offset(::core::mem::size_of::<uint64_t>() as u64 as isize);
        memcpy(
            &mut mask as *mut [u8; 4] as *mut libc::c_void,
            frame.offset(10 as i32 as isize) as *const libc::c_void,
            ::core::mem::size_of::<[u8; 4]>() as u64,
        );
    } else {
        free(frame as *mut libc::c_void);
        return ws_state::WS_ERROR;
    }
    if len > sz.wrapping_sub(p.offset_from(frame) as i64 as u64) {
        free(frame as *mut libc::c_void);
        return ws_state::WS_READING;
    }
    let mut ev_copy: i32 = 0 as i32;
    if !out_msg.is_null() {
        let mut msg: *mut ws_msg = ws_msg_new(frame_type);
        if msg.is_null() {
            free(frame as *mut libc::c_void);
            return ws_state::WS_ERROR;
        }
        *out_msg = msg;
        let mut add_ret: i32 = ws_msg_add(msg, p, len, mask.as_mut_ptr());
        if add_ret < 0 as i32 {
            free(frame as *mut libc::c_void);
            return ws_state::WS_ERROR;
        }
        let mut processed_sz: size_t = len
            .wrapping_add(p.offset_from(frame) as i64 as u64);
        (*msg).total_sz = ((*msg).total_sz as u64).wrapping_add(processed_sz) as size_t
            as size_t;
        ev_copy = evbuffer_drain((*ws).rbuf, processed_sz);
    }
    free(frame as *mut libc::c_void);
    if ev_copy < 0 as i32 {
        return ws_state::WS_ERROR
    } else if fin_bit_set != 0 {
        return ws_state::WS_MSG_COMPLETE
    } else {
        return ws_state::WS_READING
    };
}
#[no_mangle]
pub unsafe extern "C" fn ws_process_read_data(
    mut ws: *mut ws_client,
    mut out_processed: *mut u32,
) -> ws_state {
    let mut state: ws_state = ws_state::WS_ERROR;
    if !out_processed.is_null() {
        *out_processed = 0 as i32 as u32;
    }
    state = ws_peek_data(ws, 0 as *mut *mut ws_msg);
    while state as u32 == ws_state::WS_MSG_COMPLETE as i32 as u32 {
        let mut ret: i32 = 0 as i32;
        let mut msg: *mut ws_msg = 0 as *mut ws_msg;
        ws_peek_data(ws, &mut msg);
        if !msg.is_null()
            && ((*msg).type_0 as i32 == WS_TEXT_FRAME as i32
                || (*msg).type_0 as i32 == WS_BINARY_FRAME as i32)
        {
            ret = ws_execute(ws, msg);
            if !out_processed.is_null() {
                *out_processed = (*out_processed).wrapping_add(1);
                *out_processed;
            }
        } else if !msg.is_null() && (*msg).type_0 as i32 == WS_PING as i32 {
            ws_frame_and_send_response(ws, WS_PONG, (*msg).payload, (*msg).payload_sz);
        } else if !msg.is_null() && (*msg).type_0 as i32 == WS_CONNECTION_CLOSE as i32 {
            (*ws).close_after_events = 1 as i32;
            ws_frame_and_send_response(
                ws,
                WS_CONNECTION_CLOSE,
                (*msg).payload,
                (*msg).payload_sz,
            );
        } else if !msg.is_null() {
            let mut format: [i8; 40] = *::core::mem::transmute::<
                &[u8; 40],
                &mut [i8; 40],
            >(b"Received unexpected WS frame type: 0x%x\0");
            let mut error: [i8; 40] = [0; 40];
            let mut error_len: i32 = snprintf(
                error.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 40]>() as u64,
                format.as_mut_ptr(),
                (*msg).type_0 as i32,
            );
            slog(
                (*(*ws).http_client).s,
                log_level::WEBDIS_WARNING,
                error.as_mut_ptr(),
                error_len as size_t,
            );
        }
        if !msg.is_null() {
            ws_msg_free(msg);
        }
        if ret != 0 as i32 {
            slog(
                (*(*ws).http_client).s,
                log_level::WEBDIS_DEBUG,
                b"ws_process_read_data: ws_execute failed\0" as *const u8 as *const i8,
                0 as i32 as size_t,
            );
            return ws_state::WS_ERROR;
        }
        state = ws_peek_data(ws, 0 as *mut *mut ws_msg);
    }
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn ws_frame_and_send_response(
    mut ws: *mut ws_client,
    mut frame_type: ws_frame_type,
    mut p: *const i8,
    mut sz: size_t,
) -> i32 {
    let mut frame: *mut i8 = malloc(sz.wrapping_add(14 as i32 as u64)) as *mut i8;
    let mut frame_sz: size_t = 0 as i32 as size_t;
    if frame.is_null() {
        return -(1 as i32);
    }
    *frame.offset(0 as i32 as isize) = (0x80 as i32 | frame_type as i32) as i8;
    if sz <= 125 as i32 as u64 {
        *frame.offset(1 as i32 as isize) = sz as i8;
        memcpy(
            frame.offset(2 as i32 as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            sz,
        );
        frame_sz = sz.wrapping_add(2 as i32 as u64);
    } else if sz <= 65536 as i32 as u64 {
        let mut sz16: uint16_t = ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = sz as libc::c_ushort;
            if 0 != 0 {
                __v = (__x as i32 >> 8 as i32 & 0xff as i32
                    | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
            } else {
                let fresh13 = &mut __v;
                let fresh14;
                let fresh15 = __x;
                asm!(
                    "rorw $8, {0:x}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh13, fresh15) => fresh14,
                    options(pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh13, fresh15, fresh14);
            }
            __v
        });
        *frame.offset(1 as i32 as isize) = 126 as i32 as i8;
        memcpy(
            frame.offset(2 as i32 as isize) as *mut libc::c_void,
            &mut sz16 as *mut uint16_t as *const libc::c_void,
            2 as i32 as u64,
        );
        memcpy(
            frame.offset(4 as i32 as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            sz,
        );
        frame_sz = sz.wrapping_add(4 as i32 as u64);
    } else {
        let mut sz_be: uint64_t = (({
            let mut __v: u32 = 0;
            let mut __x: u32 = (sz & 0xffffffff as u32 as u64) as u32;
            if 0 != 0 {
                __v = (__x & 0xff000000 as u32) >> 24 as i32
                    | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                    | (__x & 0xff00 as i32 as u32) << 8 as i32
                    | (__x & 0xff as i32 as u32) << 24 as i32;
            } else {
                let fresh16 = &mut __v;
                let fresh17;
                let fresh18 = __x;
                asm!(
                    "bswap {0}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh16, fresh18) => fresh17,
                    options(preserves_flags, pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh16, fresh18, fresh17);
            }
            __v
        }) as uint64_t) << 32 as i32
            | ({
                let mut __v: u32 = 0;
                let mut __x: u32 = (sz >> 32 as i32) as u32;
                if 0 != 0 {
                    __v = (__x & 0xff000000 as u32) >> 24 as i32
                        | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                        | (__x & 0xff00 as i32 as u32) << 8 as i32
                        | (__x & 0xff as i32 as u32) << 24 as i32;
                } else {
                    let fresh19 = &mut __v;
                    let fresh20;
                    let fresh21 = __x;
                    asm!(
                        "bswap {0}", inlateout(reg)
                        c2rust_asm_casts::AsmCast::cast_in(fresh19, fresh21) => fresh20,
                        options(preserves_flags, pure, readonly, att_syntax)
                    );
                    c2rust_asm_casts::AsmCast::cast_out(fresh19, fresh21, fresh20);
                }
                __v
            }) as u64;
        let mut sz64: [i8; 8] = [0; 8];
        memcpy(
            sz64.as_mut_ptr() as *mut libc::c_void,
            &mut sz_be as *mut uint64_t as *const libc::c_void,
            8 as i32 as u64,
        );
        *frame.offset(1 as i32 as isize) = 127 as i32 as i8;
        memcpy(
            frame.offset(2 as i32 as isize) as *mut libc::c_void,
            sz64.as_mut_ptr() as *const libc::c_void,
            8 as i32 as u64,
        );
        memcpy(
            frame.offset(10 as i32 as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            sz,
        );
        frame_sz = sz.wrapping_add(10 as i32 as u64);
    }
    let mut add_ret: i32 = evbuffer_add(
        (*ws).wbuf,
        frame as *const libc::c_void,
        frame_sz,
    );
    free(frame as *mut libc::c_void);
    if add_ret < 0 as i32 {
        slog(
            (*(*(*ws).http_client).w).s,
            log_level::WEBDIS_ERROR,
            b"Failed response allocation in ws_frame_and_send_response\0" as *const u8
                as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    return ws_schedule_write(ws);
}
#[no_mangle]
pub unsafe extern "C" fn ws_close_if_able(mut ws: *mut ws_client) {
    (*ws).close_after_events = 1 as i32;
    if (*ws).scheduled_read != 0 || (*ws).scheduled_write != 0 {
        return;
    }
    ws_client_free(ws);
}
unsafe extern "C" fn ws_can_read(
    mut fd: i32,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut ret: i32 = 0;
    let mut ws: *mut ws_client = p as *mut ws_client;
    (*ws).scheduled_read = 0 as i32;
    ret = evbuffer_read((*ws).rbuf, fd, 4096 as i32);
    if ret <= 0 as i32 || (*ws).close_after_events != 0 {
        ws_close_if_able(ws);
    } else {
        let mut state: ws_state = ws_process_read_data(ws, 0 as *mut u32);
        if state as u32 == ws_state::WS_READING as i32 as u32 {
            ws_monitor_input(ws);
        } else if state as u32 == ws_state::WS_ERROR as i32 as u32 {
            ws_close_if_able(ws);
        }
    };
}
unsafe extern "C" fn ws_can_write(
    mut fd: i32,
    mut event: libc::c_short,
    mut p: *mut libc::c_void,
) {
    let mut ret: i32 = 0;
    let mut ws: *mut ws_client = p as *mut ws_client;
    (*ws).scheduled_write = 0 as i32;
    ret = evbuffer_write_atmost((*ws).wbuf, fd, 4096 as i32 as ssize_t);
    if ret <= 0 as i32 {
        ws_close_if_able(ws);
    } else if evbuffer_get_length((*ws).wbuf) > 0 as i32 as u64 {
        ws_schedule_write(ws);
    } else if (*ws).close_after_events != 0 {
        ws_close_if_able(ws);
    } else {
        let mut processed: u32 = 0 as i32 as u32;
        ws_process_read_data(ws, &mut processed);
        ws_monitor_input(ws);
    };
}
unsafe extern "C" fn ws_schedule_write(mut ws: *mut ws_client) -> i32 {
    let mut c: *mut http_client = (*ws).http_client;
    if (*ws).scheduled_write == 0 {
        (*ws).scheduled_write = 1 as i32;
        return event_base_once(
            (*(*c).w).base,
            (*c).fd,
            0x4 as i32 as libc::c_short,
            Some(
                ws_can_write
                    as unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
            ),
            ws as *mut libc::c_void,
            0 as *const timeval,
        );
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn ws_monitor_input(mut ws: *mut ws_client) -> i32 {
    let mut c: *mut http_client = (*ws).http_client;
    if (*ws).scheduled_read == 0 {
        (*ws).scheduled_read = 1 as i32;
        return event_base_once(
            (*(*c).w).base,
            (*c).fd,
            0x2 as i32 as libc::c_short,
            Some(
                ws_can_read
                    as unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
            ),
            ws as *mut libc::c_void,
            0 as *const timeval,
        );
    }
    return 0 as i32;
}