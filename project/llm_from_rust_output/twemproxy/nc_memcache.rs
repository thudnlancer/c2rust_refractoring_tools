use std::ptr;
use std::os::raw::{c_int, c_uint, c_uchar, c_ushort, c_ulong, c_long, c_void};
use std::mem;
use std::ffi::CStr;
use libc::{self, sockaddr, socklen_t, sa_family_t, in_addr_t, in_port_t};

type RstatusT = c_int;
type ErrT = c_int;
type Uint8T = c_uchar;
type Uint16T = c_ushort;
type Uint32T = c_uint;
type Uint64T = c_ulong;
type Int64T = c_long;
type SizeT = c_ulong;

#[derive(Debug, Clone, Copy)]
struct Array {
    nelem: Uint32T,
    elem: *mut c_void,
    size: SizeT,
    nalloc: Uint32T,
}

#[derive(Debug, Clone, Copy)]
struct String {
    len: Uint32T,
    data: *mut Uint8T,
}

#[derive(Debug, Clone, Copy)]
struct Context {
    id: Uint32T,
    cf: *mut Conf,
    stats: *mut Stats,
    pool: Array,
    evb: *mut EventBase,
    max_timeout: c_int,
    timeout: c_int,
    max_nfd: Uint32T,
    max_ncconn: Uint32T,
    max_nsconn: Uint32T,
}

#[derive(Debug, Clone, Copy)]
struct EventBase {
    ep: c_int,
    event: *mut EpollEvent,
    nevent: c_int,
    cb: EventCbT,
}

type EventCbT = Option<unsafe extern "C" fn(*mut c_void, Uint32T) -> c_int>;

#[derive(Debug, Clone, Copy)]
struct Stats {
    port: Uint16T,
    interval: c_int,
    addr: String,
    start_ts: Int64T,
    buf: StatsBuffer,
    current: Array,
    shadow: Array,
    sum: Array,
    tid: PthreadT,
    sd: c_int,
    service_str: String,
    service: String,
    source_str: String,
    source: String,
    version_str: String,
    version: String,
    uptime_str: String,
    timestamp_str: String,
    ntotal_conn_str: String,
    ncurr_conn_str: String,
    aggregate: c_int,
    updated: c_int,
}

type PthreadT = c_ulong;

#[derive(Debug, Clone, Copy)]
struct StatsBuffer {
    len: SizeT,
    data: *mut Uint8T,
    size: SizeT,
}

#[derive(Debug, Clone, Copy)]
struct Conn {
    conn_tqe: C2RustUnnamed6,
    owner: *mut c_void,
    sd: c_int,
    family: c_int,
    addrlen: socklen_t,
    addr: *mut sockaddr,
    imsg_q: MsgTqh,
    omsg_q: MsgTqh,
    rmsg: *mut Msg,
    smsg: *mut Msg,
    recv: ConnRecvT,
    recv_next: ConnRecvNextT,
    recv_done: ConnRecvDoneT,
    send: ConnSendT,
    send_next: ConnSendNextT,
    send_done: ConnSendDoneT,
    close: ConnCloseT,
    active: ConnActiveT,
    post_connect: ConnPostConnectT,
    swallow_msg: ConnSwallowMsgT,
    ref_: ConnRefT,
    unref: ConnUnrefT,
    enqueue_inq: ConnMsgqT,
    dequeue_inq: ConnMsgqT,
    enqueue_outq: ConnMsgqT,
    dequeue_outq: ConnMsgqT,
    recv_bytes: SizeT,
    send_bytes: SizeT,
    events: Uint32T,
    err: ErrT,
    flags: [u8; 2],
    padding: [u8; 6],
}

type ConnMsgqT = Option<unsafe extern "C" fn(*mut Context, *mut Conn, *mut Msg) -> ()>;

