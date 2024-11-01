#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn sched_setparam(__pid: __pid_t, __param: *const sched_param) -> libc::c_int;
    fn sched_setscheduler(
        __pid: __pid_t,
        __policy: libc::c_int,
        __param: *const sched_param,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn fchdir(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn confstr(__name: libc::c_int, __buf: *mut libc::c_char, __len: size_t) -> size_t;
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn fork() -> __pid_t;
    fn setegid(__gid: __gid_t) -> libc::c_int;
    fn seteuid(__uid: __uid_t) -> libc::c_int;
    fn vfork() -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __spawn_action {
    pub tag: C2RustUnnamed_5,
    pub action: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub close_action: C2RustUnnamed_4,
    pub dup2_action: C2RustUnnamed_3,
    pub open_action: C2RustUnnamed_2,
    pub chdir_action: C2RustUnnamed_1,
    pub fchdir_action: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fd: libc::c_int,
    pub path: *mut libc::c_char,
    pub oflag: libc::c_int,
    pub mode: mode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub fd: libc::c_int,
    pub newfd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub fd: libc::c_int,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const spawn_do_fchdir: C2RustUnnamed_5 = 4;
pub const spawn_do_chdir: C2RustUnnamed_5 = 3;
pub const spawn_do_open: C2RustUnnamed_5 = 2;
pub const spawn_do_dup2: C2RustUnnamed_5 = 1;
pub const spawn_do_close: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawnattr_t {
    pub _flags: libc::c_short,
    pub _pgrp: pid_t,
    pub _sd: sigset_t,
    pub _ss: sigset_t,
    pub _sp: sched_param,
    pub _policy: libc::c_int,
    pub __pad: [libc::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawn_file_actions_t {
    pub _allocated: libc::c_int,
    pub _used: libc::c_int,
    pub _actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
pub const _CS_PATH: C2RustUnnamed_17 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_6,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_16,
    pub _timer: C2RustUnnamed_15,
    pub _rt: C2RustUnnamed_14,
    pub _sigchld: C2RustUnnamed_13,
    pub _sigfault: C2RustUnnamed_10,
    pub _sigpoll: C2RustUnnamed_9,
    pub _sigsys: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub _addr_bnd: C2RustUnnamed_12,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const _CS_V7_ENV: C2RustUnnamed_17 = 1149;
pub const _CS_V6_ENV: C2RustUnnamed_17 = 1148;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed_17 = 1147;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: C2RustUnnamed_17 = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed_17 = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed_17 = 1144;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: C2RustUnnamed_17 = 1143;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: C2RustUnnamed_17 = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: C2RustUnnamed_17 = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: C2RustUnnamed_17 = 1140;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed_17 = 1139;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: C2RustUnnamed_17 = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed_17 = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: C2RustUnnamed_17 = 1136;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: C2RustUnnamed_17 = 1135;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: C2RustUnnamed_17 = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: C2RustUnnamed_17 = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: C2RustUnnamed_17 = 1132;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed_17 = 1131;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: C2RustUnnamed_17 = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed_17 = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed_17 = 1128;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: C2RustUnnamed_17 = 1127;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: C2RustUnnamed_17 = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: C2RustUnnamed_17 = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: C2RustUnnamed_17 = 1124;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed_17 = 1123;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: C2RustUnnamed_17 = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed_17 = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: C2RustUnnamed_17 = 1120;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: C2RustUnnamed_17 = 1119;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: C2RustUnnamed_17 = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: C2RustUnnamed_17 = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: C2RustUnnamed_17 = 1116;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: C2RustUnnamed_17 = 1115;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: C2RustUnnamed_17 = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: C2RustUnnamed_17 = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: C2RustUnnamed_17 = 1112;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: C2RustUnnamed_17 = 1111;
pub const _CS_XBS5_LP64_OFF64_LIBS: C2RustUnnamed_17 = 1110;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: C2RustUnnamed_17 = 1109;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: C2RustUnnamed_17 = 1108;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: C2RustUnnamed_17 = 1107;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: C2RustUnnamed_17 = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: C2RustUnnamed_17 = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: C2RustUnnamed_17 = 1104;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: C2RustUnnamed_17 = 1103;
pub const _CS_XBS5_ILP32_OFF32_LIBS: C2RustUnnamed_17 = 1102;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: C2RustUnnamed_17 = 1101;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: C2RustUnnamed_17 = 1100;
pub const _CS_LFS64_LINTFLAGS: C2RustUnnamed_17 = 1007;
pub const _CS_LFS64_LIBS: C2RustUnnamed_17 = 1006;
pub const _CS_LFS64_LDFLAGS: C2RustUnnamed_17 = 1005;
pub const _CS_LFS64_CFLAGS: C2RustUnnamed_17 = 1004;
pub const _CS_LFS_LINTFLAGS: C2RustUnnamed_17 = 1003;
pub const _CS_LFS_LIBS: C2RustUnnamed_17 = 1002;
pub const _CS_LFS_LDFLAGS: C2RustUnnamed_17 = 1001;
pub const _CS_LFS_CFLAGS: C2RustUnnamed_17 = 1000;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: C2RustUnnamed_17 = 5;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: C2RustUnnamed_17 = 4;
pub const _CS_GNU_LIBPTHREAD_VERSION: C2RustUnnamed_17 = 3;
pub const _CS_GNU_LIBC_VERSION: C2RustUnnamed_17 = 2;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: C2RustUnnamed_17 = 1;
#[no_mangle]
pub unsafe extern "C" fn gl_posix_spawn_internal(
    mut pid: *mut pid_t,
    mut file: *const libc::c_char,
    mut file_actions: *const rpl_posix_spawn_file_actions_t,
    mut attrp: *const rpl_posix_spawnattr_t,
    mut argv: *const *const libc::c_char,
    mut envp: *const *const libc::c_char,
    mut use_path: libc::c_int,
) -> libc::c_int {
    let mut new_pid: pid_t = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut pathlen: size_t = 0;
    let mut flags: libc::c_short = (if attrp.is_null() {
        0 as libc::c_int
    } else {
        (*attrp)._flags as libc::c_int
    }) as libc::c_short;
    &mut flags;
    if flags as libc::c_int & 0x40 as libc::c_int != 0 as libc::c_int
        || flags as libc::c_int
            & (0x8 as libc::c_int | 0x4 as libc::c_int | 0x10 as libc::c_int
                | 0x20 as libc::c_int | 0x2 as libc::c_int | 0x1 as libc::c_int)
            == 0 as libc::c_int && file_actions.is_null()
    {
        new_pid = vfork();
    } else {
        new_pid = fork();
    }
    if new_pid != 0 as libc::c_int {
        if new_pid < 0 as libc::c_int {
            return *__errno_location();
        }
        if !pid.is_null() {
            *pid = new_pid;
        }
        return 0 as libc::c_int;
    }
    if flags as libc::c_int & 0x8 as libc::c_int != 0 as libc::c_int
        && sigprocmask(2 as libc::c_int, &(*attrp)._ss, 0 as *mut sigset_t)
            != 0 as libc::c_int
    {
        _exit(127 as libc::c_int);
    }
    if flags as libc::c_int & 0x4 as libc::c_int != 0 as libc::c_int {
        let mut sig: libc::c_int = 0;
        let mut sa: sigaction = sigaction {
            __sigaction_handler: C2RustUnnamed_6 {
                sa_handler: None,
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        memset(
            &mut sa as *mut sigaction as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<sigaction>() as libc::c_ulong,
        );
        sa.__sigaction_handler.sa_handler = None;
        sig = 1 as libc::c_int;
        while sig <= 64 as libc::c_int + 1 as libc::c_int {
            if sigismember(&(*attrp)._sd, sig) != 0 as libc::c_int
                && sigaction(sig, &mut sa, 0 as *mut sigaction) != 0 as libc::c_int
            {
                _exit(127 as libc::c_int);
            }
            sig += 1;
            sig;
        }
    }
    if flags as libc::c_int & (0x10 as libc::c_int | 0x20 as libc::c_int)
        == 0x10 as libc::c_int
    {
        if sched_setparam(0 as libc::c_int, &(*attrp)._sp) == -(1 as libc::c_int) {
            _exit(127 as libc::c_int);
        }
    } else if flags as libc::c_int & 0x20 as libc::c_int != 0 as libc::c_int {
        if sched_setscheduler(
            0 as libc::c_int,
            (*attrp)._policy,
            (if flags as libc::c_int & 0x10 as libc::c_int != 0 as libc::c_int {
                &(*attrp)._sp
            } else {
                0 as *const sched_param
            }),
        ) == -(1 as libc::c_int)
        {
            _exit(127 as libc::c_int);
        }
    }
    if flags as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int
        && setpgid(0 as libc::c_int, (*attrp)._pgrp) != 0 as libc::c_int
    {
        _exit(127 as libc::c_int);
    }
    if flags as libc::c_int & 0x1 as libc::c_int != 0 as libc::c_int
        && (seteuid(getuid()) != 0 as libc::c_int
            || setegid(getgid()) != 0 as libc::c_int)
    {
        _exit(127 as libc::c_int);
    }
    if !file_actions.is_null() {
        let mut cnt: libc::c_int = 0;
        cnt = 0 as libc::c_int;
        while cnt < (*file_actions)._used {
            let mut action: *mut __spawn_action = &mut *((*file_actions)._actions)
                .offset(cnt as isize) as *mut __spawn_action;
            match (*action).tag as libc::c_uint {
                0 => {
                    if close((*action).action.close_action.fd) != 0 as libc::c_int {
                        _exit(127 as libc::c_int);
                    }
                }
                2 => {
                    let mut new_fd: libc::c_int = open(
                        (*action).action.open_action.path,
                        (*action).action.open_action.oflag | 0 as libc::c_int,
                        (*action).action.open_action.mode,
                    );
                    if new_fd == -(1 as libc::c_int) {
                        _exit(127 as libc::c_int);
                    }
                    if new_fd != (*action).action.open_action.fd {
                        if dup2(new_fd, (*action).action.open_action.fd)
                            != (*action).action.open_action.fd
                        {
                            _exit(127 as libc::c_int);
                        }
                        if close(new_fd) != 0 as libc::c_int {
                            _exit(127 as libc::c_int);
                        }
                    }
                }
                1 => {
                    if dup2(
                        (*action).action.dup2_action.fd,
                        (*action).action.dup2_action.newfd,
                    ) != (*action).action.dup2_action.newfd
                    {
                        _exit(127 as libc::c_int);
                    }
                }
                3 => {
                    if chdir((*action).action.chdir_action.path) < 0 as libc::c_int {
                        _exit(127 as libc::c_int);
                    }
                }
                4 => {
                    if fchdir((*action).action.fchdir_action.fd) < 0 as libc::c_int {
                        _exit(127 as libc::c_int);
                    }
                }
                _ => {}
            }
            cnt += 1;
            cnt;
        }
    }
    if use_path == 0 || !(strchr(file, '/' as i32)).is_null() {
        execve(file, argv as *const *mut libc::c_char, envp as *const *mut libc::c_char);
        _exit(127 as libc::c_int);
    }
    path = getenv(b"PATH\0" as *const u8 as *const libc::c_char);
    if path.is_null() {
        len = confstr(
            _CS_PATH as libc::c_int,
            0 as *mut libc::c_void as *mut libc::c_char,
            0 as libc::c_int as size_t,
        );
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (1 as libc::c_int as libc::c_ulong).wrapping_add(len) as usize,
        );
        path = fresh0.as_mut_ptr() as *mut libc::c_char;
        *path.offset(0 as libc::c_int as isize) = ':' as i32 as libc::c_char;
        confstr(_CS_PATH as libc::c_int, path.offset(1 as libc::c_int as isize), len);
    }
    len = (strlen(file)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    pathlen = strlen(path);
    let mut fresh1 = ::std::vec::from_elem(
        0,
        pathlen.wrapping_add(len).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as usize,
    );
    name = fresh1.as_mut_ptr() as *mut libc::c_char;
    name = memcpy(
        name.offset(pathlen as isize).offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        file as *const libc::c_void,
        len,
    ) as *mut libc::c_char;
    name = name.offset(-1);
    *name = '/' as i32 as libc::c_char;
    p = path;
    loop {
        let mut startp: *mut libc::c_char = 0 as *mut libc::c_char;
        path = p;
        p = strchrnul(path, ':' as i32);
        if p == path {
            startp = name.offset(1 as libc::c_int as isize);
        } else {
            startp = memcpy(
                name.offset(-(p.offset_from(path) as libc::c_long as isize))
                    as *mut libc::c_void,
                path as *const libc::c_void,
                p.offset_from(path) as libc::c_long as libc::c_ulong,
            ) as *mut libc::c_char;
        }
        execve(
            startp,
            argv as *const *mut libc::c_char,
            envp as *const *mut libc::c_char,
        );
        match *__errno_location() {
            13 | 2 | 116 | 20 => {}
            _ => {
                _exit(127 as libc::c_int);
            }
        }
        let fresh2 = p;
        p = p.offset(1);
        if !(*fresh2 as libc::c_int != '\0' as i32) {
            break;
        }
    }
    _exit(127 as libc::c_int);
}
