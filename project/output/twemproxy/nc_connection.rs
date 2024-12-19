#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn _nc_alloc(
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn memcache_post_connect(ctx: *mut context, conn: *mut conn, server: *mut server);
    fn memcache_swallow_msg(conn: *mut conn, pmsg: *mut msg, msg: *mut msg);
    fn redis_post_connect(ctx: *mut context, conn: *mut conn, server: *mut server);
    fn redis_swallow_msg(conn: *mut conn, pmsg: *mut msg, msg: *mut msg);
    fn msg_recv(ctx: *mut context, conn: *mut conn) -> rstatus_t;
    fn msg_send(ctx: *mut context, conn: *mut conn) -> rstatus_t;
    fn req_server_enqueue_imsgq(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn req_server_dequeue_imsgq(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn req_client_enqueue_omsgq(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn req_server_enqueue_omsgq(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn req_client_dequeue_omsgq(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn req_server_dequeue_omsgq(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn req_recv_next(ctx: *mut context, conn: *mut conn, alloc: bool) -> *mut msg;
    fn req_recv_done(ctx: *mut context, conn: *mut conn, msg: *mut msg, nmsg: *mut msg);
    fn req_send_next(ctx: *mut context, conn: *mut conn) -> *mut msg;
    fn req_send_done(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn rsp_recv_next(ctx: *mut context, conn: *mut conn, alloc: bool) -> *mut msg;
    fn rsp_recv_done(ctx: *mut context, conn: *mut conn, msg: *mut msg, nmsg: *mut msg);
    fn rsp_send_next(ctx: *mut context, conn: *mut conn) -> *mut msg;
    fn rsp_send_done(ctx: *mut context, conn: *mut conn, msg: *mut msg);
    fn server_unref(conn: *mut conn);
    fn server_ref(conn: *mut conn, owner: *mut libc::c_void);
    fn server_active(conn: *const conn) -> bool;
    fn server_close(ctx: *mut context, conn: *mut conn);
    fn client_active(conn: *const conn) -> bool;
    fn client_ref(conn: *mut conn, owner: *mut libc::c_void);
    fn client_unref(conn: *mut conn);
    fn client_close(ctx: *mut context, conn: *mut conn);
    fn proxy_ref(conn: *mut conn, owner: *mut libc::c_void);
    fn proxy_unref(conn: *mut conn);
    fn proxy_close(ctx: *mut context, conn: *mut conn);
    fn proxy_recv(ctx: *mut context, conn: *mut conn) -> rstatus_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
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
pub type uint16_t = __uint16_t;
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
    pub c_tqe: C2RustUnnamed_2,
    pub s_tqe: C2RustUnnamed_1,
    pub m_tqe: C2RustUnnamed_0,
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
pub type uint64_t = __uint64_t;
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
    MSG_PARSE_AGAIN,
    MSG_PARSE_REPAIR,
    MSG_PARSE_ERROR,
    MSG_PARSE_OK,
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
    pub next: C2RustUnnamed,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
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
    pub addr: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
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
pub type socklen_t = __socklen_t;
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
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
static mut nfree_connq: uint32_t = 0;
static mut free_connq: conn_tqh = conn_tqh {
    tqh_first: 0 as *const conn as *mut conn,
    tqh_last: 0 as *const *mut conn as *mut *mut conn,
};
static mut ntotal_conn: uint64_t = 0;
static mut ncurr_conn: uint32_t = 0;
static mut ncurr_cconn: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn conn_to_ctx(mut conn: *const conn) -> *mut context {
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    if (*conn).proxy() as libc::c_int != 0 || (*conn).client() as libc::c_int != 0 {
        pool = (*conn).owner as *mut server_pool;
    } else {
        let mut server: *mut server = (*conn).owner as *mut server;
        pool = (*server).owner;
    }
    return (*pool).ctx;
}
unsafe extern "C" fn _conn_get() -> *mut conn {
    let mut conn: *mut conn = 0 as *mut conn;
    if !(free_connq.tqh_first).is_null() {
        conn = free_connq.tqh_first;
        nfree_connq = nfree_connq.wrapping_sub(1);
        nfree_connq;
        let mut oldnext: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_next
            as *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
        let mut oldprev: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_prev
            as *mut *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
        if !((*conn).conn_tqe.tqe_next).is_null() {
            (*(*conn).conn_tqe.tqe_next).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_prev;
        } else {
            free_connq.tqh_last = (*conn).conn_tqe.tqe_prev;
        }
        *(*conn).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_next;
        *oldnext = 0 as *mut libc::c_void;
        *oldprev = 0 as *mut libc::c_void;
    } else {
        conn = _nc_alloc(
            ::core::mem::size_of::<conn>() as libc::c_ulong,
            b"nc_connection.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
        ) as *mut conn;
        if conn.is_null() {
            return 0 as *mut conn;
        }
    }
    (*conn).owner = 0 as *mut libc::c_void;
    (*conn).sd = -(1 as libc::c_int);
    (*conn).imsg_q.tqh_first = 0 as *mut msg;
    (*conn).imsg_q.tqh_last = &mut (*conn).imsg_q.tqh_first;
    (*conn).omsg_q.tqh_first = 0 as *mut msg;
    (*conn).omsg_q.tqh_last = &mut (*conn).omsg_q.tqh_first;
    (*conn).rmsg = 0 as *mut msg;
    (*conn).smsg = 0 as *mut msg;
    (*conn).send_bytes = 0 as libc::c_int as size_t;
    (*conn).recv_bytes = 0 as libc::c_int as size_t;
    (*conn).events = 0 as libc::c_int as uint32_t;
    (*conn).err = 0 as libc::c_int;
    (*conn).set_recv_active(0 as libc::c_int as libc::c_uint);
    (*conn).set_recv_ready(0 as libc::c_int as libc::c_uint);
    (*conn).set_send_active(0 as libc::c_int as libc::c_uint);
    (*conn).set_send_ready(0 as libc::c_int as libc::c_uint);
    (*conn).set_client(0 as libc::c_int as libc::c_uint);
    (*conn).set_proxy(0 as libc::c_int as libc::c_uint);
    (*conn).set_connecting(0 as libc::c_int as libc::c_uint);
    (*conn).set_connected(0 as libc::c_int as libc::c_uint);
    (*conn).set_eof(0 as libc::c_int as libc::c_uint);
    (*conn).set_done(0 as libc::c_int as libc::c_uint);
    (*conn).set_redis(0 as libc::c_int as libc::c_uint);
    (*conn).set_authenticated(0 as libc::c_int as libc::c_uint);
    ntotal_conn = ntotal_conn.wrapping_add(1);
    ntotal_conn;
    ncurr_conn = ncurr_conn.wrapping_add(1);
    ncurr_conn;
    return conn;
}
#[no_mangle]
pub unsafe extern "C" fn conn_get(
    mut owner: *mut libc::c_void,
    mut client: bool,
    mut redis: bool,
) -> *mut conn {
    let mut conn: *mut conn = 0 as *mut conn;
    conn = _conn_get();
    if conn.is_null() {
        return 0 as *mut conn;
    }
    (*conn)
        .set_redis(
            (if redis as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_uint,
        );
    (*conn)
        .set_client(
            (if client as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
        );
    if (*conn).client() != 0 {
        (*conn)
            .recv = Some(
            msg_recv as unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
        );
        (*conn)
            .recv_next = Some(
            req_recv_next
                as unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
        );
        (*conn)
            .recv_done = Some(
            req_recv_done
                as unsafe extern "C" fn(
                    *mut context,
                    *mut conn,
                    *mut msg,
                    *mut msg,
                ) -> (),
        );
        (*conn)
            .send = Some(
            msg_send as unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
        );
        (*conn)
            .send_next = Some(
            rsp_send_next as unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
        );
        (*conn)
            .send_done = Some(
            rsp_send_done
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        (*conn)
            .close = Some(
            client_close as unsafe extern "C" fn(*mut context, *mut conn) -> (),
        );
        (*conn)
            .active = Some(client_active as unsafe extern "C" fn(*const conn) -> bool);
        (*conn)
            .ref_0 = Some(
            client_ref as unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> (),
        );
        (*conn).unref = Some(client_unref as unsafe extern "C" fn(*mut conn) -> ());
        (*conn).enqueue_inq = None;
        (*conn).dequeue_inq = None;
        (*conn)
            .enqueue_outq = Some(
            req_client_enqueue_omsgq
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        (*conn)
            .dequeue_outq = Some(
            req_client_dequeue_omsgq
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        (*conn).post_connect = None;
        (*conn).swallow_msg = None;
        ncurr_cconn = ncurr_cconn.wrapping_add(1);
        ncurr_cconn;
    } else {
        (*conn)
            .recv = Some(
            msg_recv as unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
        );
        (*conn)
            .recv_next = Some(
            rsp_recv_next
                as unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
        );
        (*conn)
            .recv_done = Some(
            rsp_recv_done
                as unsafe extern "C" fn(
                    *mut context,
                    *mut conn,
                    *mut msg,
                    *mut msg,
                ) -> (),
        );
        (*conn)
            .send = Some(
            msg_send as unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
        );
        (*conn)
            .send_next = Some(
            req_send_next as unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
        );
        (*conn)
            .send_done = Some(
            req_send_done
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        (*conn)
            .close = Some(
            server_close as unsafe extern "C" fn(*mut context, *mut conn) -> (),
        );
        (*conn)
            .active = Some(server_active as unsafe extern "C" fn(*const conn) -> bool);
        (*conn)
            .ref_0 = Some(
            server_ref as unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> (),
        );
        (*conn).unref = Some(server_unref as unsafe extern "C" fn(*mut conn) -> ());
        (*conn)
            .enqueue_inq = Some(
            req_server_enqueue_imsgq
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        (*conn)
            .dequeue_inq = Some(
            req_server_dequeue_imsgq
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        (*conn)
            .enqueue_outq = Some(
            req_server_enqueue_omsgq
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        (*conn)
            .dequeue_outq = Some(
            req_server_dequeue_omsgq
                as unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
        );
        if redis {
            (*conn)
                .post_connect = Some(
                redis_post_connect
                    as unsafe extern "C" fn(*mut context, *mut conn, *mut server) -> (),
            );
            (*conn)
                .swallow_msg = Some(
                redis_swallow_msg
                    as unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
            );
        } else {
            (*conn)
                .post_connect = Some(
                memcache_post_connect
                    as unsafe extern "C" fn(*mut context, *mut conn, *mut server) -> (),
            );
            (*conn)
                .swallow_msg = Some(
                memcache_swallow_msg
                    as unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
            );
        }
    }
    ((*conn).ref_0).expect("non-null function pointer")(conn, owner);
    return conn;
}
#[no_mangle]
pub unsafe extern "C" fn conn_get_proxy(mut pool: *mut server_pool) -> *mut conn {
    let mut conn: *mut conn = 0 as *mut conn;
    conn = _conn_get();
    if conn.is_null() {
        return 0 as *mut conn;
    }
    (*conn).set_redis((*pool).redis());
    (*conn).set_proxy(1 as libc::c_int as libc::c_uint);
    (*conn)
        .recv = Some(
        proxy_recv as unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
    );
    (*conn).recv_next = None;
    (*conn).recv_done = None;
    (*conn).send = None;
    (*conn).send_next = None;
    (*conn).send_done = None;
    (*conn)
        .close = Some(
        proxy_close as unsafe extern "C" fn(*mut context, *mut conn) -> (),
    );
    (*conn).active = None;
    (*conn)
        .ref_0 = Some(
        proxy_ref as unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> (),
    );
    (*conn).unref = Some(proxy_unref as unsafe extern "C" fn(*mut conn) -> ());
    (*conn).enqueue_inq = None;
    (*conn).dequeue_inq = None;
    (*conn).enqueue_outq = None;
    (*conn).dequeue_outq = None;
    ((*conn).ref_0).expect("non-null function pointer")(conn, pool as *mut libc::c_void);
    return conn;
}
unsafe extern "C" fn conn_free(mut conn: *mut conn) {
    _nc_free(
        conn as *mut libc::c_void,
        b"nc_connection.c\0" as *const u8 as *const libc::c_char,
        291 as libc::c_int,
    );
    conn = 0 as *mut conn;
}
#[no_mangle]
pub unsafe extern "C" fn conn_put(mut conn: *mut conn) {
    nfree_connq = nfree_connq.wrapping_add(1);
    nfree_connq;
    (*conn).conn_tqe.tqe_next = free_connq.tqh_first;
    if !((*conn).conn_tqe.tqe_next).is_null() {
        (*free_connq.tqh_first).conn_tqe.tqe_prev = &mut (*conn).conn_tqe.tqe_next;
    } else {
        free_connq.tqh_last = &mut (*conn).conn_tqe.tqe_next;
    }
    free_connq.tqh_first = conn;
    (*conn).conn_tqe.tqe_prev = &mut free_connq.tqh_first;
    if (*conn).client() != 0 {
        ncurr_cconn = ncurr_cconn.wrapping_sub(1);
        ncurr_cconn;
    }
    ncurr_conn = ncurr_conn.wrapping_sub(1);
    ncurr_conn;
}
#[no_mangle]
pub unsafe extern "C" fn conn_init() {
    nfree_connq = 0 as libc::c_int as uint32_t;
    free_connq.tqh_first = 0 as *mut conn;
    free_connq.tqh_last = &mut free_connq.tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn conn_deinit() {
    let mut conn: *mut conn = 0 as *mut conn;
    let mut nconn: *mut conn = 0 as *mut conn;
    conn = free_connq.tqh_first;
    while !conn.is_null() {
        nconn = (*conn).conn_tqe.tqe_next;
        conn_free(conn);
        conn = nconn;
        nfree_connq = nfree_connq.wrapping_sub(1);
        nfree_connq;
    }
}
#[no_mangle]
pub unsafe extern "C" fn conn_recv(
    mut conn: *mut conn,
    mut buf: *mut libc::c_void,
    mut size: size_t,
) -> ssize_t {
    let mut n: ssize_t = 0;
    loop {
        n = read((*conn).sd, buf, size);
        if n > 0 as libc::c_int as libc::c_long {
            if n < size as ssize_t {
                (*conn).set_recv_ready(0 as libc::c_int as libc::c_uint);
            }
            (*conn)
                .recv_bytes = ((*conn).recv_bytes as libc::c_ulong)
                .wrapping_add(n as size_t) as size_t as size_t;
            return n;
        }
        if n == 0 as libc::c_int as libc::c_long {
            (*conn).set_recv_ready(0 as libc::c_int as libc::c_uint);
            (*conn).set_eof(1 as libc::c_int as libc::c_uint);
            return n;
        }
        if *__errno_location() == 4 as libc::c_int {
            continue;
        }
        if *__errno_location() == 11 as libc::c_int
            || *__errno_location() == 11 as libc::c_int
        {
            (*conn).set_recv_ready(0 as libc::c_int as libc::c_uint);
            return -(2 as libc::c_int) as ssize_t;
        } else {
            (*conn).set_recv_ready(0 as libc::c_int as libc::c_uint);
            (*conn).err = *__errno_location();
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc_connection.c\0" as *const u8 as *const libc::c_char,
                    373 as libc::c_int,
                    0 as libc::c_int,
                    b"recv on sd %d failed: %s\0" as *const u8 as *const libc::c_char,
                    (*conn).sd,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int) as ssize_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn conn_sendv(
    mut conn: *mut conn,
    mut sendv: *const array,
    mut nsend: size_t,
) -> ssize_t {
    let mut n: ssize_t = 0;
    loop {
        n = writev(
            (*conn).sd,
            (*sendv).elem as *const iovec,
            (*sendv).nelem as libc::c_int,
        );
        if n > 0 as libc::c_int as libc::c_long {
            if n < nsend as ssize_t {
                (*conn).set_send_ready(0 as libc::c_int as libc::c_uint);
            }
            (*conn)
                .send_bytes = ((*conn).send_bytes as libc::c_ulong)
                .wrapping_add(n as size_t) as size_t as size_t;
            return n;
        }
        if n == 0 as libc::c_int as libc::c_long {
            if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc_connection.c\0" as *const u8 as *const libc::c_char,
                    407 as libc::c_int,
                    0 as libc::c_int,
                    b"sendv on sd %d returned zero\0" as *const u8
                        as *const libc::c_char,
                    (*conn).sd,
                );
            }
            (*conn).set_send_ready(0 as libc::c_int as libc::c_uint);
            return 0 as libc::c_int as ssize_t;
        }
        if *__errno_location() == 4 as libc::c_int {
            continue;
        }
        if *__errno_location() == 11 as libc::c_int
            || *__errno_location() == 11 as libc::c_int
        {
            (*conn).set_send_ready(0 as libc::c_int as libc::c_uint);
            return -(2 as libc::c_int) as ssize_t;
        } else {
            (*conn).set_send_ready(0 as libc::c_int as libc::c_uint);
            (*conn).err = *__errno_location();
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc_connection.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int,
                    0 as libc::c_int,
                    b"sendv on sd %d failed: %s\0" as *const u8 as *const libc::c_char,
                    (*conn).sd,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int) as ssize_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn conn_ncurr_conn() -> uint32_t {
    return ncurr_conn;
}
#[no_mangle]
pub unsafe extern "C" fn conn_ntotal_conn() -> uint64_t {
    return ntotal_conn;
}
#[no_mangle]
pub unsafe extern "C" fn conn_ncurr_cconn() -> uint32_t {
    return ncurr_cconn;
}
#[no_mangle]
pub unsafe extern "C" fn conn_authenticated(mut conn: *const conn) -> bool {
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    pool = (if (*conn).client() as libc::c_int != 0 {
        (*conn).owner
    } else {
        (*((*conn).owner as *mut server)).owner as *mut libc::c_void
    }) as *mut server_pool;
    if (*pool).require_auth == 0 {
        return 1 as libc::c_int != 0;
    }
    if (*conn).authenticated() == 0 {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
