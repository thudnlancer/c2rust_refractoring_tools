#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn __errno_location() -> *mut i32;
    fn pth_yield(_: pth_t) -> i32;
    fn pth_wait(_: pth_event_t) -> i32;
    fn pth_cancel_state(_: i32, _: *mut i32);
    fn pth_event(_: u64, _: ...) -> pth_event_t;
    fn pth_event_concat(_: pth_event_t, _: ...) -> pth_event_t;
    fn pth_event_isolate(_: pth_event_t) -> pth_event_t;
    fn pth_event_status(_: pth_event_t) -> pth_status_t;
    fn pth_cleanup_push(
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    ) -> i32;
    fn pth_cleanup_pop(_: i32) -> i32;
    static mut __pth_current: pth_t;
    fn __pth_ring_append(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    fn __pth_ring_delete(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    static mut __pth_errno_flag: i32;
    static mut __pth_errno_storage: i32;
}
pub type size_t = u64;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
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
pub type pth_key_t = i32;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_9 {
    PTH_RWLOCK_RD,
    PTH_RWLOCK_RW,
}
impl C2RustUnnamed_9 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_9::PTH_RWLOCK_RD => 0,
            C2RustUnnamed_9::PTH_RWLOCK_RW => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_9 {
        match value {
            0 => C2RustUnnamed_9::PTH_RWLOCK_RD,
            1 => C2RustUnnamed_9::PTH_RWLOCK_RW,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_rwlock_st {
    pub rw_state: i32,
    pub rw_mode: u32,
    pub rw_readers: u64,
    pub rw_mutex_rd: pth_mutex_t,
    pub rw_mutex_rw: pth_mutex_t,
}
pub type pth_rwlock_t = pth_rwlock_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_barrier_st {
    pub br_state: u64,
    pub br_threshold: i32,
    pub br_count: i32,
    pub br_cycle: i32,
    pub br_cond: pth_cond_t,
    pub br_mutex: pth_mutex_t,
}
pub type pth_barrier_t = pth_barrier_st;
#[no_mangle]
pub unsafe extern "C" fn pth_mutex_init(mut mutex: *mut pth_mutex_t) -> i32 {
    if mutex.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    (*mutex).mx_state = (1 as i32) << 0 as i32;
    (*mutex).mx_owner = 0 as pth_t;
    (*mutex).mx_count = 0 as i32 as u64;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_mutex_acquire(
    mut mutex: *mut pth_mutex_t,
    mut tryonly: i32,
    mut ev_extra: pth_event_t,
) -> i32 {
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    if mutex.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*mutex).mx_state & (1 as i32) << 0 as i32 == 0 {
        *__errno_location() = 35 as i32;
        return 0 as i32;
    }
    if (*mutex).mx_state & (1 as i32) << 1 as i32 == 0 {
        (*mutex).mx_state |= (1 as i32) << 1 as i32;
        (*mutex).mx_owner = __pth_current;
        (*mutex).mx_count = 1 as i32 as u64;
        __pth_ring_append(&mut (*__pth_current).mutexring, &mut (*mutex).mx_node);
        return (0 as i32 == 0) as i32;
    }
    if (*mutex).mx_count >= 1 as i32 as u64 && (*mutex).mx_owner == __pth_current {
        (*mutex).mx_count = ((*mutex).mx_count).wrapping_add(1);
        (*mutex).mx_count;
        return (0 as i32 == 0) as i32;
    }
    if tryonly != 0 {
        *__errno_location() = 16 as i32;
        return 0 as i32;
    }
    loop {
        ev = pth_event(
            ((1 as i32) << 6 as i32 | (1 as i32) << 22 as i32) as u64,
            &mut ev_key as *mut pth_key_t,
            mutex,
        );
        if !ev_extra.is_null() {
            pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
        }
        pth_wait(ev);
        if !ev_extra.is_null() {
            pth_event_isolate(ev);
            if pth_event_status(ev) as u32
                == pth_status_t::PTH_STATUS_PENDING as i32 as u32
            {
                *__errno_location() = 4 as i32;
                return 0 as i32;
            }
        }
        if (*mutex).mx_state & (1 as i32) << 1 as i32 == 0 {
            break;
        }
    }
    (*mutex).mx_state |= (1 as i32) << 1 as i32;
    (*mutex).mx_owner = __pth_current;
    (*mutex).mx_count = 1 as i32 as u64;
    __pth_ring_append(&mut (*__pth_current).mutexring, &mut (*mutex).mx_node);
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_mutex_release(mut mutex: *mut pth_mutex_t) -> i32 {
    if mutex.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*mutex).mx_state & (1 as i32) << 0 as i32 == 0 {
        *__errno_location() = 35 as i32;
        return 0 as i32;
    }
    if (*mutex).mx_state & (1 as i32) << 1 as i32 == 0 {
        *__errno_location() = 35 as i32;
        return 0 as i32;
    }
    if (*mutex).mx_owner != __pth_current {
        *__errno_location() = 13 as i32;
        return 0 as i32;
    }
    (*mutex).mx_count = ((*mutex).mx_count).wrapping_sub(1);
    (*mutex).mx_count;
    if (*mutex).mx_count <= 0 as i32 as u64 {
        (*mutex).mx_state &= !((1 as i32) << 1 as i32);
        (*mutex).mx_owner = 0 as pth_t;
        (*mutex).mx_count = 0 as i32 as u64;
        __pth_ring_delete(&mut (*__pth_current).mutexring, &mut (*mutex).mx_node);
    }
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_mutex_releaseall(mut thread: pth_t) {
    let mut rn: *mut pth_ringnode_t = 0 as *mut pth_ringnode_t;
    let mut rnf: *mut pth_ringnode_t = 0 as *mut pth_ringnode_t;
    if thread.is_null() {
        return;
    }
    rnf = if (&mut (*thread).mutexring as *mut pth_ring_t).is_null() {
        0 as *mut pth_ringnode_t
    } else {
        (*thread).mutexring.r_hook
    };
    rn = rnf;
    while !rn.is_null() {
        pth_mutex_release(rn as *mut pth_mutex_t);
        rn = if (&mut (*thread).mutexring as *mut pth_ring_t).is_null() || rn.is_null() {
            0 as *mut pth_ringnode_t
        } else if (*rn).rn_next == (*thread).mutexring.r_hook {
            0 as *mut pth_ringnode_t
        } else {
            (*rn).rn_next
        };
        if rn == rnf {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn pth_rwlock_init(mut rwlock: *mut pth_rwlock_t) -> i32 {
    if rwlock.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    (*rwlock).rw_state = (1 as i32) << 0 as i32;
    (*rwlock).rw_readers = 0 as i32 as u64;
    pth_mutex_init(&mut (*rwlock).rw_mutex_rd);
    pth_mutex_init(&mut (*rwlock).rw_mutex_rw);
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_rwlock_acquire(
    mut rwlock: *mut pth_rwlock_t,
    mut op: i32,
    mut tryonly: i32,
    mut ev_extra: pth_event_t,
) -> i32 {
    if rwlock.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*rwlock).rw_state & (1 as i32) << 0 as i32 == 0 {
        *__errno_location() = 35 as i32;
        return 0 as i32;
    }
    if op == C2RustUnnamed_9::PTH_RWLOCK_RW as i32 {
        if pth_mutex_acquire(&mut (*rwlock).rw_mutex_rw, tryonly, ev_extra) == 0 {
            return 0 as i32;
        }
        (*rwlock).rw_mode = C2RustUnnamed_9::PTH_RWLOCK_RW as i32 as u32;
    } else {
        if pth_mutex_acquire(&mut (*rwlock).rw_mutex_rd, tryonly, ev_extra) == 0 {
            return 0 as i32;
        }
        (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_add(1);
        (*rwlock).rw_readers;
        if (*rwlock).rw_readers == 1 as i32 as u64 {
            if pth_mutex_acquire(&mut (*rwlock).rw_mutex_rw, tryonly, ev_extra) == 0 {
                (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_sub(1);
                (*rwlock).rw_readers;
                __pth_errno_storage = *__errno_location();
                __pth_errno_flag = (0 as i32 == 0) as i32;
                while __pth_errno_flag != 0 {
                    pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
                    *__errno_location() = __pth_errno_storage;
                    __pth_errno_flag = 0 as i32;
                }
                return 0 as i32;
            }
        }
        (*rwlock).rw_mode = C2RustUnnamed_9::PTH_RWLOCK_RD as i32 as u32;
        pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
    }
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_rwlock_release(mut rwlock: *mut pth_rwlock_t) -> i32 {
    if rwlock.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*rwlock).rw_state & (1 as i32) << 0 as i32 == 0 {
        *__errno_location() = 35 as i32;
        return 0 as i32;
    }
    if (*rwlock).rw_mode == C2RustUnnamed_9::PTH_RWLOCK_RW as i32 as u32 {
        if pth_mutex_release(&mut (*rwlock).rw_mutex_rw) == 0 {
            return 0 as i32;
        }
    } else {
        if pth_mutex_acquire(&mut (*rwlock).rw_mutex_rd, 0 as i32, 0 as pth_event_t) == 0
        {
            return 0 as i32;
        }
        (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_sub(1);
        (*rwlock).rw_readers;
        if (*rwlock).rw_readers == 0 as i32 as u64 {
            if pth_mutex_release(&mut (*rwlock).rw_mutex_rw) == 0 {
                (*rwlock).rw_readers = ((*rwlock).rw_readers).wrapping_add(1);
                (*rwlock).rw_readers;
                __pth_errno_storage = *__errno_location();
                __pth_errno_flag = (0 as i32 == 0) as i32;
                while __pth_errno_flag != 0 {
                    pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
                    *__errno_location() = __pth_errno_storage;
                    __pth_errno_flag = 0 as i32;
                }
                return 0 as i32;
            }
        }
        (*rwlock).rw_mode = C2RustUnnamed_9::PTH_RWLOCK_RD as i32 as u32;
        pth_mutex_release(&mut (*rwlock).rw_mutex_rd);
    }
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_cond_init(mut cond: *mut pth_cond_t) -> i32 {
    if cond.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    (*cond).cn_state = ((1 as i32) << 0 as i32) as u64;
    (*cond).cn_waiters = 0 as i32 as u32;
    return (0 as i32 == 0) as i32;
}
unsafe extern "C" fn pth_cond_cleanup_handler(mut _cleanvec: *mut libc::c_void) {
    let mut mutex: *mut pth_mutex_t = *(_cleanvec as *mut *mut libc::c_void)
        .offset(0 as i32 as isize) as *mut pth_mutex_t;
    let mut cond: *mut pth_cond_t = *(_cleanvec as *mut *mut libc::c_void)
        .offset(1 as i32 as isize) as *mut pth_cond_t;
    pth_mutex_acquire(mutex, 0 as i32, 0 as pth_event_t);
    (*cond).cn_waiters = ((*cond).cn_waiters).wrapping_sub(1);
    (*cond).cn_waiters;
}
#[no_mangle]
pub unsafe extern "C" fn pth_cond_await(
    mut cond: *mut pth_cond_t,
    mut mutex: *mut pth_mutex_t,
    mut ev_extra: pth_event_t,
) -> i32 {
    static mut ev_key: pth_key_t = -(1 as i32);
    let mut cleanvec: [*mut libc::c_void; 2] = [0 as *mut libc::c_void; 2];
    let mut ev: pth_event_t = 0 as *mut pth_event_st;
    if cond.is_null() || mutex.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*cond).cn_state & ((1 as i32) << 0 as i32) as u64 == 0 {
        *__errno_location() = 35 as i32;
        return 0 as i32;
    }
    if (*cond).cn_state & ((1 as i32) << 1 as i32) as u64 != 0
        && (*cond).cn_state & ((1 as i32) << 2 as i32) as u64 == 0
    {
        (*cond).cn_state &= !((1 as i32) << 1 as i32) as u64;
        (*cond).cn_state &= !((1 as i32) << 2 as i32) as u64;
        (*cond).cn_state &= !((1 as i32) << 3 as i32) as u64;
        return (0 as i32 == 0) as i32;
    }
    (*cond).cn_waiters = ((*cond).cn_waiters).wrapping_add(1);
    (*cond).cn_waiters;
    pth_mutex_release(mutex);
    ev = pth_event(
        ((1 as i32) << 7 as i32 | (1 as i32) << 22 as i32) as u64,
        &mut ev_key as *mut pth_key_t,
        cond,
    );
    if !ev_extra.is_null() {
        pth_event_concat(ev, ev_extra, 0 as *mut libc::c_void);
    }
    cleanvec[0 as i32 as usize] = mutex as *mut libc::c_void;
    cleanvec[1 as i32 as usize] = cond as *mut libc::c_void;
    pth_cleanup_push(
        Some(pth_cond_cleanup_handler as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        cleanvec.as_mut_ptr() as *mut libc::c_void,
    );
    pth_wait(ev);
    pth_cleanup_pop(0 as i32);
    if !ev_extra.is_null() {
        pth_event_isolate(ev);
    }
    pth_mutex_acquire(mutex, 0 as i32, 0 as pth_event_t);
    (*cond).cn_waiters = ((*cond).cn_waiters).wrapping_sub(1);
    (*cond).cn_waiters;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_cond_notify(
    mut cond: *mut pth_cond_t,
    mut broadcast: i32,
) -> i32 {
    if cond.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*cond).cn_state & ((1 as i32) << 0 as i32) as u64 == 0 {
        *__errno_location() = 35 as i32;
        return 0 as i32;
    }
    if (*cond).cn_waiters > 0 as i32 as u32 {
        (*cond).cn_state |= ((1 as i32) << 1 as i32) as u64;
        if broadcast != 0 {
            (*cond).cn_state |= ((1 as i32) << 2 as i32) as u64;
        } else {
            (*cond).cn_state &= !((1 as i32) << 2 as i32) as u64;
        }
        (*cond).cn_state &= !((1 as i32) << 3 as i32) as u64;
        pth_yield(0 as pth_t);
    }
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_barrier_init(
    mut barrier: *mut pth_barrier_t,
    mut threshold: i32,
) -> i32 {
    if barrier.is_null() || threshold <= 0 as i32 {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if pth_mutex_init(&mut (*barrier).br_mutex) == 0 {
        return 0 as i32;
    }
    if pth_cond_init(&mut (*barrier).br_cond) == 0 {
        return 0 as i32;
    }
    (*barrier).br_state = ((1 as i32) << 0 as i32) as u64;
    (*barrier).br_threshold = threshold;
    (*barrier).br_count = threshold;
    (*barrier).br_cycle = 0 as i32;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_barrier_reach(mut barrier: *mut pth_barrier_t) -> i32 {
    let mut cancel: i32 = 0;
    let mut cycle: i32 = 0;
    let mut rv: i32 = 0;
    if barrier.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if (*barrier).br_state & ((1 as i32) << 0 as i32) as u64 == 0 {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    if pth_mutex_acquire(&mut (*barrier).br_mutex, 0 as i32, 0 as pth_event_t) == 0 {
        return 0 as i32;
    }
    cycle = (*barrier).br_cycle;
    (*barrier).br_count -= 1;
    if (*barrier).br_count == 0 as i32 {
        (*barrier).br_cycle = ((*barrier).br_cycle == 0) as i32;
        (*barrier).br_count = (*barrier).br_threshold;
        rv = pth_cond_notify(&mut (*barrier).br_cond, (0 as i32 == 0) as i32);
        if rv != 0 {
            rv = -(2 as i32);
        }
    } else {
        pth_cancel_state((1 as i32) << 1 as i32, &mut cancel);
        if (*barrier).br_threshold == (*barrier).br_count {
            rv = -(1 as i32);
        } else {
            rv = (0 as i32 == 0) as i32;
        }
        while cycle == (*barrier).br_cycle {
            rv = pth_cond_await(
                &mut (*barrier).br_cond,
                &mut (*barrier).br_mutex,
                0 as pth_event_t,
            );
            if rv == 0 {
                break;
            }
        }
        pth_cancel_state(cancel, 0 as *mut i32);
    }
    pth_mutex_release(&mut (*barrier).br_mutex);
    return rv;
}