use libc::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr::{null, null_mut};
use std::ffi::CString;
use std::os::raw::c_uint;
use std::mem;

const _SC_ARG_MAX: c_int = 0;
const LONGINT_OK: c_uint = 0;
const LONGINT_INVALID: c_uint = 4;
const BC_INIT_OK: c_uint = 0;
const BC_INIT_ENV_TOO_BIG: c_uint = 1;
const BC_INIT_CANNOT_ACCOMODATE_HEADROOM: c_uint = 2;

type size_t = c_ulong;
type strtol_error = c_uint;

#[derive(Debug)]
struct BuildcmdState {
    cmd_argc: size_t,
    cmd_argv: *mut *mut c_char,
    cmd_argv_alloc: size_t,
    argbuf: *mut c_char,
    cmd_argv_chars: size_t,
    cmd_initial_argv_chars: size_t,
    usercontext: *mut c_void,
    todo: c_int,
    dir_fd: c_int,
    largest_successful_arg_count: size_t,
    smallest_failed_arg_count: size_t,
}

#[derive(Debug)]
struct BuildcmdControl {
    exit_if_size_exceeded: c_int,
    posix_arg_size_max: size_t,
    posix_arg_size_min: size_t,
    arg_max: size_t,
    max_arg_count: size_t,
    rplen: size_t,
    replace_pat: *const c_char,
    initial_argc: size_t,
    exec_callback: Option<extern "C" fn(*mut BuildcmdControl, *mut c_void, c_int, *mut *mut c_char) -> c_int>,
    lines_per_exec: c_ulong,
    args_per_exec: size_t,
}

static SPECIAL_TERMINATING_ARG: &[u8] = b"do_not_care\0";

extern "C" {
    fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: c_uint,
        __function: *const c_char,
    ) -> !;
    fn __errno_location() -> *mut c_int;
    fn getenv(__name: *const c_char) -> *mut c_char;
    fn rpl_free(ptr: *mut c_void);
    fn strcpy(_: *mut c_char, _: *const c_char) -> *mut c_char;
    fn strncpy(_: *mut c_char, _: *const c_char, _: c_ulong) -> *mut c_char;
    fn strlen(_: *const c_char) -> c_ulong;
    fn mbsstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    static mut environ: *mut *mut c_char;
    fn sysconf(__name: c_int) -> c_long;
    fn xmalloc(s: size_t) -> *mut c_void;
    fn xrealloc(p: *mut c_void, s: size_t) -> *mut c_void;
    fn xstrtoul(
        _: *const c_char,
        _: *mut *mut c_char,
        _: c_int,
        _: *mut c_ulong,
        _: *const c_char,
    ) -> strtol_error;
    fn dcgettext(
        __domainname: *const c_char,
        __msgid: *const c_char,
        __category: c_int,
    ) -> *mut c_char;
    fn error(__status: c_int, __errnum: c_int, __format: *const c_char, _: ...);
}

fn bc_args_complete(ctl: *mut BuildcmdControl, state: *mut BuildcmdState) {
    unsafe {
        bc_push_arg(
            ctl,
            state,
            SPECIAL_TERMINATING_ARG.as_ptr() as *const c_char,
            0,
            null(),
            0,
            0,
        );
    }
}

fn update_limit(
    ctl: *mut BuildcmdControl,
    state: *mut BuildcmdState,
    success: bool,
    limit: size_t,
) -> size_t {
    unsafe {
        let state_ref = &mut *state;
        if success {
            if limit > state_ref.largest_successful_arg_count {
                state_ref.largest_successful_arg_count = limit;
            }
        } else if limit < state_ref.smallest_failed_arg_count
            || state_ref.smallest_failed_arg_count == 0
        {
            state_ref.smallest_failed_arg_count = limit;
        }

        if state_ref.largest_successful_arg_count == 0
            || state_ref.smallest_failed_arg_count <= state_ref.largest_successful_arg_count
        {
            if success {
                if limit < usize::MAX {
                    limit + 1
                } else {
                    limit
                }
            } else {
                limit / 2
            }
        } else {
            let shift = (state_ref.smallest_failed_arg_count
                - state_ref.largest_successful_arg_count)
                / 2;
            if success {
                if shift != 0 {
                    limit + shift
                } else {
                    limit + 1
                }
            } else if shift != 0 {
                limit - shift
            } else {
                limit - 1
            }
        }
    }
}

fn copy_args(
    ctl: *mut BuildcmdControl,
    state: *mut BuildcmdState,
    working_args: *mut *mut c_char,
    limit: size_t,
    done: size_t,
) -> size_t {
    unsafe {
        let ctl_ref = &*ctl;
        let state_ref = &*state;
        let mut dst_pos = 0;
        let mut src_pos = 0;

        while src_pos < ctl_ref.initial_argc {
            *working_args.add(dst_pos) = *state_ref.cmd_argv.add(src_pos);
            src_pos += 1;
            dst_pos += 1;
        }

        src_pos += done;
        while src_pos < state_ref.cmd_argc && dst_pos < limit {
            *working_args.add(dst_pos) = *state_ref.cmd_argv.add(src_pos);
            src_pos += 1;
            dst_pos += 1;
        }

        *working_args.add(dst_pos) = null_mut();
        dst_pos
    }
}

