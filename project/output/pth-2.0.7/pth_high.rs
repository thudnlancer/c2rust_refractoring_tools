#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn execl(__path: *const i8, __arg: *const i8, _: ...) -> i32;
    fn select(
        __nfds: i32,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn connect(__fd: i32, __addr: *const sockaddr, __len: socklen_t) -> i32;
    fn recv(__fd: i32, __buf: *mut libc::c_void, __n: size_t, __flags: i32) -> ssize_t;
    fn sendto(
        __fd: i32,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: i32,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: i32,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: i32,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn getsockopt(
        __fd: i32,
        __level: i32,
        __optname: i32,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> i32;
    fn accept(__fd: i32, __addr: *mut sockaddr, __addr_len: *mut socklen_t) -> i32;
    fn readv(__fd: i32, __iovec: *const iovec, __count: i32) -> ssize_t;
    fn writev(__fd: i32, __iovec: *const iovec, __count: i32) -> ssize_t;
    fn pth_init() -> i32;
    fn pth_wait(_: pth_event_t) -> i32;
    fn pth_fdmode(_: i32, _: i32) -> i32;
    fn pth_time(_: i64, _: i64) -> pth_time_t;
    fn pth_timeout(_: i64, _: i64) -> pth_time_t;
    fn pth_event(_: u64, _: ...) -> pth_event_t;
    fn pth_event_concat(_: pth_event_t, _: ...) -> pth_event_t;
    fn pth_event_isolate(_: pth_event_t) -> pth_event_t;
    fn pth_event_status(_: pth_event_t) -> pth_status_t;
    fn pth_fork() -> pid_t;
    fn pth_mutex_acquire(_: *mut pth_mutex_t, _: i32, _: pth_event_t) -> i32;
    fn pth_mutex_release(_: *mut pth_mutex_t) -> i32;
    fn __pth_util_sigdelete(_: i32) -> i32;
    static mut __pth_errno_flag: i32;
    static mut __pth_errno_storage: i32;
    fn __pth_util_fd_valid(_: i32) -> i32;
    static mut __pth_initialized: i32;
    static mut __pth_time_zero: pth_time_t;
    fn __pth_time_cmp(_: *mut pth_time_t, _: *mut pth_time_t) -> i32;
    fn __pth_scheduler_kill();
    static mut __pth_current: pth_t;
    fn sigpending(__set: *mut sigset_t) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigismember(__set: *const sigset_t, __signo: i32) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
}
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __pid_t = i32;
pub type __clock_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type __socklen_t = u32;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: i32,
    pub ss_size: size_t,
}
pub type greg_t = libc::c_longlong;
pub type gregset_t = [greg_t; 23];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [libc::c_ushort; 4],
    pub exponent: libc::c_ushort,
    pub __glibc_reserved1: [libc::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    pub __glibc_reserved1: [__uint32_t; 24],
}
pub type fpregset_t = *mut _libc_fpstate;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [libc::c_ulonglong; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: u64,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_10 {
    MSG_CMSG_CLOEXEC = 1073741824,
    MSG_FASTOPEN = 536870912,
    MSG_ZEROCOPY = 67108864,
    MSG_BATCH = 262144,
    MSG_WAITFORONE = 65536,
    MSG_MORE = 32768,
    MSG_NOSIGNAL = 16384,
    MSG_ERRQUEUE = 8192,
    MSG_RST = 4096,
    MSG_CONFIRM = 2048,
    MSG_SYN = 1024,
    MSG_FIN = 512,
    MSG_WAITALL = 256,
    MSG_EOR = 128,
    MSG_DONTWAIT = 64,
    MSG_TRUNC = 32,
    MSG_PROXY = 16,
    MSG_CTRUNC = 8,
    MSG_DONTROUTE = 4,
    MSG_PEEK = 2,
    MSG_OOB = 1,
}
impl C2RustUnnamed_10 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_10::MSG_CMSG_CLOEXEC => 1073741824,
            C2RustUnnamed_10::MSG_FASTOPEN => 536870912,
            C2RustUnnamed_10::MSG_ZEROCOPY => 67108864,
            C2RustUnnamed_10::MSG_BATCH => 262144,
            C2RustUnnamed_10::MSG_WAITFORONE => 65536,
            C2RustUnnamed_10::MSG_MORE => 32768,
            C2RustUnnamed_10::MSG_NOSIGNAL => 16384,
            C2RustUnnamed_10::MSG_ERRQUEUE => 8192,
            C2RustUnnamed_10::MSG_RST => 4096,
            C2RustUnnamed_10::MSG_CONFIRM => 2048,
            C2RustUnnamed_10::MSG_SYN => 1024,
            C2RustUnnamed_10::MSG_FIN => 512,
            C2RustUnnamed_10::MSG_WAITALL => 256,
            C2RustUnnamed_10::MSG_EOR => 128,
            C2RustUnnamed_10::MSG_DONTWAIT => 64,
            C2RustUnnamed_10::MSG_TRUNC => 32,
            C2RustUnnamed_10::MSG_PROXY => 16,
            C2RustUnnamed_10::MSG_CTRUNC => 8,
            C2RustUnnamed_10::MSG_DONTROUTE => 4,
            C2RustUnnamed_10::MSG_PEEK => 2,
            C2RustUnnamed_10::MSG_OOB => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_10 {
        match value {
            1073741824 => C2RustUnnamed_10::MSG_CMSG_CLOEXEC,
            536870912 => C2RustUnnamed_10::MSG_FASTOPEN,
            67108864 => C2RustUnnamed_10::MSG_ZEROCOPY,
            262144 => C2RustUnnamed_10::MSG_BATCH,
            65536 => C2RustUnnamed_10::MSG_WAITFORONE,
            32768 => C2RustUnnamed_10::MSG_MORE,
            16384 => C2RustUnnamed_10::MSG_NOSIGNAL,
            8192 => C2RustUnnamed_10::MSG_ERRQUEUE,
            4096 => C2RustUnnamed_10::MSG_RST,
            2048 => C2RustUnnamed_10::MSG_CONFIRM,
            1024 => C2RustUnnamed_10::MSG_SYN,
            512 => C2RustUnnamed_10::MSG_FIN,
            256 => C2RustUnnamed_10::MSG_WAITALL,
            128 => C2RustUnnamed_10::MSG_EOR,
            64 => C2RustUnnamed_10::MSG_DONTWAIT,
            32 => C2RustUnnamed_10::MSG_TRUNC,
            16 => C2RustUnnamed_10::MSG_PROXY,
            8 => C2RustUnnamed_10::MSG_CTRUNC,
            4 => C2RustUnnamed_10::MSG_DONTROUTE,
            2 => C2RustUnnamed_10::MSG_PEEK,
            1 => C2RustUnnamed_10::MSG_OOB,
            _ => panic!("Invalid value for C2RustUnnamed_10: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_10 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_10 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_10 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_10 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_10 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn add(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn sub(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn mul(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn div(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_10 {
    type Output = C2RustUnnamed_10;
    fn rem(self, rhs: u32) -> C2RustUnnamed_10 {
        C2RustUnnamed_10::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type pth_time_t = timeval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_st {
    pub q_next: pth_t,
    pub q_prev: pth_t,
    pub q_prio: i32,
    pub prio: i32,
    pub name: [i8; 40],
    pub dispatches: i32,
    pub state: pth_state_t,
    pub spawned: pth_time_t,
    pub lastran: pth_time_t,
    pub running: pth_time_t,
    pub events: pth_event_t,
    pub sigpending: sigset_t,
    pub sigpendcnt: i32,
    pub mctx: pth_mctx_t,
    pub stack: *mut i8,
    pub stacksize: u32,
    pub stackguard: *mut i64,
    pub stackloan: i32,
    pub start_func: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub start_arg: *mut libc::c_void,
    pub joinable: i32,
    pub join_arg: *mut libc::c_void,
    pub data_value: *mut *const libc::c_void,
    pub data_count: i32,
    pub cancelreq: i32,
    pub cancelstate: u32,
    pub cleanups: *mut pth_cleanup_t,
    pub mutexring: pth_ring_t,
}
pub type pth_ring_t = pth_ring_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ring_st {
    pub r_hook: *mut pth_ringnode_t,
    pub r_nodes: u32,
}
pub type pth_ringnode_t = pth_ringnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ringnode_st {
    pub rn_next: *mut pth_ringnode_t,
    pub rn_prev: *mut pth_ringnode_t,
}
pub type pth_cleanup_t = pth_cleanup_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_cleanup_st {
    pub next: *mut pth_cleanup_t,
    pub func: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type pth_mctx_t = pth_mctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mctx_st {
    pub uc: ucontext_t,
    pub restored: i32,
    pub sigs: sigset_t,
    pub error: i32,
}
pub type pth_event_t = *mut pth_event_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_event_st {
    pub ev_next: *mut pth_event_st,
    pub ev_prev: *mut pth_event_st,
    pub ev_status: pth_status_t,
    pub ev_type: i32,
    pub ev_goal: i32,
    pub ev_args: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub FD: C2RustUnnamed_20,
    pub SELECT: C2RustUnnamed_19,
    pub SIGS: C2RustUnnamed_18,
    pub TIME: C2RustUnnamed_17,
    pub MSG: C2RustUnnamed_16,
    pub MUTEX: C2RustUnnamed_15,
    pub COND: C2RustUnnamed_14,
    pub TID: C2RustUnnamed_13,
    pub FUNC: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub func: pth_event_func_t,
    pub arg: *mut libc::c_void,
    pub tv: pth_time_t,
}
pub type pth_event_func_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub tid: pth_t,
}
pub type pth_t = *mut pth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub cond: *mut pth_cond_t,
}
pub type pth_cond_t = pth_cond_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_cond_st {
    pub cn_state: u64,
    pub cn_waiters: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub mutex: *mut pth_mutex_t,
}
pub type pth_mutex_t = pth_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mutex_st {
    pub mx_node: pth_ringnode_t,
    pub mx_state: i32,
    pub mx_owner: pth_t,
    pub mx_count: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub mp: pth_msgport_t,
}
pub type pth_msgport_t = *mut pth_msgport_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_msgport_st {
    pub mp_node: pth_ringnode_t,
    pub mp_name: *const i8,
    pub mp_tid: pth_t,
    pub mp_queue: pth_ring_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub tv: pth_time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub sigs: *mut sigset_t,
    pub sig: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub n: *mut i32,
    pub nfd: i32,
    pub rfds: *mut fd_set,
    pub wfds: *mut fd_set,
    pub efds: *mut fd_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub fd: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum pth_status_t {
    PTH_STATUS_PENDING,
    PTH_STATUS_OCCURRED,
    PTH_STATUS_FAILED,
}
impl pth_status_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            pth_status_t::PTH_STATUS_PENDING => 0,
            pth_status_t::PTH_STATUS_OCCURRED => 1,
            pth_status_t::PTH_STATUS_FAILED => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> pth_status_t {
        match value {
            0 => pth_status_t::PTH_STATUS_PENDING,
            1 => pth_status_t::PTH_STATUS_OCCURRED,
            2 => pth_status_t::PTH_STATUS_FAILED,
            _ => panic!("Invalid value for pth_status_t: {}", value),
        }
    }
}
impl AddAssign<u32> for pth_status_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for pth_status_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for pth_status_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for pth_status_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for pth_status_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = pth_status_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for pth_status_t {
    type Output = pth_status_t;
    fn add(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for pth_status_t {
    type Output = pth_status_t;
    fn sub(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for pth_status_t {
    type Output = pth_status_t;
    fn mul(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for pth_status_t {
    type Output = pth_status_t;
    fn div(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for pth_status_t {
    type Output = pth_status_t;
    fn rem(self, rhs: u32) -> pth_status_t {
        pth_status_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type pth_state_t = pth_state_en;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum pth_state_en {
    PTH_STATE_SCHEDULER = 0,
    PTH_STATE_NEW,
    PTH_STATE_READY,
    PTH_STATE_WAITING,
    PTH_STATE_DEAD,
}
impl pth_state_en {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            pth_state_en::PTH_STATE_SCHEDULER => 0,
            pth_state_en::PTH_STATE_NEW => 1,
            pth_state_en::PTH_STATE_READY => 2,
            pth_state_en::PTH_STATE_WAITING => 3,
            pth_state_en::PTH_STATE_DEAD => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> pth_state_en {
        match value {
            0 => pth_state_en::PTH_STATE_SCHEDULER,
            1 => pth_state_en::PTH_STATE_NEW,
            2 => pth_state_en::PTH_STATE_READY,
            3 => pth_state_en::PTH_STATE_WAITING,
            4 => pth_state_en::PTH_STATE_DEAD,
            _ => panic!("Invalid value for pth_state_en: {}", value),
        }
    }
}
impl AddAssign<u32> for pth_state_en {
    fn add_assign(&mut self, rhs: u32) {
        *self = pth_state_en::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for pth_state_en {
    fn sub_assign(&mut self, rhs: u32) {
        *self = pth_state_en::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for pth_state_en {
    fn mul_assign(&mut self, rhs: u32) {
        *self = pth_state_en::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for pth_state_en {
    fn div_assign(&mut self, rhs: u32) {
        *self = pth_state_en::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for pth_state_en {
    fn rem_assign(&mut self, rhs: u32) {
        *self = pth_state_en::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for pth_state_en {
    type Output = pth_state_en;
    fn add(self, rhs: u32) -> pth_state_en {
        pth_state_en::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for pth_state_en {
    type Output = pth_state_en;
    fn sub(self, rhs: u32) -> pth_state_en {
        pth_state_en::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for pth_state_en {
    type Output = pth_state_en;
    fn mul(self, rhs: u32) -> pth_state_en {
        pth_state_en::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for pth_state_en {
    type Output = pth_state_en;
    fn div(self, rhs: u32) -> pth_state_en {
        pth_state_en::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for pth_state_en {
    type Output = pth_state_en;
    fn rem(self, rhs: u32) -> pth_state_en {
        pth_state_en::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type pth_key_t = i32;
pub type C2RustUnnamed_21 = i32;
pub const PTH_FDMODE_NONBLOCK: C2RustUnnamed_21 = 2;
pub const PTH_FDMODE_BLOCK: C2RustUnnamed_21 = 1;
pub const PTH_FDMODE_POLL: C2RustUnnamed_21 = 0;
pub const PTH_FDMODE_ERROR: C2RustUnnamed_21 = -1;
pub type nfds_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: i32,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn pth_nanosleep(
    mut rqtp: *const timespec,
    mut rmtp: *mut timespec,
) -> i32 {
    let mut until: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut offset: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut now: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    if rqtp.is_null() {
        *__errno_location() = 14 as i32;
        return -(1 as i32);
    }
    if (*rqtp).tv_nsec < 0 as i32 as i64
        || (*rqtp).tv_nsec > (1000 as i32 * 1000000 as i32) as i64
    {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    if (*rqtp).tv_sec == 0 as i32 as i64 && (*rqtp).tv_nsec == 0 as i32 as i64 {
        return 0 as i32;
    }
    offset = pth_time((*rqtp).tv_sec, (*rqtp).tv_nsec / 1000 as i32 as i64);
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut until, 0 as *mut timezone);
    } else {
        until.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        until.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    until.tv_sec += offset.tv_sec;
    until.tv_usec += offset.tv_usec;
    if until.tv_usec > 1000000 as i32 as i64 {
        until.tv_sec += 1 as i32 as i64;
        until.tv_usec -= 1000000 as i32 as i64;
    }
    ev = pth_event(
        ((1 as i32) << 4 as i32 | (1 as i32) << 22 as i32) as u64,
        &mut ev_key as *mut pth_key_t,
        until,
    );
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return -(1 as i32);
    }
    pth_wait(ev);
    if !rmtp.is_null() {
        if (0 as *mut pth_time_t).is_null() {
            gettimeofday(&mut now, 0 as *mut timezone);
        } else {
            now.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
            now.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
        }
        until.tv_sec -= now.tv_sec;
        until.tv_usec -= now.tv_usec;
        if until.tv_usec < 0 as i32 as i64 {
            until.tv_sec -= 1 as i32 as i64;
            until.tv_usec += 1000000 as i32 as i64;
        }
        (*rmtp).tv_sec = until.tv_sec;
        (*rmtp).tv_nsec = until.tv_usec * 1000 as i32 as i64;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_usleep(mut usec: u32) -> i32 {
    let mut until: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut offset: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    if usec == 0 as i32 as u32 {
        return 0 as i32;
    }
    offset = pth_time(
        usec.wrapping_div(1000000 as i32 as u32) as i64,
        usec.wrapping_rem(1000000 as i32 as u32) as i64,
    );
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut until, 0 as *mut timezone);
    } else {
        until.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        until.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    until.tv_sec += offset.tv_sec;
    until.tv_usec += offset.tv_usec;
    if until.tv_usec > 1000000 as i32 as i64 {
        until.tv_sec += 1 as i32 as i64;
        until.tv_usec -= 1000000 as i32 as i64;
    }
    ev = pth_event(
        ((1 as i32) << 4 as i32 | (1 as i32) << 22 as i32) as u64,
        &mut ev_key as *mut pth_key_t,
        until,
    );
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return -(1 as i32);
    }
    pth_wait(ev);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_sleep(mut sec: u32) -> u32 {
    let mut until: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut offset: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    if sec == 0 as i32 as u32 {
        return 0 as i32 as u32;
    }
    offset = pth_time(sec as i64, 0 as i32 as i64);
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut until, 0 as *mut timezone);
    } else {
        until.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        until.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    until.tv_sec += offset.tv_sec;
    until.tv_usec += offset.tv_usec;
    if until.tv_usec > 1000000 as i32 as i64 {
        until.tv_sec += 1 as i32 as i64;
        until.tv_usec -= 1000000 as i32 as i64;
    }
    ev = pth_event(
        ((1 as i32) << 4 as i32 | (1 as i32) << 22 as i32) as u64,
        &mut ev_key as *mut pth_key_t,
        until,
    );
    if ev.is_null() {
        return sec;
    }
    pth_wait(ev);
    return 0 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_sigmask(
    mut how: i32,
    mut set: *const sigset_t,
    mut oset: *mut sigset_t,
) -> i32 {
    let mut rv: i32 = 0;
    if !set.is_null() {
        sigprocmask(how, &mut (*__pth_current).mctx.sigs, 0 as *mut sigset_t);
    }
    rv = sigprocmask(how, set, oset);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn pth_sigwait(
    mut set: *const sigset_t,
    mut sigp: *mut i32,
) -> i32 {
    return pth_sigwait_ev(set, sigp, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_sigwait_ev(
    mut set: *const sigset_t,
    mut sigp: *mut i32,
    mut ev_extra: pth_event_t,
) -> i32 {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut pending: sigset_t = sigset_t { __val: [0; 16] };
    let mut sig: i32 = 0;
    if set.is_null() || sigp.is_null() {
        *__errno_location() = 22 as i32;
        return 22 as i32;
    }
    if sigpending(&mut pending) < 0 as i32 {
        sigemptyset(&mut pending);
    }
    sig = 1 as i32;
    while sig < 65 as i32 {
        if sigismember(set, sig) != 0 && sigismember(&mut pending, sig) != 0 {
            __pth_util_sigdelete(sig);
            *sigp = sig;
            return 0 as i32;
        }
        sig += 1;
        sig;
    }
    ev = pth_event(
        ((1 as i32) << 3 as i32 | (1 as i32) << 22 as i32) as u64,
        &mut ev_key as *mut pth_key_t,
        set,
        sigp,
    );
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return *__errno_location();
    }
    if !ev_extra.is_null() {
        pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
    }
    pth_wait(ev);
    if !ev_extra.is_null() {
        pth_event_isolate(ev);
        if pth_event_status(ev) as u32 != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
        {
            *__errno_location() = 4 as i32;
            return 4 as i32;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_waitpid(
    mut wpid: pid_t,
    mut status: *mut i32,
    mut options: i32,
) -> pid_t {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut pid: pid_t = 0;
    loop {
        loop {
            pid = waitpid(wpid, status, options | 1 as i32);
            if !(pid < 0 as i32 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if pid == -(1 as i32) || pid > 0 as i32
            || pid == 0 as i32 && options & 1 as i32 != 0
        {
            break;
        }
        ev = pth_event(
            ((1 as i32) << 4 as i32 | (1 as i32) << 22 as i32) as u64,
            &mut ev_key as *mut pth_key_t,
            pth_timeout(0 as i32 as i64, 250000 as i32 as i64),
        );
        pth_wait(ev);
    }
    return pid;
}
#[no_mangle]
pub unsafe extern "C" fn pth_system(mut cmd: *const i8) -> i32 {
    let mut sa_ign: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sa_int: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sa_quit: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut ss_block: sigset_t = sigset_t { __val: [0; 16] };
    let mut ss_old: sigset_t = sigset_t { __val: [0; 16] };
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut pid: pid_t = 0;
    let mut pstat: i32 = 0;
    if cmd.is_null() {
        if stat(b"/bin/sh\0" as *const u8 as *const i8, &mut sb) == -(1 as i32) {
            return 0 as i32;
        }
        return 1 as i32;
    }
    sa_ign.__sigaction_handler.sa_handler = ::core::mem::transmute::<
        libc::intptr_t,
        __sighandler_t,
    >(1 as i32 as libc::intptr_t);
    sigemptyset(&mut sa_ign.sa_mask);
    sa_ign.sa_flags = 0 as i32;
    sigaction(2 as i32, &mut sa_ign, &mut sa_int);
    sigaction(3 as i32, &mut sa_ign, &mut sa_quit);
    sigemptyset(&mut ss_block);
    sigaddset(&mut ss_block, 17 as i32);
    sigprocmask(0 as i32, &mut ss_block, &mut ss_old);
    pstat = -(1 as i32);
    pid = pth_fork();
    match pid {
        -1 => {}
        0 => {
            sigaction(2 as i32, &mut sa_int, 0 as *mut sigaction);
            sigaction(3 as i32, &mut sa_quit, 0 as *mut sigaction);
            sigprocmask(2 as i32, &mut ss_old, 0 as *mut sigset_t);
            __pth_scheduler_kill();
            execl(
                b"/bin/sh\0" as *const u8 as *const i8,
                b"sh\0" as *const u8 as *const i8,
                b"-c\0" as *const u8 as *const i8,
                cmd,
                0 as *mut libc::c_void as *mut i8,
            );
            exit(127 as i32);
        }
        _ => {
            pid = pth_waitpid(pid, &mut pstat, 0 as i32);
        }
    }
    sigaction(2 as i32, &mut sa_int, 0 as *mut sigaction);
    sigaction(3 as i32, &mut sa_quit, 0 as *mut sigaction);
    sigprocmask(2 as i32, &mut ss_old, 0 as *mut sigset_t);
    return if pid == -(1 as i32) { -(1 as i32) } else { pstat };
}
#[no_mangle]
pub unsafe extern "C" fn pth_select(
    mut nfds: i32,
    mut rfds: *mut fd_set,
    mut wfds: *mut fd_set,
    mut efds: *mut fd_set,
    mut timeout: *mut timeval,
) -> i32 {
    return pth_select_ev(nfds, rfds, wfds, efds, timeout, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_select_ev(
    mut nfd: i32,
    mut rfds: *mut fd_set,
    mut wfds: *mut fd_set,
    mut efds: *mut fd_set,
    mut timeout: *mut timeval,
    mut ev_extra: pth_event_t,
) -> i32 {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    let mut ev_select: pth_event_t = 0 as *mut pth_event_st;
    let mut ev_timeout: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key_select: pth_key_t = -(1 as i32);
    static mut ev_key_timeout: pth_key_t = -(1 as i32);
    let mut rspare: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut wspare: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut espare: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut rtmp: *mut fd_set = 0 as *mut fd_set;
    let mut wtmp: *mut fd_set = 0 as *mut fd_set;
    let mut etmp: *mut fd_set = 0 as *mut fd_set;
    let mut selected: i32 = 0;
    let mut rc: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nfd < 0 as i32 || nfd > 1024 as i32 {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    if !timeout.is_null() {
        if (*timeout).tv_sec < 0 as i32 as i64 || (*timeout).tv_usec < 0 as i32 as i64
            || (*timeout).tv_usec >= 1000000 as i32 as i64
        {
            *__errno_location() = 22 as i32;
            return -(1 as i32);
        }
        if (*timeout).tv_sec > (31 as i32 * 24 as i32 * 60 as i32 * 60 as i32) as i64 {
            (*timeout).tv_sec = (31 as i32 * 24 as i32 * 60 as i32 * 60 as i32)
                as __time_t;
        }
    }
    if nfd == 0 as i32 && rfds.is_null() && wfds.is_null() && efds.is_null()
        && !timeout.is_null()
    {
        if (*timeout).tv_sec == 0 as i32 as i64
            && (*timeout).tv_usec <= 10000 as i32 as i64
        {
            while select(
                0 as i32,
                0 as *mut fd_set,
                0 as *mut fd_set,
                0 as *mut fd_set,
                timeout,
            ) < 0 as i32 && *__errno_location() == 4 as i32
            {}
        } else {
            ev = pth_event(
                ((1 as i32) << 4 as i32 | (1 as i32) << 22 as i32) as u64,
                &mut ev_key_timeout as *mut pth_key_t,
                pth_timeout((*timeout).tv_sec, (*timeout).tv_usec),
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as u32
                    != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
                {
                    *__errno_location() = 4 as i32;
                    return -(1 as i32);
                }
            }
        }
        if !rfds.is_null() {
            let mut __d0: i32 = 0;
            let mut __d1: i32 = 0;
            let fresh0 = &mut __d0;
            let fresh1;
            let fresh2 = (::core::mem::size_of::<fd_set>() as u64)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
            let fresh3 = &mut __d1;
            let fresh4;
            let fresh5 = &mut *((*rfds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh0, fresh2) => fresh1,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3, fresh5) =>
                fresh4, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
            c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
        }
        if !wfds.is_null() {
            let mut __d0_0: i32 = 0;
            let mut __d1_0: i32 = 0;
            let fresh6 = &mut __d0_0;
            let fresh7;
            let fresh8 = (::core::mem::size_of::<fd_set>() as u64)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
            let fresh9 = &mut __d1_0;
            let fresh10;
            let fresh11 = &mut *((*wfds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh6, fresh8) => fresh7,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9, fresh11) =>
                fresh10, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
            c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
        }
        if !efds.is_null() {
            let mut __d0_1: i32 = 0;
            let mut __d1_1: i32 = 0;
            let fresh12 = &mut __d0_1;
            let fresh13;
            let fresh14 = (::core::mem::size_of::<fd_set>() as u64)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
            let fresh15 = &mut __d1_1;
            let fresh16;
            let fresh17 = &mut *((*efds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh12, fresh14) => fresh13,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh15, fresh17) =>
                fresh16, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
            c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
        }
        return 0 as i32;
    }
    delay.tv_sec = 0 as i32 as __time_t;
    delay.tv_usec = 0 as i32 as __suseconds_t;
    rtmp = 0 as *mut fd_set;
    if !rfds.is_null() {
        memcpy(
            &mut rspare as *mut fd_set as *mut libc::c_void,
            rfds as *const libc::c_void,
            ::core::mem::size_of::<fd_set>() as u64,
        );
        rtmp = &mut rspare;
    }
    wtmp = 0 as *mut fd_set;
    if !wfds.is_null() {
        memcpy(
            &mut wspare as *mut fd_set as *mut libc::c_void,
            wfds as *const libc::c_void,
            ::core::mem::size_of::<fd_set>() as u64,
        );
        wtmp = &mut wspare;
    }
    etmp = 0 as *mut fd_set;
    if !efds.is_null() {
        memcpy(
            &mut espare as *mut fd_set as *mut libc::c_void,
            efds as *const libc::c_void,
            ::core::mem::size_of::<fd_set>() as u64,
        );
        etmp = &mut espare;
    }
    loop {
        rc = select(nfd, rtmp, wtmp, etmp, &mut delay);
        if !(rc < 0 as i32 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    if rc < 0 as i32 {
        *__errno_location() = *__errno_location();
        return -(1 as i32);
    } else if rc > 0 as i32
        || rc == 0 as i32 && !timeout.is_null()
            && __pth_time_cmp(timeout, &mut __pth_time_zero) == 0 as i32
    {
        if !rfds.is_null() {
            memcpy(
                rfds as *mut libc::c_void,
                &mut rspare as *mut fd_set as *const libc::c_void,
                ::core::mem::size_of::<fd_set>() as u64,
            );
        }
        if !wfds.is_null() {
            memcpy(
                wfds as *mut libc::c_void,
                &mut wspare as *mut fd_set as *const libc::c_void,
                ::core::mem::size_of::<fd_set>() as u64,
            );
        }
        if !efds.is_null() {
            memcpy(
                efds as *mut libc::c_void,
                &mut espare as *mut fd_set as *const libc::c_void,
                ::core::mem::size_of::<fd_set>() as u64,
            );
        }
        return rc;
    }
    rc = -(1 as i32);
    ev_select = pth_event(
        ((1 as i32) << 2 as i32 | (1 as i32) << 22 as i32) as u64,
        &mut ev_key_select as *mut pth_key_t,
        &mut rc as *mut i32,
        nfd,
        rfds,
        wfds,
        efds,
    );
    ev = ev_select;
    ev_timeout = 0 as pth_event_t;
    if !timeout.is_null() {
        ev_timeout = pth_event(
            ((1 as i32) << 4 as i32 | (1 as i32) << 22 as i32) as u64,
            &mut ev_key_timeout as *mut pth_key_t,
            pth_timeout((*timeout).tv_sec, (*timeout).tv_usec),
        );
        pth_event_concat(ev, ev_timeout, 0 as *mut libc::c_void);
    }
    if !ev_extra.is_null() {
        pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
    }
    pth_wait(ev);
    if !ev_extra.is_null() {
        pth_event_isolate(ev_extra);
    }
    if !timeout.is_null() {
        pth_event_isolate(ev_timeout);
    }
    if pth_event_status(ev_select) as u32
        == pth_status_t::PTH_STATUS_FAILED as i32 as u32
    {
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    selected = 0 as i32;
    if pth_event_status(ev_select) as u32
        == pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
    {
        selected = (0 as i32 == 0) as i32;
    }
    if !timeout.is_null()
        && pth_event_status(ev_timeout) as u32
            == pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
    {
        selected = (0 as i32 == 0) as i32;
        if !rfds.is_null() {
            let mut __d0_2: i32 = 0;
            let mut __d1_2: i32 = 0;
            let fresh18 = &mut __d0_2;
            let fresh19;
            let fresh20 = (::core::mem::size_of::<fd_set>() as u64)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
            let fresh21 = &mut __d1_2;
            let fresh22;
            let fresh23 = &mut *((*rfds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh18, fresh20) => fresh19,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh21, fresh23) =>
                fresh22, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh18, fresh20, fresh19);
            c2rust_asm_casts::AsmCast::cast_out(fresh21, fresh23, fresh22);
        }
        if !wfds.is_null() {
            let mut __d0_3: i32 = 0;
            let mut __d1_3: i32 = 0;
            let fresh24 = &mut __d0_3;
            let fresh25;
            let fresh26 = (::core::mem::size_of::<fd_set>() as u64)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
            let fresh27 = &mut __d1_3;
            let fresh28;
            let fresh29 = &mut *((*wfds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh24, fresh26) => fresh25,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh27, fresh29) =>
                fresh28, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh24, fresh26, fresh25);
            c2rust_asm_casts::AsmCast::cast_out(fresh27, fresh29, fresh28);
        }
        if !efds.is_null() {
            let mut __d0_4: i32 = 0;
            let mut __d1_4: i32 = 0;
            let fresh30 = &mut __d0_4;
            let fresh31;
            let fresh32 = (::core::mem::size_of::<fd_set>() as u64)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
            let fresh33 = &mut __d1_4;
            let fresh34;
            let fresh35 = &mut *((*efds).__fds_bits)
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut __fd_mask;
            asm!(
                "cld; rep; stosq", inlateout("cx")
                c2rust_asm_casts::AsmCast::cast_in(fresh30, fresh32) => fresh31,
                inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh33, fresh35) =>
                fresh34, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
                att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh30, fresh32, fresh31);
            c2rust_asm_casts::AsmCast::cast_out(fresh33, fresh35, fresh34);
        }
        rc = 0 as i32;
    }
    if !ev_extra.is_null() && selected == 0 {
        *__errno_location() = 4 as i32;
        return -(1 as i32);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn pth_pselect(
    mut nfds: i32,
    mut rfds: *mut fd_set,
    mut wfds: *mut fd_set,
    mut efds: *mut fd_set,
    mut ts: *const timespec,
    mut mask: *const sigset_t,
) -> i32 {
    let mut omask: sigset_t = sigset_t { __val: [0; 16] };
    let mut tv: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tvp: *mut timeval = 0 as *mut timeval;
    let mut rv: i32 = 0;
    if !ts.is_null() {
        tv.tv_sec = (*ts).tv_sec;
        tv.tv_usec = (*ts).tv_nsec / 1000 as i32 as i64;
        tvp = &mut tv;
    } else {
        tvp = 0 as *mut timeval;
    }
    if !mask.is_null() {
        if sigprocmask(2 as i32, mask, &mut omask) < 0 as i32 {
            *__errno_location() = *__errno_location();
            return -(1 as i32);
        }
    }
    rv = pth_select(nfds, rfds, wfds, efds, tvp);
    if !mask.is_null() {
        __pth_errno_storage = *__errno_location();
        __pth_errno_flag = (0 as i32 == 0) as i32;
        while __pth_errno_flag != 0 {
            sigprocmask(2 as i32, &mut omask, 0 as *mut sigset_t);
            *__errno_location() = __pth_errno_storage;
            __pth_errno_flag = 0 as i32;
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn pth_poll(
    mut pfd: *mut pollfd,
    mut nfd: nfds_t,
    mut timeout: i32,
) -> i32 {
    return pth_poll_ev(pfd, nfd, timeout, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_poll_ev(
    mut pfd: *mut pollfd,
    mut nfd: nfds_t,
    mut timeout: i32,
    mut ev_extra: pth_event_t,
) -> i32 {
    let mut rfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut wfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut efds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut xfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ptv: *mut timeval = 0 as *mut timeval;
    let mut maxfd: i32 = 0;
    let mut rc: i32 = 0;
    let mut n: i32 = 0;
    let mut i: u32 = 0;
    let mut data: [i8; 64] = [0; 64];
    if __pth_initialized == 0 {
        pth_init();
    }
    if pfd.is_null() {
        *__errno_location() = 14 as i32;
        return -(1 as i32);
    }
    if nfd < 0 as i32 as u64 || nfd > 1024 as i32 as u64 {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    ptv = &mut tv;
    if timeout == 0 as i32 {
        (*ptv).tv_sec = 0 as i32 as __time_t;
        (*ptv).tv_usec = 0 as i32 as __suseconds_t;
    } else if timeout == -(1 as i32) {
        ptv = 0 as *mut timeval;
    } else if timeout > 0 as i32 {
        (*ptv).tv_sec = (timeout / 1000 as i32) as __time_t;
        (*ptv).tv_usec = (timeout % 1000 as i32 * 1000 as i32) as __suseconds_t;
    } else {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    maxfd = -(1 as i32);
    let mut __d0: i32 = 0;
    let mut __d1: i32 = 0;
    let fresh36 = &mut __d0;
    let fresh37;
    let fresh38 = (::core::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
    let fresh39 = &mut __d1;
    let fresh40;
    let fresh41 = &mut *(rfds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh36,
        fresh38) => fresh37, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh39,
        fresh41) => fresh40, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh36, fresh38, fresh37);
    c2rust_asm_casts::AsmCast::cast_out(fresh39, fresh41, fresh40);
    let mut __d0_0: i32 = 0;
    let mut __d1_0: i32 = 0;
    let fresh42 = &mut __d0_0;
    let fresh43;
    let fresh44 = (::core::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
    let fresh45 = &mut __d1_0;
    let fresh46;
    let fresh47 = &mut *(wfds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh42,
        fresh44) => fresh43, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh45,
        fresh47) => fresh46, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh42, fresh44, fresh43);
    c2rust_asm_casts::AsmCast::cast_out(fresh45, fresh47, fresh46);
    let mut __d0_1: i32 = 0;
    let mut __d1_1: i32 = 0;
    let fresh48 = &mut __d0_1;
    let fresh49;
    let fresh50 = (::core::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
    let fresh51 = &mut __d1_1;
    let fresh52;
    let fresh53 = &mut *(efds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh48,
        fresh50) => fresh49, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh51,
        fresh53) => fresh52, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh48, fresh50, fresh49);
    c2rust_asm_casts::AsmCast::cast_out(fresh51, fresh53, fresh52);
    let mut __d0_2: i32 = 0;
    let mut __d1_2: i32 = 0;
    let fresh54 = &mut __d0_2;
    let fresh55;
    let fresh56 = (::core::mem::size_of::<fd_set>() as u64)
        .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
    let fresh57 = &mut __d1_2;
    let fresh58;
    let fresh59 = &mut *(xfds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh54,
        fresh56) => fresh55, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh57,
        fresh59) => fresh58, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh54, fresh56, fresh55);
    c2rust_asm_casts::AsmCast::cast_out(fresh57, fresh59, fresh58);
    i = 0 as i32 as u32;
    while (i as u64) < nfd {
        if __pth_util_fd_valid((*pfd.offset(i as isize)).fd) == 0 {
            xfds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                |= ((1 as u64)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask;
        } else {
            if (*pfd.offset(i as isize)).events as i32 & (0x1 as i32 | 0x40 as i32) != 0
            {
                rfds
                    .__fds_bits[((*pfd.offset(i as isize)).fd
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    |= ((1 as u64)
                        << (*pfd.offset(i as isize)).fd
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask;
            }
            if (*pfd.offset(i as isize)).events as i32
                & (0x4 as i32 | 0x100 as i32 | 0x200 as i32) != 0
            {
                wfds
                    .__fds_bits[((*pfd.offset(i as isize)).fd
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    |= ((1 as u64)
                        << (*pfd.offset(i as isize)).fd
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask;
            }
            if (*pfd.offset(i as isize)).events as i32 & (0x2 as i32 | 0x80 as i32) != 0
            {
                efds
                    .__fds_bits[((*pfd.offset(i as isize)).fd
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    |= ((1 as u64)
                        << (*pfd.offset(i as isize)).fd
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask;
            }
            if (*pfd.offset(i as isize)).fd >= maxfd
                && (*pfd.offset(i as isize)).events as i32
                    & (0x1 as i32 | 0x4 as i32 | 0x2 as i32 | 0x40 as i32 | 0x80 as i32
                        | 0x100 as i32 | 0x200 as i32) != 0
            {
                maxfd = (*pfd.offset(i as isize)).fd;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    rc = -(1 as i32);
    if maxfd != -(1 as i32) {
        rc = pth_select_ev(
            maxfd + 1 as i32,
            &mut rfds,
            &mut wfds,
            &mut efds,
            ptv,
            ev_extra,
        );
        if rc < 0 as i32 {
            *__errno_location() = *__errno_location();
            return -(1 as i32);
        } else if rc == 0 as i32 {
            return 0 as i32
        }
    }
    n = 0 as i32;
    i = 0 as i32 as u32;
    while (i as u64) < nfd {
        (*pfd.offset(i as isize)).revents = 0 as i32 as libc::c_short;
        if xfds
            .__fds_bits[((*pfd.offset(i as isize)).fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            & ((1 as u64)
                << (*pfd.offset(i as isize)).fd
                    % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask != 0 as i32 as i64
        {
            if (*pfd.offset(i as isize)).fd >= 0 as i32 {
                let ref mut fresh60 = (*pfd.offset(i as isize)).revents;
                *fresh60 = (*fresh60 as i32 | 0x20 as i32) as libc::c_short;
                n += 1;
                n;
            }
        } else if !(maxfd == -(1 as i32)) {
            if rfds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
            {
                if (*pfd.offset(i as isize)).events as i32 & 0x1 as i32 != 0 {
                    let ref mut fresh61 = (*pfd.offset(i as isize)).revents;
                    *fresh61 = (*fresh61 as i32 | 0x1 as i32) as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as i32 & 0x40 as i32 != 0 {
                    let ref mut fresh62 = (*pfd.offset(i as isize)).revents;
                    *fresh62 = (*fresh62 as i32 | 0x40 as i32) as libc::c_short;
                }
                n += 1;
                n;
                if recv(
                    (*pfd.offset(i as isize)).fd,
                    data.as_mut_ptr() as *mut libc::c_void,
                    ::core::mem::size_of::<[i8; 64]>() as u64,
                    C2RustUnnamed_10::MSG_PEEK as i32,
                ) == -(1 as i32) as i64
                    && (*__errno_location() == 108 as i32
                        || *__errno_location() == 104 as i32
                        || *__errno_location() == 103 as i32
                        || *__errno_location() == 102 as i32)
                {
                    let ref mut fresh63 = (*pfd.offset(i as isize)).revents;
                    *fresh63 = (*fresh63 as i32 & !(0x1 as i32)) as libc::c_short;
                    let ref mut fresh64 = (*pfd.offset(i as isize)).revents;
                    *fresh64 = (*fresh64 as i32 & !(0x40 as i32)) as libc::c_short;
                    let ref mut fresh65 = (*pfd.offset(i as isize)).revents;
                    *fresh65 = (*fresh65 as i32 | 0x10 as i32) as libc::c_short;
                }
            } else if wfds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
            {
                if (*pfd.offset(i as isize)).events as i32 & 0x4 as i32 != 0 {
                    let ref mut fresh66 = (*pfd.offset(i as isize)).revents;
                    *fresh66 = (*fresh66 as i32 | 0x4 as i32) as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as i32 & 0x100 as i32 != 0 {
                    let ref mut fresh67 = (*pfd.offset(i as isize)).revents;
                    *fresh67 = (*fresh67 as i32 | 0x100 as i32) as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as i32 & 0x200 as i32 != 0 {
                    let ref mut fresh68 = (*pfd.offset(i as isize)).revents;
                    *fresh68 = (*fresh68 as i32 | 0x200 as i32) as libc::c_short;
                }
                n += 1;
                n;
            } else if efds
                .__fds_bits[((*pfd.offset(i as isize)).fd
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << (*pfd.offset(i as isize)).fd
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
            {
                if (*pfd.offset(i as isize)).events as i32 & 0x2 as i32 != 0 {
                    let ref mut fresh69 = (*pfd.offset(i as isize)).revents;
                    *fresh69 = (*fresh69 as i32 | 0x2 as i32) as libc::c_short;
                }
                if (*pfd.offset(i as isize)).events as i32 & 0x80 as i32 != 0 {
                    let ref mut fresh70 = (*pfd.offset(i as isize)).revents;
                    *fresh70 = (*fresh70 as i32 | 0x80 as i32) as libc::c_short;
                }
                n += 1;
                n;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn pth_connect(
    mut s: i32,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
) -> i32 {
    return pth_connect_ev(s, addr, addrlen, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_connect_ev(
    mut s: i32,
    mut addr: *const sockaddr,
    mut addrlen: socklen_t,
    mut ev_extra: pth_event_t,
) -> i32 {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut rv: i32 = 0;
    let mut err: i32 = 0;
    let mut errlen: socklen_t = 0;
    let mut fdmode: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if __pth_util_fd_valid(s) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    fdmode = pth_fdmode(s, PTH_FDMODE_NONBLOCK as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    loop {
        rv = connect(s, addr as *mut sockaddr, addrlen);
        if !(rv == -(1 as i32) && *__errno_location() == 4 as i32) {
            break;
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        pth_fdmode(s, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    if rv == -(1 as i32) && *__errno_location() == 115 as i32
        && fdmode != PTH_FDMODE_NONBLOCK as i32
    {
        ev = pth_event(
            ((1 as i32) << 1 as i32 | (1 as i32) << 13 as i32 | (1 as i32) << 22 as i32)
                as u64,
            &mut ev_key as *mut pth_key_t,
            s,
        );
        if ev.is_null() {
            *__errno_location() = *__errno_location();
            return -(1 as i32);
        }
        if !ev_extra.is_null() {
            pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
        }
        pth_wait(ev);
        if !ev_extra.is_null() {
            pth_event_isolate(ev);
            if pth_event_status(ev) as u32
                != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
            {
                *__errno_location() = 4 as i32;
                return -(1 as i32);
            }
        }
        errlen = ::core::mem::size_of::<i32>() as u64 as socklen_t;
        if getsockopt(
            s,
            1 as i32,
            4 as i32,
            &mut err as *mut i32 as *mut libc::c_void,
            &mut errlen,
        ) == -(1 as i32)
        {
            return -(1 as i32);
        }
        if err == 0 as i32 {
            return 0 as i32;
        }
        *__errno_location() = err;
        return rv;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn pth_accept(
    mut s: i32,
    mut addr: *mut sockaddr,
    mut addrlen: *mut socklen_t,
) -> i32 {
    return pth_accept_ev(s, addr, addrlen, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_accept_ev(
    mut s: i32,
    mut addr: *mut sockaddr,
    mut addrlen: *mut socklen_t,
    mut ev_extra: pth_event_t,
) -> i32 {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut fdmode: i32 = 0;
    let mut rv: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if __pth_util_fd_valid(s) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    fdmode = pth_fdmode(s, PTH_FDMODE_NONBLOCK as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32);
    }
    ev = 0 as pth_event_t;
    loop {
        rv = accept(s, addr, addrlen);
        if !(rv == -(1 as i32)
            && (*__errno_location() == 11 as i32 || *__errno_location() == 11 as i32)
            && fdmode != PTH_FDMODE_NONBLOCK as i32)
        {
            break;
        }
        if ev.is_null() {
            ev = pth_event(
                ((1 as i32) << 1 as i32 | (1 as i32) << 12 as i32
                    | (1 as i32) << 22 as i32) as u64,
                &mut ev_key as *mut pth_key_t,
                s,
            );
            if ev.is_null() {
                *__errno_location() = *__errno_location();
                return -(1 as i32);
            }
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
        }
        pth_wait(ev);
        if !ev_extra.is_null() {
            pth_event_isolate(ev);
            if pth_event_status(ev) as u32
                != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
            {
                pth_fdmode(s, fdmode);
                *__errno_location() = 4 as i32;
                return -(1 as i32);
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        pth_fdmode(s, fdmode);
        if rv != -(1 as i32) {
            pth_fdmode(rv, fdmode);
        }
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn pth_read(
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
) -> ssize_t {
    return pth_read_ev(fd, buf, nbytes, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_read_ev(
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: i32 = 0;
    let mut n: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as i32 as u64 {
        return 0 as i32 as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_POLL as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    if fdmode == PTH_FDMODE_BLOCK as i32 {
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh71 = &mut __d0;
        let fresh72;
        let fresh73 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh74 = &mut __d1;
        let fresh75;
        let fresh76 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh71, fresh73) => fresh72,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh74, fresh76) =>
            fresh75, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh71, fresh73, fresh72);
        c2rust_asm_casts::AsmCast::cast_out(fresh74, fresh76, fresh75);
        fds
            .__fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        delay.tv_sec = 0 as i32 as __time_t;
        delay.tv_usec = 0 as i32 as __suseconds_t;
        loop {
            n = select(
                fd + 1 as i32,
                &mut fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if n < 0 as i32
            && (*__errno_location() == 22 as i32 || *__errno_location() == 9 as i32)
        {
            *__errno_location() = *__errno_location();
            return -(1 as i32) as ssize_t;
        }
        if n == 0 as i32 {
            ev = pth_event(
                ((1 as i32) << 1 as i32 | (1 as i32) << 12 as i32
                    | (1 as i32) << 22 as i32) as u64,
                &mut ev_key as *mut pth_key_t,
                fd,
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            n = pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as u32
                    != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
                {
                    *__errno_location() = 4 as i32;
                    return -(1 as i32) as ssize_t;
                }
            }
        }
    }
    loop {
        n = read(fd, buf, nbytes) as i32;
        if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return n as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn pth_write(
    mut fd: i32,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
) -> ssize_t {
    return pth_write_ev(fd, buf, nbytes, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_write_ev(
    mut fd: i32,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: i32 = 0;
    let mut rv: ssize_t = 0;
    let mut s: ssize_t = 0;
    let mut n: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as i32 as u64 {
        return 0 as i32 as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_NONBLOCK as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    if fdmode != PTH_FDMODE_NONBLOCK as i32 {
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh77 = &mut __d0;
        let fresh78;
        let fresh79 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh80 = &mut __d1;
        let fresh81;
        let fresh82 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh77, fresh79) => fresh78,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh80, fresh82) =>
            fresh81, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh77, fresh79, fresh78);
        c2rust_asm_casts::AsmCast::cast_out(fresh80, fresh82, fresh81);
        fds
            .__fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        delay.tv_sec = 0 as i32 as __time_t;
        delay.tv_usec = 0 as i32 as __suseconds_t;
        loop {
            n = select(
                fd + 1 as i32,
                0 as *mut fd_set,
                &mut fds,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if n < 0 as i32
            && (*__errno_location() == 22 as i32 || *__errno_location() == 9 as i32)
        {
            *__errno_location() = *__errno_location();
            return -(1 as i32) as ssize_t;
        }
        rv = 0 as i32 as ssize_t;
        loop {
            if n < 1 as i32 {
                ev = pth_event(
                    ((1 as i32) << 1 as i32 | (1 as i32) << 13 as i32
                        | (1 as i32) << 22 as i32) as u64,
                    &mut ev_key as *mut pth_key_t,
                    fd,
                );
                if !ev_extra.is_null() {
                    pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
                }
                pth_wait(ev);
                if !ev_extra.is_null() {
                    pth_event_isolate(ev);
                    if pth_event_status(ev) as u32
                        != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
                    {
                        pth_fdmode(fd, fdmode);
                        *__errno_location() = 4 as i32;
                        return -(1 as i32) as ssize_t;
                    }
                }
            }
            loop {
                s = write(fd, buf, nbytes);
                if !(s < 0 as i32 as i64 && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if s > 0 as i32 as i64 {
                rv += s;
            }
            if s > 0 as i32 as i64 && s < nbytes as ssize_t {
                nbytes = (nbytes as u64).wrapping_sub(s as u64) as size_t as size_t;
                buf = (buf as *mut i8).offset(s as isize) as *mut libc::c_void;
                n = 0 as i32;
            } else {
                if s < 0 as i32 as i64 && rv == 0 as i32 as i64 {
                    rv = -(1 as i32) as ssize_t;
                }
                break;
            }
        }
    } else {
        loop {
            rv = write(fd, buf, nbytes);
            if !(rv < 0 as i32 as i64 && *__errno_location() == 4 as i32) {
                break;
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        pth_fdmode(fd, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn pth_readv(
    mut fd: i32,
    mut iov: *const iovec,
    mut iovcnt: i32,
) -> ssize_t {
    return pth_readv_ev(fd, iov, iovcnt, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_readv_ev(
    mut fd: i32,
    mut iov: *const iovec,
    mut iovcnt: i32,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: i32 = 0;
    let mut n: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if iovcnt <= 0 as i32 || iovcnt > 1024 as i32 {
        *__errno_location() = 22 as i32;
        return -(1 as i32) as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_POLL as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    if fdmode == PTH_FDMODE_BLOCK as i32 {
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh83 = &mut __d0;
        let fresh84;
        let fresh85 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh86 = &mut __d1;
        let fresh87;
        let fresh88 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh83, fresh85) => fresh84,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh86, fresh88) =>
            fresh87, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh83, fresh85, fresh84);
        c2rust_asm_casts::AsmCast::cast_out(fresh86, fresh88, fresh87);
        fds
            .__fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        delay.tv_sec = 0 as i32 as __time_t;
        delay.tv_usec = 0 as i32 as __suseconds_t;
        loop {
            n = select(
                fd + 1 as i32,
                &mut fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if n < 1 as i32 {
            ev = pth_event(
                ((1 as i32) << 1 as i32 | (1 as i32) << 12 as i32
                    | (1 as i32) << 22 as i32) as u64,
                &mut ev_key as *mut pth_key_t,
                fd,
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            n = pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as u32
                    != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
                {
                    *__errno_location() = 4 as i32;
                    return -(1 as i32) as ssize_t;
                }
            }
        }
    }
    loop {
        n = readv(fd, iov, iovcnt) as i32;
        if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return n as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_readv_faked(
    mut fd: i32,
    mut iov: *const iovec,
    mut iovcnt: i32,
) -> ssize_t {
    let mut buffer: *mut i8 = 0 as *mut i8;
    let mut bytes: size_t = 0;
    let mut copy: size_t = 0;
    let mut rv: size_t = 0;
    let mut i: i32 = 0;
    bytes = 0 as i32 as size_t;
    i = 0 as i32;
    while i < iovcnt {
        if (*iov.offset(i as isize)).iov_len <= 0 as i32 as u64 {
            *__errno_location() = 22 as i32;
            return -(1 as i32) as ssize_t;
        }
        bytes = (bytes as u64).wrapping_add((*iov.offset(i as isize)).iov_len) as size_t
            as size_t;
        i += 1;
        i;
    }
    if bytes <= 0 as i32 as u64 {
        *__errno_location() = 22 as i32;
        return -(1 as i32) as ssize_t;
    }
    buffer = malloc(bytes) as *mut i8;
    if buffer.is_null() {
        return -(1 as i32) as ssize_t;
    }
    rv = read(fd, buffer as *mut libc::c_void, bytes) as size_t;
    if rv > 0 as i32 as u64 {
        bytes = rv;
        i = 0 as i32;
        while i < iovcnt {
            copy = if (*iov.offset(i as isize)).iov_len > bytes {
                bytes
            } else {
                (*iov.offset(i as isize)).iov_len
            };
            memcpy(
                (*iov.offset(i as isize)).iov_base,
                buffer as *const libc::c_void,
                copy,
            );
            buffer = buffer.offset(copy as isize);
            bytes = (bytes as u64).wrapping_sub(copy) as size_t as size_t;
            if bytes <= 0 as i32 as u64 {
                break;
            }
            i += 1;
            i;
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        free(buffer as *mut libc::c_void);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    return rv as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn pth_writev(
    mut fd: i32,
    mut iov: *const iovec,
    mut iovcnt: i32,
) -> ssize_t {
    return pth_writev_ev(fd, iov, iovcnt, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_writev_ev(
    mut fd: i32,
    mut iov: *const iovec,
    mut iovcnt: i32,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: i32 = 0;
    let mut liov: *mut iovec = 0 as *mut iovec;
    let mut liovcnt: i32 = 0;
    let mut nbytes: size_t = 0;
    let mut rv: ssize_t = 0;
    let mut s: ssize_t = 0;
    let mut n: i32 = 0;
    let mut tiov_stack: [iovec; 32] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 32];
    let mut tiov: *mut iovec = 0 as *mut iovec;
    let mut tiovcnt: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if iovcnt <= 0 as i32 || iovcnt > 1024 as i32 {
        *__errno_location() = 22 as i32;
        return -(1 as i32) as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_NONBLOCK as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    if fdmode != PTH_FDMODE_NONBLOCK as i32 {
        if iovcnt as u64 > ::core::mem::size_of::<[iovec; 32]>() as u64 {
            tiovcnt = (::core::mem::size_of::<iovec>() as u64)
                .wrapping_mul(1024 as i32 as u64) as i32;
            tiov = malloc(tiovcnt as u64) as *mut iovec;
            if tiov.is_null() {
                *__errno_location() = *__errno_location();
                return -(1 as i32) as ssize_t;
            }
        } else {
            tiovcnt = ::core::mem::size_of::<[iovec; 32]>() as u64 as i32;
            tiov = tiov_stack.as_mut_ptr();
        }
        rv = 0 as i32 as ssize_t;
        nbytes = __pth_writev_iov_bytes(iov, iovcnt) as size_t;
        liov = 0 as *mut iovec;
        liovcnt = 0 as i32;
        __pth_writev_iov_advance(
            iov,
            iovcnt,
            0 as i32 as size_t,
            &mut liov,
            &mut liovcnt,
            tiov,
            tiovcnt,
        );
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh89 = &mut __d0;
        let fresh90;
        let fresh91 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh92 = &mut __d1;
        let fresh93;
        let fresh94 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh89, fresh91) => fresh90,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh92, fresh94) =>
            fresh93, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh89, fresh91, fresh90);
        c2rust_asm_casts::AsmCast::cast_out(fresh92, fresh94, fresh93);
        fds
            .__fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        delay.tv_sec = 0 as i32 as __time_t;
        delay.tv_usec = 0 as i32 as __suseconds_t;
        loop {
            n = select(
                fd + 1 as i32,
                0 as *mut fd_set,
                &mut fds,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        loop {
            if n < 1 as i32 {
                ev = pth_event(
                    ((1 as i32) << 1 as i32 | (1 as i32) << 13 as i32
                        | (1 as i32) << 22 as i32) as u64,
                    &mut ev_key as *mut pth_key_t,
                    fd,
                );
                if !ev_extra.is_null() {
                    pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
                }
                pth_wait(ev);
                if !ev_extra.is_null() {
                    pth_event_isolate(ev);
                    if pth_event_status(ev) as u32
                        != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
                    {
                        pth_fdmode(fd, fdmode);
                        if iovcnt as u64 > ::core::mem::size_of::<[iovec; 32]>() as u64 {
                            free(tiov as *mut libc::c_void);
                        }
                        *__errno_location() = 4 as i32;
                        return -(1 as i32) as ssize_t;
                    }
                }
            }
            loop {
                s = writev(fd, liov, liovcnt);
                if !(s < 0 as i32 as i64 && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if s > 0 as i32 as i64 {
                rv += s;
            }
            if s > 0 as i32 as i64 && s < nbytes as ssize_t {
                nbytes = (nbytes as u64).wrapping_sub(s as u64) as size_t as size_t;
                __pth_writev_iov_advance(
                    iov,
                    iovcnt,
                    s as size_t,
                    &mut liov,
                    &mut liovcnt,
                    tiov,
                    tiovcnt,
                );
                n = 0 as i32;
            } else {
                if s < 0 as i32 as i64 && rv == 0 as i32 as i64 {
                    rv = -(1 as i32) as ssize_t;
                }
                break;
            }
        }
        if iovcnt as u64 > ::core::mem::size_of::<[iovec; 32]>() as u64 {
            free(tiov as *mut libc::c_void);
        }
    } else {
        loop {
            rv = writev(fd, iov, iovcnt);
            if !(rv < 0 as i32 as i64 && *__errno_location() == 4 as i32) {
                break;
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        pth_fdmode(fd, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_writev_iov_bytes(
    mut iov: *const iovec,
    mut iovcnt: i32,
) -> ssize_t {
    let mut bytes: ssize_t = 0;
    let mut i: i32 = 0;
    bytes = 0 as i32 as ssize_t;
    i = 0 as i32;
    while i < iovcnt {
        if !((*iov.offset(i as isize)).iov_len <= 0 as i32 as u64) {
            bytes = (bytes as u64).wrapping_add((*iov.offset(i as isize)).iov_len)
                as ssize_t as ssize_t;
        }
        i += 1;
        i;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_writev_iov_advance(
    mut riov: *const iovec,
    mut riovcnt: i32,
    mut advance: size_t,
    mut liov: *mut *mut iovec,
    mut liovcnt: *mut i32,
    mut tiov: *mut iovec,
    mut tiovcnt: i32,
) {
    let mut i: i32 = 0;
    if (*liov).is_null() && *liovcnt == 0 as i32 {
        *liov = riov as *mut iovec;
        *liovcnt = riovcnt;
    }
    if advance > 0 as i32 as u64 {
        if *liov == riov as *mut iovec && *liovcnt == riovcnt {
            *liov = &mut *tiov.offset(0 as i32 as isize) as *mut iovec;
            i = 0 as i32;
            while i < riovcnt {
                let ref mut fresh95 = (*tiov.offset(i as isize)).iov_base;
                *fresh95 = (*riov.offset(i as isize)).iov_base;
                (*tiov.offset(i as isize)).iov_len = (*riov.offset(i as isize)).iov_len;
                i += 1;
                i;
            }
        }
        while *liovcnt > 0 as i32 && advance > 0 as i32 as u64 {
            if (**liov).iov_len > advance {
                (**liov).iov_base = ((**liov).iov_base as *mut i8)
                    .offset(advance as isize) as *mut libc::c_void;
                (**liov).iov_len = ((**liov).iov_len as u64).wrapping_sub(advance)
                    as size_t as size_t;
                break;
            } else {
                advance = (advance as u64).wrapping_sub((**liov).iov_len) as size_t
                    as size_t;
                *liovcnt -= 1;
                *liovcnt;
                *liov = (*liov).offset(1);
                *liov;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __pth_writev_faked(
    mut fd: i32,
    mut iov: *const iovec,
    mut iovcnt: i32,
) -> ssize_t {
    let mut buffer: *mut i8 = 0 as *mut i8;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut bytes: size_t = 0;
    let mut to_copy: size_t = 0;
    let mut copy: size_t = 0;
    let mut rv: size_t = 0;
    let mut i: i32 = 0;
    bytes = 0 as i32 as size_t;
    i = 0 as i32;
    while i < iovcnt {
        if (*iov.offset(i as isize)).iov_len <= 0 as i32 as u64 {
            *__errno_location() = 22 as i32;
            return -(1 as i32) as ssize_t;
        }
        bytes = (bytes as u64).wrapping_add((*iov.offset(i as isize)).iov_len) as size_t
            as size_t;
        i += 1;
        i;
    }
    if bytes <= 0 as i32 as u64 {
        *__errno_location() = 22 as i32;
        return -(1 as i32) as ssize_t;
    }
    buffer = malloc(bytes) as *mut i8;
    if buffer.is_null() {
        return -(1 as i32) as ssize_t;
    }
    to_copy = bytes;
    cp = buffer;
    i = 0 as i32;
    while i < iovcnt {
        copy = if (*iov.offset(i as isize)).iov_len > to_copy {
            to_copy
        } else {
            (*iov.offset(i as isize)).iov_len
        };
        memcpy(cp as *mut libc::c_void, (*iov.offset(i as isize)).iov_base, copy);
        to_copy = (to_copy as u64).wrapping_sub(copy) as size_t as size_t;
        if to_copy <= 0 as i32 as u64 {
            break;
        }
        i += 1;
        i;
    }
    rv = write(fd, buffer as *const libc::c_void, bytes) as size_t;
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        free(buffer as *mut libc::c_void);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    return rv as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn pth_pread(
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
    mut offset: off_t,
) -> ssize_t {
    static mut mutex: pth_mutex_t = {
        let mut init = pth_mutex_st {
            mx_node: {
                let mut init = pth_ringnode_st {
                    rn_next: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                    rn_prev: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                };
                init
            },
            mx_state: (1 as i32) << 0 as i32,
            mx_owner: 0 as *const pth_st as pth_t,
            mx_count: 0 as i32 as u64,
        };
        init
    };
    let mut old_offset: off_t = 0;
    let mut rc: ssize_t = 0;
    if pth_mutex_acquire(&mut mutex, 0 as i32, 0 as pth_event_t) == 0 {
        return -(1 as i32) as ssize_t;
    }
    old_offset = lseek(fd, 0 as i32 as __off_t, 1 as i32);
    if old_offset == -(1 as i32) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as i32) as ssize_t;
    }
    if lseek(fd, offset, 0 as i32) == -(1 as i32) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as i32) as ssize_t;
    }
    rc = pth_read(fd, buf, nbytes);
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        lseek(fd, old_offset, 0 as i32);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    pth_mutex_release(&mut mutex);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn pth_pwrite(
    mut fd: i32,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
    mut offset: off_t,
) -> ssize_t {
    static mut mutex: pth_mutex_t = {
        let mut init = pth_mutex_st {
            mx_node: {
                let mut init = pth_ringnode_st {
                    rn_next: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                    rn_prev: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
                };
                init
            },
            mx_state: (1 as i32) << 0 as i32,
            mx_owner: 0 as *const pth_st as pth_t,
            mx_count: 0 as i32 as u64,
        };
        init
    };
    let mut old_offset: off_t = 0;
    let mut rc: ssize_t = 0;
    if pth_mutex_acquire(&mut mutex, 0 as i32, 0 as pth_event_t) == 0 {
        return -(1 as i32) as ssize_t;
    }
    old_offset = lseek(fd, 0 as i32 as __off_t, 1 as i32);
    if old_offset == -(1 as i32) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as i32) as ssize_t;
    }
    if lseek(fd, offset, 0 as i32) == -(1 as i32) as off_t {
        pth_mutex_release(&mut mutex);
        return -(1 as i32) as ssize_t;
    }
    rc = pth_write(fd, buf, nbytes);
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        lseek(fd, old_offset, 0 as i32);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    pth_mutex_release(&mut mutex);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn pth_recv(
    mut s: i32,
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut flags: i32,
) -> ssize_t {
    return pth_recv_ev(s, buf, len, flags, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_recv_ev(
    mut s: i32,
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut flags: i32,
    mut ev: pth_event_t,
) -> ssize_t {
    return pth_recvfrom_ev(
        s,
        buf,
        len,
        flags,
        0 as *mut sockaddr,
        0 as *mut socklen_t,
        ev,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pth_recvfrom(
    mut s: i32,
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut flags: i32,
    mut from: *mut sockaddr,
    mut fromlen: *mut socklen_t,
) -> ssize_t {
    return pth_recvfrom_ev(s, buf, len, flags, from, fromlen, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_recvfrom_ev(
    mut fd: i32,
    mut buf: *mut libc::c_void,
    mut nbytes: size_t,
    mut flags: i32,
    mut from: *mut sockaddr,
    mut fromlen: *mut socklen_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: i32 = 0;
    let mut n: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as i32 as u64 {
        return 0 as i32 as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_POLL as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    if fdmode == PTH_FDMODE_BLOCK as i32 {
        if __pth_util_fd_valid(fd) == 0 {
            *__errno_location() = 9 as i32;
            return -(1 as i32) as ssize_t;
        }
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh96 = &mut __d0;
        let fresh97;
        let fresh98 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh99 = &mut __d1;
        let fresh100;
        let fresh101 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh96, fresh98) => fresh97,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh99, fresh101) =>
            fresh100, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh96, fresh98, fresh97);
        c2rust_asm_casts::AsmCast::cast_out(fresh99, fresh101, fresh100);
        fds
            .__fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        delay.tv_sec = 0 as i32 as __time_t;
        delay.tv_usec = 0 as i32 as __suseconds_t;
        loop {
            n = select(
                fd + 1 as i32,
                &mut fds,
                0 as *mut fd_set,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if n < 0 as i32
            && (*__errno_location() == 22 as i32 || *__errno_location() == 9 as i32)
        {
            *__errno_location() = *__errno_location();
            return -(1 as i32) as ssize_t;
        }
        if n == 0 as i32 {
            ev = pth_event(
                ((1 as i32) << 1 as i32 | (1 as i32) << 12 as i32
                    | (1 as i32) << 22 as i32) as u64,
                &mut ev_key as *mut pth_key_t,
                fd,
            );
            if !ev_extra.is_null() {
                pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
            }
            n = pth_wait(ev);
            if !ev_extra.is_null() {
                pth_event_isolate(ev);
                if pth_event_status(ev) as u32
                    != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
                {
                    *__errno_location() = 4 as i32;
                    return -(1 as i32) as ssize_t;
                }
            }
        }
    }
    loop {
        n = recvfrom(fd, buf, nbytes, flags, from, fromlen) as i32;
        if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return n as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn pth_send(
    mut s: i32,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut flags: i32,
) -> ssize_t {
    return pth_send_ev(s, buf, len, flags, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_send_ev(
    mut s: i32,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut flags: i32,
    mut ev: pth_event_t,
) -> ssize_t {
    return pth_sendto_ev(
        s,
        buf,
        len,
        flags,
        0 as *const sockaddr,
        0 as i32 as socklen_t,
        ev,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pth_sendto(
    mut s: i32,
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut flags: i32,
    mut to: *const sockaddr,
    mut tolen: socklen_t,
) -> ssize_t {
    return pth_sendto_ev(s, buf, len, flags, to, tolen, 0 as pth_event_t);
}
#[no_mangle]
pub unsafe extern "C" fn pth_sendto_ev(
    mut fd: i32,
    mut buf: *const libc::c_void,
    mut nbytes: size_t,
    mut flags: i32,
    mut to: *const sockaddr,
    mut tolen: socklen_t,
    mut ev_extra: pth_event_t,
) -> ssize_t {
    let mut delay: timeval = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fdmode: i32 = 0;
    let mut rv: ssize_t = 0;
    let mut s: ssize_t = 0;
    let mut n: i32 = 0;
    if __pth_initialized == 0 {
        pth_init();
    }
    if nbytes == 0 as i32 as u64 {
        return 0 as i32 as ssize_t;
    }
    if __pth_util_fd_valid(fd) == 0 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    fdmode = pth_fdmode(fd, PTH_FDMODE_NONBLOCK as i32);
    if fdmode == PTH_FDMODE_ERROR as i32 {
        *__errno_location() = 9 as i32;
        return -(1 as i32) as ssize_t;
    }
    if fdmode != PTH_FDMODE_NONBLOCK as i32 {
        if __pth_util_fd_valid(fd) == 0 {
            pth_fdmode(fd, fdmode);
            *__errno_location() = 9 as i32;
            return -(1 as i32) as ssize_t;
        }
        let mut __d0: i32 = 0;
        let mut __d1: i32 = 0;
        let fresh102 = &mut __d0;
        let fresh103;
        let fresh104 = (::core::mem::size_of::<fd_set>() as u64)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as u64);
        let fresh105 = &mut __d1;
        let fresh106;
        let fresh107 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as i32 as isize)
            as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx")
            c2rust_asm_casts::AsmCast::cast_in(fresh102, fresh104) => fresh103,
            inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh105, fresh107) =>
            fresh106, inlateout("ax") 0 as libc::c_int => _, options(preserves_flags,
            att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh102, fresh104, fresh103);
        c2rust_asm_casts::AsmCast::cast_out(fresh105, fresh107, fresh106);
        fds
            .__fds_bits[(fd
            / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32)) as usize]
            |= ((1 as u64)
                << fd % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as __fd_mask;
        delay.tv_sec = 0 as i32 as __time_t;
        delay.tv_usec = 0 as i32 as __suseconds_t;
        loop {
            n = select(
                fd + 1 as i32,
                0 as *mut fd_set,
                &mut fds,
                0 as *mut fd_set,
                &mut delay,
            );
            if !(n < 0 as i32 && *__errno_location() == 4 as i32) {
                break;
            }
        }
        if n < 0 as i32
            && (*__errno_location() == 22 as i32 || *__errno_location() == 9 as i32)
        {
            *__errno_location() = *__errno_location();
            return -(1 as i32) as ssize_t;
        }
        rv = 0 as i32 as ssize_t;
        loop {
            if n == 0 as i32 {
                ev = pth_event(
                    ((1 as i32) << 1 as i32 | (1 as i32) << 13 as i32
                        | (1 as i32) << 22 as i32) as u64,
                    &mut ev_key as *mut pth_key_t,
                    fd,
                );
                if !ev_extra.is_null() {
                    pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
                }
                pth_wait(ev);
                if !ev_extra.is_null() {
                    pth_event_isolate(ev);
                    if pth_event_status(ev) as u32
                        != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32
                    {
                        pth_fdmode(fd, fdmode);
                        *__errno_location() = 4 as i32;
                        return -(1 as i32) as ssize_t;
                    }
                }
            }
            loop {
                s = sendto(fd, buf, nbytes, flags, to, tolen);
                if !(s < 0 as i32 as i64 && *__errno_location() == 4 as i32) {
                    break;
                }
            }
            if s > 0 as i32 as i64 {
                rv += s;
            }
            if s > 0 as i32 as i64 && s < nbytes as ssize_t {
                nbytes = (nbytes as u64).wrapping_sub(s as u64) as size_t as size_t;
                buf = (buf as *mut i8).offset(s as isize) as *mut libc::c_void;
                n = 0 as i32;
            } else {
                if s < 0 as i32 as i64 && rv == 0 as i32 as i64 {
                    rv = -(1 as i32) as ssize_t;
                }
                break;
            }
        }
    } else {
        loop {
            rv = sendto(fd, buf, nbytes, flags, to, tolen);
            if !(rv < 0 as i32 as i64 && *__errno_location() == 4 as i32) {
                break;
            }
        }
    }
    __pth_errno_storage = *__errno_location();
    __pth_errno_flag = (0 as i32 == 0) as i32;
    while __pth_errno_flag != 0 {
        pth_fdmode(fd, fdmode);
        *__errno_location() = __pth_errno_storage;
        __pth_errno_flag = 0 as i32;
    }
    return rv;
}