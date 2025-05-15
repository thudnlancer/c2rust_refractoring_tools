use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::ptr;
use std::mem;
use std::os::raw::{c_int, c_void};
use std::cmp;
use std::env;
use std::process;
use libc::{self, size_t, c_char};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum BCInitStatus {
    Ok,
    EnvTooBig,
    CannotAccomodateHeadroom,
}

#[derive(Debug)]
pub struct BuildCmdControl {
    exit_if_size_exceeded: bool,
    posix_arg_size_max: size_t,
    posix_arg_size_min: size_t,
    arg_max: size_t,
    max_arg_count: size_t,
    rplen: size_t,
    replace_pat: Option<CString>,
    initial_argc: size_t,
    exec_callback: Option<Box<dyn FnMut(&BuildCmdControl, *mut c_void, c_int, *mut *mut c_char) -> c_int>>,
    lines_per_exec: u64,
    args_per_exec: size_t,
}

#[derive(Debug)]
pub struct BuildCmdState {
    cmd_argc: size_t,
    cmd_argv: Vec<*mut c_char>,
    cmd_argv_alloc: size_t,
    argbuf: Vec<u8>,
    cmd_argv_chars: size_t,
    cmd_initial_argv_chars: size_t,
    usercontext: *mut c_void,
    todo: c_int,
    dir_fd: c_int,
    largest_successful_arg_count: size_t,
    smallest_failed_arg_count: size_t,
}

static SPECIAL_TERMINATING_ARG: &str = "do_not_care";

impl BuildCmdControl {
    pub fn new() -> Self {
        BuildCmdControl {
            exit_if_size_exceeded: false,
            posix_arg_size_max: 0,
            posix_arg_size_min: libc::_POSIX_ARG_MAX as size_t,
            arg_max: 0,
            max_arg_count: 0,
            rplen: 0,
            replace_pat: None,
            initial_argc: 0,
            exec_callback: None,
            lines_per_exec: 0,
            args_per_exec: 0,
        }
    }

    pub fn init_controlinfo(&mut self, headroom: size_t) -> BCInitStatus {
        let size_of_environment = Self::size_of_environment();
        self.posix_arg_size_max = Self::get_arg_max();

        if size_of_environment > self.posix_arg_size_max {
            return BCInitStatus::EnvTooBig;
        } else if (headroom + size_of_environment) >= self.posix_arg_size_max {
            return BCInitStatus::CannotAccomodateHeadroom;
        } else {
            self.posix_arg_size_max -= size_of_environment;
            self.posix_arg_size_max -= headroom;
        }

        self.max_arg_count = (self.posix_arg_size_max / mem::size_of::<*mut c_char>()) - 2;
        assert!(self.max_arg_count > 0);
        self.arg_max = self.posix_arg_size_max;

        BCInitStatus::Ok
    }

    pub fn use_sensible_arg_max(&mut self) {
        const DEFAULT_ARG_SIZE: size_t = 128 * 1024;
        self.arg_max = if DEFAULT_ARG_SIZE > self.posix_arg_size_max {
            self.posix_arg_size_max
        } else if DEFAULT_ARG_SIZE < self.posix_arg_size_min {
            self.posix_arg_size_min
        } else {
            DEFAULT_ARG_SIZE
        };
    }

    pub fn size_of_environment() -> size_t {
        env::vars_os()
            .map(|(k, v)| k.len() + v.len() + 2) // +2 for '=' and null terminator
            .sum()
    }

    pub fn get_arg_max() -> size_t {
        unsafe {
            let val = libc::sysconf(libc::_SC_ARG_MAX);
            if val > 0 {
                return val as size_t;
            }

            #[cfg(has_arg_max)]
            {
                const ARG_MAX: size_t = libc::ARG_MAX as size_t;
                if ARG_MAX > 0 {
                    return ARG_MAX;
                }
            }

            libc::LONG_MAX as size_t
        }
    }
}

