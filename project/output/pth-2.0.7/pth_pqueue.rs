use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_pqueue_st {
    pub q_head: pth_t,
    pub q_num: i32,
}
pub type pth_pqueue_t = pth_pqueue_st;
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_init(mut q: *mut pth_pqueue_t) {
    if !q.is_null() {
        (*q).q_head = 0 as pth_t;
        (*q).q_num = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_insert(
    mut q: *mut pth_pqueue_t,
    mut prio: i32,
    mut t: pth_t,
) {
    let mut c: pth_t = 0 as *mut pth_st;
    let mut p: i32 = 0;
    if q.is_null() {
        return;
    }
    if ((*q).q_head).is_null() || (*q).q_num == 0 as i32 {
        (*t).q_prev = t;
        (*t).q_next = t;
        (*t).q_prio = prio;
        (*q).q_head = t;
    } else if (*(*q).q_head).q_prio < prio {
        (*t).q_prev = (*(*q).q_head).q_prev;
        (*t).q_next = (*q).q_head;
        (*(*t).q_prev).q_next = t;
        (*(*t).q_next).q_prev = t;
        (*t).q_prio = prio;
        (*(*t).q_next).q_prio = prio - (*(*t).q_next).q_prio;
        (*q).q_head = t;
    } else {
        c = (*q).q_head;
        p = (*c).q_prio;
        while p - (*(*c).q_next).q_prio >= prio && (*c).q_next != (*q).q_head {
            c = (*c).q_next;
            p -= (*c).q_prio;
        }
        (*t).q_prev = c;
        (*t).q_next = (*c).q_next;
        (*(*t).q_prev).q_next = t;
        (*(*t).q_next).q_prev = t;
        (*t).q_prio = p - prio;
        if (*t).q_next != (*q).q_head {
            (*(*t).q_next).q_prio -= (*t).q_prio;
        }
    }
    (*q).q_num += 1;
    (*q).q_num;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_delmax(mut q: *mut pth_pqueue_t) -> pth_t {
    let mut t: pth_t = 0 as *mut pth_st;
    if q.is_null() {
        return 0 as pth_t;
    }
    if ((*q).q_head).is_null() {
        t = 0 as pth_t;
    } else if (*(*q).q_head).q_next == (*q).q_head {
        t = (*q).q_head;
        (*t).q_next = 0 as pth_t;
        (*t).q_prev = 0 as pth_t;
        (*t).q_prio = 0 as i32;
        (*q).q_head = 0 as pth_t;
        (*q).q_num = 0 as i32;
    } else {
        t = (*q).q_head;
        (*(*t).q_prev).q_next = (*t).q_next;
        (*(*t).q_next).q_prev = (*t).q_prev;
        (*(*t).q_next).q_prio = (*t).q_prio - (*(*t).q_next).q_prio;
        (*t).q_prio = 0 as i32;
        (*q).q_head = (*t).q_next;
        (*q).q_num -= 1;
        (*q).q_num;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_delete(mut q: *mut pth_pqueue_t, mut t: pth_t) {
    if q.is_null() {
        return;
    }
    if ((*q).q_head).is_null() {
        return
    } else if (*q).q_head == t {
        if (*t).q_next == t {
            (*t).q_next = 0 as pth_t;
            (*t).q_prev = 0 as pth_t;
            (*t).q_prio = 0 as i32;
            (*q).q_head = 0 as pth_t;
            (*q).q_num = 0 as i32;
        } else {
            (*(*t).q_prev).q_next = (*t).q_next;
            (*(*t).q_next).q_prev = (*t).q_prev;
            (*(*t).q_next).q_prio = (*t).q_prio - (*(*t).q_next).q_prio;
            (*t).q_prio = 0 as i32;
            (*q).q_head = (*t).q_next;
            (*q).q_num -= 1;
            (*q).q_num;
        }
    } else {
        (*(*t).q_prev).q_next = (*t).q_next;
        (*(*t).q_next).q_prev = (*t).q_prev;
        if (*t).q_next != (*q).q_head {
            (*(*t).q_next).q_prio += (*t).q_prio;
        }
        (*t).q_prio = 0 as i32;
        (*q).q_num -= 1;
        (*q).q_num;
    };
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_favorite(
    mut q: *mut pth_pqueue_t,
    mut t: pth_t,
) -> i32 {
    if q.is_null() {
        return 0 as i32;
    }
    if ((*q).q_head).is_null() || (*q).q_num == 0 as i32 {
        return 0 as i32;
    }
    if (*q).q_num == 1 as i32 {
        return (0 as i32 == 0) as i32;
    }
    __pth_pqueue_delete(q, t);
    __pth_pqueue_insert(
        q,
        if !((*q).q_head).is_null() {
            (*(*q).q_head).q_prio + 1 as i32
        } else {
            5 as i32
        },
        t,
    );
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_increase(mut q: *mut pth_pqueue_t) {
    if q.is_null() {
        return;
    }
    if ((*q).q_head).is_null() {
        return;
    }
    (*(*q).q_head).q_prio += 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_tail(mut q: *mut pth_pqueue_t) -> pth_t {
    if q.is_null() {
        return 0 as pth_t;
    }
    if ((*q).q_head).is_null() {
        return 0 as pth_t;
    }
    return (*(*q).q_head).q_prev;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_walk(
    mut q: *mut pth_pqueue_t,
    mut t: pth_t,
    mut direction: i32,
) -> pth_t {
    let mut tn: pth_t = 0 as *mut pth_st;
    if q.is_null() || t.is_null() {
        return 0 as pth_t;
    }
    tn = 0 as pth_t;
    if direction == (1 as i32) << 2 as i32 {
        if t != (*q).q_head {
            tn = (*t).q_prev;
        }
    } else if direction == (1 as i32) << 1 as i32 {
        tn = (*t).q_next;
        if tn == (*q).q_head {
            tn = 0 as pth_t;
        }
    }
    return tn;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_pqueue_contains(
    mut q: *mut pth_pqueue_t,
    mut t: pth_t,
) -> i32 {
    let mut tc: pth_t = 0 as *mut pth_st;
    let mut found: i32 = 0;
    found = 0 as i32;
    tc = if q.is_null() { 0 as pth_t } else { (*q).q_head };
    while !tc.is_null() {
        if tc == t {
            found = (0 as i32 == 0) as i32;
            break;
        } else {
            tc = __pth_pqueue_walk(q, tc, (1 as i32) << 1 as i32);
        }
    }
    return found;
}