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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    pub type event_base;
    pub type evbuffer;
    pub type dict;
    pub type sockadr;
    pub type pool;
    pub type acl;
    pub type cmd;
    pub type http_header;
    fn socket(__domain: i32, __type: i32, __protocol: i32) -> i32;
    fn bind(__fd: i32, __addr: *const sockaddr, __len: socklen_t) -> i32;
    fn getsockname(__fd: i32, __addr: *mut sockaddr, __len: *mut socklen_t) -> i32;
    fn setsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> i32;
    fn listen(__fd: i32, __n: i32) -> i32;
    fn accept(__fd: i32, __addr: *mut sockaddr, __addr_len: *mut socklen_t) -> i32;
    fn __errno_location() -> *mut i32;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn event_base_new() -> *mut event_base;
    fn event_reinit(base: *mut event_base) -> i32;
    fn event_base_dispatch(_: *mut event_base) -> i32;
    fn event_base_set(_: *mut event_base, _: *mut event) -> i32;
    fn event_add(ev: *mut event, timeout: *const timeval) -> i32;
    fn event_set(
        _: *mut event,
        _: i32,
        _: libc::c_short,
        _: Option<unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    );
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> i32;
    fn worker_new(s: *mut server) -> *mut worker;
    fn worker_start(w: *mut worker);
    fn worker_add_client(w: *mut worker, c: *mut http_client);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn inet_addr(__cp: *const i8) -> in_addr_t;
    fn exit(_: i32) -> !;
    fn http_client_new(w: *mut worker, fd: i32, addr: in_addr_t) -> *mut http_client;
    fn conf_read(filename: *const i8) -> *mut conf;
    fn slog_fsync_init(s: *mut server);
    fn slog_init(s: *mut server);
    fn slog(s: *mut server, level: log_level, body: *const i8, sz: size_t);
    fn close(__fd: i32) -> i32;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn fsync(__fd: i32) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
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
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    IPPROTO_MAX = 256,
    IPPROTO_RAW = 255,
    IPPROTO_MPLS = 137,
    IPPROTO_UDPLITE = 136,
    IPPROTO_SCTP = 132,
    IPPROTO_COMP = 108,
    IPPROTO_PIM = 103,
    IPPROTO_ENCAP = 98,
    IPPROTO_BEETPH = 94,
    IPPROTO_MTP = 92,
    IPPROTO_AH = 51,
    IPPROTO_ESP = 50,
    IPPROTO_GRE = 47,
    IPPROTO_RSVP = 46,
    IPPROTO_IPV6 = 41,
    IPPROTO_DCCP = 33,
    IPPROTO_TP = 29,
    IPPROTO_IDP = 22,
    IPPROTO_UDP = 17,
    IPPROTO_PUP = 12,
    IPPROTO_EGP = 8,
    IPPROTO_TCP = 6,
    IPPROTO_IPIP = 4,
    IPPROTO_IGMP = 2,
    IPPROTO_ICMP = 1,
    IPPROTO_IP = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::IPPROTO_MAX => 256,
            C2RustUnnamed::IPPROTO_RAW => 255,
            C2RustUnnamed::IPPROTO_MPLS => 137,
            C2RustUnnamed::IPPROTO_UDPLITE => 136,
            C2RustUnnamed::IPPROTO_SCTP => 132,
            C2RustUnnamed::IPPROTO_COMP => 108,
            C2RustUnnamed::IPPROTO_PIM => 103,
            C2RustUnnamed::IPPROTO_ENCAP => 98,
            C2RustUnnamed::IPPROTO_BEETPH => 94,
            C2RustUnnamed::IPPROTO_MTP => 92,
            C2RustUnnamed::IPPROTO_AH => 51,
            C2RustUnnamed::IPPROTO_ESP => 50,
            C2RustUnnamed::IPPROTO_GRE => 47,
            C2RustUnnamed::IPPROTO_RSVP => 46,
            C2RustUnnamed::IPPROTO_IPV6 => 41,
            C2RustUnnamed::IPPROTO_DCCP => 33,
            C2RustUnnamed::IPPROTO_TP => 29,
            C2RustUnnamed::IPPROTO_IDP => 22,
            C2RustUnnamed::IPPROTO_UDP => 17,
            C2RustUnnamed::IPPROTO_PUP => 12,
            C2RustUnnamed::IPPROTO_EGP => 8,
            C2RustUnnamed::IPPROTO_TCP => 6,
            C2RustUnnamed::IPPROTO_IPIP => 4,
            C2RustUnnamed::IPPROTO_IGMP => 2,
            C2RustUnnamed::IPPROTO_ICMP => 1,
            C2RustUnnamed::IPPROTO_IP => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            256 => C2RustUnnamed::IPPROTO_MAX,
            255 => C2RustUnnamed::IPPROTO_RAW,
            137 => C2RustUnnamed::IPPROTO_MPLS,
            136 => C2RustUnnamed::IPPROTO_UDPLITE,
            132 => C2RustUnnamed::IPPROTO_SCTP,
            108 => C2RustUnnamed::IPPROTO_COMP,
            103 => C2RustUnnamed::IPPROTO_PIM,
            98 => C2RustUnnamed::IPPROTO_ENCAP,
            94 => C2RustUnnamed::IPPROTO_BEETPH,
            92 => C2RustUnnamed::IPPROTO_MTP,
            51 => C2RustUnnamed::IPPROTO_AH,
            50 => C2RustUnnamed::IPPROTO_ESP,
            47 => C2RustUnnamed::IPPROTO_GRE,
            46 => C2RustUnnamed::IPPROTO_RSVP,
            41 => C2RustUnnamed::IPPROTO_IPV6,
            33 => C2RustUnnamed::IPPROTO_DCCP,
            29 => C2RustUnnamed::IPPROTO_TP,
            22 => C2RustUnnamed::IPPROTO_IDP,
            17 => C2RustUnnamed::IPPROTO_UDP,
            12 => C2RustUnnamed::IPPROTO_PUP,
            8 => C2RustUnnamed::IPPROTO_EGP,
            6 => C2RustUnnamed::IPPROTO_TCP,
            4 => C2RustUnnamed::IPPROTO_IPIP,
            2 => C2RustUnnamed::IPPROTO_IGMP,
            1 => C2RustUnnamed::IPPROTO_ICMP,
            0 => C2RustUnnamed::IPPROTO_IP,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