impl BuildCmdState {
    pub fn new(control: &BuildCmdControl, context: *mut c_void) -> Self {
        BuildCmdState {
            cmd_argc: 0,
            cmd_argv: Vec::new(),
            cmd_argv_alloc: 0,
            argbuf: vec![0; control.arg_max as usize + 1],
            cmd_argv_chars: 0,
            cmd_initial_argv_chars: 0,
            usercontext: context,
            todo: 0,
            dir_fd: -1,
            largest_successful_arg_count: 0,
            smallest_failed_arg_count: 0,
        }
    }

    pub fn clear_args(&mut self, control: &BuildCmdControl) {
        self.cmd_argc = control.initial_argc;
        self.cmd_argv_chars = self.cmd_initial_argv_chars;
        self.todo = 0;
        self.dir_fd = -1;
    }

    pub fn args_complete(&mut self, control: &mut BuildCmdControl) {
        self.push_arg(
            control,
            SPECIAL_TERMINATING_ARG.as_ptr() as *const c_char,
            0,
            ptr::null(),
            0,
            0,
        );
    }

    pub fn push_arg(
        &mut self,
        control: &mut BuildCmdControl,
        arg: *const c_char,
        len: size_t,
        prefix: *const c_char,
        pfxlen: size_t,
        initial_args: c_int,
    ) {
        let terminate = arg == SPECIAL_TERMINATING_ARG.as_ptr() as *const c_char;

        assert!(!arg.is_null());

        if !terminate {
            if self.cmd_argv_chars + len + pfxlen > control.arg_max {
                if initial_args != 0 || self.cmd_argc == control.initial_argc {
                    eprintln!("cannot fit single argument within argument list size limit");
                    process::exit(1);
                }

                if control.replace_pat.is_some()
                    || (control.exit_if_size_exceeded
                        && (control.lines_per_exec != 0 || control.args_per_exec != 0))
                {
                    eprintln!("argument list too long");
                    process::exit(1);
                }
                self.do_exec(control);
            }

            if self.argc_limit_reached(initial_args, control) {
                self.do_exec(control);
            }
        }

        if initial_args == 0 {
            self.todo = 1;
        }

        if self.cmd_argc >= self.cmd_argv_alloc {
            if self.cmd_argv.is_empty() {
                self.cmd_argv_alloc = 64;
                self.cmd_argv = vec![ptr::null_mut(); self.cmd_argv_alloc];
            } else {
                self.cmd_argv_alloc *= 2;
                self.cmd_argv.resize(self.cmd_argv_alloc, ptr::null_mut());
            }
        }

        if terminate {
            self.cmd_argv[self.cmd_argc] = ptr::null_mut();
            self.cmd_argc += 1;
        } else {
            unsafe {
                let dest = self.argbuf.as_mut_ptr().add(self.cmd_argv_chars);
                if !prefix.is_null() {
                    libc::strcpy(dest as *mut c_char, prefix);
                    self.cmd_argv_chars += pfxlen;
                }

                libc::strcpy(
                    self.argbuf.as_mut_ptr().add(self.cmd_argv_chars) as *mut c_char,
                    arg,
                );
                self.cmd_argv[self.cmd_argc] = self.argbuf.as_mut_ptr().add(self.cmd_argv_chars) as *mut c_char;
                self.cmd_argc += 1;
                self.cmd_argv_chars += len;
            }

            if self.argc_limit_reached(initial_args, control) {
                self.do_exec(control);
            }
        }

        if initial_args != 0 {
            self.cmd_initial_argv_chars = self.cmd_argv_chars;
        }
    }

    fn argc_limit_reached(&self, initial_args: c_int, control: &BuildCmdControl) -> bool {
        if initial_args == 0
            && control.args_per_exec != 0
            && (self.cmd_argc - control.initial_argc) == control.args_per_exec
        {
            return true;
        }

        self.cmd_argc == control.max_arg_count
    }

