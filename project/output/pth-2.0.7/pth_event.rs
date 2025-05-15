use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn pth_yield(_: pth_t) -> i32;
    static mut __pth_current: pth_t;
    fn pth_cancel_point();
    fn __pth_util_fd_valid(_: i32) -> i32;
    fn pth_key_setdata(_: pth_key_t, _: *const libc::c_void) -> i32;
    fn pth_key_getdata(_: pth_key_t) -> *mut libc::c_void;
    fn pth_key_create(
        _: *mut pth_key_t,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_9 {
    PTH_FREE_THIS,
    PTH_FREE_ALL,
}
impl C2RustUnnamed_9 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_9::PTH_FREE_THIS => 0,
            C2RustUnnamed_9::PTH_FREE_ALL => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_9 {
        match value {
            0 => C2RustUnnamed_9::PTH_FREE_THIS,
            1 => C2RustUnnamed_9::PTH_FREE_ALL,
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
pub type pth_key_t = i32;
unsafe extern "C" fn pth_event_destructor(mut vp: *mut libc::c_void) {
    pth_event_free(vp as pth_event_t, C2RustUnnamed_9::PTH_FREE_THIS as i32);
}
#[no_mangle]
pub unsafe extern "C" fn pth_event(mut spec: u64, mut args: ...) -> pth_event_t {
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    let mut ev_key: *mut pth_key_t = 0 as *mut pth_key_t;
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    if spec & ((1 as i32) << 20 as i32) as u64 != 0 {
        ev = ap.arg::<pth_event_t>();
    } else if spec & ((1 as i32) << 22 as i32) as u64 != 0 {
        ev_key = ap.arg::<*mut pth_key_t>();
        if *ev_key == -(1 as i32) {
            pth_key_create(
                ev_key,
                Some(
                    pth_event_destructor as unsafe extern "C" fn(*mut libc::c_void) -> (),
                ),
            );
        }
        ev = pth_key_getdata(*ev_key) as pth_event_t;
        if ev.is_null() {
            ev = malloc(::core::mem::size_of::<pth_event_st>() as u64) as pth_event_t;
            pth_key_setdata(*ev_key, ev as *const libc::c_void);
        }
    } else {
        ev = malloc(::core::mem::size_of::<pth_event_st>() as u64) as pth_event_t;
    }
    if ev.is_null() {
        *__errno_location() = *__errno_location();
        return 0 as *mut libc::c_void as pth_event_t;
    }
    if spec & ((1 as i32) << 21 as i32) as u64 != 0 {
        let mut ch: pth_event_t = ap.arg::<pth_event_t>();
        (*ev).ev_prev = (*ch).ev_prev;
        (*ev).ev_next = ch;
        (*(*ev).ev_prev).ev_next = ev;
        (*(*ev).ev_next).ev_prev = ev;
    } else {
        (*ev).ev_prev = ev;
        (*ev).ev_next = ev;
    }
    (*ev).ev_status = pth_status_t::PTH_STATUS_PENDING;
    if spec & ((1 as i32) << 1 as i32) as u64 != 0 {
        let mut fd: i32 = ap.arg::<i32>();
        if __pth_util_fd_valid(fd) == 0 {
            *__errno_location() = 9 as i32;
            return 0 as *mut libc::c_void as pth_event_t;
        }
        (*ev).ev_type = (1 as i32) << 1 as i32;
        (*ev).ev_goal = (spec
            & ((1 as i32) << 12 as i32 | (1 as i32) << 13 as i32
                | (1 as i32) << 14 as i32) as u64) as i32;
        (*ev).ev_args.FD.fd = fd;
    } else if spec & ((1 as i32) << 2 as i32) as u64 != 0 {
        let mut n: *mut i32 = ap.arg::<*mut i32>();
        let mut nfd: i32 = ap.arg::<i32>();
        let mut rfds: *mut fd_set = ap.arg::<*mut fd_set>();
        let mut wfds: *mut fd_set = ap.arg::<*mut fd_set>();
        let mut efds: *mut fd_set = ap.arg::<*mut fd_set>();
        (*ev).ev_type = (1 as i32) << 2 as i32;
        (*ev).ev_goal = (spec & ((1 as i32) << 11 as i32) as u64) as i32;
        (*ev).ev_args.SELECT.n = n;
        (*ev).ev_args.SELECT.nfd = nfd;
        (*ev).ev_args.SELECT.rfds = rfds;
        (*ev).ev_args.SELECT.wfds = wfds;
        (*ev).ev_args.SELECT.efds = efds;
    } else if spec & ((1 as i32) << 3 as i32) as u64 != 0 {
        let mut sigs: *mut sigset_t = ap.arg::<*mut sigset_t>();
        let mut sig: *mut i32 = ap.arg::<*mut i32>();
        (*ev).ev_type = (1 as i32) << 3 as i32;
        (*ev).ev_goal = (spec & ((1 as i32) << 11 as i32) as u64) as i32;
        (*ev).ev_args.SIGS.sigs = sigs;
        (*ev).ev_args.SIGS.sig = sig;
    } else if spec & ((1 as i32) << 4 as i32) as u64 != 0 {
        let mut tv: pth_time_t = ap.arg::<pth_time_t>();
        (*ev).ev_type = (1 as i32) << 4 as i32;
        (*ev).ev_goal = (spec & ((1 as i32) << 11 as i32) as u64) as i32;
        (*ev).ev_args.TIME.tv = tv;
    } else if spec & ((1 as i32) << 5 as i32) as u64 != 0 {
        let mut mp: pth_msgport_t = ap.arg::<pth_msgport_t>();
        (*ev).ev_type = (1 as i32) << 5 as i32;
        (*ev).ev_goal = (spec & ((1 as i32) << 11 as i32) as u64) as i32;
        (*ev).ev_args.MSG.mp = mp;
    } else if spec & ((1 as i32) << 6 as i32) as u64 != 0 {
        let mut mutex: *mut pth_mutex_t = ap.arg::<*mut pth_mutex_t>();
        (*ev).ev_type = (1 as i32) << 6 as i32;
        (*ev).ev_goal = (spec & ((1 as i32) << 11 as i32) as u64) as i32;
        (*ev).ev_args.MUTEX.mutex = mutex;
    } else if spec & ((1 as i32) << 7 as i32) as u64 != 0 {
        let mut cond: *mut pth_cond_t = ap.arg::<*mut pth_cond_t>();
        (*ev).ev_type = (1 as i32) << 7 as i32;
        (*ev).ev_goal = (spec & ((1 as i32) << 11 as i32) as u64) as i32;
        (*ev).ev_args.COND.cond = cond;
    } else if spec & ((1 as i32) << 8 as i32) as u64 != 0 {
        let mut tid: pth_t = ap.arg::<pth_t>();
        let mut goal: i32 = 0;
        (*ev).ev_type = (1 as i32) << 8 as i32;
        if spec & ((1 as i32) << 15 as i32) as u64 != 0 {
            goal = pth_state_en::PTH_STATE_NEW as i32;
        } else if spec & ((1 as i32) << 16 as i32) as u64 != 0 {
            goal = pth_state_en::PTH_STATE_READY as i32;
        } else if spec & ((1 as i32) << 17 as i32) as u64 != 0 {
            goal = pth_state_en::PTH_STATE_WAITING as i32;
        } else if spec & ((1 as i32) << 18 as i32) as u64 != 0 {
            goal = pth_state_en::PTH_STATE_DEAD as i32;
        } else {
            goal = pth_state_en::PTH_STATE_READY as i32;
        }
        (*ev).ev_goal = goal;
        (*ev).ev_args.TID.tid = tid;
    } else if spec & ((1 as i32) << 9 as i32) as u64 != 0 {
        (*ev).ev_type = (1 as i32) << 9 as i32;
        (*ev).ev_goal = (spec & ((1 as i32) << 11 as i32) as u64) as i32;
        (*ev).ev_args.FUNC.func = ::core::mem::transmute(
            ap.arg::<*mut unsafe extern "C" fn(*mut libc::c_void) -> i32>(),
        );
        (*ev).ev_args.FUNC.arg = ap.arg::<*mut libc::c_void>();
        (*ev).ev_args.FUNC.tv = ap.arg::<pth_time_t>();
    } else {
        *__errno_location() = 22 as i32;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    return ev;
}
#[no_mangle]
pub unsafe extern "C" fn pth_event_typeof(mut ev: pth_event_t) -> u64 {
    if ev.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32 as u64;
    }
    return ((*ev).ev_type | (*ev).ev_goal) as u64;
}
#[no_mangle]
pub unsafe extern "C" fn pth_event_extract(mut ev: pth_event_t, mut args: ...) -> i32 {
    let mut ap: ::core::ffi::VaListImpl;
    if ev.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    ap = args.clone();
    if (*ev).ev_type & (1 as i32) << 1 as i32 != 0 {
        let mut fd: *mut i32 = ap.arg::<*mut i32>();
        *fd = (*ev).ev_args.FD.fd;
    } else if (*ev).ev_type & (1 as i32) << 3 as i32 != 0 {
        let mut sigs: *mut *mut sigset_t = ap.arg::<*mut *mut sigset_t>();
        let mut sig: *mut *mut i32 = ap.arg::<*mut *mut i32>();
        *sigs = (*ev).ev_args.SIGS.sigs;
        *sig = (*ev).ev_args.SIGS.sig;
    } else if (*ev).ev_type & (1 as i32) << 4 as i32 != 0 {
        let mut tv: *mut pth_time_t = ap.arg::<*mut pth_time_t>();
        *tv = (*ev).ev_args.TIME.tv;
    } else if (*ev).ev_type & (1 as i32) << 5 as i32 != 0 {
        let mut mp: *mut pth_msgport_t = ap.arg::<*mut pth_msgport_t>();
        *mp = (*ev).ev_args.MSG.mp;
    } else if (*ev).ev_type & (1 as i32) << 6 as i32 != 0 {
        let mut mutex: *mut *mut pth_mutex_t = ap.arg::<*mut *mut pth_mutex_t>();
        *mutex = (*ev).ev_args.MUTEX.mutex;
    } else if (*ev).ev_type & (1 as i32) << 7 as i32 != 0 {
        let mut cond: *mut *mut pth_cond_t = ap.arg::<*mut *mut pth_cond_t>();
        *cond = (*ev).ev_args.COND.cond;
    } else if (*ev).ev_type & (1 as i32) << 8 as i32 != 0 {
        let mut tid: *mut pth_t = ap.arg::<*mut pth_t>();
        *tid = (*ev).ev_args.TID.tid;
    } else if (*ev).ev_type & (1 as i32) << 9 as i32 != 0 {
        let mut func: *mut pth_event_func_t = ap.arg::<*mut pth_event_func_t>();
        let mut arg: *mut *mut libc::c_void = ap.arg::<*mut *mut libc::c_void>();
        let mut tv_0: *mut pth_time_t = ap.arg::<*mut pth_time_t>();
        *func = (*ev).ev_args.FUNC.func;
        *arg = (*ev).ev_args.FUNC.arg;
        *tv_0 = (*ev).ev_args.FUNC.tv;
    } else {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_event_concat(
    mut evf: pth_event_t,
    mut args: ...
) -> pth_event_t {
    let mut evc: pth_event_t = 0 as *mut pth_event_st;
    let mut evn: pth_event_t = 0 as *mut pth_event_st;
    let mut evl: pth_event_t = 0 as *mut pth_event_st;
    let mut evt: pth_event_t = 0 as *mut pth_event_st;
    let mut ap: ::core::ffi::VaListImpl;
    if evf.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    ap = args.clone();
    evc = evf;
    evl = (*evc).ev_next;
    loop {
        evn = ap.arg::<pth_event_t>();
        if evn.is_null() {
            break;
        }
        (*evc).ev_next = evn;
        evt = (*evn).ev_prev;
        (*evn).ev_prev = evc;
        evc = evt;
    }
    (*evc).ev_next = evl;
    (*evl).ev_prev = evc;
    return evf;
}
#[no_mangle]
pub unsafe extern "C" fn pth_event_isolate(mut ev: pth_event_t) -> pth_event_t {
    let mut ring: pth_event_t = 0 as *mut pth_event_st;
    if ev.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    ring = 0 as pth_event_t;
    if !((*ev).ev_next == ev && (*ev).ev_prev == ev) {
        ring = (*ev).ev_next;
        (*(*ev).ev_prev).ev_next = (*ev).ev_next;
        (*(*ev).ev_next).ev_prev = (*ev).ev_prev;
        (*ev).ev_prev = ev;
        (*ev).ev_next = ev;
    }
    return ring;
}
#[no_mangle]
pub unsafe extern "C" fn pth_event_status(mut ev: pth_event_t) -> pth_status_t {
    if ev.is_null() {
        *__errno_location() = 22 as i32;
        return pth_status_t::from_libc_c_uint(0 as i32 as u32);
    }
    return (*ev).ev_status;
}
#[no_mangle]
pub unsafe extern "C" fn pth_event_walk(
    mut ev: pth_event_t,
    mut direction: u32,
) -> pth_event_t {
    if ev.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as *mut libc::c_void as pth_event_t;
    }
    loop {
        if direction & ((1 as i32) << 1 as i32) as u32 != 0 {
            ev = (*ev).ev_next;
        } else if direction & ((1 as i32) << 2 as i32) as u32 != 0 {
            ev = (*ev).ev_prev;
        } else {
            *__errno_location() = 22 as i32;
            return 0 as *mut libc::c_void as pth_event_t;
        }
        if !(direction & ((1 as i32) << 11 as i32) as u32 != 0
            && (*ev).ev_status as u32 != pth_status_t::PTH_STATUS_OCCURRED as i32 as u32)
        {
            break;
        }
    }
    return ev;
}
#[no_mangle]
pub unsafe extern "C" fn pth_event_free(mut ev: pth_event_t, mut mode: i32) -> i32 {
    let mut evc: pth_event_t = 0 as *mut pth_event_st;
    let mut evn: pth_event_t = 0 as *mut pth_event_st;
    if ev.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if mode == C2RustUnnamed_9::PTH_FREE_THIS as i32 {
        (*(*ev).ev_prev).ev_next = (*ev).ev_next;
        (*(*ev).ev_next).ev_prev = (*ev).ev_prev;
        free(ev as *mut libc::c_void);
    } else if mode == C2RustUnnamed_9::PTH_FREE_ALL as i32 {
        evc = ev;
        loop {
            evn = (*evc).ev_next;
            free(evc as *mut libc::c_void);
            evc = evn;
            if !(evc != ev) {
                break;
            }
        }
    }
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_wait(mut ev_ring: pth_event_t) -> i32 {
    let mut nonpending: i32 = 0;
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    if ev_ring.is_null() {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    ev = ev_ring;
    loop {
        (*ev).ev_status = pth_status_t::PTH_STATUS_PENDING;
        ev = (*ev).ev_next;
        if !(ev != ev_ring) {
            break;
        }
    }
    (*__pth_current).events = ev_ring;
    (*__pth_current).state = pth_state_en::PTH_STATE_WAITING;
    pth_yield(0 as pth_t);
    pth_cancel_point();
    (*__pth_current).events = 0 as pth_event_t;
    ev = ev_ring;
    nonpending = 0 as i32;
    loop {
        if (*ev).ev_status as u32 != pth_status_t::PTH_STATUS_PENDING as i32 as u32 {
            nonpending += 1;
            nonpending;
        }
        ev = (*ev).ev_next;
        if !(ev != ev_ring) {
            break;
        }
    }
    return nonpending;
}