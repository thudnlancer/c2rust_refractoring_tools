use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    static mut __pth_current: pth_t;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
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
    pub uc_flags: libc::c_ulong,
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
    pub q_prio: libc::c_int,
    pub prio: libc::c_int,
    pub name: [libc::c_char; 40],
    pub dispatches: libc::c_int,
    pub state: pth_state_t,
    pub spawned: pth_time_t,
    pub lastran: pth_time_t,
    pub running: pth_time_t,
    pub events: pth_event_t,
    pub sigpending: sigset_t,
    pub sigpendcnt: libc::c_int,
    pub mctx: pth_mctx_t,
    pub stack: *mut libc::c_char,
    pub stacksize: libc::c_uint,
    pub stackguard: *mut libc::c_long,
    pub stackloan: libc::c_int,
    pub start_func: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub start_arg: *mut libc::c_void,
    pub joinable: libc::c_int,
    pub join_arg: *mut libc::c_void,
    pub data_value: *mut *const libc::c_void,
    pub data_count: libc::c_int,
    pub cancelreq: libc::c_int,
    pub cancelstate: libc::c_uint,
    pub cleanups: *mut pth_cleanup_t,
    pub mutexring: pth_ring_t,
}
pub type pth_ring_t = pth_ring_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_ring_st {
    pub r_hook: *mut pth_ringnode_t,
    pub r_nodes: libc::c_uint,
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
    pub func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
pub type pth_mctx_t = pth_mctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_mctx_st {
    pub uc: ucontext_t,
    pub restored: libc::c_int,
    pub sigs: sigset_t,
    pub error: libc::c_int,
}
pub type pth_event_t = *mut pth_event_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_event_st {
    pub ev_next: *mut pth_event_st,
    pub ev_prev: *mut pth_event_st,
    pub ev_status: pth_status_t,
    pub ev_type: libc::c_int,
    pub ev_goal: libc::c_int,
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
pub type pth_event_func_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
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
    pub cn_state: libc::c_ulong,
    pub cn_waiters: libc::c_uint,
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
    pub mx_state: libc::c_int,
    pub mx_owner: pth_t,
    pub mx_count: libc::c_ulong,
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
    pub mp_name: *const libc::c_char,
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
    pub sig: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub n: *mut libc::c_int,
    pub nfd: libc::c_int,
    pub rfds: *mut fd_set,
    pub wfds: *mut fd_set,
    pub efds: *mut fd_set,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub fd: libc::c_int,
}
pub type pth_status_t = libc::c_uint;
pub const PTH_STATUS_FAILED: pth_status_t = 2;
pub const PTH_STATUS_OCCURRED: pth_status_t = 1;
pub const PTH_STATUS_PENDING: pth_status_t = 0;
pub type pth_state_t = pth_state_en;
pub type pth_state_en = libc::c_uint;
pub const PTH_STATE_DEAD: pth_state_en = 4;
pub const PTH_STATE_WAITING: pth_state_en = 3;
pub const PTH_STATE_READY: pth_state_en = 2;
pub const PTH_STATE_NEW: pth_state_en = 1;
pub const PTH_STATE_SCHEDULER: pth_state_en = 0;
pub type pth_key_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_keytab_st {
    pub used: libc::c_int,
    pub destructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
static mut pth_keytab: [pth_keytab_st; 256] = [pth_keytab_st {
    used: 0,
    destructor: None,
}; 256];
#[no_mangle]
pub unsafe extern "C" fn pth_key_create(
    mut key: *mut pth_key_t,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> libc::c_int {
    if key.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    *key = 0 as libc::c_int;
    while *key < 256 as libc::c_int {
        if pth_keytab[*key as usize].used == 0 as libc::c_int {
            pth_keytab[*key as usize].used = (0 as libc::c_int == 0) as libc::c_int;
            pth_keytab[*key as usize].destructor = func;
            return (0 as libc::c_int == 0) as libc::c_int;
        }
        *key += 1;
        *key;
    }
    *__errno_location() = 11 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pth_key_delete(mut key: pth_key_t) -> libc::c_int {
    if key < 0 as libc::c_int || key >= 256 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if pth_keytab[key as usize].used == 0 {
        *__errno_location() = 2 as libc::c_int;
        return 0 as libc::c_int;
    }
    pth_keytab[key as usize].used = 0 as libc::c_int;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pth_key_setdata(
    mut key: pth_key_t,
    mut value: *const libc::c_void,
) -> libc::c_int {
    if key < 0 as libc::c_int || key >= 256 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if pth_keytab[key as usize].used == 0 {
        *__errno_location() = 2 as libc::c_int;
        return 0 as libc::c_int;
    }
    if ((*__pth_current).data_value).is_null() {
        (*__pth_current)
            .data_value = calloc(
            1 as libc::c_int as libc::c_ulong,
            (::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(256 as libc::c_int as libc::c_ulong),
        ) as *mut *const libc::c_void;
        if ((*__pth_current).data_value).is_null() {
            *__errno_location() = 12 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    if *((*__pth_current).data_value).offset(key as isize) == 0 as *mut libc::c_void {
        if value != 0 as *mut libc::c_void {
            (*__pth_current).data_count += 1;
            (*__pth_current).data_count;
        }
    } else if value == 0 as *mut libc::c_void {
        (*__pth_current).data_count -= 1;
        (*__pth_current).data_count;
    }
    let ref mut fresh0 = *((*__pth_current).data_value).offset(key as isize);
    *fresh0 = value;
    return (0 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pth_key_getdata(mut key: pth_key_t) -> *mut libc::c_void {
    if key < 0 as libc::c_int || key >= 256 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    if pth_keytab[key as usize].used == 0 {
        *__errno_location() = 2 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    if ((*__pth_current).data_value).is_null() {
        return 0 as *mut libc::c_void;
    }
    return *((*__pth_current).data_value).offset(key as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_key_destroydata(mut t: pth_t) {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut key: libc::c_int = 0;
    let mut itr: libc::c_int = 0;
    let mut destructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = None;
    if t.is_null() {
        return;
    }
    if ((*t).data_value).is_null() {
        return;
    }
    itr = 0 as libc::c_int;
    while itr < 4 as libc::c_int {
        key = 0 as libc::c_int;
        while key < 256 as libc::c_int {
            if (*t).data_count > 0 as libc::c_int {
                destructor = None;
                data = 0 as *mut libc::c_void;
                if pth_keytab[key as usize].used != 0 {
                    if *((*t).data_value).offset(key as isize) != 0 as *mut libc::c_void
                    {
                        data = *((*t).data_value).offset(key as isize)
                            as *mut libc::c_void;
                        let ref mut fresh1 = *((*t).data_value).offset(key as isize);
                        *fresh1 = 0 as *const libc::c_void;
                        (*t).data_count -= 1;
                        (*t).data_count;
                        destructor = pth_keytab[key as usize].destructor;
                    }
                }
                if destructor.is_some() {
                    destructor.expect("non-null function pointer")(data);
                }
            }
            if (*t).data_count == 0 as libc::c_int {
                break;
            }
            key += 1;
            key;
        }
        if (*t).data_count == 0 as libc::c_int {
            break;
        }
        itr += 1;
        itr;
    }
    free((*t).data_value as *mut libc::c_void);
    (*t).data_value = 0 as *mut *const libc::c_void;
}
