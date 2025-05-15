use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    pub type conf;
    fn close(__fd: i32) -> i32;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn log_loggable(level: i32) -> i32;
    fn _log(file: *const i8, line: i32, panic: i32, fmt: *const i8, _: ...);
    fn _stats_pool_incr(
        ctx: *mut context,
        pool: *const server_pool,
        fidx: stats_pool_field_t,
    );
    fn _stats_pool_decr(
        ctx: *mut context,
        pool: *const server_pool,
        fidx: stats_pool_field_t,
    );
    fn req_put(msg: *mut msg);
    fn conn_put(conn: *mut conn);
}
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
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
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
pub type pthread_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type int64_t = __int64_t;
pub type __int64_t = i64;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
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
pub type uint64_t = __uint64_t;
pub type __uint64_t = u64;
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
    pub sun_path: [i8; 108],
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
    pub sin_zero: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type socklen_t = __socklen_t;
pub type __socklen_t = u32;
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
pub type mode_t = __mode_t;
pub type __mode_t = u32;
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
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
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
#[no_mangle]
pub unsafe extern "C" fn client_ref(mut conn: *mut conn, mut owner: *mut libc::c_void) {
    let mut pool: *mut server_pool = owner as *mut server_pool;
    (*conn).family = 0 as i32;
    (*conn).addrlen = 0 as i32 as socklen_t;
    (*conn).addr = 0 as *mut sockaddr;
    (*pool).nc_conn_q = ((*pool).nc_conn_q).wrapping_add(1);
    (*pool).nc_conn_q;
    (*conn).conn_tqe.tqe_next = 0 as *mut conn;
    (*conn).conn_tqe.tqe_prev = (*pool).c_conn_q.tqh_last;
    *(*pool).c_conn_q.tqh_last = conn;
    (*pool).c_conn_q.tqh_last = &mut (*conn).conn_tqe.tqe_next;
    (*conn).owner = owner;
}
#[no_mangle]
pub unsafe extern "C" fn client_unref(mut conn: *mut conn) {
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    pool = (*conn).owner as *mut server_pool;
    (*conn).owner = 0 as *mut libc::c_void;
    (*pool).nc_conn_q = ((*pool).nc_conn_q).wrapping_sub(1);
    (*pool).nc_conn_q;
    let mut oldnext: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_next
        as *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
    let mut oldprev: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_prev
        as *mut *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
    if !((*conn).conn_tqe.tqe_next).is_null() {
        (*(*conn).conn_tqe.tqe_next).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_prev;
    } else {
        (*pool).c_conn_q.tqh_last = (*conn).conn_tqe.tqe_prev;
    }
    *(*conn).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_next;
    *oldnext = 0 as *mut libc::c_void;
    *oldprev = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn client_active(mut conn: *const conn) -> bool {
    if !((*conn).omsg_q.tqh_first).is_null() {
        return 1 as i32 != 0;
    }
    if !((*conn).rmsg).is_null() {
        return 1 as i32 != 0;
    }
    if !((*conn).smsg).is_null() {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn client_close_stats(
    mut ctx: *mut context,
    mut pool: *mut server_pool,
    mut err: err_t,
    mut eof: u32,
) {
    _stats_pool_decr(ctx, pool, stats_pool_field::STATS_POOL_client_connections);
    if eof != 0 {
        _stats_pool_incr(ctx, pool, stats_pool_field::STATS_POOL_client_eof);
        return;
    }
    match err {
        32 | 110 | 104 | 103 | 107 | 100 | 101 | 112 | 113 | _ => {}
    }
    _stats_pool_incr(ctx, pool, stats_pool_field::STATS_POOL_client_err);
}
#[no_mangle]
pub unsafe extern "C" fn client_close(mut ctx: *mut context, mut conn: *mut conn) {
    let mut status: rstatus_t = 0;
    let mut msg: *mut msg = 0 as *mut msg;
    let mut nmsg: *mut msg = 0 as *mut msg;
    client_close_stats(
        ctx,
        (*conn).owner as *mut server_pool,
        (*conn).err,
        (*conn).eof(),
    );
    if (*conn).sd < 0 as i32 {
        ((*conn).unref).expect("non-null function pointer")(conn);
        conn_put(conn);
        return;
    }
    msg = (*conn).rmsg;
    if !msg.is_null() {
        (*conn).rmsg = 0 as *mut msg;
        req_put(msg);
    }
    msg = (*conn).omsg_q.tqh_first;
    while !msg.is_null() {
        nmsg = (*msg).c_tqe.tqe_next;
        ((*conn).dequeue_outq).expect("non-null function pointer")(ctx, conn, msg);
        if (*msg).done() != 0 {
            req_put(msg);
        } else {
            (*msg).set_swallow(1 as i32 as u32);
        }
        msg = nmsg;
    }
    ((*conn).unref).expect("non-null function pointer")(conn);
    status = close((*conn).sd);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_client.c\0" as *const u8 as *const i8,
                184 as i32,
                0 as i32,
                b"close c %d failed, ignored: %s\0" as *const u8 as *const i8,
                (*conn).sd,
                strerror(*__errno_location()),
            );
        }
    }
    (*conn).sd = -(1 as i32);
    conn_put(conn);
}