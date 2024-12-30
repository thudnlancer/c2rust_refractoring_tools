#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn floorf(_: libc::c_float) -> libc::c_float;
    fn array_get(a: *const array, idx: uint32_t) -> *mut libc::c_void;
    fn _nc_realloc(
        ptr: *mut libc::c_void,
        size: size_t,
        name: *const libc::c_char,
        line: libc::c_int,
    ) -> *mut libc::c_void;
    fn nc_usec_now() -> int64_t;
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn md5_signature(
        key: *const libc::c_uchar,
        length: libc::c_uint,
        result: *mut libc::c_uchar,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
#[no_mangle]
pub unsafe extern "C" fn ketama_hash(
    mut key: *const libc::c_char,
    mut key_length: size_t,
    mut alignment: uint32_t,
) -> uint32_t {
    let mut results: [libc::c_uchar; 16] = [0; 16];
    md5_signature(
        key as *mut libc::c_uchar,
        key_length as libc::c_uint,
        results.as_mut_ptr(),
    );
    return ((results[(3 as libc::c_int as libc::c_uint)
        .wrapping_add(alignment.wrapping_mul(4 as libc::c_int as libc::c_uint)) as usize]
        as libc::c_int & 0xff as libc::c_int) as uint32_t) << 24 as libc::c_int
        | ((results[(2 as libc::c_int as libc::c_uint)
            .wrapping_add(alignment.wrapping_mul(4 as libc::c_int as libc::c_uint))
            as usize] as libc::c_int & 0xff as libc::c_int) as uint32_t)
            << 16 as libc::c_int
        | ((results[(1 as libc::c_int as libc::c_uint)
            .wrapping_add(alignment.wrapping_mul(4 as libc::c_int as libc::c_uint))
            as usize] as libc::c_int & 0xff as libc::c_int) as uint32_t)
            << 8 as libc::c_int
        | (results[(0 as libc::c_int as libc::c_uint)
            .wrapping_add(alignment.wrapping_mul(4 as libc::c_int as libc::c_uint))
            as usize] as libc::c_int & 0xff as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn ketama_item_cmp(
    mut t1: *const libc::c_void,
    mut t2: *const libc::c_void,
) -> libc::c_int {
    let mut ct1: *const continuum = t1 as *const continuum;
    let mut ct2: *const continuum = t2 as *const continuum;
    if (*ct1).value == (*ct2).value {
        return 0 as libc::c_int
    } else if (*ct1).value > (*ct2).value {
        return 1 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ketama_update(mut pool: *mut server_pool) -> rstatus_t {
    let mut nserver: uint32_t = 0;
    let mut nlive_server: uint32_t = 0;
    let mut pointer_per_server: uint32_t = 0;
    let mut pointer_per_hash: uint32_t = 0;
    let mut pointer_counter: uint32_t = 0;
    let mut pointer_index: uint32_t = 0;
    let mut points_per_server: uint32_t = 0;
    let mut continuum_index: uint32_t = 0;
    let mut continuum_addition: uint32_t = 0;
    let mut server_index: uint32_t = 0;
    let mut value: uint32_t = 0;
    let mut total_weight: uint32_t = 0;
    let mut now: int64_t = 0;
    now = nc_usec_now();
    if now < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    nserver = array_n(&mut (*pool).server);
    nlive_server = 0 as libc::c_int as uint32_t;
    total_weight = 0 as libc::c_int as uint32_t;
    (*pool).next_rebuild = 0 as libc::c_longlong as int64_t;
    server_index = 0 as libc::c_int as uint32_t;
    while server_index < nserver {
        let mut server: *mut server = array_get(&mut (*pool).server, server_index)
            as *mut server;
        if (*pool).auto_eject_hosts() != 0 {
            if (*server).next_retry <= now {
                (*server).next_retry = 0 as libc::c_longlong as int64_t;
                nlive_server = nlive_server.wrapping_add(1);
                nlive_server;
            } else if (*pool).next_rebuild as libc::c_longlong == 0 as libc::c_longlong
                || (*server).next_retry < (*pool).next_rebuild
            {
                (*pool).next_rebuild = (*server).next_retry;
            }
        } else {
            nlive_server = nlive_server.wrapping_add(1);
            nlive_server;
        }
        if (*pool).auto_eject_hosts() == 0 || (*server).next_retry <= now {
            total_weight = (total_weight as libc::c_uint).wrapping_add((*server).weight)
                as uint32_t as uint32_t;
        }
        server_index = server_index.wrapping_add(1);
        server_index;
    }
    (*pool).nlive_server = nlive_server;
    if nlive_server == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    continuum_addition = 10 as libc::c_int as uint32_t;
    points_per_server = 160 as libc::c_int as uint32_t;
    if nlive_server > (*pool).nserver_continuum {
        let mut continuum: *mut continuum = 0 as *mut continuum;
        let mut nserver_continuum: uint32_t = nlive_server
            .wrapping_add(continuum_addition);
        let mut ncontinuum: uint32_t = nserver_continuum.wrapping_mul(points_per_server);
        continuum = _nc_realloc(
            (*pool).continuum as *mut libc::c_void,
            (::core::mem::size_of::<continuum>() as libc::c_ulong)
                .wrapping_mul(ncontinuum as libc::c_ulong),
            b"nc_ketama.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
        ) as *mut continuum;
        if continuum.is_null() {
            return -(3 as libc::c_int);
        }
        (*pool).continuum = continuum;
        (*pool).nserver_continuum = nserver_continuum;
    }
    continuum_index = 0 as libc::c_int as uint32_t;
    pointer_counter = 0 as libc::c_int as uint32_t;
    server_index = 0 as libc::c_int as uint32_t;
    while server_index < nserver {
        let mut server_0: *mut server = 0 as *mut server;
        let mut pct: libc::c_float = 0.;
        server_0 = array_get(&mut (*pool).server, server_index) as *mut server;
        if !((*pool).auto_eject_hosts() as libc::c_int != 0
            && (*server_0).next_retry > now)
        {
            pct = (*server_0).weight as libc::c_float / total_weight as libc::c_float;
            pointer_per_server = (floorf(
                ((pct * 160 as libc::c_int as libc::c_float
                    / 4 as libc::c_int as libc::c_float * nlive_server as libc::c_float)
                    as libc::c_double + 0.0000000001f64) as libc::c_float,
            ) * 4 as libc::c_int as libc::c_float) as uint32_t;
            pointer_per_hash = 4 as libc::c_int as uint32_t;
            pointer_index = 1 as libc::c_int as uint32_t;
            while pointer_index <= pointer_per_server.wrapping_div(pointer_per_hash) {
                let mut host: [libc::c_char; 273] = *::core::mem::transmute::<
                    &[u8; 273],
                    &mut [libc::c_char; 273],
                >(
                    b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                );
                let mut hostlen: size_t = 0;
                let mut x: uint32_t = 0;
                hostlen = snprintf(
                    host.as_mut_ptr(),
                    273 as libc::c_int as libc::c_ulong,
                    b"%.*s-%u\0" as *const u8 as *const libc::c_char,
                    (*server_0).name.len,
                    (*server_0).name.data,
                    pointer_index.wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) as size_t;
                if hostlen >= 273 as libc::c_int as libc::c_ulong {
                    hostlen = (273 as libc::c_int - 1 as libc::c_int) as size_t;
                    if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                        _log(
                            b"nc_ketama.c\0" as *const u8 as *const libc::c_char,
                            185 as libc::c_int,
                            0 as libc::c_int,
                            b"Unexpectedly forced to truncate a hostname in ketama pool to %d characters for %.*s\0"
                                as *const u8 as *const libc::c_char,
                            273 as libc::c_int - 1 as libc::c_int,
                            273 as libc::c_int - 1 as libc::c_int,
                            host.as_mut_ptr(),
                        );
                    }
                }
                x = 0 as libc::c_int as uint32_t;
                while x < pointer_per_hash {
                    value = ketama_hash(host.as_mut_ptr(), hostlen, x);
                    (*((*pool).continuum).offset(continuum_index as isize))
                        .index = server_index;
                    let fresh0 = continuum_index;
                    continuum_index = continuum_index.wrapping_add(1);
                    (*((*pool).continuum).offset(fresh0 as isize)).value = value;
                    x = x.wrapping_add(1);
                    x;
                }
                pointer_index = pointer_index.wrapping_add(1);
                pointer_index;
            }
            pointer_counter = (pointer_counter as libc::c_uint)
                .wrapping_add(pointer_per_server) as uint32_t as uint32_t;
        }
        server_index = server_index.wrapping_add(1);
        server_index;
    }
    (*pool).ncontinuum = pointer_counter;
    qsort(
        (*pool).continuum as *mut libc::c_void,
        (*pool).ncontinuum as size_t,
        ::core::mem::size_of::<continuum>() as libc::c_ulong,
        Some(
            ketama_item_cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    pointer_index = 0 as libc::c_int as uint32_t;
    while pointer_index
        < nlive_server
            .wrapping_mul(160 as libc::c_int as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
    {
        if pointer_index.wrapping_add(1 as libc::c_int as libc::c_uint)
            >= pointer_counter
        {
            break;
        }
        pointer_index = pointer_index.wrapping_add(1);
        pointer_index;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ketama_dispatch(
    mut continuum: *const continuum,
    mut ncontinuum: uint32_t,
    mut hash: uint32_t,
) -> uint32_t {
    let mut begin: *const continuum = 0 as *const continuum;
    let mut end: *const continuum = 0 as *const continuum;
    let mut left: *const continuum = 0 as *const continuum;
    let mut right: *const continuum = 0 as *const continuum;
    let mut middle: *const continuum = 0 as *const continuum;
    left = continuum;
    begin = left;
    right = continuum.offset(ncontinuum as isize);
    end = right;
    while left < right {
        middle = left
            .offset(
                (right.offset_from(left) as libc::c_long
                    / 2 as libc::c_int as libc::c_long) as isize,
            );
        if (*middle).value < hash {
            left = middle.offset(1 as libc::c_int as isize);
        } else {
            right = middle;
        }
    }
    if right == end {
        right = begin;
    }
    return (*right).index;
}
