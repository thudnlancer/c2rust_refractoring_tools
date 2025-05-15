use std::os::raw::{c_int, c_uint, c_long, c_ulong, c_void};

pub type __uint32_t = c_uint;
pub type __uid_t = c_uint;
pub type __pid_t = c_int;
pub type __clock_t = c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

pub type __sigval_t = sigval;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub __pad0: c_int,
    pub _sifields: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [c_int; 28],
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
    pub _call_addr: *mut c_void,
    pub _syscall: c_int,
    pub _arch: c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: c_long,
    pub si_fd: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut c_void,
    pub si_addr_lsb: c_short,
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
    pub _lower: *mut c_void,
    pub _upper: *mut c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: c_int,
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
    pub si_tid: c_int,
    pub si_overrun: c_int,
    pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}

pub type __sighandler_t = Option<extern "C" fn(c_int)>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<extern "C" fn()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

pub type sa_handler_t = Option<extern "C" fn(c_int)>;

#[no_mangle]
pub extern "C" fn get_handler(a: &sigaction) -> sa_handler_t {
    unsafe { a.__sigaction_handler.sa_handler }
}