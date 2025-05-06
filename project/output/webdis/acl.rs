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
    pub type worker;
    pub type dict;
    pub type sockadr;
    pub type evbuffer;
    pub type server;
    pub type event_base;
    fn client_get_header(c: *mut http_client, key: *const i8) -> *const i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncasecmp(__s1: *const i8, __s2: *const i8, __n: size_t) -> i32;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = u64;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
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
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_8,
    pub ev_fd: i32,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_3,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ev_io: C2RustUnnamed_6,
    pub ev_signal: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub ev_signal_next: C2RustUnnamed_5,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub ev_io_next: C2RustUnnamed_7,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ev_next_with_common_timeout: C2RustUnnamed_9,
    pub min_heap_idx: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_11,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_10,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
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
    pub log_fsync: C2RustUnnamed_13,
    pub hiredis_opts: C2RustUnnamed_12,
    pub default_root: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub keep_alive_sec: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub cidr: C2RustUnnamed_14,
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
pub struct C2RustUnnamed_14 {
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
#[no_mangle]
pub unsafe extern "C" fn acl_match_client(
    mut a: *mut acl,
    mut client: *mut http_client,
    mut ip: *mut in_addr_t,
) -> i32 {
    let mut auth: *const i8 = 0 as *const i8;
    auth = client_get_header(client, b"Authorization\0" as *const u8 as *const i8);
    if !((*a).http_basic_auth).is_null() {
        if !auth.is_null()
            && strncasecmp(
                auth,
                b"Basic \0" as *const u8 as *const i8,
                6 as i32 as size_t,
            ) == 0 as i32
        {
            if strcmp(auth.offset(6 as i32 as isize), (*a).http_basic_auth) != 0 as i32 {
                return 0 as i32;
            }
        } else {
            return 0 as i32
        }
    }
    if (*a).cidr.enabled == 0 as i32 {
        return 1 as i32;
    }
    if *ip & (*a).cidr.mask == (*a).cidr.subnet & (*a).cidr.mask {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn acl_allow_command(
    mut cmd: *mut cmd,
    mut cfg: *mut conf,
    mut client: *mut http_client,
) -> i32 {
    let mut always_off: [*mut i8; 5] = [
        b"MULTI\0" as *const u8 as *const i8 as *mut i8,
        b"EXEC\0" as *const u8 as *const i8 as *mut i8,
        b"WATCH\0" as *const u8 as *const i8 as *mut i8,
        b"DISCARD\0" as *const u8 as *const i8 as *mut i8,
        b"SELECT\0" as *const u8 as *const i8 as *mut i8,
    ];
    let mut i: u32 = 0;
    let mut authorized: i32 = 1 as i32;
    let mut a: *mut acl = 0 as *mut acl;
    let mut client_addr: in_addr_t = 0;
    let mut cmd_name: *const i8 = 0 as *const i8;
    let mut cmd_len: size_t = 0;
    if (*cmd).count == 0 as i32 {
        return 0 as i32;
    }
    cmd_name = *((*cmd).argv).offset(0 as i32 as isize);
    cmd_len = *((*cmd).argv_len).offset(0 as i32 as isize);
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[*mut i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<*mut i8>() as u64)
    {
        if strncasecmp(always_off[i as usize], cmd_name, cmd_len) == 0 as i32 {
            return 0 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    client_addr = ({
        let mut __v: u32 = 0;
        let mut __x: u32 = (*client).addr;
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
                "bswap {0}", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh0,
                fresh2) => fresh1, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
    });
    a = (*cfg).perms;
    while !a.is_null() {
        if !(acl_match_client(a, client, &mut client_addr) == 0) {
            i = 0 as i32 as u32;
            while i < (*a).enabled.count {
                if strncasecmp(
                    *((*a).enabled.commands).offset(i as isize),
                    cmd_name,
                    cmd_len,
                ) == 0 as i32
                {
                    authorized = 1 as i32;
                }
                if strncasecmp(
                    *((*a).enabled.commands).offset(i as isize),
                    b"*\0" as *const u8 as *const i8,
                    1 as i32 as size_t,
                ) == 0 as i32
                {
                    authorized = 1 as i32;
                }
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as i32 as u32;
            while i < (*a).disabled.count {
                if strncasecmp(
                    *((*a).disabled.commands).offset(i as isize),
                    cmd_name,
                    cmd_len,
                ) == 0 as i32
                {
                    authorized = 0 as i32;
                }
                if strncasecmp(
                    *((*a).disabled.commands).offset(i as isize),
                    b"*\0" as *const u8 as *const i8,
                    1 as i32 as size_t,
                ) == 0 as i32
                {
                    authorized = 0 as i32;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        a = (*a).next;
    }
    return authorized;
}