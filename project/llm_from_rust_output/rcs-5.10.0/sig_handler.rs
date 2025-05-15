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
    pub _sifields: SiginfoFields,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union SiginfoFields {
    pub _pad: [c_int; 28],
    pub _kill: KillFields,
    pub _timer: TimerFields,
    pub _rt: RtFields,
    pub _sigchld: SigchldFields,
    pub _sigfault: SigfaultFields,
    pub _sigpoll: SigpollFields,
    pub _sigsys: SigsysFields,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigsysFields {
    pub _call_addr: *mut c_void,
    pub _syscall: c_int,
    pub _arch: c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigpollFields {
    pub si_band: c_long,
    pub si_fd: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigfaultFields {
    pub si_addr: *mut c_void,
    pub si_addr_lsb: c_short,
    pub _bounds: FaultBounds,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union FaultBounds {
    pub _addr_bnd: AddrBounds,
    pub _pkey: __uint32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AddrBounds {
    pub _lower: *mut c_void,
    pub _upper: *mut c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SigchldFields {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RtFields {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TimerFields {
    pub si_tid: c_int,
    pub si_overrun: c_int,
    pub si_sigval: __sigval_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KillFields {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}

pub type __sighandler_t = Option<extern "C" fn(c_int)>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: SigactionHandler,
    pub sa_mask: __sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<extern "C" fn()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union SigactionHandler {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
}

pub type sa_handler_t = Option<extern "C" fn(c_int)>;

pub fn get_handler(a: &sigaction) -> sa_handler_t {
    unsafe { a.__sigaction_handler.sa_handler }
}