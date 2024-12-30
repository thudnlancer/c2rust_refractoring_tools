#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type epoll_event;
    pub type conf;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn array_init(a: *mut array, n: uint32_t, size: size_t) -> rstatus_t;
    fn array_deinit(a: *mut array);
    fn array_push(a: *mut array) -> *mut libc::c_void;
    fn array_pop(a: *mut array) -> *mut libc::c_void;
    fn array_get(a: *const array, idx: uint32_t) -> *mut libc::c_void;
    fn array_swap(a: *mut array, b: *mut array);
    fn nc_set_reuseaddr(sd: libc::c_int) -> libc::c_int;
    fn nc_set_reuseport(sd: libc::c_int) -> libc::c_int;
    fn _nc_alloc(
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
    fn _nc_sendn(sd: libc::c_int, vptr: *const libc::c_void, n: size_t) -> ssize_t;
    fn nc_resolve(
        name: *const string,
        port: libc::c_int,
        si: *mut sockinfo,
    ) -> libc::c_int;
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn _log_stderr(fmt: *const libc::c_char, _: ...);
    fn event_loop_stats(cb: event_stats_cb_t, arg: *mut libc::c_void);
    fn conn_ncurr_conn() -> uint32_t;
    fn conn_ntotal_conn() -> uint64_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type socklen_t = __socklen_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_NONBLOCK,
    SOCK_CLOEXEC,
    SOCK_PACKET,
    SOCK_DCCP,
    SOCK_SEQPACKET,
    SOCK_RDM,
    SOCK_RAW,
    SOCK_DGRAM,
    SOCK_STREAM,
}  // end of enum

pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type uint64_t = __uint64_t;
pub type rstatus_t = libc::c_int;
pub type err_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: libc::c_int,
    pub event: *mut epoll_event,
    pub nevent: libc::c_int,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: libc::c_int,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: libc::c_int,
    pub service_str: string,
    pub service: string,
    pub source_str: string,
    pub source: string,
    pub version_str: string,
    pub version: string,
    pub uptime_str: string,
    pub timestamp_str: string,
    pub ntotal_conn_str: string,
    pub ncurr_conn_str: string,
    pub aggregate: libc::c_int,
    pub updated: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conn {
    pub conn_tqe: C2RustUnnamed_5,
    pub owner: *mut libc::c_void,
    pub sd: libc::c_int,
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: *mut sockaddr,
    pub imsg_q: msg_tqh,
    pub omsg_q: msg_tqh,
    pub rmsg: *mut msg,
    pub smsg: *mut msg,
    pub recv: conn_recv_t,
    pub recv_next: conn_recv_next_t,
    pub recv_done: conn_recv_done_t,
    pub send: conn_send_t,
    pub send_next: conn_send_next_t,
    pub send_done: conn_send_done_t,
    pub close: conn_close_t,
    pub active: conn_active_t,
    pub post_connect: conn_post_connect_t,
    pub swallow_msg: conn_swallow_msg_t,
    pub ref_0: conn_ref_t,
    pub unref: conn_unref_t,
    pub enqueue_inq: conn_msgq_t,
    pub dequeue_inq: conn_msgq_t,
    pub enqueue_outq: conn_msgq_t,
    pub dequeue_outq: conn_msgq_t,
    pub recv_bytes: size_t,
    pub send_bytes: size_t,
    pub events: uint32_t,
    pub err: err_t,
    #[bitfield(name = "recv_active", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "recv_ready", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "send_active", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "send_ready", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "client", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "proxy", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "connecting", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "connected", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "eof", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "authenticated", ty = "libc::c_uint", bits = "11..=11")]
    pub recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
pub type conn_msgq_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct msg {
    pub c_tqe: C2RustUnnamed_3,
    pub s_tqe: C2RustUnnamed_2,
    pub m_tqe: C2RustUnnamed_1,
    pub id: uint64_t,
    pub peer: *mut msg,
    pub owner: *mut conn,
    pub tmo_rbe: rbnode,
    pub mhdr: mhdr,
    pub mlen: uint32_t,
    pub start_ts: int64_t,
    pub state: libc::c_int,
    pub pos: *mut uint8_t,
    pub token: *mut uint8_t,
    pub parser: msg_parse_t,
    pub result: msg_parse_result_t,
    pub fragment: msg_fragment_t,
    pub reply: msg_reply_t,
    pub add_auth: msg_add_auth_t,
    pub failure: msg_failure_t,
    pub pre_coalesce: msg_coalesce_t,
    pub post_coalesce: msg_coalesce_t,
    pub type_0: msg_type_t,
    pub keys: *mut array,
    pub vlen: uint32_t,
    pub end: *mut uint8_t,
    pub narg_start: *mut uint8_t,
    pub narg_end: *mut uint8_t,
    pub narg: uint32_t,
    pub rnarg: uint32_t,
    pub rlen: uint32_t,
    pub integer: uint32_t,
    pub is_top_level: uint8_t,
    pub frag_owner: *mut msg,
    pub nfrag: uint32_t,
    pub nfrag_done: uint32_t,
    pub frag_id: uint64_t,
    pub frag_seq: *mut *mut msg,
    pub err: err_t,
    #[bitfield(name = "error", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ferror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "request", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "quit", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "noreply", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "noforward", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "fdone", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "swallow", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "9..=9")]
    pub error_ferror_request_quit_noreply_noforward_done_fdone_swallow_redis: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type msg_type_t = msg_type;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum msg_type {
    MSG_SENTINEL,
    MSG_RSP_REDIS_MULTIBULK,
    MSG_RSP_REDIS_BULK,
    MSG_RSP_REDIS_INTEGER,
    MSG_RSP_REDIS_ERROR_NOREPLICAS,
    MSG_RSP_REDIS_ERROR_MASTERDOWN,
    MSG_RSP_REDIS_ERROR_EXECABORT,
    MSG_RSP_REDIS_ERROR_WRONGTYPE,
    MSG_RSP_REDIS_ERROR_READONLY,
    MSG_RSP_REDIS_ERROR_NOSCRIPT,
    MSG_RSP_REDIS_ERROR_MISCONF,
    MSG_RSP_REDIS_ERROR_BUSYKEY,
    MSG_RSP_REDIS_ERROR_LOADING,
    MSG_RSP_REDIS_ERROR_NOAUTH,
    MSG_RSP_REDIS_ERROR_BUSY,
    MSG_RSP_REDIS_ERROR_OOM,
    MSG_RSP_REDIS_ERROR_ERR,
    MSG_RSP_REDIS_ERROR,
    MSG_RSP_REDIS_STATUS,
    MSG_REQ_REDIS_LOLWUT,
    MSG_REQ_REDIS_COMMAND,
    MSG_REQ_REDIS_SELECT,
    MSG_REQ_REDIS_AUTH,
    MSG_REQ_REDIS_QUIT,
    MSG_REQ_REDIS_PING,
    MSG_REQ_REDIS_EVALSHA,
    MSG_REQ_REDIS_EVAL,
    MSG_REQ_REDIS_GEOSEARCHSTORE,
    MSG_REQ_REDIS_GEOSEARCH,
    MSG_REQ_REDIS_GEOPOS,
    MSG_REQ_REDIS_GEORADIUSBYMEMBER,
    MSG_REQ_REDIS_GEORADIUS,
    MSG_REQ_REDIS_GEOHASH,
    MSG_REQ_REDIS_GEODIST,
    MSG_REQ_REDIS_GEOADD,
    MSG_REQ_REDIS_ZUNIONSTORE,
    MSG_REQ_REDIS_ZSCORE,
    MSG_REQ_REDIS_ZSCAN,
    MSG_REQ_REDIS_ZUNION,
    MSG_REQ_REDIS_ZREVRANK,
    MSG_REQ_REDIS_ZREVRANGEBYSCORE,
    MSG_REQ_REDIS_ZREVRANGEBYLEX,
    MSG_REQ_REDIS_ZREVRANGE,
    MSG_REQ_REDIS_ZREMRANGEBYSCORE,
    MSG_REQ_REDIS_ZREMRANGEBYLEX,
    MSG_REQ_REDIS_ZREMRANGEBYRANK,
    MSG_REQ_REDIS_ZREM,
    MSG_REQ_REDIS_ZRANK,
    MSG_REQ_REDIS_ZRANGESTORE,
    MSG_REQ_REDIS_ZRANGEBYSCORE,
    MSG_REQ_REDIS_ZRANGEBYLEX,
    MSG_REQ_REDIS_ZRANGE,
    MSG_REQ_REDIS_ZRANDMEMBER,
    MSG_REQ_REDIS_ZPOPMAX,
    MSG_REQ_REDIS_ZPOPMIN,
    MSG_REQ_REDIS_ZMSCORE,
    MSG_REQ_REDIS_ZLEXCOUNT,
    MSG_REQ_REDIS_ZINTERSTORE,
    MSG_REQ_REDIS_ZINTER,
    MSG_REQ_REDIS_ZINCRBY,
    MSG_REQ_REDIS_ZDIFFSTORE,
    MSG_REQ_REDIS_ZDIFF,
    MSG_REQ_REDIS_ZCOUNT,
    MSG_REQ_REDIS_ZCARD,
    MSG_REQ_REDIS_ZADD,
    MSG_REQ_REDIS_SSCAN,
    MSG_REQ_REDIS_SUNIONSTORE,
    MSG_REQ_REDIS_SUNION,
    MSG_REQ_REDIS_SREM,
    MSG_REQ_REDIS_SRANDMEMBER,
    MSG_REQ_REDIS_SPOP,
    MSG_REQ_REDIS_SMOVE,
    MSG_REQ_REDIS_SMEMBERS,
    MSG_REQ_REDIS_SMISMEMBER,
    MSG_REQ_REDIS_SISMEMBER,
    MSG_REQ_REDIS_SINTERSTORE,
    MSG_REQ_REDIS_SINTER,
    MSG_REQ_REDIS_SDIFFSTORE,
    MSG_REQ_REDIS_SDIFF,
    MSG_REQ_REDIS_SCARD,
    MSG_REQ_REDIS_SADD,
    MSG_REQ_REDIS_RPUSHX,
    MSG_REQ_REDIS_RPUSH,
    MSG_REQ_REDIS_RPOPLPUSH,
    MSG_REQ_REDIS_RPOP,
    MSG_REQ_REDIS_PFMERGE,
    MSG_REQ_REDIS_PFCOUNT,
    MSG_REQ_REDIS_PFADD,
    MSG_REQ_REDIS_LTRIM,
    MSG_REQ_REDIS_LSET,
    MSG_REQ_REDIS_LREM,
    MSG_REQ_REDIS_LRANGE,
    MSG_REQ_REDIS_LPUSHX,
    MSG_REQ_REDIS_LPUSH,
    MSG_REQ_REDIS_LPOS,
    MSG_REQ_REDIS_LPOP,
    MSG_REQ_REDIS_LMOVE,
    MSG_REQ_REDIS_LLEN,
    MSG_REQ_REDIS_LINSERT,
    MSG_REQ_REDIS_LINDEX,
    MSG_REQ_REDIS_HVALS,
    MSG_REQ_REDIS_HSTRLEN,
    MSG_REQ_REDIS_HSCAN,
    MSG_REQ_REDIS_HSETNX,
    MSG_REQ_REDIS_HSET,
    MSG_REQ_REDIS_HRANDFIELD,
    MSG_REQ_REDIS_HMSET,
    MSG_REQ_REDIS_HMGET,
    MSG_REQ_REDIS_HLEN,
    MSG_REQ_REDIS_HKEYS,
    MSG_REQ_REDIS_HINCRBYFLOAT,
    MSG_REQ_REDIS_HINCRBY,
    MSG_REQ_REDIS_HGETALL,
    MSG_REQ_REDIS_HGET,
    MSG_REQ_REDIS_HEXISTS,
    MSG_REQ_REDIS_HDEL,
    MSG_REQ_REDIS_STRLEN,
    MSG_REQ_REDIS_SETRANGE,
    MSG_REQ_REDIS_SETNX,
    MSG_REQ_REDIS_SETEX,
    MSG_REQ_REDIS_SETBIT,
    MSG_REQ_REDIS_SET,
    MSG_REQ_REDIS_RESTORE,
    MSG_REQ_REDIS_PSETEX,
    MSG_REQ_REDIS_MSET,
    MSG_REQ_REDIS_MGET,
    MSG_REQ_REDIS_INCRBYFLOAT,
    MSG_REQ_REDIS_INCRBY,
    MSG_REQ_REDIS_INCR,
    MSG_REQ_REDIS_GETSET,
    MSG_REQ_REDIS_GETRANGE,
    MSG_REQ_REDIS_GETEX,
    MSG_REQ_REDIS_GETDEL,
    MSG_REQ_REDIS_GETBIT,
    MSG_REQ_REDIS_GET,
    MSG_REQ_REDIS_DUMP,
    MSG_REQ_REDIS_DECRBY,
    MSG_REQ_REDIS_DECR,
    MSG_REQ_REDIS_BITPOS,
    MSG_REQ_REDIS_BITFIELD,
    MSG_REQ_REDIS_BITCOUNT,
    MSG_REQ_REDIS_APPEND,
    MSG_REQ_REDIS_UNLINK,
    MSG_REQ_REDIS_TYPE,
    MSG_REQ_REDIS_TTL,
    MSG_REQ_REDIS_TOUCH,
    MSG_REQ_REDIS_SORT,
    MSG_REQ_REDIS_PTTL,
    MSG_REQ_REDIS_PERSIST,
    MSG_REQ_REDIS_PEXPIREAT,
    MSG_REQ_REDIS_PEXPIRE,
    MSG_REQ_REDIS_MOVE,
    MSG_REQ_REDIS_EXPIREAT,
    MSG_REQ_REDIS_EXPIRE,
    MSG_REQ_REDIS_EXISTS,
    MSG_REQ_REDIS_DEL,
    MSG_REQ_REDIS_COPY,
    MSG_RSP_MC_SERVER_ERROR,
    MSG_RSP_MC_CLIENT_ERROR,
    MSG_RSP_MC_ERROR,
    MSG_RSP_MC_VERSION,
    MSG_RSP_MC_TOUCHED,
    MSG_RSP_MC_DELETED,
    MSG_RSP_MC_VALUE,
    MSG_RSP_MC_END,
    MSG_RSP_MC_NOT_FOUND,
    MSG_RSP_MC_EXISTS,
    MSG_RSP_MC_NOT_STORED,
    MSG_RSP_MC_STORED,
    MSG_RSP_MC_NUM,
    MSG_REQ_MC_VERSION,
    MSG_REQ_MC_QUIT,
    MSG_REQ_MC_TOUCH,
    MSG_REQ_MC_DECR,
    MSG_REQ_MC_INCR,
    MSG_REQ_MC_PREPEND,
    MSG_REQ_MC_APPEND,
    MSG_REQ_MC_REPLACE,
    MSG_REQ_MC_ADD,
    MSG_REQ_MC_SET,
    MSG_REQ_MC_CAS,
    MSG_REQ_MC_DELETE,
    MSG_REQ_MC_GETS,
    MSG_REQ_MC_GET,
    MSG_UNKNOWN,
}  // end of enum

pub type msg_coalesce_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
pub type msg_failure_t = Option::<unsafe extern "C" fn(*const msg) -> bool>;
pub type msg_add_auth_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
>;
pub type msg_reply_t = Option::<unsafe extern "C" fn(*mut msg) -> rstatus_t>;
pub type msg_fragment_t = Option::<
    unsafe extern "C" fn(*mut msg, uint32_t, *mut msg_tqh) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_tqh {
    pub tqh_first: *mut msg,
    pub tqh_last: *mut *mut msg,
}
pub type msg_parse_result_t = msg_parse_result;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum msg_parse_result {
    MSG_PARSE_OK,
    MSG_PARSE_ERROR,
    MSG_PARSE_REPAIR,
    MSG_PARSE_AGAIN,
}  // end of enum

pub type msg_parse_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mhdr {
    pub stqh_first: *mut mbuf,
    pub stqh_last: *mut *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf {
    pub magic: uint32_t,
    pub next: C2RustUnnamed_0,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub stqe_next: *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub key: int64_t,
    pub data: *mut libc::c_void,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
pub type conn_unref_t = Option::<unsafe extern "C" fn(*mut conn) -> ()>;
pub type conn_ref_t = Option::<unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> ()>;
pub type conn_swallow_msg_t = Option::<
    unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_post_connect_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut server) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub idx: uint32_t,
    pub owner: *mut server_pool,
    pub pname: string,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub weight: uint32_t,
    pub info: sockinfo,
    pub ns_conn_q: uint32_t,
    pub s_conn_q: conn_tqh,
    pub next_retry: int64_t,
    pub failure_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_tqh {
    pub tqh_first: *mut conn,
    pub tqh_last: *mut *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub family: libc::c_int,
    pub addrlen: socklen_t,
    pub addr: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct server_pool {
    pub idx: uint32_t,
    pub ctx: *mut context,
    pub p_conn: *mut conn,
    pub nc_conn_q: uint32_t,
    pub c_conn_q: conn_tqh,
    pub server: array,
    pub ncontinuum: uint32_t,
    pub nserver_continuum: uint32_t,
    pub continuum: *mut continuum,
    pub nlive_server: uint32_t,
    pub next_rebuild: int64_t,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub info: sockinfo,
    pub perm: mode_t,
    pub dist_type: libc::c_int,
    pub key_hash_type: libc::c_int,
    pub key_hash: hash_t,
    pub hash_tag: string,
    pub timeout: libc::c_int,
    pub backlog: libc::c_int,
    pub redis_db: libc::c_int,
    pub client_connections: uint32_t,
    pub server_connections: uint32_t,
    pub server_retry_timeout: int64_t,
    pub server_failure_limit: uint32_t,
    pub redis_auth: string,
    pub require_auth: libc::c_uint,
    #[bitfield(name = "auto_eject_hosts", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "preconnect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tcpkeepalive", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "reuseport", ty = "libc::c_uint", bits = "4..=4")]
    pub auto_eject_hosts_preconnect_redis_tcpkeepalive_reuseport: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t) -> uint32_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct continuum {
    pub index: uint32_t,
    pub value: uint32_t,
}
pub type conn_active_t = Option::<unsafe extern "C" fn(*const conn) -> bool>;
pub type conn_close_t = Option::<unsafe extern "C" fn(*mut context, *mut conn) -> ()>;
pub type conn_send_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
pub type conn_send_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
>;
pub type conn_send_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
pub type conn_recv_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_recv_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
>;
pub type conn_recv_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
pub type event_stats_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stats_type {
    STATS_INVALID,
    STATS_COUNTER,
    STATS_GAUGE,
    STATS_TIMESTAMP,
    STATS_SENTINEL,
}  // end of enum

pub type stats_type_t = stats_type;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_metric {
    pub type_0: stats_type_t,
    pub name: string,
    pub value: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub counter: int64_t,
    pub timestamp: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_server {
    pub name: string,
    pub metric: array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_pool {
    pub name: string,
    pub metric: array,
    pub server: array,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stats_pool_field {
    STATS_POOL_NFIELD,
    STATS_POOL_fragments,
    STATS_POOL_forward_error,
    STATS_POOL_server_ejects,
    STATS_POOL_client_connections,
    STATS_POOL_client_err,
    STATS_POOL_client_eof,
}  // end of enum

pub type stats_pool_field_t = stats_pool_field;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stats_server_field {
    STATS_SERVER_NFIELD,
    STATS_SERVER_out_queue_bytes,
    STATS_SERVER_out_queue,
    STATS_SERVER_in_queue_bytes,
    STATS_SERVER_in_queue,
    STATS_SERVER_response_bytes,
    STATS_SERVER_responses,
    STATS_SERVER_request_bytes,
    STATS_SERVER_requests,
    STATS_SERVER_server_ejected_at,
    STATS_SERVER_server_connections,
    STATS_SERVER_server_timedout,
    STATS_SERVER_server_err,
    STATS_SERVER_server_eof,
}  // end of enum

pub type stats_server_field_t = stats_server_field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_desc {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn array_null(mut a: *mut array) {
    (*a).nelem = 0 as libc::c_int as uint32_t;
    (*a).elem = 0 as *mut libc::c_void;
    (*a).size = 0 as libc::c_int as size_t;
    (*a).nalloc = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
static mut stats_pool_codec: [stats_metric; 6] = [stats_metric {
    type_0: STATS_INVALID,
    name: string {
        len: 0,
        data: 0 as *mut uint8_t,
    },
    value: C2RustUnnamed_6 { counter: 0 },
}; 6];
static mut stats_server_codec: [stats_metric; 13] = [stats_metric {
    type_0: STATS_INVALID,
    name: string {
        len: 0,
        data: 0 as *mut uint8_t,
    },
    value: C2RustUnnamed_6 { counter: 0 },
}; 13];
static mut stats_pool_desc: [stats_desc; 6] = [
    {
        let mut init = stats_desc {
            name: b"client_eof\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# eof on client connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"client_err\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# errors on client connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"client_connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# active client connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_ejects\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# times backend server was ejected\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"forward_error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# times we encountered a forwarding error\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"fragments\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# fragments created from a multi-vector request\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
static mut stats_server_desc: [stats_desc; 13] = [
    {
        let mut init = stats_desc {
            name: b"server_eof\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# eof on server connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_err\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# errors on server connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_timedout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# timeouts on server connections\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# active server connections\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_ejected_at\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"timestamp when server was ejected in usec since epoch\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"requests\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            desc: b"# requests\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"request_bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"total request bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"responses\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# responses\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"response_bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"total response bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"in_queue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            desc: b"# requests in incoming queue\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"in_queue_bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"current request bytes in incoming queue\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"out_queue\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"# requests in outgoing queue\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"out_queue_bytes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            desc: b"current request bytes in outgoing queue\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn stats_describe() {
    let mut i: uint32_t = 0;
    _log_stderr(b"pool stats:\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as uint32_t;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[stats_desc; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<stats_desc>() as libc::c_ulong)
    {
        _log_stderr(
            b"  %-20s\"%s\"\0" as *const u8 as *const libc::c_char,
            stats_pool_desc[i as usize].name,
            stats_pool_desc[i as usize].desc,
        );
        i = i.wrapping_add(1);
        i;
    }
    _log_stderr(b"\0" as *const u8 as *const libc::c_char);
    _log_stderr(b"server stats:\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as uint32_t;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[stats_desc; 13]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<stats_desc>() as libc::c_ulong)
    {
        _log_stderr(
            b"  %-20s\"%s\"\0" as *const u8 as *const libc::c_char,
            stats_server_desc[i as usize].name,
            stats_server_desc[i as usize].desc,
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn stats_metric_init(mut stm: *mut stats_metric) {
    match (*stm).type_0 as libc::c_uint {
        1 => {
            (*stm).value.counter = 0 as libc::c_longlong as int64_t;
        }
        2 => {
            (*stm).value.counter = 0 as libc::c_longlong as int64_t;
        }
        3 => {
            (*stm).value.timestamp = 0 as libc::c_longlong as int64_t;
        }
        _ => {}
    };
}
unsafe extern "C" fn stats_metric_reset(mut stats_metric: *mut array) {
    let mut i: uint32_t = 0;
    let mut nmetric: uint32_t = 0;
    nmetric = array_n(stats_metric);
    i = 0 as libc::c_int as uint32_t;
    while i < nmetric {
        let mut stm: *mut stats_metric = array_get(stats_metric, i) as *mut stats_metric;
        stats_metric_init(stm);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn stats_pool_metric_init(mut stats_metric: *mut array) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    let mut nfield: uint32_t = STATS_POOL_NFIELD as libc::c_int as uint32_t;
    status = array_init(
        stats_metric,
        nfield,
        ::core::mem::size_of::<stats_metric>() as libc::c_ulong,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < nfield {
        let mut stm: *mut stats_metric = array_push(stats_metric) as *mut stats_metric;
        *stm = stats_pool_codec[i as usize];
        stats_metric_init(stm);
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_server_metric_init(mut sts: *mut stats_server) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    let mut nfield: uint32_t = STATS_SERVER_NFIELD as libc::c_int as uint32_t;
    status = array_init(
        &mut (*sts).metric,
        nfield,
        ::core::mem::size_of::<stats_metric>() as libc::c_ulong,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < nfield {
        let mut stm: *mut stats_metric = array_push(&mut (*sts).metric)
            as *mut stats_metric;
        *stm = stats_server_codec[i as usize];
        stats_metric_init(stm);
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_metric_deinit(mut metric: *mut array) {
    let mut i: uint32_t = 0;
    let mut nmetric: uint32_t = 0;
    nmetric = array_n(metric);
    i = 0 as libc::c_int as uint32_t;
    while i < nmetric {
        array_pop(metric);
        i = i.wrapping_add(1);
        i;
    }
    array_deinit(metric);
}
unsafe extern "C" fn stats_server_init(
    mut sts: *mut stats_server,
    mut s: *mut server,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    (*sts).name = (*s).name;
    array_null(&mut (*sts).metric);
    status = stats_server_metric_init(sts);
    if status != 0 as libc::c_int {
        return status;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_server_map(
    mut stats_server: *mut array,
    mut server: *const array,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    let mut nserver: uint32_t = 0;
    nserver = array_n(server);
    status = array_init(
        stats_server,
        nserver,
        ::core::mem::size_of::<stats_server>() as libc::c_ulong,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < nserver {
        let mut s: *mut server = array_get(server, i) as *mut server;
        let mut sts: *mut stats_server = array_push(stats_server) as *mut stats_server;
        status = stats_server_init(sts, s);
        if status != 0 as libc::c_int {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_server_unmap(mut stats_server: *mut array) {
    let mut i: uint32_t = 0;
    let mut nserver: uint32_t = 0;
    nserver = array_n(stats_server);
    i = 0 as libc::c_int as uint32_t;
    while i < nserver {
        let mut sts: *mut stats_server = array_pop(stats_server) as *mut stats_server;
        stats_metric_deinit(&mut (*sts).metric);
        i = i.wrapping_add(1);
        i;
    }
    array_deinit(stats_server);
}
unsafe extern "C" fn stats_pool_init(
    mut stp: *mut stats_pool,
    mut sp: *const server_pool,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    (*stp).name = (*sp).name;
    array_null(&mut (*stp).metric);
    array_null(&mut (*stp).server);
    status = stats_pool_metric_init(&mut (*stp).metric);
    if status != 0 as libc::c_int {
        return status;
    }
    status = stats_server_map(&mut (*stp).server, &(*sp).server);
    if status != 0 as libc::c_int {
        stats_metric_deinit(&mut (*stp).metric);
        return status;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_pool_reset(mut stats_pool: *mut array) {
    let mut i: uint32_t = 0;
    let mut npool: uint32_t = 0;
    npool = array_n(stats_pool);
    i = 0 as libc::c_int as uint32_t;
    while i < npool {
        let mut stp: *mut stats_pool = array_get(stats_pool, i) as *mut stats_pool;
        let mut j: uint32_t = 0;
        let mut nserver: uint32_t = 0;
        stats_metric_reset(&mut (*stp).metric);
        nserver = array_n(&mut (*stp).server);
        j = 0 as libc::c_int as uint32_t;
        while j < nserver {
            let mut sts: *mut stats_server = array_get(&mut (*stp).server, j)
                as *mut stats_server;
            stats_metric_reset(&mut (*sts).metric);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn stats_pool_map(
    mut stats_pool: *mut array,
    mut server_pool: *const array,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    let mut npool: uint32_t = 0;
    npool = array_n(server_pool);
    status = array_init(
        stats_pool,
        npool,
        ::core::mem::size_of::<stats_pool>() as libc::c_ulong,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < npool {
        let mut sp: *const server_pool = array_get(server_pool, i) as *const server_pool;
        let mut stp: *mut stats_pool = array_push(stats_pool) as *mut stats_pool;
        status = stats_pool_init(stp, sp);
        if status != 0 as libc::c_int {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_pool_unmap(mut stats_pool: *mut array) {
    let mut i: uint32_t = 0;
    let mut npool: uint32_t = 0;
    npool = array_n(stats_pool);
    i = 0 as libc::c_int as uint32_t;
    while i < npool {
        let mut stp: *mut stats_pool = array_pop(stats_pool) as *mut stats_pool;
        stats_metric_deinit(&mut (*stp).metric);
        stats_server_unmap(&mut (*stp).server);
        i = i.wrapping_add(1);
        i;
    }
    array_deinit(stats_pool);
}
unsafe extern "C" fn stats_create_buf(mut st: *mut stats) -> rstatus_t {
    let mut int64_max_digits: uint32_t = 20 as libc::c_int as uint32_t;
    let mut key_value_extra: uint32_t = 8 as libc::c_int as uint32_t;
    let mut pool_extra: uint32_t = 8 as libc::c_int as uint32_t;
    let mut server_extra: uint32_t = 8 as libc::c_int as uint32_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut i: uint32_t = 0;
    size = (size as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).service_str.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).service.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).source_str.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).source.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).version_str.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).version.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).uptime_str.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(int64_max_digits as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add((*st).timestamp_str.len as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(int64_max_digits as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add((*st).ntotal_conn_str.len as libc::c_ulong) as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(int64_max_digits as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add((*st).ncurr_conn_str.len as libc::c_ulong) as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(int64_max_digits as libc::c_ulong)
        as size_t as size_t;
    size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
        as size_t as size_t;
    i = 0 as libc::c_int as uint32_t;
    while i < array_n(&mut (*st).sum) {
        let mut stp: *mut stats_pool = array_get(&mut (*st).sum, i) as *mut stats_pool;
        let mut j: uint32_t = 0;
        size = (size as libc::c_ulong).wrapping_add((*stp).name.len as libc::c_ulong)
            as size_t as size_t;
        size = (size as libc::c_ulong).wrapping_add(pool_extra as libc::c_ulong)
            as size_t as size_t;
        j = 0 as libc::c_int as uint32_t;
        while j < array_n(&mut (*stp).metric) {
            let mut stm: *mut stats_metric = array_get(&mut (*stp).metric, j)
                as *mut stats_metric;
            size = (size as libc::c_ulong).wrapping_add((*stm).name.len as libc::c_ulong)
                as size_t as size_t;
            size = (size as libc::c_ulong)
                .wrapping_add(int64_max_digits as libc::c_ulong) as size_t as size_t;
            size = (size as libc::c_ulong).wrapping_add(key_value_extra as libc::c_ulong)
                as size_t as size_t;
            j = j.wrapping_add(1);
            j;
        }
        j = 0 as libc::c_int as uint32_t;
        while j < array_n(&mut (*stp).server) {
            let mut sts: *mut stats_server = array_get(&mut (*stp).server, j)
                as *mut stats_server;
            let mut k: uint32_t = 0;
            size = (size as libc::c_ulong).wrapping_add((*sts).name.len as libc::c_ulong)
                as size_t as size_t;
            size = (size as libc::c_ulong).wrapping_add(server_extra as libc::c_ulong)
                as size_t as size_t;
            k = 0 as libc::c_int as uint32_t;
            while k < array_n(&mut (*sts).metric) {
                let mut stm_0: *mut stats_metric = array_get(&mut (*sts).metric, k)
                    as *mut stats_metric;
                size = (size as libc::c_ulong)
                    .wrapping_add((*stm_0).name.len as libc::c_ulong) as size_t
                    as size_t;
                size = (size as libc::c_ulong)
                    .wrapping_add(int64_max_digits as libc::c_ulong) as size_t as size_t;
                size = (size as libc::c_ulong)
                    .wrapping_add(key_value_extra as libc::c_ulong) as size_t as size_t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    size = (size as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    size = size
        .wrapping_add(
            (::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        & !(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    (*st)
        .buf
        .data = _nc_alloc(
        size,
        b"nc_stats.c\0" as *const u8 as *const libc::c_char,
        411 as libc::c_int,
    ) as *mut uint8_t;
    if ((*st).buf.data).is_null() {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                414 as libc::c_int,
                0 as libc::c_int,
                b"create stats buffer of size %zu failed: %s\0" as *const u8
                    as *const libc::c_char,
                size,
                strerror(*__errno_location()),
            );
        }
        return -(3 as libc::c_int);
    }
    (*st).buf.size = size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_destroy_buf(mut st: *mut stats) {
    if (*st).buf.size != 0 as libc::c_int as libc::c_ulong {
        _nc_free(
            (*st).buf.data as *mut libc::c_void,
            b"nc_stats.c\0" as *const u8 as *const libc::c_char,
            429 as libc::c_int,
        );
        (*st).buf.data = 0 as *mut uint8_t;
        (*st).buf.size = 0 as libc::c_int as size_t;
    }
}
unsafe extern "C" fn stats_add_string(
    mut st: *mut stats,
    mut key: *const string,
    mut val: *const string,
) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    let mut room: size_t = 0;
    let mut n: libc::c_int = 0;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    room = ((*buf).size)
        .wrapping_sub((*buf).len)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    n = snprintf(
        pos as *mut libc::c_char,
        room,
        b"\"%.*s\":\"%.*s\", \0" as *const u8 as *const libc::c_char,
        (*key).len,
        (*key).data,
        (*val).len,
        (*val).data,
    );
    if n < 0 as libc::c_int || n >= room as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*buf)
        .len = ((*buf).len as libc::c_ulong).wrapping_add(n as size_t) as size_t
        as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_add_num(
    mut st: *mut stats,
    mut key: *const string,
    mut val: int64_t,
) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    let mut room: size_t = 0;
    let mut n: libc::c_int = 0;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    room = ((*buf).size)
        .wrapping_sub((*buf).len)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    n = snprintf(
        pos as *mut libc::c_char,
        room,
        b"\"%.*s\":%ld, \0" as *const u8 as *const libc::c_char,
        (*key).len,
        (*key).data,
        val,
    );
    if n < 0 as libc::c_int || n >= room as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*buf)
        .len = ((*buf).len as libc::c_ulong).wrapping_add(n as size_t) as size_t
        as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_add_header(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut cur_ts: int64_t = 0;
    let mut uptime: int64_t = 0;
    buf = &mut (*st).buf;
    *((*buf).data).offset(0 as libc::c_int as isize) = '{' as i32 as uint8_t;
    (*buf).len = 1 as libc::c_int as size_t;
    cur_ts = time(0 as *mut time_t);
    uptime = cur_ts - (*st).start_ts;
    status = stats_add_string(st, &mut (*st).service_str, &mut (*st).service);
    if status != 0 as libc::c_int {
        return status;
    }
    status = stats_add_string(st, &mut (*st).source_str, &mut (*st).source);
    if status != 0 as libc::c_int {
        return status;
    }
    status = stats_add_string(st, &mut (*st).version_str, &mut (*st).version);
    if status != 0 as libc::c_int {
        return status;
    }
    status = stats_add_num(st, &mut (*st).uptime_str, uptime);
    if status != 0 as libc::c_int {
        return status;
    }
    status = stats_add_num(st, &mut (*st).timestamp_str, cur_ts);
    if status != 0 as libc::c_int {
        return status;
    }
    status = stats_add_num(
        st,
        &mut (*st).ntotal_conn_str,
        conn_ntotal_conn() as int64_t,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    status = stats_add_num(st, &mut (*st).ncurr_conn_str, conn_ncurr_conn() as int64_t);
    if status != 0 as libc::c_int {
        return status;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_add_footer(mut st: *mut stats) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    buf = &mut (*st).buf;
    if (*buf).len == (*buf).size {
        return -(1 as libc::c_int);
    }
    pos = ((*buf).data).offset((*buf).len as isize).offset(-(1 as libc::c_int as isize));
    *pos.offset(0 as libc::c_int as isize) = '}' as i32 as uint8_t;
    *pos.offset(1 as libc::c_int as isize) = '\n' as i32 as uint8_t;
    (*buf)
        .len = ((*buf).len as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_begin_nesting(
    mut st: *mut stats,
    mut key: *const string,
) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    let mut room: size_t = 0;
    let mut n: libc::c_int = 0;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    room = ((*buf).size)
        .wrapping_sub((*buf).len)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    n = snprintf(
        pos as *mut libc::c_char,
        room,
        b"\"%.*s\": {\0" as *const u8 as *const libc::c_char,
        (*key).len,
        (*key).data,
    );
    if n < 0 as libc::c_int || n >= room as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*buf)
        .len = ((*buf).len as libc::c_ulong).wrapping_add(n as size_t) as size_t
        as size_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_end_nesting(mut st: *mut stats) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    pos = pos.offset(-(2 as libc::c_int as isize));
    match *pos.offset(0 as libc::c_int as isize) as libc::c_int {
        44 => {
            *pos.offset(0 as libc::c_int as isize) = '}' as i32 as uint8_t;
            *pos.offset(1 as libc::c_int as isize) = ',' as i32 as uint8_t;
        }
        125 => {
            if (*buf).len == (*buf).size {
                return -(1 as libc::c_int);
            }
            *pos.offset(1 as libc::c_int as isize) = '}' as i32 as uint8_t;
            *pos.offset(2 as libc::c_int as isize) = ',' as i32 as uint8_t;
            (*buf)
                .len = ((*buf).len as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_copy_metric(
    mut st: *mut stats,
    mut metric: *mut array,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < array_n(metric) {
        let mut stm: *mut stats_metric = array_get(metric, i) as *mut stats_metric;
        status = stats_add_num(st, &mut (*stm).name, (*stm).value.counter);
        if status != 0 as libc::c_int {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_aggregate_metric(mut dst: *mut array, mut src: *const array) {
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < array_n(src) {
        let mut stm1: *const stats_metric = 0 as *const stats_metric;
        let mut stm2: *mut stats_metric = 0 as *mut stats_metric;
        stm1 = array_get(src, i) as *const stats_metric;
        stm2 = array_get(dst, i) as *mut stats_metric;
        match (*stm1).type_0 as libc::c_uint {
            1 => {
                (*stm2).value.counter += (*stm1).value.counter;
            }
            2 => {
                (*stm2).value.counter += (*stm1).value.counter;
            }
            3 => {
                if (*stm1).value.timestamp != 0 {
                    (*stm2).value.timestamp = (*stm1).value.timestamp;
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn stats_aggregate(mut st: *mut stats) {
    let mut i: uint32_t = 0;
    if (*st).aggregate == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < array_n(&mut (*st).shadow) {
        let mut stp1: *mut stats_pool = 0 as *mut stats_pool;
        let mut stp2: *mut stats_pool = 0 as *mut stats_pool;
        let mut j: uint32_t = 0;
        stp1 = array_get(&mut (*st).shadow, i) as *mut stats_pool;
        stp2 = array_get(&mut (*st).sum, i) as *mut stats_pool;
        stats_aggregate_metric(&mut (*stp2).metric, &mut (*stp1).metric);
        j = 0 as libc::c_int as uint32_t;
        while j < array_n(&mut (*stp1).server) {
            let mut sts1: *mut stats_server = 0 as *mut stats_server;
            let mut sts2: *mut stats_server = 0 as *mut stats_server;
            sts1 = array_get(&mut (*stp1).server, j) as *mut stats_server;
            sts2 = array_get(&mut (*stp2).server, j) as *mut stats_server;
            stats_aggregate_metric(&mut (*sts2).metric, &mut (*sts1).metric);
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    ::core::ptr::write_volatile(
        &mut (*st).aggregate as *mut libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn stats_make_rsp(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    status = stats_add_header(st);
    if status != 0 as libc::c_int {
        return status;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < array_n(&mut (*st).sum) {
        let mut stp: *mut stats_pool = array_get(&mut (*st).sum, i) as *mut stats_pool;
        let mut j: uint32_t = 0;
        status = stats_begin_nesting(st, &mut (*stp).name);
        if status != 0 as libc::c_int {
            return status;
        }
        status = stats_copy_metric(st, &mut (*stp).metric);
        if status != 0 as libc::c_int {
            return status;
        }
        j = 0 as libc::c_int as uint32_t;
        while j < array_n(&mut (*stp).server) {
            let mut sts: *mut stats_server = array_get(&mut (*stp).server, j)
                as *mut stats_server;
            status = stats_begin_nesting(st, &mut (*sts).name);
            if status != 0 as libc::c_int {
                return status;
            }
            status = stats_copy_metric(st, &mut (*sts).metric);
            if status != 0 as libc::c_int {
                return status;
            }
            status = stats_end_nesting(st);
            if status != 0 as libc::c_int {
                return status;
            }
            j = j.wrapping_add(1);
            j;
        }
        status = stats_end_nesting(st);
        if status != 0 as libc::c_int {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    status = stats_add_footer(st);
    if status != 0 as libc::c_int {
        return status;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_send_rsp(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut n: ssize_t = 0;
    let mut sd: libc::c_int = 0;
    status = stats_make_rsp(st);
    if status != 0 as libc::c_int {
        return status;
    }
    sd = accept(
        (*st).sd,
        __SOCKADDR_ARG {
            __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
        },
        0 as *mut socklen_t,
    );
    if sd < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                773 as libc::c_int,
                0 as libc::c_int,
                b"accept on m %d failed: %s\0" as *const u8 as *const libc::c_char,
                (*st).sd,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    n = _nc_sendn(sd, (*st).buf.data as *const libc::c_void, (*st).buf.len);
    if n < 0 as libc::c_int as libc::c_long {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                781 as libc::c_int,
                0 as libc::c_int,
                b"send stats on sd %d failed: %s\0" as *const u8 as *const libc::c_char,
                sd,
                strerror(*__errno_location()),
            );
        }
        close(sd);
        return -(1 as libc::c_int);
    }
    close(sd);
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_loop_callback(
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
) {
    let mut st: *mut stats = arg1 as *mut stats;
    let mut n: libc::c_int = *(arg2 as *mut libc::c_int);
    stats_aggregate(st);
    if n == 0 as libc::c_int {
        return;
    }
    stats_send_rsp(st);
}
unsafe extern "C" fn stats_loop(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    event_loop_stats(
        Some(
            stats_loop_callback
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        arg,
    );
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn stats_listen(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut si: sockinfo = sockinfo {
        family: 0,
        addrlen: 0,
        addr: C2RustUnnamed_4 {
            in_0: sockaddr_in {
                sin_family: 0,
                sin_port: 0,
                sin_addr: in_addr { s_addr: 0 },
                sin_zero: [0; 8],
            },
        },
    };
    status = nc_resolve(&mut (*st).addr, (*st).port as libc::c_int, &mut si);
    if status < 0 as libc::c_int {
        return status;
    }
    (*st).sd = socket(si.family, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if (*st).sd < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                828 as libc::c_int,
                0 as libc::c_int,
                b"socket failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    nc_set_reuseport((*st).sd);
    status = nc_set_reuseaddr((*st).sd);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                834 as libc::c_int,
                0 as libc::c_int,
                b"set reuseaddr on m %d failed for stats server: %s\0" as *const u8
                    as *const libc::c_char,
                (*st).sd,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    status = bind(
        (*st).sd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut si.addr as *mut C2RustUnnamed_4 as *mut sockaddr,
        },
        si.addrlen,
    );
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                841 as libc::c_int,
                0 as libc::c_int,
                b"bind on m %d to stats server addr '%.*s:%u' failed: %s\0" as *const u8
                    as *const libc::c_char,
                (*st).sd,
                (*st).addr.len,
                (*st).addr.data,
                (*st).port as libc::c_int,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    status = listen((*st).sd, 128 as libc::c_int);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                848 as libc::c_int,
                0 as libc::c_int,
                b"listen on m %d for stats server '%.*s:%u' failed: %s\0" as *const u8
                    as *const libc::c_char,
                (*st).sd,
                (*st).addr.len,
                (*st).addr.data,
                (*st).port as libc::c_int,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_start_aggregator(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    if 1 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    status = stats_listen(st);
    if status != 0 as libc::c_int {
        return status;
    }
    status = pthread_create(
        &mut (*st).tid,
        0 as *const pthread_attr_t,
        Some(stats_loop as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        st as *mut libc::c_void,
    );
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_stats.c\0" as *const u8 as *const libc::c_char,
                874 as libc::c_int,
                0 as libc::c_int,
                b"stats aggregator create failed: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(status),
            );
        }
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stats_stop_aggregator(mut st: *mut stats) {
    if 1 as libc::c_int == 0 {
        return;
    }
    close((*st).sd);
}
#[no_mangle]
pub unsafe extern "C" fn stats_create(
    mut stats_port: uint16_t,
    mut stats_ip: *const libc::c_char,
    mut stats_interval: libc::c_int,
    mut source: *const libc::c_char,
    mut server_pool: *const array,
) -> *mut stats {
    let mut status: rstatus_t = 0;
    let mut st: *mut stats = 0 as *mut stats;
    st = _nc_alloc(
        ::core::mem::size_of::<stats>() as libc::c_ulong,
        b"nc_stats.c\0" as *const u8 as *const libc::c_char,
        898 as libc::c_int,
    ) as *mut stats;
    if st.is_null() {
        return 0 as *mut stats;
    }
    (*st).port = stats_port;
    (*st).interval = stats_interval;
    (*st).addr.len = strlen(stats_ip as *mut libc::c_char) as uint32_t;
    (*st).addr.data = stats_ip as *mut uint8_t;
    (*st).start_ts = time(0 as *mut time_t);
    (*st).buf.len = 0 as libc::c_int as size_t;
    (*st).buf.data = 0 as *mut uint8_t;
    (*st).buf.size = 0 as libc::c_int as size_t;
    array_null(&mut (*st).current);
    array_null(&mut (*st).shadow);
    array_null(&mut (*st).sum);
    (*st).tid = -(1 as libc::c_int) as pthread_t;
    (*st).sd = -(1 as libc::c_int);
    (*st)
        .service_str
        .len = (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .service_str
        .data = b"service\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*st)
        .service
        .len = (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .service
        .data = b"nutcracker\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*st)
        .source_str
        .len = (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .source_str
        .data = b"source\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*st).source.len = strlen(source as *mut libc::c_char) as uint32_t;
    (*st).source.data = source as *mut uint8_t;
    (*st)
        .version_str
        .len = (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .version_str
        .data = b"version\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*st)
        .version
        .len = (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st).version.data = b"0.5.0\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*st)
        .uptime_str
        .len = (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .uptime_str
        .data = b"uptime\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*st)
        .timestamp_str
        .len = (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .timestamp_str
        .data = b"timestamp\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*st)
        .ntotal_conn_str
        .len = (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .ntotal_conn_str
        .data = b"total_connections\0" as *const u8 as *const libc::c_char
        as *mut uint8_t;
    (*st)
        .ncurr_conn_str
        .len = (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t;
    (*st)
        .ncurr_conn_str
        .data = b"curr_connections\0" as *const u8 as *const libc::c_char
        as *mut uint8_t;
    ::core::ptr::write_volatile(
        &mut (*st).updated as *mut libc::c_int,
        0 as libc::c_int,
    );
    ::core::ptr::write_volatile(
        &mut (*st).aggregate as *mut libc::c_int,
        0 as libc::c_int,
    );
    status = stats_pool_map(&mut (*st).current, server_pool);
    if !(status != 0 as libc::c_int) {
        status = stats_pool_map(&mut (*st).shadow, server_pool);
        if !(status != 0 as libc::c_int) {
            status = stats_pool_map(&mut (*st).sum, server_pool);
            if !(status != 0 as libc::c_int) {
                status = stats_create_buf(st);
                if !(status != 0 as libc::c_int) {
                    status = stats_start_aggregator(st);
                    if !(status != 0 as libc::c_int) {
                        return st;
                    }
                }
            }
        }
    }
    stats_destroy(st);
    return 0 as *mut stats;
}
#[no_mangle]
pub unsafe extern "C" fn stats_destroy(mut st: *mut stats) {
    stats_stop_aggregator(st);
    stats_pool_unmap(&mut (*st).sum);
    stats_pool_unmap(&mut (*st).shadow);
    stats_pool_unmap(&mut (*st).current);
    stats_destroy_buf(st);
    _nc_free(
        st as *mut libc::c_void,
        b"nc_stats.c\0" as *const u8 as *const libc::c_char,
        980 as libc::c_int,
    );
    st = 0 as *mut stats;
}
#[no_mangle]
pub unsafe extern "C" fn stats_swap(mut st: *mut stats) {
    if 1 as libc::c_int == 0 {
        return;
    }
    if (*st).aggregate == 1 as libc::c_int {
        return;
    }
    if (*st).updated == 0 as libc::c_int {
        return;
    }
    array_swap(&mut (*st).current, &mut (*st).shadow);
    stats_pool_reset(&mut (*st).current);
    ::core::ptr::write_volatile(
        &mut (*st).updated as *mut libc::c_int,
        0 as libc::c_int,
    );
    ::core::ptr::write_volatile(
        &mut (*st).aggregate as *mut libc::c_int,
        1 as libc::c_int,
    );
}
unsafe extern "C" fn stats_pool_to_metric(
    mut ctx: *mut context,
    mut pool: *const server_pool,
    mut fidx: stats_pool_field_t,
) -> *mut stats_metric {
    let mut st: *mut stats = 0 as *mut stats;
    let mut stp: *mut stats_pool = 0 as *mut stats_pool;
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    let mut pidx: uint32_t = 0;
    pidx = (*pool).idx;
    st = (*ctx).stats;
    stp = array_get(&mut (*st).current, pidx) as *mut stats_pool;
    stm = array_get(&mut (*stp).metric, fidx as uint32_t) as *mut stats_metric;
    ::core::ptr::write_volatile(
        &mut (*st).updated as *mut libc::c_int,
        1 as libc::c_int,
    );
    return stm;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_pool_incr(
    mut ctx: *mut context,
    mut pool: *const server_pool,
    mut fidx: stats_pool_field_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_pool_to_metric(ctx, pool, fidx);
    (*stm).value.counter += 1;
    (*stm).value.counter;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_pool_decr(
    mut ctx: *mut context,
    mut pool: *const server_pool,
    mut fidx: stats_pool_field_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_pool_to_metric(ctx, pool, fidx);
    (*stm).value.counter -= 1;
    (*stm).value.counter;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_pool_incr_by(
    mut ctx: *mut context,
    mut pool: *const server_pool,
    mut fidx: stats_pool_field_t,
    mut val: int64_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_pool_to_metric(ctx, pool, fidx);
    (*stm).value.counter += val;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_pool_decr_by(
    mut ctx: *mut context,
    mut pool: *const server_pool,
    mut fidx: stats_pool_field_t,
    mut val: int64_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_pool_to_metric(ctx, pool, fidx);
    (*stm).value.counter -= val;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_pool_set_ts(
    mut ctx: *mut context,
    mut pool: *const server_pool,
    mut fidx: stats_pool_field_t,
    mut val: int64_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_pool_to_metric(ctx, pool, fidx);
    (*stm).value.timestamp = val;
}
unsafe extern "C" fn stats_server_to_metric(
    mut ctx: *mut context,
    mut server: *const server,
    mut fidx: stats_server_field_t,
) -> *mut stats_metric {
    let mut st: *mut stats = 0 as *mut stats;
    let mut stp: *mut stats_pool = 0 as *mut stats_pool;
    let mut sts: *mut stats_server = 0 as *mut stats_server;
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    let mut pidx: uint32_t = 0;
    let mut sidx: uint32_t = 0;
    sidx = (*server).idx;
    pidx = (*(*server).owner).idx;
    st = (*ctx).stats;
    stp = array_get(&mut (*st).current, pidx) as *mut stats_pool;
    sts = array_get(&mut (*stp).server, sidx) as *mut stats_server;
    stm = array_get(&mut (*sts).metric, fidx as uint32_t) as *mut stats_metric;
    ::core::ptr::write_volatile(
        &mut (*st).updated as *mut libc::c_int,
        1 as libc::c_int,
    );
    return stm;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_server_incr(
    mut ctx: *mut context,
    mut server: *const server,
    mut fidx: stats_server_field_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_server_to_metric(ctx, server, fidx);
    (*stm).value.counter += 1;
    (*stm).value.counter;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_server_decr(
    mut ctx: *mut context,
    mut server: *const server,
    mut fidx: stats_server_field_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_server_to_metric(ctx, server, fidx);
    (*stm).value.counter -= 1;
    (*stm).value.counter;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_server_incr_by(
    mut ctx: *mut context,
    mut server: *const server,
    mut fidx: stats_server_field_t,
    mut val: int64_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_server_to_metric(ctx, server, fidx);
    (*stm).value.counter += val;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_server_decr_by(
    mut ctx: *mut context,
    mut server: *const server,
    mut fidx: stats_server_field_t,
    mut val: int64_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_server_to_metric(ctx, server, fidx);
    (*stm).value.counter -= val;
}
#[no_mangle]
pub unsafe extern "C" fn _stats_server_set_ts(
    mut ctx: *mut context,
    mut server: *const server,
    mut fidx: stats_server_field_t,
    mut val: int64_t,
) {
    let mut stm: *mut stats_metric = 0 as *mut stats_metric;
    stm = stats_server_to_metric(ctx, server, fidx);
    (*stm).value.timestamp = val;
}
unsafe extern "C" fn run_static_initializers() {
    stats_pool_codec = [
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"client_eof\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"client_err\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 19]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"client_connections\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 14]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"server_ejects\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 14]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"forward_error\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"fragments\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
    ];
    stats_server_codec = [
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"server_eof\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 11]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"server_err\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 16]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"server_timedout\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 19]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"server_connections\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_TIMESTAMP,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 18]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"server_ejected_at\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 9]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"requests\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 14]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"request_bytes\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"responses\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 15]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"response_bytes\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 9]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"in_queue\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 15]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"in_queue_bytes\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 10]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"out_queue\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[libc::c_char; 16]>()
                            as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint32_t,
                        data: b"out_queue_bytes\0" as *const u8 as *const libc::c_char
                            as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
