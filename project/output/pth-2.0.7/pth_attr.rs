use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
    fn __pth_util_cpystrn(_: *mut i8, _: *const i8, _: size_t) -> *mut i8;
    static mut __pth_time_zero: pth_time_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type va_list = __builtin_va_list;
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
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
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
    pub ev_args: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub FD: C2RustUnnamed_8,
    pub SELECT: C2RustUnnamed_7,
    pub SIGS: C2RustUnnamed_6,
    pub TIME: C2RustUnnamed_5,
    pub MSG: C2RustUnnamed_4,
    pub MUTEX: C2RustUnnamed_3,
    pub COND: C2RustUnnamed_2,
    pub TID: C2RustUnnamed_1,
    pub FUNC: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub func: pth_event_func_t,
    pub arg: *mut libc::c_void,
    pub tv: pth_time_t,
}
pub type pth_event_func_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tid: pth_t,
}
pub type pth_t = *mut pth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
    pub tv: pth_time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub sigs: *mut sigset_t,
    pub sig: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub n: *mut i32,
    pub nfd: i32,
    pub rfds: *mut fd_set,
    pub wfds: *mut fd_set,
    pub efds: *mut fd_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_attr_st {
    pub a_tid: pth_t,
    pub a_prio: i32,
    pub a_dispatches: i32,
    pub a_name: [i8; 40],
    pub a_joinable: i32,
    pub a_cancelstate: u32,
    pub a_stacksize: u32,
    pub a_stackaddr: *mut i8,
}
pub type pth_attr_t = *mut pth_attr_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_9 {
    PTH_ATTR_BOUND = 14,
    PTH_ATTR_EVENTS = 13,
    PTH_ATTR_STATE = 12,
    PTH_ATTR_START_ARG = 11,
    PTH_ATTR_START_FUNC = 10,
    PTH_ATTR_TIME_RAN = 9,
    PTH_ATTR_TIME_LAST = 8,
    PTH_ATTR_TIME_SPAWN = 7,
    PTH_ATTR_DISPATCHES = 6,
    PTH_ATTR_STACK_ADDR = 5,
    PTH_ATTR_STACK_SIZE = 4,
    PTH_ATTR_CANCEL_STATE = 3,
    PTH_ATTR_JOINABLE = 2,
    PTH_ATTR_NAME = 1,
    PTH_ATTR_PRIO = 0,
}
impl C2RustUnnamed_9 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_9::PTH_ATTR_BOUND => 14,
            C2RustUnnamed_9::PTH_ATTR_EVENTS => 13,
            C2RustUnnamed_9::PTH_ATTR_STATE => 12,
            C2RustUnnamed_9::PTH_ATTR_START_ARG => 11,
            C2RustUnnamed_9::PTH_ATTR_START_FUNC => 10,
            C2RustUnnamed_9::PTH_ATTR_TIME_RAN => 9,
            C2RustUnnamed_9::PTH_ATTR_TIME_LAST => 8,
            C2RustUnnamed_9::PTH_ATTR_TIME_SPAWN => 7,
            C2RustUnnamed_9::PTH_ATTR_DISPATCHES => 6,
            C2RustUnnamed_9::PTH_ATTR_STACK_ADDR => 5,
            C2RustUnnamed_9::PTH_ATTR_STACK_SIZE => 4,
            C2RustUnnamed_9::PTH_ATTR_CANCEL_STATE => 3,
            C2RustUnnamed_9::PTH_ATTR_JOINABLE => 2,
            C2RustUnnamed_9::PTH_ATTR_NAME => 1,
            C2RustUnnamed_9::PTH_ATTR_PRIO => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_9 {
        match value {
            14 => C2RustUnnamed_9::PTH_ATTR_BOUND,
            13 => C2RustUnnamed_9::PTH_ATTR_EVENTS,
            12 => C2RustUnnamed_9::PTH_ATTR_STATE,
            11 => C2RustUnnamed_9::PTH_ATTR_START_ARG,
            10 => C2RustUnnamed_9::PTH_ATTR_START_FUNC,
            9 => C2RustUnnamed_9::PTH_ATTR_TIME_RAN,
            8 => C2RustUnnamed_9::PTH_ATTR_TIME_LAST,
            7 => C2RustUnnamed_9::PTH_ATTR_TIME_SPAWN,
            6 => C2RustUnnamed_9::PTH_ATTR_DISPATCHES,
            5 => C2RustUnnamed_9::PTH_ATTR_STACK_ADDR,
            4 => C2RustUnnamed_9::PTH_ATTR_STACK_SIZE,
            3 => C2RustUnnamed_9::PTH_ATTR_CANCEL_STATE,
            2 => C2RustUnnamed_9::PTH_ATTR_JOINABLE,
            1 => C2RustUnnamed_9::PTH_ATTR_NAME,
            0 => C2RustUnnamed_9::PTH_ATTR_PRIO,
            _ => panic!("Invalid value for C2RustUnnamed_9: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_9 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_9 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_9 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_9 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_9 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn add(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn sub(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn mul(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn div(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_9 {
    type Output = C2RustUnnamed_9;
    fn rem(self, rhs: u32) -> C2RustUnnamed_9 {
        C2RustUnnamed_9::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_10 {
    PTH_ATTR_GET,
    PTH_ATTR_SET,
}
impl C2RustUnnamed_10 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_10::PTH_ATTR_GET => 0,
            C2RustUnnamed_10::PTH_ATTR_SET => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_10 {
        match value {
            0 => C2RustUnnamed_10::PTH_ATTR_GET,
            1 => C2RustUnnamed_10::PTH_ATTR_SET,
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
#[no_mangle]
pub unsafe extern "C" fn pth_attr_of(mut t: pth_t) -> pth_attr_t {
    let mut a: pth_attr_t = 0 as *mut pth_attr_st;
    if t.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as *mut libc::c_void as pth_attr_t;
    }
    a = malloc(::core::mem::size_of::<pth_attr_st>() as u64) as pth_attr_t;
    if a.is_null() {
        *__errno_location() = 12 as i32;
        return 0 as *mut libc::c_void as pth_attr_t;
    }
    (*a).a_tid = t;
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn pth_attr_new() -> pth_attr_t {
    let mut a: pth_attr_t = 0 as *mut pth_attr_st;
    a = malloc(::core::mem::size_of::<pth_attr_st>() as u64) as pth_attr_t;
    if a.is_null() {
        *__errno_location() = 12 as i32;
        return 0 as *mut libc::c_void as pth_attr_t;
    }
    (*a).a_tid = 0 as pth_t;
    pth_attr_init(a);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn pth_attr_destroy(mut a: pth_attr_t) -> i32 {
    if a.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    free(a as *mut libc::c_void);
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_attr_init(mut a: pth_attr_t) -> i32 {
    if a.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if !((*a).a_tid).is_null() {
        *__errno_location() = 1 as i32;
        return 0 as i32;
    }
    (*a).a_prio = 0 as i32;
    __pth_util_cpystrn(
        ((*a).a_name).as_mut_ptr(),
        b"unknown\0" as *const u8 as *const i8,
        40 as i32 as size_t,
    );
    (*a).a_dispatches = 0 as i32;
    (*a).a_joinable = (0 as i32 == 0) as i32;
    (*a).a_cancelstate = ((1 as i32) << 0 as i32 | (1 as i32) << 3 as i32) as u32;
    (*a).a_stacksize = (64 as i32 * 1024 as i32) as u32;
    (*a).a_stackaddr = 0 as *mut i8;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_attr_get(
    mut a: pth_attr_t,
    mut op: i32,
    mut args: ...
) -> i32 {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rc: i32 = 0;
    ap = args.clone();
    rc = __pth_attr_ctrl(C2RustUnnamed_10::PTH_ATTR_GET as i32, a, op, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn pth_attr_set(
    mut a: pth_attr_t,
    mut op: i32,
    mut args: ...
) -> i32 {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rc: i32 = 0;
    ap = args.clone();
    rc = __pth_attr_ctrl(C2RustUnnamed_10::PTH_ATTR_SET as i32, a, op, ap.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_attr_ctrl(
    mut cmd: i32,
    mut a: pth_attr_t,
    mut op: i32,
    mut ap: ::core::ffi::VaList,
) -> i32 {
    if a.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    match op {
        0 => {
            let mut val: i32 = 0;
            let mut src: *mut i32 = 0 as *mut i32;
            let mut dst: *mut i32 = 0 as *mut i32;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                src = &mut val;
                val = ap.arg::<i32>();
                dst = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).prio
                } else {
                    &mut (*a).a_prio
                };
            } else {
                src = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).prio
                } else {
                    &mut (*a).a_prio
                };
                dst = ap.arg::<*mut i32>();
            }
            *dst = *src;
        }
        1 => {
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                let mut src_0: *mut i8 = 0 as *mut i8;
                let mut dst_0: *mut i8 = 0 as *mut i8;
                src_0 = ap.arg::<*mut i8>();
                dst_0 = if !((*a).a_tid).is_null() {
                    ((*(*a).a_tid).name).as_mut_ptr()
                } else {
                    ((*a).a_name).as_mut_ptr()
                };
                __pth_util_cpystrn(dst_0, src_0, 40 as i32 as size_t);
            } else {
                let mut src_1: *mut i8 = 0 as *mut i8;
                let mut dst_1: *mut *mut i8 = 0 as *mut *mut i8;
                src_1 = if !((*a).a_tid).is_null() {
                    ((*(*a).a_tid).name).as_mut_ptr()
                } else {
                    ((*a).a_name).as_mut_ptr()
                };
                dst_1 = ap.arg::<*mut *mut i8>();
                *dst_1 = src_1;
            }
        }
        6 => {
            let mut val_0: i32 = 0;
            let mut src_2: *mut i32 = 0 as *mut i32;
            let mut dst_2: *mut i32 = 0 as *mut i32;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                src_2 = &mut val_0;
                val_0 = ap.arg::<i32>();
                dst_2 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).dispatches
                } else {
                    &mut (*a).a_dispatches
                };
            } else {
                src_2 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).dispatches
                } else {
                    &mut (*a).a_dispatches
                };
                dst_2 = ap.arg::<*mut i32>();
            }
            *dst_2 = *src_2;
        }
        2 => {
            let mut val_1: i32 = 0;
            let mut src_3: *mut i32 = 0 as *mut i32;
            let mut dst_3: *mut i32 = 0 as *mut i32;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                src_3 = &mut val_1;
                val_1 = ap.arg::<i32>();
                dst_3 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).joinable
                } else {
                    &mut (*a).a_joinable
                };
            } else {
                src_3 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).joinable
                } else {
                    &mut (*a).a_joinable
                };
                dst_3 = ap.arg::<*mut i32>();
            }
            *dst_3 = *src_3;
        }
        3 => {
            let mut val_2: u32 = 0;
            let mut src_4: *mut u32 = 0 as *mut u32;
            let mut dst_4: *mut u32 = 0 as *mut u32;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                src_4 = &mut val_2;
                val_2 = ap.arg::<u32>();
                dst_4 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).cancelstate
                } else {
                    &mut (*a).a_cancelstate
                };
            } else {
                src_4 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).cancelstate
                } else {
                    &mut (*a).a_cancelstate
                };
                dst_4 = ap.arg::<*mut u32>();
            }
            *dst_4 = *src_4;
        }
        4 => {
            let mut val_3: u32 = 0;
            let mut src_5: *mut u32 = 0 as *mut u32;
            let mut dst_5: *mut u32 = 0 as *mut u32;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                if !((*a).a_tid).is_null() {
                    *__errno_location() = 1 as i32;
                    return 0 as i32;
                }
                src_5 = &mut val_3;
                val_3 = ap.arg::<u32>();
                dst_5 = &mut (*a).a_stacksize;
            } else {
                src_5 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).stacksize
                } else {
                    &mut (*a).a_stacksize
                };
                dst_5 = ap.arg::<*mut u32>();
            }
            *dst_5 = *src_5;
        }
        5 => {
            let mut val_4: *mut i8 = 0 as *mut i8;
            let mut src_6: *mut *mut i8 = 0 as *mut *mut i8;
            let mut dst_6: *mut *mut i8 = 0 as *mut *mut i8;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                if !((*a).a_tid).is_null() {
                    *__errno_location() = 1 as i32;
                    return 0 as i32;
                }
                src_6 = &mut val_4;
                val_4 = ap.arg::<*mut i8>();
                dst_6 = &mut (*a).a_stackaddr;
            } else {
                src_6 = if !((*a).a_tid).is_null() {
                    &mut (*(*a).a_tid).stack
                } else {
                    &mut (*a).a_stackaddr
                };
                dst_6 = ap.arg::<*mut *mut i8>();
            }
            *dst_6 = *src_6;
        }
        7 => {
            let mut dst_7: *mut pth_time_t = 0 as *mut pth_time_t;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            dst_7 = ap.arg::<*mut pth_time_t>();
            if !((*a).a_tid).is_null() {
                if (&mut (*(*a).a_tid).spawned as *mut pth_time_t).is_null() {
                    gettimeofday(dst_7, 0 as *mut timezone);
                } else {
                    (*dst_7).tv_sec = (*(*a).a_tid).spawned.tv_sec;
                    (*dst_7).tv_usec = (*(*a).a_tid).spawned.tv_usec;
                }
            } else if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                gettimeofday(dst_7, 0 as *mut timezone);
            } else {
                (*dst_7).tv_sec = __pth_time_zero.tv_sec;
                (*dst_7).tv_usec = __pth_time_zero.tv_usec;
            }
        }
        8 => {
            let mut dst_8: *mut pth_time_t = 0 as *mut pth_time_t;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            dst_8 = ap.arg::<*mut pth_time_t>();
            if !((*a).a_tid).is_null() {
                if (&mut (*(*a).a_tid).lastran as *mut pth_time_t).is_null() {
                    gettimeofday(dst_8, 0 as *mut timezone);
                } else {
                    (*dst_8).tv_sec = (*(*a).a_tid).lastran.tv_sec;
                    (*dst_8).tv_usec = (*(*a).a_tid).lastran.tv_usec;
                }
            } else if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                gettimeofday(dst_8, 0 as *mut timezone);
            } else {
                (*dst_8).tv_sec = __pth_time_zero.tv_sec;
                (*dst_8).tv_usec = __pth_time_zero.tv_usec;
            }
        }
        9 => {
            let mut dst_9: *mut pth_time_t = 0 as *mut pth_time_t;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            dst_9 = ap.arg::<*mut pth_time_t>();
            if !((*a).a_tid).is_null() {
                if (&mut (*(*a).a_tid).running as *mut pth_time_t).is_null() {
                    gettimeofday(dst_9, 0 as *mut timezone);
                } else {
                    (*dst_9).tv_sec = (*(*a).a_tid).running.tv_sec;
                    (*dst_9).tv_usec = (*(*a).a_tid).running.tv_usec;
                }
            } else if (&mut __pth_time_zero as *mut pth_time_t).is_null() {
                gettimeofday(dst_9, 0 as *mut timezone);
            } else {
                (*dst_9).tv_sec = __pth_time_zero.tv_sec;
                (*dst_9).tv_usec = __pth_time_zero.tv_usec;
            }
        }
        10 => {
            let mut dst_10: *mut Option<
                unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            > = 0
                as *mut Option<
                    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                >;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as i32;
                return 0 as i32;
            }
            dst_10 = ap.arg::<*mut libc::c_void>()
                as *mut Option<
                    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                >;
            *dst_10 = (*(*a).a_tid).start_func;
        }
        11 => {
            let mut dst_11: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as i32;
                return 0 as i32;
            }
            dst_11 = ap.arg::<*mut *mut libc::c_void>();
            *dst_11 = (*(*a).a_tid).start_arg;
        }
        12 => {
            let mut dst_12: *mut pth_state_t = 0 as *mut pth_state_t;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as i32;
                return 0 as i32;
            }
            dst_12 = ap.arg::<*mut pth_state_t>();
            *dst_12 = (*(*a).a_tid).state;
        }
        13 => {
            let mut dst_13: *mut pth_event_t = 0 as *mut pth_event_t;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            if ((*a).a_tid).is_null() {
                *__errno_location() = 13 as i32;
                return 0 as i32;
            }
            dst_13 = ap.arg::<*mut pth_event_t>();
            *dst_13 = (*(*a).a_tid).events;
        }
        14 => {
            let mut dst_14: *mut i32 = 0 as *mut i32;
            if cmd == C2RustUnnamed_10::PTH_ATTR_SET as i32 {
                *__errno_location() = 1 as i32;
                return 0 as i32;
            }
            dst_14 = ap.arg::<*mut i32>();
            *dst_14 = if !((*a).a_tid).is_null() {
                (0 as i32 == 0) as i32
            } else {
                0 as i32
            };
        }
        _ => {
            *__errno_location() = 22 as i32;
            return 0 as i32;
        }
    }
    return (0 as i32 == 0) as i32;
}