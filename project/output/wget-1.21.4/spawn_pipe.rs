use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type __spawn_action;
    fn close(__fd: i32) -> i32;
    static mut environ: *mut *mut i8;
    fn abort() -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn sigprocmask(__how: i32, __set: *const sigset_t, __oset: *mut sigset_t) -> i32;
    fn canonicalize_filename_mode(_: *const i8, _: canonicalize_mode_t) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn block_fatal_signals();
    fn unblock_fatal_signals();
    fn find_in_given_path(
        progname: *const i8,
        path: *const i8,
        directory: *const i8,
        optimize_for_exec: bool,
    ) -> *const i8;
    fn pipe2_safer(_: *mut i32, _: i32) -> i32;
    fn register_slave_subprocess(child: pid_t);
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn rpl_posix_spawn_file_actions_init(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
    ) -> i32;
    fn rpl_posix_spawn_file_actions_addclose(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
        __fd: i32,
    ) -> i32;
    fn rpl_posix_spawn_file_actions_destroy(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
    ) -> i32;
    fn rpl_posix_spawnattr_destroy(__attr: *mut rpl_posix_spawnattr_t) -> i32;
    fn posix_spawn_file_actions_addchdir(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
        __path: *const i8,
    ) -> i32;
    fn rpl_posix_spawnattr_init(__attr: *mut rpl_posix_spawnattr_t) -> i32;
    fn rpl_posix_spawnattr_setsigmask(
        __attr: *mut rpl_posix_spawnattr_t,
        __sigmask: *const sigset_t,
    ) -> i32;
    fn rpl_posix_spawnattr_setflags(
        __attr: *mut rpl_posix_spawnattr_t,
        __flags: libc::c_short,
    ) -> i32;
    fn rpl_posix_spawnp(
        __pid: *mut pid_t,
        __file: *const i8,
        __file_actions: *const rpl_posix_spawn_file_actions_t,
        __attrp: *const rpl_posix_spawnattr_t,
        argv: *const *mut i8,
        envp: *const *mut i8,
    ) -> i32;
    fn rpl_posix_spawn(
        __pid: *mut pid_t,
        __path: *const i8,
        __file_actions: *const rpl_posix_spawn_file_actions_t,
        __attrp: *const rpl_posix_spawnattr_t,
        argv: *const *mut i8,
        envp: *const *mut i8,
    ) -> i32;
    fn rpl_posix_spawn_file_actions_adddup2(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
        __fd: i32,
        __newfd: i32,
    ) -> i32;
    fn rpl_posix_spawn_file_actions_addopen(
        __file_actions: *mut rpl_posix_spawn_file_actions_t,
        __fd: i32,
        __path: *const i8,
        __oflag: i32,
        __mode: mode_t,
    ) -> i32;
}
pub type __mode_t = u32;
pub type __pid_t = i32;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawnattr_t {
    pub _flags: libc::c_short,
    pub _pgrp: pid_t,
    pub _sd: sigset_t,
    pub _ss: sigset_t,
    pub _sp: sched_param,
    pub _policy: i32,
    pub __pad: [i32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawn_file_actions_t {
    pub _allocated: i32,
    pub _used: i32,
    pub _actions: *mut __spawn_action,
    pub __pad: [i32; 16],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum canonicalize_mode_t {
    CAN_NOLINKS = 4,
    CAN_MISSING = 2,
    CAN_ALL_BUT_LAST = 1,
    CAN_EXISTING = 0,
}
impl canonicalize_mode_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            canonicalize_mode_t::CAN_NOLINKS => 4,
            canonicalize_mode_t::CAN_MISSING => 2,
            canonicalize_mode_t::CAN_ALL_BUT_LAST => 1,
            canonicalize_mode_t::CAN_EXISTING => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> canonicalize_mode_t {
        match value {
            4 => canonicalize_mode_t::CAN_NOLINKS,
            2 => canonicalize_mode_t::CAN_MISSING,
            1 => canonicalize_mode_t::CAN_ALL_BUT_LAST,
            0 => canonicalize_mode_t::CAN_EXISTING,
            _ => panic!("Invalid value for canonicalize_mode_t: {}", value),
        }
    }
}
impl AddAssign<u32> for canonicalize_mode_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for canonicalize_mode_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for canonicalize_mode_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for canonicalize_mode_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for canonicalize_mode_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn add(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn sub(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn mul(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn div(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for canonicalize_mode_t {
    type Output = canonicalize_mode_t;
    fn rem(self, rhs: u32) -> canonicalize_mode_t {
        canonicalize_mode_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
unsafe extern "C" fn nonintr_close(mut fd: i32) -> i32 {
    let mut retval: i32 = 0;
    loop {
        retval = close(fd);
        if !(retval < 0 as i32 && *__errno_location() == 4 as i32) {
            break;
        }
    }
    return retval;
}
unsafe extern "C" fn create_pipe(
    mut progname: *const i8,
    mut prog_path: *const i8,
    mut prog_argv: *const *const i8,
    mut directory: *const i8,
    mut pipe_stdin: bool,
    mut pipe_stdout: bool,
    mut prog_stdin: *const i8,
    mut prog_stdout: *const i8,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut i32,
) -> pid_t {
    let mut ifd: [i32; 2] = [0; 2];
    let mut ofd: [i32; 2] = [0; 2];
    let mut blocked_signals: sigset_t = sigset_t { __val: [0; 16] };
    let mut actions: rpl_posix_spawn_file_actions_t = rpl_posix_spawn_file_actions_t {
        _allocated: 0,
        _used: 0,
        _actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    let mut actions_allocated: bool = false;
    let mut attrs: rpl_posix_spawnattr_t = rpl_posix_spawnattr_t {
        _flags: 0,
        _pgrp: 0,
        _sd: sigset_t { __val: [0; 16] },
        _ss: sigset_t { __val: [0; 16] },
        _sp: sched_param { sched_priority: 0 },
        _policy: 0,
        __pad: [0; 16],
    };
    let mut attrs_allocated: bool = false;
    let mut err: i32 = 0;
    let mut child: pid_t = 0;
    let mut current_block: u64;
    let mut saved_errno: i32 = 0;
    let mut prog_path_to_free: *mut i8 = 0 as *mut i8;
    if !directory.is_null() {
        if !(*prog_path.offset(0 as i32 as isize) as i32 == '/' as i32) {
            let mut resolved_prog: *const i8 = find_in_given_path(
                prog_path,
                getenv(b"PATH\0" as *const u8 as *const i8),
                0 as *const i8,
                0 as i32 != 0,
            );
            if resolved_prog.is_null() {
                current_block = 11140881323421004112;
            } else {
                if resolved_prog != prog_path {
                    prog_path_to_free = resolved_prog as *mut i8;
                }
                prog_path = resolved_prog;
                if !(*prog_path.offset(0 as i32 as isize) as i32 == '/' as i32) {
                    let mut absolute_prog: *mut i8 = canonicalize_filename_mode(
                        prog_path,
                        canonicalize_mode_t::from_libc_c_uint(
                            (canonicalize_mode_t::CAN_MISSING as i32
                                | canonicalize_mode_t::CAN_NOLINKS as i32) as u32,
                        ),
                    );
                    if absolute_prog.is_null() {
                        rpl_free(prog_path_to_free as *mut libc::c_void);
                        current_block = 11140881323421004112;
                    } else {
                        rpl_free(prog_path_to_free as *mut libc::c_void);
                        prog_path_to_free = absolute_prog;
                        prog_path = absolute_prog;
                        if !(*prog_path.offset(0 as i32 as isize) as i32 == '/' as i32) {
                            abort();
                        }
                        current_block = 3512920355445576850;
                    }
                } else {
                    current_block = 3512920355445576850;
                }
            }
            match current_block {
                3512920355445576850 => {}
                _ => {
                    saved_errno = *__errno_location();
                    current_block = 7637866818029819706;
                }
            }
        } else {
            current_block = 3512920355445576850;
        }
    } else {
        current_block = 3512920355445576850;
    }
    match current_block {
        3512920355445576850 => {
            ifd = [0; 2];
            ofd = [0; 2];
            if pipe_stdout {
                if pipe2_safer(ifd.as_mut_ptr(), 0 as i32 | 0o2000000 as i32) < 0 as i32
                {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            b"wget-gnulib\0" as *const u8 as *const i8,
                            b"cannot create pipe\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
            if pipe_stdin {
                if pipe2_safer(ofd.as_mut_ptr(), 0 as i32 | 0o2000000 as i32) < 0 as i32
                {
                    error(
                        1 as i32,
                        *__errno_location(),
                        dcgettext(
                            b"wget-gnulib\0" as *const u8 as *const i8,
                            b"cannot create pipe\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
            blocked_signals = sigset_t { __val: [0; 16] };
            actions = rpl_posix_spawn_file_actions_t {
                _allocated: 0,
                _used: 0,
                _actions: 0 as *mut __spawn_action,
                __pad: [0; 16],
            };
            actions_allocated = false;
            attrs = rpl_posix_spawnattr_t {
                _flags: 0,
                _pgrp: 0,
                _sd: sigset_t { __val: [0; 16] },
                _ss: sigset_t { __val: [0; 16] },
                _sp: sched_param { sched_priority: 0 },
                _policy: 0,
                __pad: [0; 16],
            };
            attrs_allocated = false;
            err = 0;
            child = 0;
            if slave_process {
                sigprocmask(2 as i32, 0 as *const sigset_t, &mut blocked_signals);
                block_fatal_signals();
            }
            actions_allocated = 0 as i32 != 0;
            attrs_allocated = 0 as i32 != 0;
            err = rpl_posix_spawn_file_actions_init(&mut actions);
            if err != 0 as i32
                || {
                    actions_allocated = 1 as i32 != 0;
                    pipe_stdin as i32 != 0
                        && {
                            err = rpl_posix_spawn_file_actions_adddup2(
                                &mut actions,
                                ofd[0 as i32 as usize],
                                0 as i32,
                            );
                            err != 0 as i32
                        }
                        || pipe_stdout as i32 != 0
                            && {
                                err = rpl_posix_spawn_file_actions_adddup2(
                                    &mut actions,
                                    ifd[1 as i32 as usize],
                                    1 as i32,
                                );
                                err != 0 as i32
                            }
                        || pipe_stdin as i32 != 0
                            && {
                                err = rpl_posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ofd[0 as i32 as usize],
                                );
                                err != 0 as i32
                            }
                        || pipe_stdout as i32 != 0
                            && {
                                err = rpl_posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ifd[1 as i32 as usize],
                                );
                                err != 0 as i32
                            }
                        || pipe_stdin as i32 != 0
                            && {
                                err = rpl_posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ofd[1 as i32 as usize],
                                );
                                err != 0 as i32
                            }
                        || pipe_stdout as i32 != 0
                            && {
                                err = rpl_posix_spawn_file_actions_addclose(
                                    &mut actions,
                                    ifd[0 as i32 as usize],
                                );
                                err != 0 as i32
                            }
                        || null_stderr as i32 != 0
                            && {
                                err = rpl_posix_spawn_file_actions_addopen(
                                    &mut actions,
                                    2 as i32,
                                    b"/dev/null\0" as *const u8 as *const i8,
                                    0o2 as i32,
                                    0 as i32 as mode_t,
                                );
                                err != 0 as i32
                            }
                        || !pipe_stdin && !prog_stdin.is_null()
                            && {
                                err = rpl_posix_spawn_file_actions_addopen(
                                    &mut actions,
                                    0 as i32,
                                    prog_stdin,
                                    0 as i32,
                                    0 as i32 as mode_t,
                                );
                                err != 0 as i32
                            }
                        || !pipe_stdout && !prog_stdout.is_null()
                            && {
                                err = rpl_posix_spawn_file_actions_addopen(
                                    &mut actions,
                                    1 as i32,
                                    prog_stdout,
                                    0o1 as i32,
                                    0 as i32 as mode_t,
                                );
                                err != 0 as i32
                            }
                        || !directory.is_null()
                            && {
                                err = posix_spawn_file_actions_addchdir(
                                    &mut actions,
                                    directory,
                                );
                                err != 0
                            }
                        || slave_process as i32 != 0
                            && {
                                err = rpl_posix_spawnattr_init(&mut attrs);
                                err != 0 as i32
                                    || {
                                        attrs_allocated = 1 as i32 != 0;
                                        err = rpl_posix_spawnattr_setsigmask(
                                            &mut attrs,
                                            &mut blocked_signals,
                                        );
                                        err != 0 as i32
                                            || {
                                                err = rpl_posix_spawnattr_setflags(
                                                    &mut attrs,
                                                    0x8 as i32 as libc::c_short,
                                                );
                                                err != 0 as i32
                                            }
                                    }
                            }
                        || {
                            err = (if !directory.is_null() {
                                rpl_posix_spawn(
                                    &mut child,
                                    prog_path,
                                    &mut actions,
                                    (if attrs_allocated as i32 != 0 {
                                        &mut attrs
                                    } else {
                                        0 as *mut rpl_posix_spawnattr_t
                                    }),
                                    prog_argv as *const *mut i8,
                                    environ,
                                )
                            } else {
                                rpl_posix_spawnp(
                                    &mut child,
                                    prog_path,
                                    &mut actions,
                                    (if attrs_allocated as i32 != 0 {
                                        &mut attrs
                                    } else {
                                        0 as *mut rpl_posix_spawnattr_t
                                    }),
                                    prog_argv as *const *mut i8,
                                    environ as *const *mut i8,
                                )
                            });
                            err != 0 as i32
                        }
                }
            {
                if actions_allocated {
                    rpl_posix_spawn_file_actions_destroy(&mut actions);
                }
                if attrs_allocated {
                    rpl_posix_spawnattr_destroy(&mut attrs);
                }
                if slave_process {
                    unblock_fatal_signals();
                }
                if pipe_stdout {
                    nonintr_close(ifd[0 as i32 as usize]);
                    nonintr_close(ifd[1 as i32 as usize]);
                }
                if pipe_stdin {
                    nonintr_close(ofd[0 as i32 as usize]);
                    nonintr_close(ofd[1 as i32 as usize]);
                }
                rpl_free(prog_path_to_free as *mut libc::c_void);
                saved_errno = err;
            } else {
                rpl_posix_spawn_file_actions_destroy(&mut actions);
                if attrs_allocated {
                    rpl_posix_spawnattr_destroy(&mut attrs);
                }
                if slave_process {
                    register_slave_subprocess(child);
                    unblock_fatal_signals();
                }
                if pipe_stdin {
                    nonintr_close(ofd[0 as i32 as usize]);
                }
                if pipe_stdout {
                    nonintr_close(ifd[1 as i32 as usize]);
                }
                rpl_free(prog_path_to_free as *mut libc::c_void);
                if pipe_stdout {
                    *fd.offset(0 as i32 as isize) = ifd[0 as i32 as usize];
                }
                if pipe_stdin {
                    *fd.offset(1 as i32 as isize) = ofd[1 as i32 as usize];
                }
                return child;
            }
        }
        _ => {}
    }
    if exit_on_error as i32 != 0 || !null_stderr {
        error(
            if exit_on_error as i32 != 0 { 1 as i32 } else { 0 as i32 },
            saved_errno,
            dcgettext(
                b"wget-gnulib\0" as *const u8 as *const i8,
                b"%s subprocess failed\0" as *const u8 as *const i8,
                5 as i32,
            ),
            progname,
        );
    }
    *__errno_location() = saved_errno;
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn create_pipe_bidi(
    mut progname: *const i8,
    mut prog_path: *const i8,
    mut prog_argv: *const *const i8,
    mut directory: *const i8,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut i32,
) -> pid_t {
    let mut result: pid_t = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        1 as i32 != 0,
        1 as i32 != 0,
        0 as *const i8,
        0 as *const i8,
        null_stderr,
        slave_process,
        exit_on_error,
        fd,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn create_pipe_in(
    mut progname: *const i8,
    mut prog_path: *const i8,
    mut prog_argv: *const *const i8,
    mut directory: *const i8,
    mut prog_stdin: *const i8,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut i32,
) -> pid_t {
    let mut iofd: [i32; 2] = [0; 2];
    let mut result: pid_t = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        0 as i32 != 0,
        1 as i32 != 0,
        prog_stdin,
        0 as *const i8,
        null_stderr,
        slave_process,
        exit_on_error,
        iofd.as_mut_ptr(),
    );
    if result != -(1 as i32) {
        *fd.offset(0 as i32 as isize) = iofd[0 as i32 as usize];
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn create_pipe_out(
    mut progname: *const i8,
    mut prog_path: *const i8,
    mut prog_argv: *const *const i8,
    mut directory: *const i8,
    mut prog_stdout: *const i8,
    mut null_stderr: bool,
    mut slave_process: bool,
    mut exit_on_error: bool,
    mut fd: *mut i32,
) -> pid_t {
    let mut iofd: [i32; 2] = [0; 2];
    let mut result: pid_t = create_pipe(
        progname,
        prog_path,
        prog_argv,
        directory,
        1 as i32 != 0,
        0 as i32 != 0,
        0 as *const i8,
        prog_stdout,
        null_stderr,
        slave_process,
        exit_on_error,
        iofd.as_mut_ptr(),
    );
    if result != -(1 as i32) {
        *fd.offset(0 as i32 as isize) = iofd[1 as i32 as usize];
    }
    return result;
}