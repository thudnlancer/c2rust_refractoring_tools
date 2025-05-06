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
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn close(__fd: i32) -> i32;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
    fn bind(__fd: i32, __addr: __CONST_SOCKADDR_ARG, __len: socklen_t) -> i32;
    fn listen(__fd: i32, __n: i32) -> i32;
    fn accept(__fd: i32, __addr: __SOCKADDR_ARG, __addr_len: *mut socklen_t) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> i32;
    fn array_init(a: *mut array, n: uint32_t, size: size_t) -> rstatus_t;
    fn array_deinit(a: *mut array);
    fn array_push(a: *mut array) -> *mut libc::c_void;
    fn array_pop(a: *mut array) -> *mut libc::c_void;
    fn array_get(a: *const array, idx: uint32_t) -> *mut libc::c_void;
    fn array_swap(a: *mut array, b: *mut array);
    fn nc_set_reuseaddr(sd: i32) -> i32;
    fn nc_set_reuseport(sd: i32) -> i32;
    fn _nc_alloc(size: size_t, name: *const i8, line: i32) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const i8, line: i32);
    fn _nc_sendn(sd: i32, vptr: *const libc::c_void, n: size_t) -> ssize_t;
    fn nc_resolve(name: *const string, port: i32, si: *mut sockinfo) -> i32;
    fn log_loggable(level: i32) -> i32;
    fn _log(file: *const i8, line: i32, panic: i32, fmt: *const i8, _: ...);
    fn _log_stderr(fmt: *const i8, _: ...);
    fn event_loop_stats(cb: event_stats_cb_t, arg: *mut libc::c_void);
    fn conn_ncurr_conn() -> uint32_t;
    fn conn_ntotal_conn() -> uint64_t;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __mode_t = u32;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