fn bc_do_exec(ctl: *mut BuildcmdControl, state: *mut BuildcmdState) {
    unsafe {
        bc_args_complete(ctl, state);
        assert!((*state).cmd_argc > 0);
        assert!((*((*state).cmd_argv).add((*state).cmd_argc - 1)).is_null());

        let working_args = xmalloc(
            (1 + (*state).cmd_argc) * mem::size_of::<*mut c_char>(),
        ) as *mut *mut c_char;

        let mut done = 0;
        let mut limit = (*state).cmd_argc;

        loop {
            let dst_pos = copy_args(ctl, state, working_args, limit, done);
            let callback = (*ctl).exec_callback.unwrap();
            if callback(ctl, (*state).usercontext, dst_pos as c_int, working_args) != 0 {
                limit = update_limit(ctl, state, true, limit);
                done += dst_pos - (*ctl).initial_argc;
            } else if limit <= (*ctl).initial_argc + 1 {
                error(
                    1,
                    0,
                    dcgettext(
                        null(),
                        b"can't call exec() due to argument size restrictions\0".as_ptr(),
                        5,
                    ),
                );
            } else {
                limit = update_limit(ctl, state, false, limit);
            }

            if done + 1 >= (*state).cmd_argc - (*ctl).initial_argc {
                break;
            }
        }

        rpl_free(working_args as *mut c_void);
        bc_clear_args(ctl, state);
    }
}

fn bc_argc_limit_reached(
    initial_args: c_int,
    ctl: *const BuildcmdControl,
    state: *mut BuildcmdState,
) -> bool {
    unsafe {
        (initial_args == 0
            && (*ctl).args_per_exec != 0
            && (*state).cmd_argc - (*ctl).initial_argc == (*ctl).args_per_exec)
            || (*state).cmd_argc == (*ctl).max_arg_count
    }
}

fn bc_push_arg(
    ctl: *mut BuildcmdControl,
    state: *mut BuildcmdState,
    arg: *const c_char,
    len: size_t,
    prefix: *const c_char,
    pfxlen: size_t,
    initial_args: c_int,
) {
    unsafe {
        let terminate = arg == SPECIAL_TERMINATING_ARG.as_ptr() as *const c_char;
        assert!(!arg.is_null());

        if terminate == 0 {
            let new_size = (*state).cmd_argv_chars + len + pfxlen;
            if new_size > (*ctl).arg_max {
                if initial_args != 0 || (*state).cmd_argc == (*ctl).initial_argc {
                    error(
                        1,
                        0,
                        dcgettext(
                            null(),
                            b"cannot fit single argument within argument list size limit\0".as_ptr(),
                            5,
                        ),
                    );
                }
                if !(*ctl).replace_pat.is_null()
                    || (*ctl).exit_if_size_exceeded != 0
                        && ((*ctl).lines_per_exec != 0 || (*ctl).args_per_exec != 0)
                {
                    error(
                        1,
                        0,
                        dcgettext(null(), b"argument list too long\0".as_ptr(), 5),
                    );
                }
                bc_do_exec(ctl, state);
            }

            if bc_argc_limit_reached(initial_args, ctl, state) {
                bc_do_exec(ctl, state);
            }
        }

        if initial_args == 0 {
            (*state).todo = 1;
        }

        if (*state).cmd_argc >= (*state).cmd_argv_alloc {
            if (*state).cmd_argv.is_null() {
                (*state).cmd_argv_alloc = 64;
                (*state).cmd_argv = xmalloc(
                    mem::size_of::<*mut c_char>() * (*state).cmd_argv_alloc,
                ) as *mut *mut c_char;
            } else {
                (*state).cmd_argv_alloc *= 2;
                (*state).cmd_argv = xrealloc(
                    (*state).cmd_argv as *mut c_void,
                    mem::size_of::<*mut c_char>() * (*state).cmd_argv_alloc,
                ) as *mut *mut c_char;
            }
        }

        if terminate != 0 {
            *(*state).cmd_argv.add((*state).cmd_argc) = null_mut();
            (*state).cmd_argc += 1;
        } else {
            *(*state).cmd_argv.add((*state).cmd_argc) = (*state).argbuf.add((*state).cmd_argv_chars);
            if !prefix.is_null() {
                strcpy(
                    (*state).argbuf.add((*state).cmd_argv_chars),
                    prefix,
                );
                (*state).cmd_argv_chars += pfxlen;
            }
            strcpy(
                (*state).argbuf.add((*state).cmd_argv_chars),
                arg,
            );
            (*state).cmd_argv_chars += len;
            (*state).cmd_argc += 1;

            if bc_argc_limit_reached(initial_args, ctl, state) {
                bc_do_exec(ctl, state);
            }
        }

        if initial_args != 0 {
            (*state).cmd_initial_argv_chars = (*state).cmd_argv_chars;
        }
    }
}

