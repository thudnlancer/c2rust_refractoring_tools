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
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigfillset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigdelset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigismember(__set: *const sigset_t, __signo: i32) -> i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn sigsuspend(__set: *const sigset_t) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn sigpending(__set: *mut sigset_t) -> i32;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
}
pub type size_t = u64;
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
pub type __fd_mask = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
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
unsafe extern "C" fn pth_util_sigdelete_sighandler(mut _sig: i32) {}
#[no_mangle]
pub unsafe extern "C" fn __pth_util_sigdelete(mut sig: i32) -> i32 {
    let mut ss: sigset_t = sigset_t { __val: [0; 16] };
    let mut oss: sigset_t = sigset_t { __val: [0; 16] };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut osa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigpending(&mut ss);
    if sigismember(&mut ss, sig) == 0 {
        return 0 as i32;
    }
    sigemptyset(&mut ss);
    sigaddset(&mut ss, sig);
    sigprocmask(0 as i32, &mut ss, &mut oss);
    sa.__sigaction_handler.sa_handler = Some(
        pth_util_sigdelete_sighandler as unsafe extern "C" fn(i32) -> (),
    );
    sigfillset(&mut sa.sa_mask);
    sa.sa_flags = 0 as i32;
    if sigaction(sig, &mut sa, &mut osa) != 0 as i32 {
        sigprocmask(2 as i32, &mut oss, 0 as *mut sigset_t);
        return 0 as i32;
    }
    sigfillset(&mut ss);
    sigdelset(&mut ss, sig);
    sigsuspend(&mut ss);
    sigaction(sig, &mut osa, 0 as *mut sigaction);
    sigprocmask(2 as i32, &mut oss, 0 as *mut sigset_t);
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_util_cpystrn(
    mut dst: *mut i8,
    mut src: *const i8,
    mut dst_size: size_t,
) -> *mut i8 {
    let mut d: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    if dst_size == 0 as i32 as u64 {
        return dst;
    }
    d = dst;
    end = dst.offset(dst_size as isize).offset(-(1 as i32 as isize));
    while d < end {
        *d = *src;
        if *d as i32 == '\0' as i32 {
            return d;
        }
        d = d.offset(1);
        d;
        src = src.offset(1);
        src;
    }
    *d = '\0' as i32 as i8;
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_util_fd_valid(mut fd: i32) -> i32 {
    if fd < 0 as i32 || fd >= 1024 as i32 {
        return 0 as i32;
    }
    if fcntl(fd, 3 as i32) == -(1 as i32) && *__errno_location() == 9 as i32 {
        return 0 as i32;
    }
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_util_fds_merge(
    mut nfd: i32,
    mut ifds1: *mut fd_set,
    mut ofds1: *mut fd_set,
    mut ifds2: *mut fd_set,
    mut ofds2: *mut fd_set,
    mut ifds3: *mut fd_set,
    mut ofds3: *mut fd_set,
) {
    let mut s: i32 = 0;
    s = 0 as i32;
    while s < nfd {
        if !ifds1.is_null() {
            if (*ifds1)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
            {
                (*ofds1)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    |= ((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask;
            }
        }
        if !ifds2.is_null() {
            if (*ifds2)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
            {
                (*ofds2)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    |= ((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask;
            }
        }
        if !ifds3.is_null() {
            if (*ifds3)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
            {
                (*ofds3)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    |= ((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask;
            }
        }
        s += 1;
        s;
    }
}
#[no_mangle]
pub unsafe extern "C" fn __pth_util_fds_test(
    mut nfd: i32,
    mut ifds1: *mut fd_set,
    mut ofds1: *mut fd_set,
    mut ifds2: *mut fd_set,
    mut ofds2: *mut fd_set,
    mut ifds3: *mut fd_set,
    mut ofds3: *mut fd_set,
) -> i32 {
    let mut s: i32 = 0;
    s = 0 as i32;
    while s < nfd {
        if !ifds1.is_null() {
            if (*ifds1)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
                && (*ofds1)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    & ((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask != 0 as i32 as i64
            {
                return (0 as i32 == 0) as i32;
            }
        }
        if !ifds2.is_null() {
            if (*ifds2)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
                && (*ofds2)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    & ((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask != 0 as i32 as i64
            {
                return (0 as i32 == 0) as i32;
            }
        }
        if !ifds3.is_null() {
            if (*ifds3)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
                && (*ofds3)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    & ((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask != 0 as i32 as i64
            {
                return (0 as i32 == 0) as i32;
            }
        }
        s += 1;
        s;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_util_fds_select(
    mut nfd: i32,
    mut ifds1: *mut fd_set,
    mut ofds1: *mut fd_set,
    mut ifds2: *mut fd_set,
    mut ofds2: *mut fd_set,
    mut ifds3: *mut fd_set,
    mut ofds3: *mut fd_set,
) -> i32 {
    let mut s: i32 = 0;
    let mut n: i32 = 0;
    n = 0 as i32;
    s = 0 as i32;
    while s < nfd {
        if !ifds1.is_null()
            && (*ifds1)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
        {
            if !((*ofds1)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64)
            {
                (*ifds1)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    &= !(((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask);
            } else {
                n += 1;
                n;
            }
        }
        if !ifds2.is_null()
            && (*ifds2)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
        {
            if !((*ofds2)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64)
            {
                (*ifds2)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    &= !(((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask);
            } else {
                n += 1;
                n;
            }
        }
        if !ifds3.is_null()
            && (*ifds3)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64
        {
            if !((*ofds3)
                .__fds_bits[(s
                / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                as usize]
                & ((1 as u64)
                    << s
                        % (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as __fd_mask != 0 as i32 as i64)
            {
                (*ifds3)
                    .__fds_bits[(s
                    / (8 as i32 * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                    as usize]
                    &= !(((1 as u64)
                        << s
                            % (8 as i32
                                * ::core::mem::size_of::<__fd_mask>() as u64 as i32))
                        as __fd_mask);
            } else {
                n += 1;
                n;
            }
        }
        s += 1;
        s;
    }
    return n;
}