pub struct event {
    pub ev_evcallback: event_callback,
    pub ev_timeout_pos: C2RustUnnamed_5,
    pub ev_fd: i32,
    pub ev_base: *mut event_base,
    pub ev_: C2RustUnnamed_0,
    pub ev_events: libc::c_short,
    pub ev_res: libc::c_short,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ev_io: C2RustUnnamed_3,
    pub ev_signal: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ev_signal_next: C2RustUnnamed_2,
    pub ev_ncalls: libc::c_short,
    pub ev_pncalls: *mut libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ev_io_next: C2RustUnnamed_4,
    pub ev_timeout: timeval,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub le_next: *mut event,
    pub le_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ev_next_with_common_timeout: C2RustUnnamed_6,
    pub min_heap_idx: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub tqe_next: *mut event,
    pub tqe_prev: *mut *mut event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_callback {
    pub evcb_active_next: C2RustUnnamed_8,
    pub evcb_flags: libc::c_short,
    pub evcb_pri: uint8_t,
    pub evcb_closure: uint8_t,
    pub evcb_cb_union: C2RustUnnamed_7,
    pub evcb_arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
    pub tqe_next: *mut event_callback,
    pub tqe_prev: *mut *mut event_callback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
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
    pub ev: C2RustUnnamed_10,
    pub onDisconnect: Option<redisDisconnectCallback>,
    pub onConnect: Option<redisConnectCallback>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed_9,
    pub push_cb: Option<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
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
    pub tcp: C2RustUnnamed_12,
    pub unix_sock: C2RustUnnamed_11,
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
pub struct C2RustUnnamed_11 {
    pub path: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
impl AddAssign<u32> for redisConnectionType {
    fn add_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for redisConnectionType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for redisConnectionType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for redisConnectionType {
    fn div_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for redisConnectionType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn add(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn sub(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn mul(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn div(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for redisConnectionType {
    type Output = redisConnectionType;
    fn rem(self, rhs: u32) -> redisConnectionType {
        redisConnectionType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
    pub log: C2RustUnnamed_13,
    pub auth_log_mutex: pthread_mutex_t,
    pub auth_logged: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub log_fsync: C2RustUnnamed_15,
    pub hiredis_opts: C2RustUnnamed_14,
    pub default_root: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub keep_alive_sec: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
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
impl AddAssign<u32> for log_fsync_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for log_fsync_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for log_fsync_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for log_fsync_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for log_fsync_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn add(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn sub(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn mul(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn div(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for log_fsync_mode {
    type Output = log_fsync_mode;
    fn rem(self, rhs: u32) -> log_fsync_mode {
        log_fsync_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
impl AddAssign<u32> for log_level {
    fn add_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for log_level {
    fn sub_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for log_level {
    fn mul_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for log_level {
    fn div_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for log_level {
    fn rem_assign(&mut self, rhs: u32) {
        *self = log_level::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for log_level {
    type Output = log_level;
    fn add(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for log_level {
    type Output = log_level;
    fn sub(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for log_level {
    type Output = log_level;
    fn mul(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for log_level {
    type Output = log_level;
    fn div(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for log_level {
    type Output = log_level;
    fn rem(self, rhs: u32) -> log_level {
        log_level::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
impl AddAssign<u32> for last_cb_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for last_cb_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for last_cb_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for last_cb_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for last_cb_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = last_cb_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for last_cb_t {
    type Output = last_cb_t;
    fn add(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for last_cb_t {
    type Output = last_cb_t;
    fn sub(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for last_cb_t {
    type Output = last_cb_t;
    fn mul(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for last_cb_t {
    type Output = last_cb_t;
    fn div(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for last_cb_t {
    type Output = last_cb_t;
    fn rem(self, rhs: u32) -> last_cb_t {
        last_cb_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
unsafe extern "C" fn socket_setup(
    mut s: *mut server,
    mut ip: *const i8,
    mut port: i32,
) -> i32 {
    let mut reuse: i32 = 1 as i32;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut len: socklen_t = ::core::mem::size_of::<sockaddr_in>() as u64 as socklen_t;
    let mut fd: i32 = 0;
    let mut ret: i32 = 0;
    memset(
        &mut addr as *mut sockaddr_in as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<sockaddr_in>() as u64,
    );
    addr.sin_family = 2 as i32 as sa_family_t;
    addr.sin_port = ({
        let mut __v: libc::c_ushort = 0;
        let mut __x: libc::c_ushort = port as libc::c_ushort;
        if 0 != 0 {
            __v = (__x as i32 >> 8 as i32 & 0xff as i32
                | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
        } else {
            let fresh0 = &mut __v;
            let fresh1;
            let fresh2 = __x;
            asm!(
                "rorw $8, {0:x}", inlateout(reg)
                c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                options(pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
    });
    addr.sin_addr.s_addr = inet_addr(ip);
    fd = socket(
        2 as i32,
        __socket_type::SOCK_STREAM as i32,
        C2RustUnnamed::IPPROTO_TCP as i32,
    );
    if -(1 as i32) == fd {
        slog(
            s,
            log_level::WEBDIS_ERROR,
            strerror(*__errno_location()),
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    if setsockopt(
        fd,
        1 as i32,
        2 as i32,
        &mut reuse as *mut i32 as *const libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    ) < 0 as i32
    {
        slog(
            s,
            log_level::WEBDIS_ERROR,
            strerror(*__errno_location()),
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    ret = fcntl(fd, 2 as i32, 0o4000 as i32);
    if 0 as i32 != ret {
        slog(
            s,
            log_level::WEBDIS_ERROR,
            strerror(*__errno_location()),
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    ret = bind(fd, &mut addr as *mut sockaddr_in as *mut sockaddr, len);
    if 0 as i32 != ret {
        slog(
            s,
            log_level::WEBDIS_ERROR,
            strerror(*__errno_location()),
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    ret = listen(fd, 128 as i32);
    if 0 as i32 != ret {
        slog(
            s,
            log_level::WEBDIS_ERROR,
            strerror(*__errno_location()),
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    if getsockname(fd, &mut addr as *mut sockaddr_in as *mut sockaddr, &mut len)
        != -(1 as i32)
    {
        let mut comment: *const i8 = b"Webdis listening on port %d\0" as *const u8
            as *const i8;
        let mut port_num: i32 = ({
            let mut __v: libc::c_ushort = 0;
            let mut __x: libc::c_ushort = addr.sin_port;
            if 0 != 0 {
                __v = (__x as i32 >> 8 as i32 & 0xff as i32
                    | (__x as i32 & 0xff as i32) << 8 as i32) as libc::c_ushort;
            } else {
                let fresh3 = &mut __v;
                let fresh4;
                let fresh5 = __x;
                asm!(
                    "rorw $8, {0:x}", inlateout(reg)
                    c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) => fresh4,
                    options(pure, readonly, att_syntax)
                );
                c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
            }
            __v
        }) as i32;
        let mut buffer: *mut i8 = malloc(
            (strlen(comment))
                .wrapping_sub(2 as i32 as u64)
                .wrapping_add(strlen(b"65535\0" as *const u8 as *const i8))
                .wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        sprintf(buffer, comment, port_num);
        slog(s, log_level::WEBDIS_INFO, buffer, 0 as i32 as size_t);
        free(buffer as *mut libc::c_void);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn server_new(mut cfg_file: *const i8) -> *mut server {
    let mut i: i32 = 0;
    let mut s: *mut server = calloc(
        1 as i32 as u64,
        ::core::mem::size_of::<server>() as u64,
    ) as *mut server;
    (*s).log.fd = -(1 as i32);
    (*s).cfg = conf_read(cfg_file);
    slog_init(s);
    (*s).w = calloc(
        (*(*s).cfg).http_threads as u64,
        ::core::mem::size_of::<*mut worker>() as u64,
    ) as *mut *mut worker;
    i = 0 as i32;
    while i < (*(*s).cfg).http_threads {
        let ref mut fresh6 = *((*s).w).offset(i as isize);
        *fresh6 = worker_new(s);
        i += 1;
        i;
    }
    pthread_mutex_init(&mut (*s).auth_log_mutex, 0 as *const pthread_mutexattr_t);
    return s;
}
unsafe extern "C" fn server_can_accept(
    mut fd: i32,
    mut event: libc::c_short,
    mut ptr: *mut libc::c_void,
) {
    let mut s: *mut server = ptr as *mut server;
    let mut w: *mut worker = 0 as *mut worker;
    let mut c: *mut http_client = 0 as *mut http_client;
    let mut client_fd: i32 = 0;
    let mut addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr_sz: socklen_t = ::core::mem::size_of::<sockaddr_in>() as u64
        as socklen_t;
    let mut on: i32 = 1 as i32;
    w = *((*s).w).offset((*s).next_worker as isize);
    client_fd = accept(fd, &mut addr as *mut sockaddr_in as *mut sockaddr, &mut addr_sz);
    let mut status: i32 = ioctl(client_fd, 0x5421 as i32 as u64, &mut on as *mut i32);
    if status == -(1 as i32) {
        let mut log_msg: [i8; 200] = [0; 200];
        let mut log_msg_sz: i32 = snprintf(
            log_msg.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 200]>() as u64,
            b"ioctl failed (%d): %s\0" as *const u8 as *const i8,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        slog(s, log_level::WEBDIS_ERROR, log_msg.as_mut_ptr(), log_msg_sz as size_t);
    }
    if client_fd > 0 as i32 {
        c = http_client_new(w, client_fd, addr.sin_addr.s_addr);
        worker_add_client(w, c);
        (*s).next_worker = ((*s).next_worker + 1 as i32) % (*(*s).cfg).http_threads;
    } else {
        slog(
            s,
            log_level::WEBDIS_NOTICE,
            b"Too many connections\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
    };
}
unsafe extern "C" fn server_daemonize(mut s: *mut server, mut pidfile: *const i8) {
    let mut fd: i32 = 0;
    if fork() != 0 as i32 {
        exit(0 as i32);
    }
    setsid();
    fd = open(b"/dev/null\0" as *const u8 as *const i8, 0o2 as i32, 0 as i32);
    if fd != -(1 as i32) {
        dup2(fd, 0 as i32);
        dup2(fd, 1 as i32);
        dup2(fd, 2 as i32);
        if fd > 2 as i32 {
            close(fd);
        }
    }
    if !pidfile.is_null() {
        let mut pid_fd: i32 = open(
            pidfile,
            0o1 as i32 | 0o100 as i32 | 0o1000 as i32,
            0o600 as i32,
        );
        if pid_fd > 0 as i32 {
            let mut pid_buffer: [i8; 13] = [0; 13];
            let mut pid_sz: i32 = snprintf(
                pid_buffer.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 13]>() as u64,
                b"%d\n\0" as *const u8 as *const i8,
                getpid(),
            );
            let mut written: ssize_t = 0;
            let mut written_total: i32 = 0 as i32;
            loop {
                written = write(
                    pid_fd,
                    pid_buffer.as_mut_ptr().offset(written_total as isize)
                        as *const libc::c_void,
                    (pid_sz - written_total) as size_t,
                );
                if !(written > 0 as i32 as i64 && written_total < pid_sz) {
                    break;
                }
                written_total = (written_total as i64 + written) as i32;
            }
            close(pid_fd);
        } else {
            let err_msg: [i8; 26] = *::core::mem::transmute::<
                &[u8; 26],
                &[i8; 26],
            >(b"Failed to create PID file\0");
            slog(
                s,
                log_level::WEBDIS_ERROR,
                err_msg.as_ptr(),
                (::core::mem::size_of::<[i8; 26]>() as u64).wrapping_sub(1 as i32 as u64),
            );
            if *__errno_location() != 0 {
                let mut errno_msg: *mut i8 = strerror(*__errno_location());
                slog(s, log_level::WEBDIS_ERROR, errno_msg, strlen(errno_msg));
            }
        }
    }
}
static mut __server: *mut server = 0 as *const server as *mut server;
unsafe extern "C" fn server_handle_signal(mut id: i32) {
    let mut ret: i32 = 0;
    match id {
        1 => {
            slog_init(__server);
        }
        15 | 2 => {
            slog(
                __server,
                log_level::WEBDIS_INFO,
                b"Webdis terminating\0" as *const u8 as *const i8,
                0 as i32 as size_t,
            );
            ret = fsync((*__server).log.fd);
            exit(0 as i32);
        }
        _ => {}
    };
}
unsafe extern "C" fn server_install_signal_handlers(mut s: *mut server) {
    __server = s;
    signal(1 as i32, Some(server_handle_signal as unsafe extern "C" fn(i32) -> ()));
    signal(15 as i32, Some(server_handle_signal as unsafe extern "C" fn(i32) -> ()));
    signal(2 as i32, Some(server_handle_signal as unsafe extern "C" fn(i32) -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn server_start(mut s: *mut server) -> i32 {
    let mut i: i32 = 0;
    let mut ret: i32 = 0;
    (*s).base = event_base_new();
    if (*(*s).cfg).daemonize != 0 {
        server_daemonize(s, (*(*s).cfg).pidfile);
        if event_reinit((*s).base) != 0 as i32 {
            fprintf(
                stderr,
                b"Error: event_reinit failed after fork\0" as *const u8 as *const i8,
            );
        }
    }
    signal(
        13 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    );
    server_install_signal_handlers(s);
    i = 0 as i32;
    while i < (*(*s).cfg).http_threads {
        worker_start(*((*s).w).offset(i as isize));
        i += 1;
        i;
    }
    (*s).fd = socket_setup(s, (*(*s).cfg).http_host, (*(*s).cfg).http_port);
    if (*s).fd < 0 as i32 {
        return -(1 as i32);
    }
    let mut keep_alive: i32 = 1 as i32;
    setsockopt(
        (*s).fd,
        1 as i32,
        9 as i32,
        &mut keep_alive as *mut i32 as *mut libc::c_void,
        ::core::mem::size_of::<i32>() as u64 as socklen_t,
    );
    event_set(
        &mut (*s).ev,
        (*s).fd,
        (0x2 as i32 | 0x10 as i32) as libc::c_short,
        Some(
            server_can_accept
                as unsafe extern "C" fn(i32, libc::c_short, *mut libc::c_void) -> (),
        ),
        s as *mut libc::c_void,
    );
    event_base_set((*s).base, &mut (*s).ev);
    ret = event_add(&mut (*s).ev, 0 as *const timeval);
    if ret < 0 as i32 {
        slog(
            s,
            log_level::WEBDIS_ERROR,
            b"Error calling event_add on socket\0" as *const u8 as *const i8,
            0 as i32 as size_t,
        );
        return -(1 as i32);
    }
    slog_fsync_init(s);
    slog(
        s,
        log_level::WEBDIS_INFO,
        b"Webdis 0.1.24-dev up and running\0" as *const u8 as *const i8,
        0 as i32 as size_t,
    );
    event_base_dispatch((*s).base);
    return 0 as i32;
}