fn bc_get_arg_max() -> size_t {
    unsafe {
        assert!(usize::MAX >= c_long::MAX as usize);
        let val = sysconf(_SC_ARG_MAX);
        if val > 0 {
            val as size_t
        } else {
            c_long::MAX as size_t
        }
    }
}

extern "C" fn cb_exec_noop(
    _ctl: *mut BuildcmdControl,
    _usercontext: *mut c_void,
    _argc: c_int,
    _argv: *mut *mut c_char,
) -> c_int {
    0
}

fn bc_size_of_environment() -> size_t {
    unsafe {
        let mut len = 0;
        let mut envp = environ;
        while !(*envp).is_null() {
            len += strlen(*envp) + 1;
            envp = envp.add(1);
        }
        len
    }
}

fn bc_init_controlinfo(ctl: *mut BuildcmdControl, headroom: size_t) -> c_uint {
    unsafe {
        let size_of_environment = bc_size_of_environment();
        (*ctl).posix_arg_size_min = 4096;
        (*ctl).posix_arg_size_max = bc_get_arg_max();
        (*ctl).exit_if_size_exceeded = 0;

        if size_of_environment > (*ctl).posix_arg_size_max {
            BC_INIT_ENV_TOO_BIG
        } else if headroom + size_of_environment >= (*ctl).posix_arg_size_max {
            BC_INIT_CANNOT_ACCOMODATE_HEADROOM
        } else {
            (*ctl).posix_arg_size_max -= size_of_environment;
            (*ctl).posix_arg_size_max -= headroom;
            (*ctl).max_arg_count = (*ctl).posix_arg_size_max / mem::size_of::<*mut c_char>() - 2;
            assert!((*ctl).max_arg_count > 0);

            (*ctl).rplen = 0;
            (*ctl).replace_pat = null();
            (*ctl).initial_argc = 0;
            (*ctl).exec_callback = Some(cb_exec_noop);
            (*ctl).lines_per_exec = 0;
            (*ctl).args_per_exec = 0;
            (*ctl).arg_max = (*ctl).posix_arg_size_max;
            BC_INIT_OK
        }
    }
}

fn bc_use_sensible_arg_max(ctl: *mut BuildcmdControl) {
    const ARG_SIZE: size_t = 131072;
    unsafe {
        if ARG_SIZE > (*ctl).posix_arg_size_max {
            (*ctl).arg_max = (*ctl).posix_arg_size_max;
        } else if ARG_SIZE < (*ctl).posix_arg_size_min {
            (*ctl).arg_max = (*ctl).posix_arg_size_min;
        } else {
            (*ctl).arg_max = ARG_SIZE;
        }
    }
}

fn bc_init_state(
    ctl: *const BuildcmdControl,
    state: *mut BuildcmdState,
    context: *mut c_void,
) {
    unsafe {
        (*state).cmd_argc = 0;
        (*state).cmd_argv_chars = 0;
        (*state).cmd_argv = null_mut();
        (*state).cmd_argv_alloc = 0;
        (*state).largest_successful_arg_count = 0;
        (*state).smallest_failed_arg_count = 0;
        assert!((*ctl).arg_max <= (c_long::MAX - 2048) as usize);
        (*state).argbuf = xmalloc((*ctl).arg_max + 1) as *mut c_char;
        (*state).cmd_initial_argv_chars = 0;
        (*state).cmd_argv_chars = (*state).cmd_initial_argv_chars;
        (*state).todo = 0;
        (*state).dir_fd = -1;
        (*state).usercontext = context;
    }
}

fn bc_clear_args(ctl: *const BuildcmdControl, state: *mut BuildcmdState) {
    unsafe {
        (*state).cmd_argc = (*ctl).initial_argc;
        (*state).cmd_argv_chars = (*state).cmd_initial_argv_chars;
        (*state).todo = 0;
        (*state).dir_fd = -1;
    }
}

fn exceeds(env_var_name: *const c_char, quantity: size_t) -> bool {
    unsafe {
        let val = getenv(env_var_name);
        if !val.is_null() {
            let mut tmp = null_mut();
            let mut limit = 0;
            if xstrtoul(val, &mut tmp, 10, &mut limit, null()) == LONGINT_OK {
                quantity > limit
            } else {
                error(
                    1,
                    *__errno_location(),
                    dcgettext(
                        null(),
                        b"Environment variable %s is not set to a valid decimal number\0".as_ptr(),
                        5,
                    ),
                    env_var_name,
                );
                false
            }
        } else {
            false
        }
    }
}

fn bc_args_exceed_testing_limit(argv: *mut *mut c_char) -> bool {
    unsafe {
        let mut chars = 0;
        let mut args = 0;
        let mut current = argv;

        while !(*current).is_null() {
            args += 1;
            chars += strlen(*current);
            current = current.add(1);
        }

        exceeds(
            b"__GNU_FINDUTILS_EXEC_ARG_COUNT_LIMIT\0".as_ptr(),
            args,
        ) || exceeds(
            b"__GNU_FINDUTILS_EXEC_ARG_LENGTH_LIMIT\0".as_ptr(),
            chars,
        )
    }
}