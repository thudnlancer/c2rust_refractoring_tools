use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn __pth_ring_append(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    fn __pth_ring_init(_: *mut pth_ring_t);
    static mut __pth_current: pth_t;
    fn __pth_ring_delete(_: *mut pth_ring_t, _: *mut pth_ringnode_t);
    fn __pth_ring_pop(_: *mut pth_ring_t) -> *mut pth_ringnode_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_message_st {
    pub m_node: pth_ringnode_t,
    pub m_replyport: pth_msgport_t,
    pub m_size: u32,
    pub m_data: *mut libc::c_void,
}
pub type pth_message_t = pth_message_st;
static mut pth_msgport: pth_ring_t = {
    let mut init = pth_ring_st {
        r_hook: 0 as *const pth_ringnode_t as *mut pth_ringnode_t,
        r_nodes: 0,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn pth_msgport_create(mut name: *const i8) -> pth_msgport_t {
    let mut mp: pth_msgport_t = 0 as *mut pth_msgport_st;
    mp = malloc(::core::mem::size_of::<pth_msgport_st>() as u64) as pth_msgport_t;
    if mp.is_null() {
        *__errno_location() = 12 as i32;
        return 0 as *mut libc::c_void as pth_msgport_t;
    }
    (*mp).mp_name = name;
    (*mp).mp_tid = __pth_current;
    __pth_ring_init(&mut (*mp).mp_queue);
    __pth_ring_append(&mut pth_msgport, &mut (*mp).mp_node);
    return mp;
}
#[no_mangle]
pub unsafe extern "C" fn pth_msgport_destroy(mut mp: pth_msgport_t) {
    let mut m: *mut pth_message_t = 0 as *mut pth_message_t;
    if mp.is_null() {
        return;
    }
    loop {
        m = pth_msgport_get(mp);
        if m.is_null() {
            break;
        }
        pth_msgport_reply(m);
    }
    __pth_ring_delete(&mut pth_msgport, &mut (*mp).mp_node);
    free(mp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn pth_msgport_find(mut name: *const i8) -> pth_msgport_t {
    let mut mp: pth_msgport_t = 0 as *mut pth_msgport_st;
    let mut mpf: pth_msgport_t = 0 as *mut pth_msgport_st;
    if name.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as *mut libc::c_void as pth_msgport_t;
    }
    mpf = (if (&mut pth_msgport as *mut pth_ring_t).is_null() {
        0 as *mut pth_ringnode_t
    } else {
        pth_msgport.r_hook
    }) as pth_msgport_t;
    mp = mpf;
    while !mp.is_null() {
        if !((*mp).mp_name).is_null() {
            if strcmp((*mp).mp_name, name) == 0 as i32 {
                break;
            }
        }
        mp = (if (&mut pth_msgport as *mut pth_ring_t).is_null()
            || (mp as *mut pth_ringnode_t).is_null()
        {
            0 as *mut pth_ringnode_t
        } else if (*(mp as *mut pth_ringnode_t)).rn_next == pth_msgport.r_hook {
            0 as *mut pth_ringnode_t
        } else {
            (*(mp as *mut pth_ringnode_t)).rn_next
        }) as pth_msgport_t;
        if !(mp == mpf) {
            continue;
        }
        mp = 0 as pth_msgport_t;
        break;
    }
    return mp;
}
#[no_mangle]
pub unsafe extern "C" fn pth_msgport_pending(mut mp: pth_msgport_t) -> i32 {
    if mp.is_null() {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    return (if (&mut (*mp).mp_queue as *mut pth_ring_t).is_null() {
        -(1 as i32) as u32
    } else {
        (*mp).mp_queue.r_nodes
    }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_msgport_put(
    mut mp: pth_msgport_t,
    mut m: *mut pth_message_t,
) -> i32 {
    if mp.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    __pth_ring_append(&mut (*mp).mp_queue, m as *mut pth_ringnode_t);
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_msgport_get(mut mp: pth_msgport_t) -> *mut pth_message_t {
    let mut m: *mut pth_message_t = 0 as *mut pth_message_t;
    if mp.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as *mut libc::c_void as *mut pth_message_t;
    }
    m = __pth_ring_pop(&mut (*mp).mp_queue) as *mut pth_message_t;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn pth_msgport_reply(mut m: *mut pth_message_t) -> i32 {
    if m.is_null() {
        *__errno_location() = 22 as i32;
        return 0 as i32;
    }
    return pth_msgport_put((*m).m_replyport, m);
}