#[derive(Debug, Clone, Copy)]
struct Msg {
    c_tqe: C2RustUnnamed3,
    s_tqe: C2RustUnnamed2,
    m_tqe: C2RustUnnamed1,
    id: Uint64T,
    peer: *mut Msg,
    owner: *mut Conn,
    tmo_rbe: RbNode,
    mhdr: Mhdr,
    mlen: Uint32T,
    start_ts: Int64T,
    state: c_int,
    pos: *mut Uint8T,
    token: *mut Uint8T,
    parser: MsgParseT,
    result: MsgParseResult,
    fragment: MsgFragmentT,
    reply: MsgReplyT,
    add_auth: MsgAddAuthT,
    failure: MsgFailureT,
    pre_coalesce: MsgCoalesceT,
    post_coalesce: MsgCoalesceT,
    type_: MsgType,
    keys: *mut Array,
    vlen: Uint32T,
    end: *mut Uint8T,
    narg_start: *mut Uint8T,
    narg_end: *mut Uint8T,
    narg: Uint32T,
    rnarg: Uint32T,
    rlen: Uint32T,
    integer: Uint32T,
    is_top_level: Uint8T,
    frag_owner: *mut Msg,
    nfrag: Uint32T,
    nfrag_done: Uint32T,
    frag_id: Uint64T,
    frag_seq: *mut *mut Msg,
    err: ErrT,
    flags: [u8; 2],
    padding: [u8; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum MsgType {
    Unknown = 0,
    ReqMcGet = 1,
    ReqMcGets = 2,
    ReqMcDelete = 3,
    ReqMcCas = 4,
    ReqMcSet = 5,
    ReqMcAdd = 6,
    ReqMcReplace = 7,
    ReqMcAppend = 8,
    ReqMcPrepend = 9,
    ReqMcIncr = 10,
    ReqMcDecr = 11,
    ReqMcTouch = 12,
    ReqMcQuit = 13,
    ReqMcVersion = 14,
    RspMcNum = 15,
    RspMcStored = 16,
    RspMcNotStored = 17,
    RspMcExists = 18,
    RspMcNotFound = 19,
    RspMcEnd = 20,
    RspMcValue = 21,
    RspMcDeleted = 22,
    RspMcTouched = 23,
    RspMcVersion = 24,
    RspMcError = 25,
    RspMcClientError = 26,
    RspMcServerError = 27,
    // ... other variants
    Sentinel = 184,
}

type MsgCoalesceT = Option<unsafe extern "C" fn(*mut Msg) -> ()>;
type MsgFailureT = Option<unsafe extern "C" fn(*const Msg) -> bool>;
type MsgAddAuthT = Option<unsafe extern "C" fn(*mut Context, *mut Conn, *mut Conn) -> RstatusT>;
type MsgReplyT = Option<unsafe extern "C" fn(*mut Msg) -> RstatusT>;
type MsgFragmentT = Option<unsafe extern "C" fn(*mut Msg, Uint32T, *mut MsgTqh) -> RstatusT>;

#[derive(Debug, Clone, Copy)]
struct MsgTqh {
    tqh_first: *mut Msg,
    tqh_last: *mut *mut Msg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum MsgParseResult {
    Ok = 0,
    Error = 1,
    Repair = 2,
    Again = 3,
}

type MsgParseT = Option<unsafe extern "C" fn(*mut Msg) -> ()>;

#[derive(Debug, Clone, Copy)]
struct Mhdr {
    stqh_first: *mut Mbuf,
    stqh_last: *mut *mut Mbuf,
}

#[derive(Debug, Clone, Copy)]
struct Mbuf {
    magic: Uint32T,
    next: C2RustUnnamed0,
    pos: *mut Uint8T,
    last: *mut Uint8T,
    start: *mut Uint8T,
    end: *mut Uint8T,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed0 {
    stqe_next: *mut Mbuf,
}

#[derive(Debug, Clone, Copy)]
struct RbNode {
    left: *mut RbNode,
    right: *mut RbNode,
    parent: *mut RbNode,
    key: Int64T,
    data: *mut c_void,
    color: Uint8T,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed1 {
    tqe_next: *mut Msg,
    tqe_prev: *mut *mut Msg,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed2 {
    tqe_next: *mut Msg,
    tqe_prev: *mut *mut Msg,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed3 {
    tqe_next: *mut Msg,
    tqe_prev: *mut *mut Msg,
}

type ConnUnrefT = Option<unsafe extern "C" fn(*mut Conn) -> ()>;
type ConnRefT = Option<unsafe extern "C" fn(*mut Conn, *mut c_void) -> ()>;
type ConnSwallowMsgT = Option<unsafe extern "C" fn(*mut Conn, *mut Msg, *mut Msg) -> ()>;
type ConnPostConnectT = Option<unsafe extern "C" fn(*mut Context, *mut Conn, *mut Server) -> ()>;

#[derive(Debug, Clone, Copy)]
struct Server {
    idx: Uint32T,
    owner: *mut ServerPool,
    pname: String,
    name: String,
    addrstr: String,
    port: Uint16T,
    weight: Uint32T,
    info: SockInfo,
    ns_conn_q: Uint32T,
    s_conn_q: ConnTqh,
    next_retry: Int64T,
    failure_count: Uint32T,
}

#[derive(Debug, Clone, Copy)]
struct ConnTqh {
    tqh_first: *mut Conn,
    tqh_last: *mut *mut Conn,
}

#[derive(Debug, Clone, Copy)]
struct SockInfo {
    family: c_int,
    addrlen: socklen_t,
    addr: C2RustUnnamed4,
}

#[derive(Debug, Clone, Copy)]
union C2RustUnnamed4 {
    in_: sockaddr_in,
    in6: sockaddr_in6,
    un: sockaddr_un,
}

#[derive(Debug, Clone, Copy)]
struct sockaddr_in {
    sin_family: sa_family_t,
    sin_port: in_port_t,
    sin_addr: in_addr,
    sin_zero: [c_uchar; 8],
}

#[derive(Debug, Clone, Copy)]
struct in_addr {
    s_addr: in_addr_t,
}

#[derive(Debug, Clone, Copy)]
struct sockaddr_in6 {
    sin6_family: sa_family_t,
    sin6_port: in_port_t,
    sin6_flowinfo: Uint32T,
    sin6_addr: in6_addr,
    sin6_scope_id: Uint32T,
}

#[derive(Debug, Clone, Copy)]
struct in6_addr {
    __in6_u: C2RustUnnamed5,
}

#[derive(Debug, Clone, Copy)]
union C2RustUnnamed5 {
    __u6_addr8: [Uint8T; 16],
    __u6_addr16: [Uint16T; 8],
    __u6_addr32: [Uint32T; 4],
}

#[derive(Debug, Clone, Copy)]
struct sockaddr_un {
    sun_family: sa_family_t,
    sun_path: [c_char; 108],
}

#[derive(Debug, Clone, Copy)]
struct ServerPool {
    idx: Uint32T,
    ctx: *mut Context,
    p_conn: *mut Conn,
    nc_conn_q: Uint32T,
    c_conn_q: ConnTqh,
    server: Array,
    ncontinuum: Uint32T,
    nserver_continuum: Uint32T,
    continuum: *mut Continuum,
    nlive_server: Uint32T,
    next_rebuild: Int64T,
    name: String,
    addrstr: String,
    port: Uint16T,
    info: SockInfo,
    perm: ModeT,
    dist_type: c_int,
    key_hash_type: c_int,
    key_hash: HashT,
    hash_tag: String,
    timeout: c_int,
    backlog: c_int,
    redis_db: c_int,
    client_connections: Uint32T,
    server_connections: Uint32T,
    server_retry_timeout: Int64T,
    server_failure_limit: Uint32T,
    redis_auth: String,
    require_auth: c_uint,
    flags: [u8; 1],
    padding: [u8; 3],
}

type HashT = Option<unsafe extern "C" fn(*const c_char, SizeT) -> Uint32T>;
type ModeT = c_uint;

#[derive(Debug, Clone, Copy)]
struct Continuum {
    index: Uint32T,
    value: Uint32T,
}

type ConnActiveT = Option<unsafe extern "C" fn(*const Conn) -> bool>;
type ConnCloseT = Option<unsafe extern "C" fn(*mut Context, *mut Conn) -> ()>;
type ConnSendDoneT = Option<unsafe extern "C" fn(*mut Context, *mut Conn, *mut Msg) -> ()>;
type ConnSendNextT = Option<unsafe extern "C" fn(*mut Context, *mut Conn) -> *mut Msg>;
type ConnSendT = Option<unsafe extern "C" fn(*mut Context, *mut Conn) -> RstatusT>;
type ConnRecvDoneT = Option<unsafe extern "C" fn(*mut Context, *mut Conn, *mut Msg, *mut Msg) -> ()>;
type ConnRecvNextT = Option<unsafe extern "C" fn(*mut Context, *mut Conn, bool) -> *mut Msg>;
type ConnRecvT = Option<unsafe extern "C" fn(*mut Context, *mut Conn) -> RstatusT>;

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed6 {
    tqe_next: *mut Conn,
    tqe_prev: *mut *mut Conn,
}

#[derive(Debug, Clone, Copy)]
struct KeyPos {
    start: *mut Uint8T,
    end: *mut Uint8T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum ParseStateReq {
    Start = 0,
    ReqType = 1,
    SpacesBeforeKey = 2,
    Key = 3,
    SpacesBeforeKeys = 4,
    SpacesBeforeFlags = 5,
    Flags = 6,
    SpacesBeforeExpiry = 7,
    Expiry = 8,
    SpacesBeforeVlen = 9,
    Vlen = 10,
    SpacesBeforeCas = 11,
    Cas = 12,
    Crlf = 13,
    Val = 14,
    SpacesBeforeNum = 15,
    Num = 16,
    RuntoCrlf = 17,
    AlmostDone = 18,
    AfterNoreply = 19,
    Noreply = 20,
    ValLf = 21,
    Sentinel = 22,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum ParseStateRsp {
    Start = 0,
    RspNum = 1,
    RspStr = 2,
    SpacesBeforeKey = 3,
    Key = 4,
    SpacesBeforeFlags = 5,
    Flags = 6,
    SpacesBeforeVlen = 7,
    Vlen = 8,
    Crlf = 9,
    Val = 10,
    ValLf = 11,
    End = 12,
    RuntoCrlf = 13,
    RuntoVal = 14,
    AlmostDone = 15,
    Sentinel = 16,
}

// Helper functions
fn array_n(a: *const Array) -> Uint32T {
    unsafe { (*a).nelem }
}

fn mbuf_empty(mbuf: *const Mbuf) -> bool {
    unsafe { (*mbuf).pos == (*mbuf).last }
}

fn memcache_storage(r: *const Msg) -> bool {
    unsafe {
        matches!(
            (*r).type_,
            MsgType::ReqMcSet
                | MsgType::ReqMcCas
                | MsgType::ReqMcAdd
                | MsgType::ReqMcReplace
                | MsgType::ReqMcAppend
                | MsgType::ReqMcPrepend
        )
    }
}

fn memcache_cas(r: *const Msg) -> bool {
    unsafe { (*r).type_ == MsgType::ReqMcCas }
}

fn memcache_retrieval(r: *const Msg) -> bool {
    unsafe { matches!((*r).type_, MsgType::ReqMcGet | MsgType::ReqMcGets) }
}

fn memcache_should_fragment(r: *const Msg) -> bool {
    unsafe {
        matches!((*r).type_, MsgType::ReqMcGet | MsgType::ReqMcGets) && array_n((*r).keys) !=