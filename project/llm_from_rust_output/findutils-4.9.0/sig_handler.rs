use std::os::raw::{c_int, c_uint, c_long, c_ulong, c_void};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigset {
    pub val: [c_ulong; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Siginfo {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub pad0: c_int,
    pub fields: SiginfoFields,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SiginfoFields {
    pub pad: [c_int; 28],
    pub kill: Sigkill,
    pub timer: Sigtimer,
    pub rt: Sigrt,
    pub sigchld: Sigchld,
    pub sigfault: Sigfault,
    pub sigpoll: Sigpoll,
    pub sigsys: Sigsys,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigsys {
    pub call_addr: *mut c_void,
    pub syscall: c_int,
    pub arch: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigpoll {
    pub si_band: c_long,
    pub si_fd: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigfault {
    pub si_addr: *mut c_void,
    pub si_addr_lsb: i16,
    pub bounds: SigfaultBounds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SigfaultBounds {
    pub addr_bnd: SigfaultAddr,
    pub pkey: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SigfaultAddr {
    pub lower: *mut c_void,
    pub upper: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigchld {
    pub si_pid: c_int,
    pub si_uid: c_uint,
    pub si_status: c_int,
    pub si_utime: c_long,
    pub si_stime: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigrt {
    pub si_pid: c_int,
    pub si_uid: c_uint,
    pub si_sigval: Sigval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigtimer {
    pub si_tid: c_int,
    pub si_overrun: c_int,
    pub si_sigval: Sigval,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigkill {
    pub si_pid: c_int,
    pub si_uid: c_uint,
}

pub type Sighandler = Option<extern "C" fn(c_int)>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sigaction {
    pub handler: SigactionHandler,
    pub sa_mask: Sigset,
    pub sa_flags: c_int,
    pub sa_restorer: Option<extern "C" fn()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SigactionHandler {
    pub sa_handler: Sighandler,
    pub sa_sigaction: Option<extern "C" fn(c_int, *mut Siginfo, *mut c_void)>,
}

pub type SaHandler = Option<extern "C" fn(c_int)>;

pub fn get_handler(a: &Sigaction) -> SaHandler {
    unsafe { a.handler.sa_handler }
}