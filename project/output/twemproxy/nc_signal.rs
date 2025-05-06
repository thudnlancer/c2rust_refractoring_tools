#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn exit(_: i32) -> !;
    fn raise(__sig: i32) -> i32;
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn log_level_up();
    fn log_level_down();
    fn log_stacktrace();
    fn log_reopen();
    fn log_loggable(level: i32) -> i32;
    fn _log(file: *const i8, line: i32, panic: i32, fmt: *const i8, _: ...);
    fn _log_safe(fmt: *const i8, _: ...);
}
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __pid_t = i32;
pub type __clock_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
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
pub type rstatus_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct signal {
    pub signo: i32,
    pub signame: *mut i8,
    pub flags: i32,
    pub handler: Option<unsafe extern "C" fn(i32) -> ()>,
}
static mut signals: [signal; 9] = [signal {
    signo: 0,
    signame: 0 as *mut i8,
    flags: 0,
    handler: None,
}; 9];
#[no_mangle]
pub unsafe extern "C" fn signal_init() -> rstatus_t {
    let mut sig: *const signal = 0 as *const signal;
    sig = signals.as_ptr();
    while (*sig).signo != 0 as i32 {
        let mut status: rstatus_t = 0;
        let mut sa: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: None,
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &mut sa as *mut sigaction as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<sigaction>() as u64,
        );
        sa.__sigaction_handler.sa_handler = (*sig).handler;
        sa.sa_flags = (*sig).flags;
        sigemptyset(&mut sa.sa_mask);
        status = sigaction((*sig).signo, &mut sa, 0 as *mut sigaction);
        if status < 0 as i32 {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc_signal.c\0" as *const u8 as *const i8,
                    53 as i32,
                    0 as i32,
                    b"sigaction(%s) failed: %s\0" as *const u8 as *const i8,
                    (*sig).signame,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as i32);
        }
        sig = sig.offset(1);
        sig;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn signal_deinit() {}
#[no_mangle]
pub unsafe extern "C" fn signal_handler(mut signo: i32) {
    let mut sig: *const signal = 0 as *const signal;
    let mut action: Option<unsafe extern "C" fn() -> ()> = None;
    let mut actionstr: *mut i8 = 0 as *mut i8;
    let mut done: bool = false;
    sig = signals.as_ptr();
    while (*sig).signo != 0 as i32 {
        if (*sig).signo == signo {
            break;
        }
        sig = sig.offset(1);
        sig;
    }
    actionstr = b"\0" as *const u8 as *const i8 as *mut i8;
    action = None;
    done = 0 as i32 != 0;
    match signo {
        21 => {
            actionstr = b", up logging level\0" as *const u8 as *const i8 as *mut i8;
            action = Some(log_level_up as unsafe extern "C" fn() -> ());
        }
        22 => {
            actionstr = b", down logging level\0" as *const u8 as *const i8 as *mut i8;
            action = Some(log_level_down as unsafe extern "C" fn() -> ());
        }
        1 => {
            actionstr = b", reopening log file\0" as *const u8 as *const i8 as *mut i8;
            action = Some(log_reopen as unsafe extern "C" fn() -> ());
        }
        2 => {
            done = 1 as i32 != 0;
            actionstr = b", exiting\0" as *const u8 as *const i8 as *mut i8;
        }
        11 => {
            log_stacktrace();
            actionstr = b", core dumping\0" as *const u8 as *const i8 as *mut i8;
            raise(11 as i32);
        }
        10 | 12 | _ => {}
    }
    _log_safe(
        b"signal %d (%s) received%s\0" as *const u8 as *const i8,
        signo,
        (*sig).signame,
        actionstr,
    );
    if action.is_some() {
        action.expect("non-null function pointer")();
    }
    if done {
        exit(1 as i32);
    }
}
unsafe extern "C" fn run_static_initializers() {
    signals = [
        {
            let mut init = signal {
                signo: 10 as i32,
                signame: b"SIGUSR1\0" as *const u8 as *const i8 as *mut i8,
                flags: 0 as i32,
                handler: Some(signal_handler as unsafe extern "C" fn(i32) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 12 as i32,
                signame: b"SIGUSR2\0" as *const u8 as *const i8 as *mut i8,
                flags: 0 as i32,
                handler: Some(signal_handler as unsafe extern "C" fn(i32) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 21 as i32,
                signame: b"SIGTTIN\0" as *const u8 as *const i8 as *mut i8,
                flags: 0 as i32,
                handler: Some(signal_handler as unsafe extern "C" fn(i32) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 22 as i32,
                signame: b"SIGTTOU\0" as *const u8 as *const i8 as *mut i8,
                flags: 0 as i32,
                handler: Some(signal_handler as unsafe extern "C" fn(i32) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 1 as i32,
                signame: b"SIGHUP\0" as *const u8 as *const i8 as *mut i8,
                flags: 0 as i32,
                handler: Some(signal_handler as unsafe extern "C" fn(i32) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 2 as i32,
                signame: b"SIGINT\0" as *const u8 as *const i8 as *mut i8,
                flags: 0 as i32,
                handler: Some(signal_handler as unsafe extern "C" fn(i32) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 11 as i32,
                signame: b"SIGSEGV\0" as *const u8 as *const i8 as *mut i8,
                flags: 0x80000000 as u32 as i32,
                handler: Some(signal_handler as unsafe extern "C" fn(i32) -> ()),
            };
            init
        },
        {
            let mut init = signal {
                signo: 13 as i32,
                signame: b"SIGPIPE\0" as *const u8 as *const i8 as *mut i8,
                flags: 0 as i32,
                handler: ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as i32 as libc::intptr_t),
            };
            init
        },
        {
            let mut init = signal {
                signo: 0 as i32,
                signame: 0 as *mut i8,
                flags: 0 as i32,
                handler: None,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];