pub type int64_t = __int64_t;
pub type pthread_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}
pub type socklen_t = __socklen_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_NONBLOCK = 2048,
    SOCK_CLOEXEC = 524288,
    SOCK_PACKET = 10,
    SOCK_DCCP = 6,
    SOCK_SEQPACKET = 5,
    SOCK_RDM = 4,
    SOCK_RAW = 3,
    SOCK_DGRAM = 2,
    SOCK_STREAM = 1,
}
impl __socket_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            __socket_type::SOCK_NONBLOCK => 2048,
            __socket_type::SOCK_CLOEXEC => 524288,
            __socket_type::SOCK_PACKET => 10,
            __socket_type::SOCK_DCCP => 6,
            __socket_type::SOCK_SEQPACKET => 5,
            __socket_type::SOCK_RDM => 4,
            __socket_type::SOCK_RAW => 3,
            __socket_type::SOCK_DGRAM => 2,
            __socket_type::SOCK_STREAM => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> __socket_type {
        match value {
            2048 => __socket_type::SOCK_NONBLOCK,
            524288 => __socket_type::SOCK_CLOEXEC,
            10 => __socket_type::SOCK_PACKET,
            6 => __socket_type::SOCK_DCCP,
            5 => __socket_type::SOCK_SEQPACKET,
            4 => __socket_type::SOCK_RDM,
            3 => __socket_type::SOCK_RAW,
            2 => __socket_type::SOCK_DGRAM,
            1 => __socket_type::SOCK_STREAM,
            _ => panic!("Invalid value for __socket_type: {}", value),
        }
    }
}
impl AddAssign<u32> for __socket_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for __socket_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for __socket_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for __socket_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for __socket_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for __socket_type {
    type Output = __socket_type;
    fn add(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for __socket_type {
    type Output = __socket_type;
    fn sub(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for __socket_type {
    type Output = __socket_type;
    fn mul(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for __socket_type {
    type Output = __socket_type;
    fn div(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for __socket_type {
    type Output = __socket_type;
    fn rem(self, rhs: u32) -> __socket_type {
        __socket_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
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
    pub sun_path: [i8; 108],
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
    pub sin_zero: [u8; 8],
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
pub type rstatus_t = i32;
pub type err_t = i32;
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
    pub max_timeout: i32,
    pub timeout: i32,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: i32,
    pub event: *mut epoll_event,
    pub nevent: i32,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option<unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: i32,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: i32,
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
    pub aggregate: i32,
    pub updated: i32,
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
    pub sd: i32,
    pub family: i32,
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
pub type conn_msgq_t = Option<
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
    pub state: i32,
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
    MSG_SENTINEL = 184,
    MSG_RSP_REDIS_MULTIBULK = 183,
    MSG_RSP_REDIS_BULK = 182,
    MSG_RSP_REDIS_INTEGER = 181,
    MSG_RSP_REDIS_ERROR_NOREPLICAS = 180,
    MSG_RSP_REDIS_ERROR_MASTERDOWN = 179,
    MSG_RSP_REDIS_ERROR_EXECABORT = 178,
    MSG_RSP_REDIS_ERROR_WRONGTYPE = 177,
    MSG_RSP_REDIS_ERROR_READONLY = 176,
    MSG_RSP_REDIS_ERROR_NOSCRIPT = 175,
    MSG_RSP_REDIS_ERROR_MISCONF = 174,
    MSG_RSP_REDIS_ERROR_BUSYKEY = 173,
    MSG_RSP_REDIS_ERROR_LOADING = 172,
    MSG_RSP_REDIS_ERROR_NOAUTH = 171,
    MSG_RSP_REDIS_ERROR_BUSY = 170,
    MSG_RSP_REDIS_ERROR_OOM = 169,
    MSG_RSP_REDIS_ERROR_ERR = 168,
    MSG_RSP_REDIS_ERROR = 167,
    MSG_RSP_REDIS_STATUS = 166,
    MSG_REQ_REDIS_LOLWUT = 165,
    MSG_REQ_REDIS_COMMAND = 164,
    MSG_REQ_REDIS_SELECT = 163,
    MSG_REQ_REDIS_AUTH = 162,
    MSG_REQ_REDIS_QUIT = 161,
    MSG_REQ_REDIS_PING = 160,
    MSG_REQ_REDIS_EVALSHA = 159,
    MSG_REQ_REDIS_EVAL = 158,
    MSG_REQ_REDIS_GEOSEARCHSTORE = 157,
    MSG_REQ_REDIS_GEOSEARCH = 156,
    MSG_REQ_REDIS_GEOPOS = 155,
    MSG_REQ_REDIS_GEORADIUSBYMEMBER = 154,
    MSG_REQ_REDIS_GEORADIUS = 153,
    MSG_REQ_REDIS_GEOHASH = 152,
    MSG_REQ_REDIS_GEODIST = 151,
    MSG_REQ_REDIS_GEOADD = 150,
    MSG_REQ_REDIS_ZUNIONSTORE = 149,
    MSG_REQ_REDIS_ZSCORE = 148,
    MSG_REQ_REDIS_ZSCAN = 147,
    MSG_REQ_REDIS_ZUNION = 146,
    MSG_REQ_REDIS_ZREVRANK = 145,
    MSG_REQ_REDIS_ZREVRANGEBYSCORE = 144,
    MSG_REQ_REDIS_ZREVRANGEBYLEX = 143,
    MSG_REQ_REDIS_ZREVRANGE = 142,
    MSG_REQ_REDIS_ZREMRANGEBYSCORE = 141,
    MSG_REQ_REDIS_ZREMRANGEBYLEX = 140,
    MSG_REQ_REDIS_ZREMRANGEBYRANK = 139,
    MSG_REQ_REDIS_ZREM = 138,
    MSG_REQ_REDIS_ZRANK = 137,
    MSG_REQ_REDIS_ZRANGESTORE = 136,
    MSG_REQ_REDIS_ZRANGEBYSCORE = 135,
    MSG_REQ_REDIS_ZRANGEBYLEX = 134,
    MSG_REQ_REDIS_ZRANGE = 133,
    MSG_REQ_REDIS_ZRANDMEMBER = 132,
    MSG_REQ_REDIS_ZPOPMAX = 131,
    MSG_REQ_REDIS_ZPOPMIN = 130,
    MSG_REQ_REDIS_ZMSCORE = 129,
    MSG_REQ_REDIS_ZLEXCOUNT = 128,
    MSG_REQ_REDIS_ZINTERSTORE = 127,
    MSG_REQ_REDIS_ZINTER = 126,
    MSG_REQ_REDIS_ZINCRBY = 125,
    MSG_REQ_REDIS_ZDIFFSTORE = 124,
    MSG_REQ_REDIS_ZDIFF = 123,
    MSG_REQ_REDIS_ZCOUNT = 122,
    MSG_REQ_REDIS_ZCARD = 121,
    MSG_REQ_REDIS_ZADD = 120,
    MSG_REQ_REDIS_SSCAN = 119,
    MSG_REQ_REDIS_SUNIONSTORE = 118,
    MSG_REQ_REDIS_SUNION = 117,
    MSG_REQ_REDIS_SREM = 116,
    MSG_REQ_REDIS_SRANDMEMBER = 115,
    MSG_REQ_REDIS_SPOP = 114,
    MSG_REQ_REDIS_SMOVE = 113,
    MSG_REQ_REDIS_SMEMBERS = 112,
    MSG_REQ_REDIS_SMISMEMBER = 111,
    MSG_REQ_REDIS_SISMEMBER = 110,
    MSG_REQ_REDIS_SINTERSTORE = 109,
    MSG_REQ_REDIS_SINTER = 108,
    MSG_REQ_REDIS_SDIFFSTORE = 107,
    MSG_REQ_REDIS_SDIFF = 106,
    MSG_REQ_REDIS_SCARD = 105,
    MSG_REQ_REDIS_SADD = 104,
    MSG_REQ_REDIS_RPUSHX = 103,
    MSG_REQ_REDIS_RPUSH = 102,
    MSG_REQ_REDIS_RPOPLPUSH = 101,
    MSG_REQ_REDIS_RPOP = 100,
    MSG_REQ_REDIS_PFMERGE = 99,
    MSG_REQ_REDIS_PFCOUNT = 98,
    MSG_REQ_REDIS_PFADD = 97,
    MSG_REQ_REDIS_LTRIM = 96,
    MSG_REQ_REDIS_LSET = 95,
    MSG_REQ_REDIS_LREM = 94,
    MSG_REQ_REDIS_LRANGE = 93,
    MSG_REQ_REDIS_LPUSHX = 92,
    MSG_REQ_REDIS_LPUSH = 91,
    MSG_REQ_REDIS_LPOS = 90,
    MSG_REQ_REDIS_LPOP = 89,
    MSG_REQ_REDIS_LMOVE = 88,
    MSG_REQ_REDIS_LLEN = 87,
    MSG_REQ_REDIS_LINSERT = 86,
    MSG_REQ_REDIS_LINDEX = 85,
    MSG_REQ_REDIS_HVALS = 84,
    MSG_REQ_REDIS_HSTRLEN = 83,
    MSG_REQ_REDIS_HSCAN = 82,
    MSG_REQ_REDIS_HSETNX = 81,
    MSG_REQ_REDIS_HSET = 80,
    MSG_REQ_REDIS_HRANDFIELD = 79,
    MSG_REQ_REDIS_HMSET = 78,
    MSG_REQ_REDIS_HMGET = 77,
    MSG_REQ_REDIS_HLEN = 76,
    MSG_REQ_REDIS_HKEYS = 75,
    MSG_REQ_REDIS_HINCRBYFLOAT = 74,
    MSG_REQ_REDIS_HINCRBY = 73,
    MSG_REQ_REDIS_HGETALL = 72,
    MSG_REQ_REDIS_HGET = 71,
    MSG_REQ_REDIS_HEXISTS = 70,
    MSG_REQ_REDIS_HDEL = 69,
    MSG_REQ_REDIS_STRLEN = 68,
    MSG_REQ_REDIS_SETRANGE = 67,
    MSG_REQ_REDIS_SETNX = 66,
    MSG_REQ_REDIS_SETEX = 65,
    MSG_REQ_REDIS_SETBIT = 64,
    MSG_REQ_REDIS_SET = 63,
    MSG_REQ_REDIS_RESTORE = 62,
    MSG_REQ_REDIS_PSETEX = 61,
    MSG_REQ_REDIS_MSET = 60,
    MSG_REQ_REDIS_MGET = 59,
    MSG_REQ_REDIS_INCRBYFLOAT = 58,
    MSG_REQ_REDIS_INCRBY = 57,
    MSG_REQ_REDIS_INCR = 56,
    MSG_REQ_REDIS_GETSET = 55,
    MSG_REQ_REDIS_GETRANGE = 54,
    MSG_REQ_REDIS_GETEX = 53,
    MSG_REQ_REDIS_GETDEL = 52,
    MSG_REQ_REDIS_GETBIT = 51,
    MSG_REQ_REDIS_GET = 50,
    MSG_REQ_REDIS_DUMP = 49,
    MSG_REQ_REDIS_DECRBY = 48,
    MSG_REQ_REDIS_DECR = 47,
    MSG_REQ_REDIS_BITPOS = 46,
    MSG_REQ_REDIS_BITFIELD = 45,
    MSG_REQ_REDIS_BITCOUNT = 44,
    MSG_REQ_REDIS_APPEND = 43,
    MSG_REQ_REDIS_UNLINK = 42,
    MSG_REQ_REDIS_TYPE = 41,
    MSG_REQ_REDIS_TTL = 40,
    MSG_REQ_REDIS_TOUCH = 39,
    MSG_REQ_REDIS_SORT = 38,
    MSG_REQ_REDIS_PTTL = 37,
    MSG_REQ_REDIS_PERSIST = 36,
    MSG_REQ_REDIS_PEXPIREAT = 35,
    MSG_REQ_REDIS_PEXPIRE = 34,
    MSG_REQ_REDIS_MOVE = 33,
    MSG_REQ_REDIS_EXPIREAT = 32,
    MSG_REQ_REDIS_EXPIRE = 31,
    MSG_REQ_REDIS_EXISTS = 30,
    MSG_REQ_REDIS_DEL = 29,
    MSG_REQ_REDIS_COPY = 28,
    MSG_RSP_MC_SERVER_ERROR = 27,
    MSG_RSP_MC_CLIENT_ERROR = 26,
    MSG_RSP_MC_ERROR = 25,
    MSG_RSP_MC_VERSION = 24,
    MSG_RSP_MC_TOUCHED = 23,
    MSG_RSP_MC_DELETED = 22,
    MSG_RSP_MC_VALUE = 21,
    MSG_RSP_MC_END = 20,
    MSG_RSP_MC_NOT_FOUND = 19,
    MSG_RSP_MC_EXISTS = 18,
    MSG_RSP_MC_NOT_STORED = 17,
    MSG_RSP_MC_STORED = 16,
    MSG_RSP_MC_NUM = 15,
    MSG_REQ_MC_VERSION = 14,
    MSG_REQ_MC_QUIT = 13,
    MSG_REQ_MC_TOUCH = 12,
    MSG_REQ_MC_DECR = 11,
    MSG_REQ_MC_INCR = 10,
    MSG_REQ_MC_PREPEND = 9,
    MSG_REQ_MC_APPEND = 8,
    MSG_REQ_MC_REPLACE = 7,
    MSG_REQ_MC_ADD = 6,
    MSG_REQ_MC_SET = 5,
    MSG_REQ_MC_CAS = 4,
    MSG_REQ_MC_DELETE = 3,
    MSG_REQ_MC_GETS = 2,
    MSG_REQ_MC_GET = 1,
    MSG_UNKNOWN = 0,
}
impl msg_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            msg_type::MSG_SENTINEL => 184,
            msg_type::MSG_RSP_REDIS_MULTIBULK => 183,
            msg_type::MSG_RSP_REDIS_BULK => 182,
            msg_type::MSG_RSP_REDIS_INTEGER => 181,
            msg_type::MSG_RSP_REDIS_ERROR_NOREPLICAS => 180,
            msg_type::MSG_RSP_REDIS_ERROR_MASTERDOWN => 179,
            msg_type::MSG_RSP_REDIS_ERROR_EXECABORT => 178,
            msg_type::MSG_RSP_REDIS_ERROR_WRONGTYPE => 177,
            msg_type::MSG_RSP_REDIS_ERROR_READONLY => 176,
            msg_type::MSG_RSP_REDIS_ERROR_NOSCRIPT => 175,
            msg_type::MSG_RSP_REDIS_ERROR_MISCONF => 174,
            msg_type::MSG_RSP_REDIS_ERROR_BUSYKEY => 173,
            msg_type::MSG_RSP_REDIS_ERROR_LOADING => 172,
            msg_type::MSG_RSP_REDIS_ERROR_NOAUTH => 171,
            msg_type::MSG_RSP_REDIS_ERROR_BUSY => 170,
            msg_type::MSG_RSP_REDIS_ERROR_OOM => 169,
            msg_type::MSG_RSP_REDIS_ERROR_ERR => 168,
            msg_type::MSG_RSP_REDIS_ERROR => 167,
            msg_type::MSG_RSP_REDIS_STATUS => 166,
            msg_type::MSG_REQ_REDIS_LOLWUT => 165,
            msg_type::MSG_REQ_REDIS_COMMAND => 164,
            msg_type::MSG_REQ_REDIS_SELECT => 163,
            msg_type::MSG_REQ_REDIS_AUTH => 162,
            msg_type::MSG_REQ_REDIS_QUIT => 161,
            msg_type::MSG_REQ_REDIS_PING => 160,
            msg_type::MSG_REQ_REDIS_EVALSHA => 159,
            msg_type::MSG_REQ_REDIS_EVAL => 158,
            msg_type::MSG_REQ_REDIS_GEOSEARCHSTORE => 157,
            msg_type::MSG_REQ_REDIS_GEOSEARCH => 156,
            msg_type::MSG_REQ_REDIS_GEOPOS => 155,
            msg_type::MSG_REQ_REDIS_GEORADIUSBYMEMBER => 154,
            msg_type::MSG_REQ_REDIS_GEORADIUS => 153,
            msg_type::MSG_REQ_REDIS_GEOHASH => 152,
            msg_type::MSG_REQ_REDIS_GEODIST => 151,
            msg_type::MSG_REQ_REDIS_GEOADD => 150,
            msg_type::MSG_REQ_REDIS_ZUNIONSTORE => 149,
            msg_type::MSG_REQ_REDIS_ZSCORE => 148,
            msg_type::MSG_REQ_REDIS_ZSCAN => 147,
            msg_type::MSG_REQ_REDIS_ZUNION => 146,
            msg_type::MSG_REQ_REDIS_ZREVRANK => 145,
            msg_type::MSG_REQ_REDIS_ZREVRANGEBYSCORE => 144,
            msg_type::MSG_REQ_REDIS_ZREVRANGEBYLEX => 143,
            msg_type::MSG_REQ_REDIS_ZREVRANGE => 142,
            msg_type::MSG_REQ_REDIS_ZREMRANGEBYSCORE => 141,
            msg_type::MSG_REQ_REDIS_ZREMRANGEBYLEX => 140,
            msg_type::MSG_REQ_REDIS_ZREMRANGEBYRANK => 139,
            msg_type::MSG_REQ_REDIS_ZREM => 138,
            msg_type::MSG_REQ_REDIS_ZRANK => 137,
            msg_type::MSG_REQ_REDIS_ZRANGESTORE => 136,
            msg_type::MSG_REQ_REDIS_ZRANGEBYSCORE => 135,
            msg_type::MSG_REQ_REDIS_ZRANGEBYLEX => 134,
            msg_type::MSG_REQ_REDIS_ZRANGE => 133,
            msg_type::MSG_REQ_REDIS_ZRANDMEMBER => 132,
            msg_type::MSG_REQ_REDIS_ZPOPMAX => 131,
            msg_type::MSG_REQ_REDIS_ZPOPMIN => 130,
            msg_type::MSG_REQ_REDIS_ZMSCORE => 129,
            msg_type::MSG_REQ_REDIS_ZLEXCOUNT => 128,
            msg_type::MSG_REQ_REDIS_ZINTERSTORE => 127,
            msg_type::MSG_REQ_REDIS_ZINTER => 126,
            msg_type::MSG_REQ_REDIS_ZINCRBY => 125,
            msg_type::MSG_REQ_REDIS_ZDIFFSTORE => 124,
            msg_type::MSG_REQ_REDIS_ZDIFF => 123,
            msg_type::MSG_REQ_REDIS_ZCOUNT => 122,
            msg_type::MSG_REQ_REDIS_ZCARD => 121,
            msg_type::MSG_REQ_REDIS_ZADD => 120,
            msg_type::MSG_REQ_REDIS_SSCAN => 119,
            msg_type::MSG_REQ_REDIS_SUNIONSTORE => 118,
            msg_type::MSG_REQ_REDIS_SUNION => 117,
            msg_type::MSG_REQ_REDIS_SREM => 116,
            msg_type::MSG_REQ_REDIS_SRANDMEMBER => 115,
            msg_type::MSG_REQ_REDIS_SPOP => 114,
            msg_type::MSG_REQ_REDIS_SMOVE => 113,
            msg_type::MSG_REQ_REDIS_SMEMBERS => 112,
            msg_type::MSG_REQ_REDIS_SMISMEMBER => 111,
            msg_type::MSG_REQ_REDIS_SISMEMBER => 110,
            msg_type::MSG_REQ_REDIS_SINTERSTORE => 109,
            msg_type::MSG_REQ_REDIS_SINTER => 108,
            msg_type::MSG_REQ_REDIS_SDIFFSTORE => 107,
            msg_type::MSG_REQ_REDIS_SDIFF => 106,
            msg_type::MSG_REQ_REDIS_SCARD => 105,
            msg_type::MSG_REQ_REDIS_SADD => 104,
            msg_type::MSG_REQ_REDIS_RPUSHX => 103,
            msg_type::MSG_REQ_REDIS_RPUSH => 102,
            msg_type::MSG_REQ_REDIS_RPOPLPUSH => 101,
            msg_type::MSG_REQ_REDIS_RPOP => 100,
            msg_type::MSG_REQ_REDIS_PFMERGE => 99,
            msg_type::MSG_REQ_REDIS_PFCOUNT => 98,
            msg_type::MSG_REQ_REDIS_PFADD => 97,
            msg_type::MSG_REQ_REDIS_LTRIM => 96,
            msg_type::MSG_REQ_REDIS_LSET => 95,
            msg_type::MSG_REQ_REDIS_LREM => 94,
            msg_type::MSG_REQ_REDIS_LRANGE => 93,
            msg_type::MSG_REQ_REDIS_LPUSHX => 92,
            msg_type::MSG_REQ_REDIS_LPUSH => 91,
            msg_type::MSG_REQ_REDIS_LPOS => 90,
            msg_type::MSG_REQ_REDIS_LPOP => 89,
            msg_type::MSG_REQ_REDIS_LMOVE => 88,
            msg_type::MSG_REQ_REDIS_LLEN => 87,
            msg_type::MSG_REQ_REDIS_LINSERT => 86,
            msg_type::MSG_REQ_REDIS_LINDEX => 85,
            msg_type::MSG_REQ_REDIS_HVALS => 84,
            msg_type::MSG_REQ_REDIS_HSTRLEN => 83,
            msg_type::MSG_REQ_REDIS_HSCAN => 82,
            msg_type::MSG_REQ_REDIS_HSETNX => 81,
            msg_type::MSG_REQ_REDIS_HSET => 80,
            msg_type::MSG_REQ_REDIS_HRANDFIELD => 79,
            msg_type::MSG_REQ_REDIS_HMSET => 78,
            msg_type::MSG_REQ_REDIS_HMGET => 77,
            msg_type::MSG_REQ_REDIS_HLEN => 76,
            msg_type::MSG_REQ_REDIS_HKEYS => 75,
            msg_type::MSG_REQ_REDIS_HINCRBYFLOAT => 74,
            msg_type::MSG_REQ_REDIS_HINCRBY => 73,
            msg_type::MSG_REQ_REDIS_HGETALL => 72,
            msg_type::MSG_REQ_REDIS_HGET => 71,
            msg_type::MSG_REQ_REDIS_HEXISTS => 70,
            msg_type::MSG_REQ_REDIS_HDEL => 69,
            msg_type::MSG_REQ_REDIS_STRLEN => 68,
            msg_type::MSG_REQ_REDIS_SETRANGE => 67,
            msg_type::MSG_REQ_REDIS_SETNX => 66,
            msg_type::MSG_REQ_REDIS_SETEX => 65,
            msg_type::MSG_REQ_REDIS_SETBIT => 64,
            msg_type::MSG_REQ_REDIS_SET => 63,
            msg_type::MSG_REQ_REDIS_RESTORE => 62,
            msg_type::MSG_REQ_REDIS_PSETEX => 61,
            msg_type::MSG_REQ_REDIS_MSET => 60,
            msg_type::MSG_REQ_REDIS_MGET => 59,
            msg_type::MSG_REQ_REDIS_INCRBYFLOAT => 58,
            msg_type::MSG_REQ_REDIS_INCRBY => 57,
            msg_type::MSG_REQ_REDIS_INCR => 56,
            msg_type::MSG_REQ_REDIS_GETSET => 55,
            msg_type::MSG_REQ_REDIS_GETRANGE => 54,
            msg_type::MSG_REQ_REDIS_GETEX => 53,
            msg_type::MSG_REQ_REDIS_GETDEL => 52,
            msg_type::MSG_REQ_REDIS_GETBIT => 51,
            msg_type::MSG_REQ_REDIS_GET => 50,
            msg_type::MSG_REQ_REDIS_DUMP => 49,
            msg_type::MSG_REQ_REDIS_DECRBY => 48,
            msg_type::MSG_REQ_REDIS_DECR => 47,
            msg_type::MSG_REQ_REDIS_BITPOS => 46,
            msg_type::MSG_REQ_REDIS_BITFIELD => 45,
            msg_type::MSG_REQ_REDIS_BITCOUNT => 44,
            msg_type::MSG_REQ_REDIS_APPEND => 43,
            msg_type::MSG_REQ_REDIS_UNLINK => 42,
            msg_type::MSG_REQ_REDIS_TYPE => 41,
            msg_type::MSG_REQ_REDIS_TTL => 40,
            msg_type::MSG_REQ_REDIS_TOUCH => 39,
            msg_type::MSG_REQ_REDIS_SORT => 38,
            msg_type::MSG_REQ_REDIS_PTTL => 37,
            msg_type::MSG_REQ_REDIS_PERSIST => 36,
            msg_type::MSG_REQ_REDIS_PEXPIREAT => 35,
            msg_type::MSG_REQ_REDIS_PEXPIRE => 34,
            msg_type::MSG_REQ_REDIS_MOVE => 33,
            msg_type::MSG_REQ_REDIS_EXPIREAT => 32,
            msg_type::MSG_REQ_REDIS_EXPIRE => 31,
            msg_type::MSG_REQ_REDIS_EXISTS => 30,
            msg_type::MSG_REQ_REDIS_DEL => 29,
            msg_type::MSG_REQ_REDIS_COPY => 28,
            msg_type::MSG_RSP_MC_SERVER_ERROR => 27,
            msg_type::MSG_RSP_MC_CLIENT_ERROR => 26,
            msg_type::MSG_RSP_MC_ERROR => 25,
            msg_type::MSG_RSP_MC_VERSION => 24,
            msg_type::MSG_RSP_MC_TOUCHED => 23,
            msg_type::MSG_RSP_MC_DELETED => 22,
            msg_type::MSG_RSP_MC_VALUE => 21,
            msg_type::MSG_RSP_MC_END => 20,
            msg_type::MSG_RSP_MC_NOT_FOUND => 19,
            msg_type::MSG_RSP_MC_EXISTS => 18,
            msg_type::MSG_RSP_MC_NOT_STORED => 17,
            msg_type::MSG_RSP_MC_STORED => 16,
            msg_type::MSG_RSP_MC_NUM => 15,
            msg_type::MSG_REQ_MC_VERSION => 14,
            msg_type::MSG_REQ_MC_QUIT => 13,
            msg_type::MSG_REQ_MC_TOUCH => 12,
            msg_type::MSG_REQ_MC_DECR => 11,
            msg_type::MSG_REQ_MC_INCR => 10,
            msg_type::MSG_REQ_MC_PREPEND => 9,
            msg_type::MSG_REQ_MC_APPEND => 8,
            msg_type::MSG_REQ_MC_REPLACE => 7,
            msg_type::MSG_REQ_MC_ADD => 6,
            msg_type::MSG_REQ_MC_SET => 5,
            msg_type::MSG_REQ_MC_CAS => 4,
            msg_type::MSG_REQ_MC_DELETE => 3,
            msg_type::MSG_REQ_MC_GETS => 2,
            msg_type::MSG_REQ_MC_GET => 1,
            msg_type::MSG_UNKNOWN => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> msg_type {
        match value {
            184 => msg_type::MSG_SENTINEL,
            183 => msg_type::MSG_RSP_REDIS_MULTIBULK,
            182 => msg_type::MSG_RSP_REDIS_BULK,
            181 => msg_type::MSG_RSP_REDIS_INTEGER,
            180 => msg_type::MSG_RSP_REDIS_ERROR_NOREPLICAS,
            179 => msg_type::MSG_RSP_REDIS_ERROR_MASTERDOWN,
            178 => msg_type::MSG_RSP_REDIS_ERROR_EXECABORT,
            177 => msg_type::MSG_RSP_REDIS_ERROR_WRONGTYPE,
            176 => msg_type::MSG_RSP_REDIS_ERROR_READONLY,
            175 => msg_type::MSG_RSP_REDIS_ERROR_NOSCRIPT,
            174 => msg_type::MSG_RSP_REDIS_ERROR_MISCONF,
            173 => msg_type::MSG_RSP_REDIS_ERROR_BUSYKEY,
            172 => msg_type::MSG_RSP_REDIS_ERROR_LOADING,
            171 => msg_type::MSG_RSP_REDIS_ERROR_NOAUTH,
            170 => msg_type::MSG_RSP_REDIS_ERROR_BUSY,
            169 => msg_type::MSG_RSP_REDIS_ERROR_OOM,
            168 => msg_type::MSG_RSP_REDIS_ERROR_ERR,
            167 => msg_type::MSG_RSP_REDIS_ERROR,
            166 => msg_type::MSG_RSP_REDIS_STATUS,
            165 => msg_type::MSG_REQ_REDIS_LOLWUT,
            164 => msg_type::MSG_REQ_REDIS_COMMAND,
            163 => msg_type::MSG_REQ_REDIS_SELECT,
            162 => msg_type::MSG_REQ_REDIS_AUTH,
            161 => msg_type::MSG_REQ_REDIS_QUIT,
            160 => msg_type::MSG_REQ_REDIS_PING,
            159 => msg_type::MSG_REQ_REDIS_EVALSHA,
            158 => msg_type::MSG_REQ_REDIS_EVAL,
            157 => msg_type::MSG_REQ_REDIS_GEOSEARCHSTORE,
            156 => msg_type::MSG_REQ_REDIS_GEOSEARCH,
            155 => msg_type::MSG_REQ_REDIS_GEOPOS,
            154 => msg_type::MSG_REQ_REDIS_GEORADIUSBYMEMBER,
            153 => msg_type::MSG_REQ_REDIS_GEORADIUS,
            152 => msg_type::MSG_REQ_REDIS_GEOHASH,
            151 => msg_type::MSG_REQ_REDIS_GEODIST,
            150 => msg_type::MSG_REQ_REDIS_GEOADD,
            149 => msg_type::MSG_REQ_REDIS_ZUNIONSTORE,
            148 => msg_type::MSG_REQ_REDIS_ZSCORE,
            147 => msg_type::MSG_REQ_REDIS_ZSCAN,
            146 => msg_type::MSG_REQ_REDIS_ZUNION,
            145 => msg_type::MSG_REQ_REDIS_ZREVRANK,
            144 => msg_type::MSG_REQ_REDIS_ZREVRANGEBYSCORE,
            143 => msg_type::MSG_REQ_REDIS_ZREVRANGEBYLEX,
            142 => msg_type::MSG_REQ_REDIS_ZREVRANGE,
            141 => msg_type::MSG_REQ_REDIS_ZREMRANGEBYSCORE,
            140 => msg_type::MSG_REQ_REDIS_ZREMRANGEBYLEX,
            139 => msg_type::MSG_REQ_REDIS_ZREMRANGEBYRANK,
            138 => msg_type::MSG_REQ_REDIS_ZREM,
            137 => msg_type::MSG_REQ_REDIS_ZRANK,
            136 => msg_type::MSG_REQ_REDIS_ZRANGESTORE,
            135 => msg_type::MSG_REQ_REDIS_ZRANGEBYSCORE,
            134 => msg_type::MSG_REQ_REDIS_ZRANGEBYLEX,
            133 => msg_type::MSG_REQ_REDIS_ZRANGE,
            132 => msg_type::MSG_REQ_REDIS_ZRANDMEMBER,
            131 => msg_type::MSG_REQ_REDIS_ZPOPMAX,
            130 => msg_type::MSG_REQ_REDIS_ZPOPMIN,
            129 => msg_type::MSG_REQ_REDIS_ZMSCORE,
            128 => msg_type::MSG_REQ_REDIS_ZLEXCOUNT,
            127 => msg_type::MSG_REQ_REDIS_ZINTERSTORE,
            126 => msg_type::MSG_REQ_REDIS_ZINTER,
            125 => msg_type::MSG_REQ_REDIS_ZINCRBY,
            124 => msg_type::MSG_REQ_REDIS_ZDIFFSTORE,
            123 => msg_type::MSG_REQ_REDIS_ZDIFF,
            122 => msg_type::MSG_REQ_REDIS_ZCOUNT,
            121 => msg_type::MSG_REQ_REDIS_ZCARD,
            120 => msg_type::MSG_REQ_REDIS_ZADD,
            119 => msg_type::MSG_REQ_REDIS_SSCAN,
            118 => msg_type::MSG_REQ_REDIS_SUNIONSTORE,
            117 => msg_type::MSG_REQ_REDIS_SUNION,
            116 => msg_type::MSG_REQ_REDIS_SREM,
            115 => msg_type::MSG_REQ_REDIS_SRANDMEMBER,
            114 => msg_type::MSG_REQ_REDIS_SPOP,
            113 => msg_type::MSG_REQ_REDIS_SMOVE,
            112 => msg_type::MSG_REQ_REDIS_SMEMBERS,
            111 => msg_type::MSG_REQ_REDIS_SMISMEMBER,
            110 => msg_type::MSG_REQ_REDIS_SISMEMBER,
            109 => msg_type::MSG_REQ_REDIS_SINTERSTORE,
            108 => msg_type::MSG_REQ_REDIS_SINTER,
            107 => msg_type::MSG_REQ_REDIS_SDIFFSTORE,
            106 => msg_type::MSG_REQ_REDIS_SDIFF,
            105 => msg_type::MSG_REQ_REDIS_SCARD,
            104 => msg_type::MSG_REQ_REDIS_SADD,
            103 => msg_type::MSG_REQ_REDIS_RPUSHX,
            102 => msg_type::MSG_REQ_REDIS_RPUSH,
            101 => msg_type::MSG_REQ_REDIS_RPOPLPUSH,
            100 => msg_type::MSG_REQ_REDIS_RPOP,
            99 => msg_type::MSG_REQ_REDIS_PFMERGE,
            98 => msg_type::MSG_REQ_REDIS_PFCOUNT,
            97 => msg_type::MSG_REQ_REDIS_PFADD,
            96 => msg_type::MSG_REQ_REDIS_LTRIM,
            95 => msg_type::MSG_REQ_REDIS_LSET,
            94 => msg_type::MSG_REQ_REDIS_LREM,
            93 => msg_type::MSG_REQ_REDIS_LRANGE,
            92 => msg_type::MSG_REQ_REDIS_LPUSHX,
            91 => msg_type::MSG_REQ_REDIS_LPUSH,
            90 => msg_type::MSG_REQ_REDIS_LPOS,
            89 => msg_type::MSG_REQ_REDIS_LPOP,
            88 => msg_type::MSG_REQ_REDIS_LMOVE,
            87 => msg_type::MSG_REQ_REDIS_LLEN,
            86 => msg_type::MSG_REQ_REDIS_LINSERT,
            85 => msg_type::MSG_REQ_REDIS_LINDEX,
            84 => msg_type::MSG_REQ_REDIS_HVALS,
            83 => msg_type::MSG_REQ_REDIS_HSTRLEN,
            82 => msg_type::MSG_REQ_REDIS_HSCAN,
            81 => msg_type::MSG_REQ_REDIS_HSETNX,
            80 => msg_type::MSG_REQ_REDIS_HSET,
            79 => msg_type::MSG_REQ_REDIS_HRANDFIELD,
            78 => msg_type::MSG_REQ_REDIS_HMSET,
            77 => msg_type::MSG_REQ_REDIS_HMGET,
            76 => msg_type::MSG_REQ_REDIS_HLEN,
            75 => msg_type::MSG_REQ_REDIS_HKEYS,
            74 => msg_type::MSG_REQ_REDIS_HINCRBYFLOAT,
            73 => msg_type::MSG_REQ_REDIS_HINCRBY,
            72 => msg_type::MSG_REQ_REDIS_HGETALL,
            71 => msg_type::MSG_REQ_REDIS_HGET,
            70 => msg_type::MSG_REQ_REDIS_HEXISTS,
            69 => msg_type::MSG_REQ_REDIS_HDEL,
            68 => msg_type::MSG_REQ_REDIS_STRLEN,
            67 => msg_type::MSG_REQ_REDIS_SETRANGE,
            66 => msg_type::MSG_REQ_REDIS_SETNX,
            65 => msg_type::MSG_REQ_REDIS_SETEX,
            64 => msg_type::MSG_REQ_REDIS_SETBIT,
            63 => msg_type::MSG_REQ_REDIS_SET,
            62 => msg_type::MSG_REQ_REDIS_RESTORE,
            61 => msg_type::MSG_REQ_REDIS_PSETEX,
            60 => msg_type::MSG_REQ_REDIS_MSET,
            59 => msg_type::MSG_REQ_REDIS_MGET,
            58 => msg_type::MSG_REQ_REDIS_INCRBYFLOAT,
            57 => msg_type::MSG_REQ_REDIS_INCRBY,
            56 => msg_type::MSG_REQ_REDIS_INCR,
            55 => msg_type::MSG_REQ_REDIS_GETSET,
            54 => msg_type::MSG_REQ_REDIS_GETRANGE,
            53 => msg_type::MSG_REQ_REDIS_GETEX,
            52 => msg_type::MSG_REQ_REDIS_GETDEL,
            51 => msg_type::MSG_REQ_REDIS_GETBIT,
            50 => msg_type::MSG_REQ_REDIS_GET,
            49 => msg_type::MSG_REQ_REDIS_DUMP,
            48 => msg_type::MSG_REQ_REDIS_DECRBY,
            47 => msg_type::MSG_REQ_REDIS_DECR,
            46 => msg_type::MSG_REQ_REDIS_BITPOS,
            45 => msg_type::MSG_REQ_REDIS_BITFIELD,
            44 => msg_type::MSG_REQ_REDIS_BITCOUNT,
            43 => msg_type::MSG_REQ_REDIS_APPEND,
            42 => msg_type::MSG_REQ_REDIS_UNLINK,
            41 => msg_type::MSG_REQ_REDIS_TYPE,
            40 => msg_type::MSG_REQ_REDIS_TTL,
            39 => msg_type::MSG_REQ_REDIS_TOUCH,
            38 => msg_type::MSG_REQ_REDIS_SORT,
            37 => msg_type::MSG_REQ_REDIS_PTTL,
            36 => msg_type::MSG_REQ_REDIS_PERSIST,
            35 => msg_type::MSG_REQ_REDIS_PEXPIREAT,
            34 => msg_type::MSG_REQ_REDIS_PEXPIRE,
            33 => msg_type::MSG_REQ_REDIS_MOVE,
            32 => msg_type::MSG_REQ_REDIS_EXPIREAT,
            31 => msg_type::MSG_REQ_REDIS_EXPIRE,
            30 => msg_type::MSG_REQ_REDIS_EXISTS,
            29 => msg_type::MSG_REQ_REDIS_DEL,
            28 => msg_type::MSG_REQ_REDIS_COPY,
            27 => msg_type::MSG_RSP_MC_SERVER_ERROR,
            26 => msg_type::MSG_RSP_MC_CLIENT_ERROR,
            25 => msg_type::MSG_RSP_MC_ERROR,
            24 => msg_type::MSG_RSP_MC_VERSION,
            23 => msg_type::MSG_RSP_MC_TOUCHED,
            22 => msg_type::MSG_RSP_MC_DELETED,
            21 => msg_type::MSG_RSP_MC_VALUE,
            20 => msg_type::MSG_RSP_MC_END,
            19 => msg_type::MSG_RSP_MC_NOT_FOUND,
            18 => msg_type::MSG_RSP_MC_EXISTS,
            17 => msg_type::MSG_RSP_MC_NOT_STORED,
            16 => msg_type::MSG_RSP_MC_STORED,
            15 => msg_type::MSG_RSP_MC_NUM,
            14 => msg_type::MSG_REQ_MC_VERSION,
            13 => msg_type::MSG_REQ_MC_QUIT,
            12 => msg_type::MSG_REQ_MC_TOUCH,
            11 => msg_type::MSG_REQ_MC_DECR,
            10 => msg_type::MSG_REQ_MC_INCR,
            9 => msg_type::MSG_REQ_MC_PREPEND,
            8 => msg_type::MSG_REQ_MC_APPEND,
            7 => msg_type::MSG_REQ_MC_REPLACE,
            6 => msg_type::MSG_REQ_MC_ADD,
            5 => msg_type::MSG_REQ_MC_SET,
            4 => msg_type::MSG_REQ_MC_CAS,
            3 => msg_type::MSG_REQ_MC_DELETE,
            2 => msg_type::MSG_REQ_MC_GETS,
            1 => msg_type::MSG_REQ_MC_GET,
            0 => msg_type::MSG_UNKNOWN,
            _ => panic!("Invalid value for msg_type: {}", value),
        }
    }
}
impl AddAssign<u32> for msg_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for msg_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for msg_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for msg_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for msg_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for msg_type {
    type Output = msg_type;
    fn add(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for msg_type {
    type Output = msg_type;
    fn sub(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for msg_type {
    type Output = msg_type;
    fn mul(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for msg_type {
    type Output = msg_type;
    fn div(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for msg_type {
    type Output = msg_type;
    fn rem(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type msg_coalesce_t = Option<unsafe extern "C" fn(*mut msg) -> ()>;
pub type msg_failure_t = Option<unsafe extern "C" fn(*const msg) -> bool>;
pub type msg_add_auth_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
>;
pub type msg_reply_t = Option<unsafe extern "C" fn(*mut msg) -> rstatus_t>;
pub type msg_fragment_t = Option<
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
}
impl msg_parse_result {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            msg_parse_result::MSG_PARSE_OK => 0,
            msg_parse_result::MSG_PARSE_ERROR => 1,
            msg_parse_result::MSG_PARSE_REPAIR => 2,
            msg_parse_result::MSG_PARSE_AGAIN => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> msg_parse_result {
        match value {
            0 => msg_parse_result::MSG_PARSE_OK,
            1 => msg_parse_result::MSG_PARSE_ERROR,
            2 => msg_parse_result::MSG_PARSE_REPAIR,
            3 => msg_parse_result::MSG_PARSE_AGAIN,
            _ => panic!("Invalid value for msg_parse_result: {}", value),
        }
    }
}
impl AddAssign<u32> for msg_parse_result {
    fn add_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for msg_parse_result {
    fn sub_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for msg_parse_result {
    fn mul_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for msg_parse_result {
    fn div_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for msg_parse_result {
    fn rem_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn add(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn sub(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn mul(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn div(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn rem(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type msg_parse_t = Option<unsafe extern "C" fn(*mut msg) -> ()>;
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
pub type conn_unref_t = Option<unsafe extern "C" fn(*mut conn) -> ()>;
pub type conn_ref_t = Option<unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> ()>;
pub type conn_swallow_msg_t = Option<
    unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_post_connect_t = Option<
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
    pub family: i32,
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
    pub dist_type: i32,
    pub key_hash_type: i32,
    pub key_hash: hash_t,
    pub hash_tag: string,
    pub timeout: i32,
    pub backlog: i32,
    pub redis_db: i32,
    pub client_connections: uint32_t,
    pub server_connections: uint32_t,
    pub server_retry_timeout: int64_t,
    pub server_failure_limit: uint32_t,
    pub redis_auth: string,
    pub require_auth: u32,
    #[bitfield(name = "auto_eject_hosts", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "preconnect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tcpkeepalive", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "reuseport", ty = "libc::c_uint", bits = "4..=4")]
    pub auto_eject_hosts_preconnect_redis_tcpkeepalive_reuseport: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_t = Option<unsafe extern "C" fn(*const i8, size_t) -> uint32_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct continuum {
    pub index: uint32_t,
    pub value: uint32_t,
}
pub type conn_active_t = Option<unsafe extern "C" fn(*const conn) -> bool>;
pub type conn_close_t = Option<unsafe extern "C" fn(*mut context, *mut conn) -> ()>;
pub type conn_send_done_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
pub type conn_send_next_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
>;
pub type conn_send_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
pub type conn_recv_done_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_recv_next_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
>;
pub type conn_recv_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
pub type event_stats_cb_t = Option<
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
}
impl stats_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            stats_type::STATS_INVALID => 0,
            stats_type::STATS_COUNTER => 1,
            stats_type::STATS_GAUGE => 2,
            stats_type::STATS_TIMESTAMP => 3,
            stats_type::STATS_SENTINEL => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> stats_type {
        match value {
            0 => stats_type::STATS_INVALID,
            1 => stats_type::STATS_COUNTER,
            2 => stats_type::STATS_GAUGE,
            3 => stats_type::STATS_TIMESTAMP,
            4 => stats_type::STATS_SENTINEL,
            _ => panic!("Invalid value for stats_type: {}", value),
        }
    }
}
impl AddAssign<u32> for stats_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = stats_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for stats_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = stats_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for stats_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = stats_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for stats_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = stats_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for stats_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = stats_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for stats_type {
    type Output = stats_type;
    fn add(self, rhs: u32) -> stats_type {
        stats_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for stats_type {
    type Output = stats_type;
    fn sub(self, rhs: u32) -> stats_type {
        stats_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for stats_type {
    type Output = stats_type;
    fn mul(self, rhs: u32) -> stats_type {
        stats_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for stats_type {
    type Output = stats_type;
    fn div(self, rhs: u32) -> stats_type {
        stats_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for stats_type {
    type Output = stats_type;
    fn rem(self, rhs: u32) -> stats_type {
        stats_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
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
    STATS_POOL_NFIELD = 6,
    STATS_POOL_fragments = 5,
    STATS_POOL_forward_error = 4,
    STATS_POOL_server_ejects = 3,
    STATS_POOL_client_connections = 2,
    STATS_POOL_client_err = 1,
    STATS_POOL_client_eof = 0,
}
impl stats_pool_field {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            stats_pool_field::STATS_POOL_NFIELD => 6,
            stats_pool_field::STATS_POOL_fragments => 5,
            stats_pool_field::STATS_POOL_forward_error => 4,
            stats_pool_field::STATS_POOL_server_ejects => 3,
            stats_pool_field::STATS_POOL_client_connections => 2,
            stats_pool_field::STATS_POOL_client_err => 1,
            stats_pool_field::STATS_POOL_client_eof => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> stats_pool_field {
        match value {
            6 => stats_pool_field::STATS_POOL_NFIELD,
            5 => stats_pool_field::STATS_POOL_fragments,
            4 => stats_pool_field::STATS_POOL_forward_error,
            3 => stats_pool_field::STATS_POOL_server_ejects,
            2 => stats_pool_field::STATS_POOL_client_connections,
            1 => stats_pool_field::STATS_POOL_client_err,
            0 => stats_pool_field::STATS_POOL_client_eof,
            _ => panic!("Invalid value for stats_pool_field: {}", value),
        }
    }
}
impl AddAssign<u32> for stats_pool_field {
    fn add_assign(&mut self, rhs: u32) {
        *self = stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for stats_pool_field {
    fn sub_assign(&mut self, rhs: u32) {
        *self = stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for stats_pool_field {
    fn mul_assign(&mut self, rhs: u32) {
        *self = stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for stats_pool_field {
    fn div_assign(&mut self, rhs: u32) {
        *self = stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for stats_pool_field {
    fn rem_assign(&mut self, rhs: u32) {
        *self = stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for stats_pool_field {
    type Output = stats_pool_field;
    fn add(self, rhs: u32) -> stats_pool_field {
        stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for stats_pool_field {
    type Output = stats_pool_field;
    fn sub(self, rhs: u32) -> stats_pool_field {
        stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for stats_pool_field {
    type Output = stats_pool_field;
    fn mul(self, rhs: u32) -> stats_pool_field {
        stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for stats_pool_field {
    type Output = stats_pool_field;
    fn div(self, rhs: u32) -> stats_pool_field {
        stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for stats_pool_field {
    type Output = stats_pool_field;
    fn rem(self, rhs: u32) -> stats_pool_field {
        stats_pool_field::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type stats_pool_field_t = stats_pool_field;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stats_server_field {
    STATS_SERVER_NFIELD = 13,
    STATS_SERVER_out_queue_bytes = 12,
    STATS_SERVER_out_queue = 11,
    STATS_SERVER_in_queue_bytes = 10,
    STATS_SERVER_in_queue = 9,
    STATS_SERVER_response_bytes = 8,
    STATS_SERVER_responses = 7,
    STATS_SERVER_request_bytes = 6,
    STATS_SERVER_requests = 5,
    STATS_SERVER_server_ejected_at = 4,
    STATS_SERVER_server_connections = 3,
    STATS_SERVER_server_timedout = 2,
    STATS_SERVER_server_err = 1,
    STATS_SERVER_server_eof = 0,
}
impl stats_server_field {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            stats_server_field::STATS_SERVER_NFIELD => 13,
            stats_server_field::STATS_SERVER_out_queue_bytes => 12,
            stats_server_field::STATS_SERVER_out_queue => 11,
            stats_server_field::STATS_SERVER_in_queue_bytes => 10,
            stats_server_field::STATS_SERVER_in_queue => 9,
            stats_server_field::STATS_SERVER_response_bytes => 8,
            stats_server_field::STATS_SERVER_responses => 7,
            stats_server_field::STATS_SERVER_request_bytes => 6,
            stats_server_field::STATS_SERVER_requests => 5,
            stats_server_field::STATS_SERVER_server_ejected_at => 4,
            stats_server_field::STATS_SERVER_server_connections => 3,
            stats_server_field::STATS_SERVER_server_timedout => 2,
            stats_server_field::STATS_SERVER_server_err => 1,
            stats_server_field::STATS_SERVER_server_eof => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> stats_server_field {
        match value {
            13 => stats_server_field::STATS_SERVER_NFIELD,
            12 => stats_server_field::STATS_SERVER_out_queue_bytes,
            11 => stats_server_field::STATS_SERVER_out_queue,
            10 => stats_server_field::STATS_SERVER_in_queue_bytes,
            9 => stats_server_field::STATS_SERVER_in_queue,
            8 => stats_server_field::STATS_SERVER_response_bytes,
            7 => stats_server_field::STATS_SERVER_responses,
            6 => stats_server_field::STATS_SERVER_request_bytes,
            5 => stats_server_field::STATS_SERVER_requests,
            4 => stats_server_field::STATS_SERVER_server_ejected_at,
            3 => stats_server_field::STATS_SERVER_server_connections,
            2 => stats_server_field::STATS_SERVER_server_timedout,
            1 => stats_server_field::STATS_SERVER_server_err,
            0 => stats_server_field::STATS_SERVER_server_eof,
            _ => panic!("Invalid value for stats_server_field: {}", value),
        }
    }
}
impl AddAssign<u32> for stats_server_field {
    fn add_assign(&mut self, rhs: u32) {
        *self = stats_server_field::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for stats_server_field {
    fn sub_assign(&mut self, rhs: u32) {
        *self = stats_server_field::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for stats_server_field {
    fn mul_assign(&mut self, rhs: u32) {
        *self = stats_server_field::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for stats_server_field {
    fn div_assign(&mut self, rhs: u32) {
        *self = stats_server_field::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for stats_server_field {
    fn rem_assign(&mut self, rhs: u32) {
        *self = stats_server_field::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for stats_server_field {
    type Output = stats_server_field;
    fn add(self, rhs: u32) -> stats_server_field {
        stats_server_field::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for stats_server_field {
    type Output = stats_server_field;
    fn sub(self, rhs: u32) -> stats_server_field {
        stats_server_field::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for stats_server_field {
    type Output = stats_server_field;
    fn mul(self, rhs: u32) -> stats_server_field {
        stats_server_field::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for stats_server_field {
    type Output = stats_server_field;
    fn div(self, rhs: u32) -> stats_server_field {
        stats_server_field::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for stats_server_field {
    type Output = stats_server_field;
    fn rem(self, rhs: u32) -> stats_server_field {
        stats_server_field::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type stats_server_field_t = stats_server_field;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_desc {
    pub name: *mut i8,
    pub desc: *mut i8,
}
#[inline]
unsafe extern "C" fn array_null(mut a: *mut array) {
    (*a).nelem = 0 as i32 as uint32_t;
    (*a).elem = 0 as *mut libc::c_void;
    (*a).size = 0 as i32 as size_t;
    (*a).nalloc = 0 as i32 as uint32_t;
}
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
static mut stats_pool_codec: [stats_metric; 6] = [stats_metric {
    type_0: stats_type::STATS_INVALID,
    name: string {
        len: 0,
        data: 0 as *mut uint8_t,
    },
    value: C2RustUnnamed_6 { counter: 0 },
}; 6];
static mut stats_server_codec: [stats_metric; 13] = [stats_metric {
    type_0: stats_type::STATS_INVALID,
    name: string {
        len: 0,
        data: 0 as *mut uint8_t,
    },
    value: C2RustUnnamed_6 { counter: 0 },
}; 13];
static mut stats_pool_desc: [stats_desc; 6] = [
    {
        let mut init = stats_desc {
            name: b"client_eof\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# eof on client connections\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"client_err\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# errors on client connections\0" as *const u8 as *const i8
                as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"client_connections\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# active client connections\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_ejects\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# times backend server was ejected\0" as *const u8 as *const i8
                as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"forward_error\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# times we encountered a forwarding error\0" as *const u8
                as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"fragments\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# fragments created from a multi-vector request\0" as *const u8
                as *const i8 as *mut i8,
        };
        init
    },
];
static mut stats_server_desc: [stats_desc; 13] = [
    {
        let mut init = stats_desc {
            name: b"server_eof\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# eof on server connections\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_err\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# errors on server connections\0" as *const u8 as *const i8
                as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_timedout\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# timeouts on server connections\0" as *const u8 as *const i8
                as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_connections\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# active server connections\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"server_ejected_at\0" as *const u8 as *const i8 as *mut i8,
            desc: b"timestamp when server was ejected in usec since epoch\0" as *const u8
                as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"requests\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# requests\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"request_bytes\0" as *const u8 as *const i8 as *mut i8,
            desc: b"total request bytes\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"responses\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# responses\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"response_bytes\0" as *const u8 as *const i8 as *mut i8,
            desc: b"total response bytes\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"in_queue\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# requests in incoming queue\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"in_queue_bytes\0" as *const u8 as *const i8 as *mut i8,
            desc: b"current request bytes in incoming queue\0" as *const u8 as *const i8
                as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"out_queue\0" as *const u8 as *const i8 as *mut i8,
            desc: b"# requests in outgoing queue\0" as *const u8 as *const i8 as *mut i8,
        };
        init
    },
    {
        let mut init = stats_desc {
            name: b"out_queue_bytes\0" as *const u8 as *const i8 as *mut i8,
            desc: b"current request bytes in outgoing queue\0" as *const u8 as *const i8
                as *mut i8,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn stats_describe() {
    let mut i: uint32_t = 0;
    _log_stderr(b"pool stats:\0" as *const u8 as *const i8);
    i = 0 as i32 as uint32_t;
    while (i as u64)
        < (::core::mem::size_of::<[stats_desc; 6]>() as u64)
            .wrapping_div(::core::mem::size_of::<stats_desc>() as u64)
    {
        _log_stderr(
            b"  %-20s\"%s\"\0" as *const u8 as *const i8,
            stats_pool_desc[i as usize].name,
            stats_pool_desc[i as usize].desc,
        );
        i = i.wrapping_add(1);
        i;
    }
    _log_stderr(b"\0" as *const u8 as *const i8);
    _log_stderr(b"server stats:\0" as *const u8 as *const i8);
    i = 0 as i32 as uint32_t;
    while (i as u64)
        < (::core::mem::size_of::<[stats_desc; 13]>() as u64)
            .wrapping_div(::core::mem::size_of::<stats_desc>() as u64)
    {
        _log_stderr(
            b"  %-20s\"%s\"\0" as *const u8 as *const i8,
            stats_server_desc[i as usize].name,
            stats_server_desc[i as usize].desc,
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn stats_metric_init(mut stm: *mut stats_metric) {
    match (*stm).type_0 as u32 {
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
    i = 0 as i32 as uint32_t;
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
    let mut nfield: uint32_t = stats_pool_field::STATS_POOL_NFIELD as i32 as uint32_t;
    status = array_init(
        stats_metric,
        nfield,
        ::core::mem::size_of::<stats_metric>() as u64,
    );
    if status != 0 as i32 {
        return status;
    }
    i = 0 as i32 as uint32_t;
    while i < nfield {
        let mut stm: *mut stats_metric = array_push(stats_metric) as *mut stats_metric;
        *stm = stats_pool_codec[i as usize];
        stats_metric_init(stm);
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_server_metric_init(mut sts: *mut stats_server) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    let mut nfield: uint32_t = stats_server_field::STATS_SERVER_NFIELD as i32
        as uint32_t;
    status = array_init(
        &mut (*sts).metric,
        nfield,
        ::core::mem::size_of::<stats_metric>() as u64,
    );
    if status != 0 as i32 {
        return status;
    }
    i = 0 as i32 as uint32_t;
    while i < nfield {
        let mut stm: *mut stats_metric = array_push(&mut (*sts).metric)
            as *mut stats_metric;
        *stm = stats_server_codec[i as usize];
        stats_metric_init(stm);
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_metric_deinit(mut metric: *mut array) {
    let mut i: uint32_t = 0;
    let mut nmetric: uint32_t = 0;
    nmetric = array_n(metric);
    i = 0 as i32 as uint32_t;
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
    if status != 0 as i32 {
        return status;
    }
    return 0 as i32;
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
        ::core::mem::size_of::<stats_server>() as u64,
    );
    if status != 0 as i32 {
        return status;
    }
    i = 0 as i32 as uint32_t;
    while i < nserver {
        let mut s: *mut server = array_get(server, i) as *mut server;
        let mut sts: *mut stats_server = array_push(stats_server) as *mut stats_server;
        status = stats_server_init(sts, s);
        if status != 0 as i32 {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_server_unmap(mut stats_server: *mut array) {
    let mut i: uint32_t = 0;
    let mut nserver: uint32_t = 0;
    nserver = array_n(stats_server);
    i = 0 as i32 as uint32_t;
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
    if status != 0 as i32 {
        return status;
    }
    status = stats_server_map(&mut (*stp).server, &(*sp).server);
    if status != 0 as i32 {
        stats_metric_deinit(&mut (*stp).metric);
        return status;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_pool_reset(mut stats_pool: *mut array) {
    let mut i: uint32_t = 0;
    let mut npool: uint32_t = 0;
    npool = array_n(stats_pool);
    i = 0 as i32 as uint32_t;
    while i < npool {
        let mut stp: *mut stats_pool = array_get(stats_pool, i) as *mut stats_pool;
        let mut j: uint32_t = 0;
        let mut nserver: uint32_t = 0;
        stats_metric_reset(&mut (*stp).metric);
        nserver = array_n(&mut (*stp).server);
        j = 0 as i32 as uint32_t;
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
    status = array_init(stats_pool, npool, ::core::mem::size_of::<stats_pool>() as u64);
    if status != 0 as i32 {
        return status;
    }
    i = 0 as i32 as uint32_t;
    while i < npool {
        let mut sp: *const server_pool = array_get(server_pool, i) as *const server_pool;
        let mut stp: *mut stats_pool = array_push(stats_pool) as *mut stats_pool;
        status = stats_pool_init(stp, sp);
        if status != 0 as i32 {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_pool_unmap(mut stats_pool: *mut array) {
    let mut i: uint32_t = 0;
    let mut npool: uint32_t = 0;
    npool = array_n(stats_pool);
    i = 0 as i32 as uint32_t;
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
    let mut int64_max_digits: uint32_t = 20 as i32 as uint32_t;
    let mut key_value_extra: uint32_t = 8 as i32 as uint32_t;
    let mut pool_extra: uint32_t = 8 as i32 as uint32_t;
    let mut server_extra: uint32_t = 8 as i32 as uint32_t;
    let mut size: size_t = 0 as i32 as size_t;
    let mut i: uint32_t = 0;
    size = (size as u64).wrapping_add(1 as i32 as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).service_str.len as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).service.len as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(key_value_extra as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).source_str.len as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).source.len as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(key_value_extra as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).version_str.len as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).version.len as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(key_value_extra as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).uptime_str.len as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(int64_max_digits as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(key_value_extra as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).timestamp_str.len as u64) as size_t
        as size_t;
    size = (size as u64).wrapping_add(int64_max_digits as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(key_value_extra as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).ntotal_conn_str.len as u64) as size_t
        as size_t;
    size = (size as u64).wrapping_add(int64_max_digits as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(key_value_extra as u64) as size_t as size_t;
    size = (size as u64).wrapping_add((*st).ncurr_conn_str.len as u64) as size_t
        as size_t;
    size = (size as u64).wrapping_add(int64_max_digits as u64) as size_t as size_t;
    size = (size as u64).wrapping_add(key_value_extra as u64) as size_t as size_t;
    i = 0 as i32 as uint32_t;
    while i < array_n(&mut (*st).sum) {
        let mut stp: *mut stats_pool = array_get(&mut (*st).sum, i) as *mut stats_pool;
        let mut j: uint32_t = 0;
        size = (size as u64).wrapping_add((*stp).name.len as u64) as size_t as size_t;
        size = (size as u64).wrapping_add(pool_extra as u64) as size_t as size_t;
        j = 0 as i32 as uint32_t;
        while j < array_n(&mut (*stp).metric) {
            let mut stm: *mut stats_metric = array_get(&mut (*stp).metric, j)
                as *mut stats_metric;
            size = (size as u64).wrapping_add((*stm).name.len as u64) as size_t
                as size_t;
            size = (size as u64).wrapping_add(int64_max_digits as u64) as size_t
                as size_t;
            size = (size as u64).wrapping_add(key_value_extra as u64) as size_t
                as size_t;
            j = j.wrapping_add(1);
            j;
        }
        j = 0 as i32 as uint32_t;
        while j < array_n(&mut (*stp).server) {
            let mut sts: *mut stats_server = array_get(&mut (*stp).server, j)
                as *mut stats_server;
            let mut k: uint32_t = 0;
            size = (size as u64).wrapping_add((*sts).name.len as u64) as size_t
                as size_t;
            size = (size as u64).wrapping_add(server_extra as u64) as size_t as size_t;
            k = 0 as i32 as uint32_t;
            while k < array_n(&mut (*sts).metric) {
                let mut stm_0: *mut stats_metric = array_get(&mut (*sts).metric, k)
                    as *mut stats_metric;
                size = (size as u64).wrapping_add((*stm_0).name.len as u64) as size_t
                    as size_t;
                size = (size as u64).wrapping_add(int64_max_digits as u64) as size_t
                    as size_t;
                size = (size as u64).wrapping_add(key_value_extra as u64) as size_t
                    as size_t;
                k = k.wrapping_add(1);
                k;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    size = (size as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
    size = size
        .wrapping_add(
            (::core::mem::size_of::<u64>() as u64).wrapping_sub(1 as i32 as u64),
        ) & !(::core::mem::size_of::<u64>() as u64).wrapping_sub(1 as i32 as u64);
    (*st).buf.data = _nc_alloc(
        size,
        b"nc_stats.c\0" as *const u8 as *const i8,
        411 as i32,
    ) as *mut uint8_t;
    if ((*st).buf.data).is_null() {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                414 as i32,
                0 as i32,
                b"create stats buffer of size %zu failed: %s\0" as *const u8
                    as *const i8,
                size,
                strerror(*__errno_location()),
            );
        }
        return -(3 as i32);
    }
    (*st).buf.size = size;
    return 0 as i32;
}
unsafe extern "C" fn stats_destroy_buf(mut st: *mut stats) {
    if (*st).buf.size != 0 as i32 as u64 {
        _nc_free(
            (*st).buf.data as *mut libc::c_void,
            b"nc_stats.c\0" as *const u8 as *const i8,
            429 as i32,
        );
        (*st).buf.data = 0 as *mut uint8_t;
        (*st).buf.size = 0 as i32 as size_t;
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
    let mut n: i32 = 0;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    room = ((*buf).size).wrapping_sub((*buf).len).wrapping_sub(1 as i32 as u64);
    n = snprintf(
        pos as *mut i8,
        room,
        b"\"%.*s\":\"%.*s\", \0" as *const u8 as *const i8,
        (*key).len,
        (*key).data,
        (*val).len,
        (*val).data,
    );
    if n < 0 as i32 || n >= room as i32 {
        return -(1 as i32);
    }
    (*buf).len = ((*buf).len as u64).wrapping_add(n as size_t) as size_t as size_t;
    return 0 as i32;
}
unsafe extern "C" fn stats_add_num(
    mut st: *mut stats,
    mut key: *const string,
    mut val: int64_t,
) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    let mut room: size_t = 0;
    let mut n: i32 = 0;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    room = ((*buf).size).wrapping_sub((*buf).len).wrapping_sub(1 as i32 as u64);
    n = snprintf(
        pos as *mut i8,
        room,
        b"\"%.*s\":%ld, \0" as *const u8 as *const i8,
        (*key).len,
        (*key).data,
        val,
    );
    if n < 0 as i32 || n >= room as i32 {
        return -(1 as i32);
    }
    (*buf).len = ((*buf).len as u64).wrapping_add(n as size_t) as size_t as size_t;
    return 0 as i32;
}
unsafe extern "C" fn stats_add_header(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut cur_ts: int64_t = 0;
    let mut uptime: int64_t = 0;
    buf = &mut (*st).buf;
    *((*buf).data).offset(0 as i32 as isize) = '{' as i32 as uint8_t;
    (*buf).len = 1 as i32 as size_t;
    cur_ts = time(0 as *mut time_t);
    uptime = cur_ts - (*st).start_ts;
    status = stats_add_string(st, &mut (*st).service_str, &mut (*st).service);
    if status != 0 as i32 {
        return status;
    }
    status = stats_add_string(st, &mut (*st).source_str, &mut (*st).source);
    if status != 0 as i32 {
        return status;
    }
    status = stats_add_string(st, &mut (*st).version_str, &mut (*st).version);
    if status != 0 as i32 {
        return status;
    }
    status = stats_add_num(st, &mut (*st).uptime_str, uptime);
    if status != 0 as i32 {
        return status;
    }
    status = stats_add_num(st, &mut (*st).timestamp_str, cur_ts);
    if status != 0 as i32 {
        return status;
    }
    status = stats_add_num(
        st,
        &mut (*st).ntotal_conn_str,
        conn_ntotal_conn() as int64_t,
    );
    if status != 0 as i32 {
        return status;
    }
    status = stats_add_num(st, &mut (*st).ncurr_conn_str, conn_ncurr_conn() as int64_t);
    if status != 0 as i32 {
        return status;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_add_footer(mut st: *mut stats) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    buf = &mut (*st).buf;
    if (*buf).len == (*buf).size {
        return -(1 as i32);
    }
    pos = ((*buf).data).offset((*buf).len as isize).offset(-(1 as i32 as isize));
    *pos.offset(0 as i32 as isize) = '}' as i32 as uint8_t;
    *pos.offset(1 as i32 as isize) = '\n' as i32 as uint8_t;
    (*buf).len = ((*buf).len as u64).wrapping_add(1 as i32 as u64) as size_t as size_t;
    return 0 as i32;
}
unsafe extern "C" fn stats_begin_nesting(
    mut st: *mut stats,
    mut key: *const string,
) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    let mut room: size_t = 0;
    let mut n: i32 = 0;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    room = ((*buf).size).wrapping_sub((*buf).len).wrapping_sub(1 as i32 as u64);
    n = snprintf(
        pos as *mut i8,
        room,
        b"\"%.*s\": {\0" as *const u8 as *const i8,
        (*key).len,
        (*key).data,
    );
    if n < 0 as i32 || n >= room as i32 {
        return -(1 as i32);
    }
    (*buf).len = ((*buf).len as u64).wrapping_add(n as size_t) as size_t as size_t;
    return 0 as i32;
}
unsafe extern "C" fn stats_end_nesting(mut st: *mut stats) -> rstatus_t {
    let mut buf: *mut stats_buffer = 0 as *mut stats_buffer;
    let mut pos: *mut uint8_t = 0 as *mut uint8_t;
    buf = &mut (*st).buf;
    pos = ((*buf).data).offset((*buf).len as isize);
    pos = pos.offset(-(2 as i32 as isize));
    match *pos.offset(0 as i32 as isize) as i32 {
        44 => {
            *pos.offset(0 as i32 as isize) = '}' as i32 as uint8_t;
            *pos.offset(1 as i32 as isize) = ',' as i32 as uint8_t;
        }
        125 => {
            if (*buf).len == (*buf).size {
                return -(1 as i32);
            }
            *pos.offset(1 as i32 as isize) = '}' as i32 as uint8_t;
            *pos.offset(2 as i32 as isize) = ',' as i32 as uint8_t;
            (*buf).len = ((*buf).len as u64).wrapping_add(1 as i32 as u64) as size_t
                as size_t;
        }
        _ => {}
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_copy_metric(
    mut st: *mut stats,
    mut metric: *mut array,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    i = 0 as i32 as uint32_t;
    while i < array_n(metric) {
        let mut stm: *mut stats_metric = array_get(metric, i) as *mut stats_metric;
        status = stats_add_num(st, &mut (*stm).name, (*stm).value.counter);
        if status != 0 as i32 {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_aggregate_metric(mut dst: *mut array, mut src: *const array) {
    let mut i: uint32_t = 0;
    i = 0 as i32 as uint32_t;
    while i < array_n(src) {
        let mut stm1: *const stats_metric = 0 as *const stats_metric;
        let mut stm2: *mut stats_metric = 0 as *mut stats_metric;
        stm1 = array_get(src, i) as *const stats_metric;
        stm2 = array_get(dst, i) as *mut stats_metric;
        match (*stm1).type_0 as u32 {
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
    if (*st).aggregate == 0 as i32 {
        return;
    }
    i = 0 as i32 as uint32_t;
    while i < array_n(&mut (*st).shadow) {
        let mut stp1: *mut stats_pool = 0 as *mut stats_pool;
        let mut stp2: *mut stats_pool = 0 as *mut stats_pool;
        let mut j: uint32_t = 0;
        stp1 = array_get(&mut (*st).shadow, i) as *mut stats_pool;
        stp2 = array_get(&mut (*st).sum, i) as *mut stats_pool;
        stats_aggregate_metric(&mut (*stp2).metric, &mut (*stp1).metric);
        j = 0 as i32 as uint32_t;
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
    ::core::ptr::write_volatile(&mut (*st).aggregate as *mut i32, 0 as i32);
}
unsafe extern "C" fn stats_make_rsp(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    status = stats_add_header(st);
    if status != 0 as i32 {
        return status;
    }
    i = 0 as i32 as uint32_t;
    while i < array_n(&mut (*st).sum) {
        let mut stp: *mut stats_pool = array_get(&mut (*st).sum, i) as *mut stats_pool;
        let mut j: uint32_t = 0;
        status = stats_begin_nesting(st, &mut (*stp).name);
        if status != 0 as i32 {
            return status;
        }
        status = stats_copy_metric(st, &mut (*stp).metric);
        if status != 0 as i32 {
            return status;
        }
        j = 0 as i32 as uint32_t;
        while j < array_n(&mut (*stp).server) {
            let mut sts: *mut stats_server = array_get(&mut (*stp).server, j)
                as *mut stats_server;
            status = stats_begin_nesting(st, &mut (*sts).name);
            if status != 0 as i32 {
                return status;
            }
            status = stats_copy_metric(st, &mut (*sts).metric);
            if status != 0 as i32 {
                return status;
            }
            status = stats_end_nesting(st);
            if status != 0 as i32 {
                return status;
            }
            j = j.wrapping_add(1);
            j;
        }
        status = stats_end_nesting(st);
        if status != 0 as i32 {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    status = stats_add_footer(st);
    if status != 0 as i32 {
        return status;
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_send_rsp(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut n: ssize_t = 0;
    let mut sd: i32 = 0;
    status = stats_make_rsp(st);
    if status != 0 as i32 {
        return status;
    }
    sd = accept(
        (*st).sd,
        __SOCKADDR_ARG {
            __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
        },
        0 as *mut socklen_t,
    );
    if sd < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                773 as i32,
                0 as i32,
                b"accept on m %d failed: %s\0" as *const u8 as *const i8,
                (*st).sd,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    n = _nc_sendn(sd, (*st).buf.data as *const libc::c_void, (*st).buf.len);
    if n < 0 as i32 as i64 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                781 as i32,
                0 as i32,
                b"send stats on sd %d failed: %s\0" as *const u8 as *const i8,
                sd,
                strerror(*__errno_location()),
            );
        }
        close(sd);
        return -(1 as i32);
    }
    close(sd);
    return 0 as i32;
}
unsafe extern "C" fn stats_loop_callback(
    mut arg1: *mut libc::c_void,
    mut arg2: *mut libc::c_void,
) {
    let mut st: *mut stats = arg1 as *mut stats;
    let mut n: i32 = *(arg2 as *mut i32);
    stats_aggregate(st);
    if n == 0 as i32 {
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
    status = nc_resolve(&mut (*st).addr, (*st).port as i32, &mut si);
    if status < 0 as i32 {
        return status;
    }
    (*st).sd = socket(si.family, __socket_type::SOCK_STREAM as i32, 0 as i32);
    if (*st).sd < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                828 as i32,
                0 as i32,
                b"socket failed: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    nc_set_reuseport((*st).sd);
    status = nc_set_reuseaddr((*st).sd);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                834 as i32,
                0 as i32,
                b"set reuseaddr on m %d failed for stats server: %s\0" as *const u8
                    as *const i8,
                (*st).sd,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    status = bind(
        (*st).sd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut si.addr as *mut C2RustUnnamed_4 as *mut sockaddr,
        },
        si.addrlen,
    );
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                841 as i32,
                0 as i32,
                b"bind on m %d to stats server addr '%.*s:%u' failed: %s\0" as *const u8
                    as *const i8,
                (*st).sd,
                (*st).addr.len,
                (*st).addr.data,
                (*st).port as i32,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    status = listen((*st).sd, 128 as i32);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                848 as i32,
                0 as i32,
                b"listen on m %d for stats server '%.*s:%u' failed: %s\0" as *const u8
                    as *const i8,
                (*st).sd,
                (*st).addr.len,
                (*st).addr.data,
                (*st).port as i32,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_start_aggregator(mut st: *mut stats) -> rstatus_t {
    let mut status: rstatus_t = 0;
    if 1 as i32 == 0 {
        return 0 as i32;
    }
    status = stats_listen(st);
    if status != 0 as i32 {
        return status;
    }
    status = pthread_create(
        &mut (*st).tid,
        0 as *const pthread_attr_t,
        Some(stats_loop as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        st as *mut libc::c_void,
    );
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_stats.c\0" as *const u8 as *const i8,
                874 as i32,
                0 as i32,
                b"stats aggregator create failed: %s\0" as *const u8 as *const i8,
                strerror(status),
            );
        }
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn stats_stop_aggregator(mut st: *mut stats) {
    if 1 as i32 == 0 {
        return;
    }
    close((*st).sd);
}
#[no_mangle]
pub unsafe extern "C" fn stats_create(
    mut stats_port: uint16_t,
    mut stats_ip: *const i8,
    mut stats_interval: i32,
    mut source: *const i8,
    mut server_pool: *const array,
) -> *mut stats {
    let mut status: rstatus_t = 0;
    let mut st: *mut stats = 0 as *mut stats;
    st = _nc_alloc(
        ::core::mem::size_of::<stats>() as u64,
        b"nc_stats.c\0" as *const u8 as *const i8,
        898 as i32,
    ) as *mut stats;
    if st.is_null() {
        return 0 as *mut stats;
    }
    (*st).port = stats_port;
    (*st).interval = stats_interval;
    (*st).addr.len = strlen(stats_ip as *mut i8) as uint32_t;
    (*st).addr.data = stats_ip as *mut uint8_t;
    (*st).start_ts = time(0 as *mut time_t);
    (*st).buf.len = 0 as i32 as size_t;
    (*st).buf.data = 0 as *mut uint8_t;
    (*st).buf.size = 0 as i32 as size_t;
    array_null(&mut (*st).current);
    array_null(&mut (*st).shadow);
    array_null(&mut (*st).sum);
    (*st).tid = -(1 as i32) as pthread_t;
    (*st).sd = -(1 as i32);
    (*st).service_str.len = (::core::mem::size_of::<[i8; 8]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).service_str.data = b"service\0" as *const u8 as *const i8 as *mut uint8_t;
    (*st).service.len = (::core::mem::size_of::<[i8; 11]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).service.data = b"nutcracker\0" as *const u8 as *const i8 as *mut uint8_t;
    (*st).source_str.len = (::core::mem::size_of::<[i8; 7]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).source_str.data = b"source\0" as *const u8 as *const i8 as *mut uint8_t;
    (*st).source.len = strlen(source as *mut i8) as uint32_t;
    (*st).source.data = source as *mut uint8_t;
    (*st).version_str.len = (::core::mem::size_of::<[i8; 8]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).version_str.data = b"version\0" as *const u8 as *const i8 as *mut uint8_t;
    (*st).version.len = (::core::mem::size_of::<[i8; 6]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).version.data = b"0.5.0\0" as *const u8 as *const i8 as *mut uint8_t;
    (*st).uptime_str.len = (::core::mem::size_of::<[i8; 7]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).uptime_str.data = b"uptime\0" as *const u8 as *const i8 as *mut uint8_t;
    (*st).timestamp_str.len = (::core::mem::size_of::<[i8; 10]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).timestamp_str.data = b"timestamp\0" as *const u8 as *const i8 as *mut uint8_t;
    (*st).ntotal_conn_str.len = (::core::mem::size_of::<[i8; 18]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).ntotal_conn_str.data = b"total_connections\0" as *const u8 as *const i8
        as *mut uint8_t;
    (*st).ncurr_conn_str.len = (::core::mem::size_of::<[i8; 17]>() as u64)
        .wrapping_sub(1 as i32 as u64) as uint32_t;
    (*st).ncurr_conn_str.data = b"curr_connections\0" as *const u8 as *const i8
        as *mut uint8_t;
    ::core::ptr::write_volatile(&mut (*st).updated as *mut i32, 0 as i32);
    ::core::ptr::write_volatile(&mut (*st).aggregate as *mut i32, 0 as i32);
    status = stats_pool_map(&mut (*st).current, server_pool);
    if !(status != 0 as i32) {
        status = stats_pool_map(&mut (*st).shadow, server_pool);
        if !(status != 0 as i32) {
            status = stats_pool_map(&mut (*st).sum, server_pool);
            if !(status != 0 as i32) {
                status = stats_create_buf(st);
                if !(status != 0 as i32) {
                    status = stats_start_aggregator(st);
                    if !(status != 0 as i32) {
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
        b"nc_stats.c\0" as *const u8 as *const i8,
        980 as i32,
    );
    st = 0 as *mut stats;
}
#[no_mangle]
pub unsafe extern "C" fn stats_swap(mut st: *mut stats) {
    if 1 as i32 == 0 {
        return;
    }
    if (*st).aggregate == 1 as i32 {
        return;
    }
    if (*st).updated == 0 as i32 {
        return;
    }
    array_swap(&mut (*st).current, &mut (*st).shadow);
    stats_pool_reset(&mut (*st).current);
    ::core::ptr::write_volatile(&mut (*st).updated as *mut i32, 0 as i32);
    ::core::ptr::write_volatile(&mut (*st).aggregate as *mut i32, 1 as i32);
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
    ::core::ptr::write_volatile(&mut (*st).updated as *mut i32, 1 as i32);
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
    ::core::ptr::write_volatile(&mut (*st).updated as *mut i32, 1 as i32);
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
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 11]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"client_eof\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 11]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"client_err\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 19]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"client_connections\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 14]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_ejects\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 14]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"forward_error\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 10]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"fragments\0" as *const u8 as *const i8 as *mut uint8_t,
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
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 11]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_eof\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 11]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_err\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 16]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_timedout\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 19]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_connections\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_TIMESTAMP,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 18]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_ejected_at\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 9]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"requests\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 14]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"request_bytes\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 10]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"responses\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_COUNTER,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 15]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"response_bytes\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 9]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"in_queue\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 15]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"in_queue_bytes\0" as *const u8 as *const i8
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
                type_0: stats_type::STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 10]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"out_queue\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                value: C2RustUnnamed_6 { counter: 0 },
            };
            init
        },
        {
            let mut init = stats_metric {
                type_0: stats_type::STATS_GAUGE,
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 16]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"out_queue_bytes\0" as *const u8 as *const i8
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