    pub fn do_exec(&mut self, control: &mut BuildCmdControl) {
        self.args_complete(control);
        assert!(self.cmd_argc > 0);
        assert!(self.cmd_argv[self.cmd_argc - 1].is_null());

        let mut working_args: Vec<*mut c_char> = vec![ptr::null_mut(); self.cmd_argc + 1];
        let mut done = 0;
        let mut limit = self.cmd_argc;

        loop {
            let dst_pos = self.copy_args(control, &mut working_args, limit, done);
            if let Some(callback) = &mut control.exec_callback {
                if callback(control, self.usercontext, dst_pos as c_int, working_args.as_mut_ptr()) != 0 {
                    limit = self.update_limit(control, true, limit);
                    done += dst_pos - control.initial_argc;
                } else {
                    if limit <= control.initial_argc + 1 {
                        eprintln!("can't call exec() due to argument size restrictions");
                        process::exit(1);
                    } else {
                        limit = self.update_limit(control, false, limit);
                    }
                }
            }

            if (done + 1) >= (self.cmd_argc - control.initial_argc) {
                break;
            }
        }

        self.clear_args(control);
    }

    fn update_limit(&mut self, control: &BuildCmdControl, success: bool, limit: size_t) -> size_t {
        if success {
            if limit > self.largest_successful_arg_count {
                self.largest_successful_arg_count = limit;
            }
        } else {
            if limit < self.smallest_failed_arg_count || self.smallest_failed_arg_count == 0 {
                self.smallest_failed_arg_count = limit;
            }
        }

        let new_limit = if self.largest_successful_arg_count == 0
            || self.smallest_failed_arg_count <= self.largest_successful_arg_count
        {
            if success {
                if limit < size_t::MAX {
                    limit + 1
                } else {
                    limit
                }
            } else {
                limit / 2
            }
        } else {
            let shift = (self.smallest_failed_arg_count - self.largest_successful_arg_count) / 2;
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
        };

        cmp::max(
            if control.initial_argc != 0 && new_limit <= control.initial_argc + 1 {
                control.initial_argc + 1
            } else if new_limit == 0 {
                1
            } else {
                new_limit
            },
            1,
        )
    }

    fn copy_args(
        &self,
        control: &BuildCmdControl,
        working_args: &mut [*mut c_char],
        limit: size_t,
        done: size_t,
    ) -> size_t {
        let mut dst_pos = 0;
        let mut src_pos = 0;

        while src_pos < control.initial_argc {
            working_args[dst_pos] = self.cmd_argv[src_pos];
            dst_pos += 1;
            src_pos += 1;
        }

        src_pos += done;
        while src_pos < self.cmd_argc && dst_pos < limit {
            working_args[dst_pos] = self.cmd_argv[src_pos];
            dst_pos += 1;
            src_pos += 1;
        }

        working_args[dst_pos] = ptr::null_mut();
        dst_pos
    }
}

pub fn bc_args_exceed_testing_limit(argv: &[*mut c_char]) -> bool {
    let (chars, args) = argv.iter().take_while(|&&arg| !arg.is_null()).fold(
        (0, 0),
        |(chars, args), &arg| {
            (
                chars + unsafe { libc::strlen(arg) },
                args + 1,
            )
        },
    );

    exceeds("__GNU_FINDUTILS_EXEC_ARG_COUNT_LIMIT", args)
        || exceeds("__GNU_FINDUTILS_EXEC_ARG_LENGTH_LIMIT", chars)
}

fn exceeds(env_var_name: &str, quantity: size_t) -> bool {
    if let Ok(val) = env::var(env_var_name) {
        if let Ok(limit) = val.parse::<usize>() {
            return quantity > limit;
        } else {
            eprintln!(
                "Environment variable {} is not set to a valid decimal number",
                env_var_name
            );
            process::exit(1);
        }
    }
    false
}