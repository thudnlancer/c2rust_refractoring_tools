/*
 * fork.rs - Provide fork and waitpid functions for gawk.
 *
 * Translated from C to Rust with safety and functionality preserved.
 */

/*
 * Copyright (C) 2001, 2004, 2011, 2012, 2013, 2018
 * the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use std::process::{Command, Child, exit};
use std::os::unix::process::CommandExt;
use nix::unistd::{fork, ForkResult, Pid};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::errno::Errno;
use gawkapi::{GawkApi, AwkValue, AwkArray, AwkExtFunc, AwkValueType};

static API: &'static GawkApi = &GawkApi;
static EXT_ID: usize = 0;
static EXT_VERSION: &'static str = "fork extension: version 1.0";
static INIT_FUNC: Option<fn() -> bool> = None;

static PLUGIN_IS_GPL_COMPATIBLE: bool = true;

/* array_set --- set an array element */
fn array_set_numeric(array: &mut AwkArray, sub: &str, num: f64) {
    let index = AwkValue::from_string(sub.to_string());
    let value = AwkValue::from_number(num);
    array.set_element(&index, &value);
}

/* do_fork --- provide dynamically loaded fork() builtin for gawk */
fn do_fork(_nargs: usize, _result: &mut AwkValue, _unused: &AwkExtFunc) -> AwkValue {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            AwkValue::from_number(child.as_raw() as f64)
        }
        Ok(ForkResult::Child) => {
            // Update PROCINFO in the child, if the array exists
            if let Ok(procinfo) = API.sym_lookup("PROCINFO", AwkValueType::Array) {
                if let AwkValue::Array(ref mut array) = procinfo {
                    array_set_numeric(array, "pid", std::process::id() as f64);
                    array_set_numeric(array, "ppid", nix::unistd::getppid().as_raw() as f64);
                } else if API.do_lint() {
                    API.lintwarn(EXT_ID, "fork: PROCINFO is not an array!");
                }
            }
            AwkValue::from_number(0.0)
        }
        Err(e) => {
            API.update_errno_int(e as i32);
            AwkValue::from_number(-1.0)
        }
    }
}

/* do_waitpid --- provide dynamically loaded waitpid() builtin for gawk */
fn do_waitpid(nargs: usize, args: &[AwkValue], _result: &mut AwkValue, _unused: &AwkExtFunc) -> AwkValue {
    if nargs > 0 {
        if let AwkValue::Number(pid) = args[0] {
            let options = WaitPidFlag::WNOHANG | WaitPidFlag::WUNTRACED;
            match waitpid(Pid::from_raw(pid as i32), Some(options)) {
                Ok(WaitStatus::Exited(pid, _)) => AwkValue::from_number(pid.as_raw() as f64),
                Ok(WaitStatus::Signaled(pid, _, _)) => AwkValue::from_number(pid.as_raw() as f64),
                Ok(WaitStatus::Stopped(pid, _)) => AwkValue::from_number(pid.as_raw() as f64),
                Err(e) => {
                    API.update_errno_int(e as i32);
                    AwkValue::from_number(-1.0)
                }
                _ => AwkValue::from_number(0.0),
            }
        } else {
            API.update_errno_int(libc::EINVAL);
            AwkValue::from_number(-1.0)
        }
    } else {
        API.update_errno_int(libc::EINVAL);
        AwkValue::from_number(-1.0)
    }
}

/* do_wait --- provide dynamically loaded wait() builtin for gawk */
fn do_wait(_nargs: usize, _args: &[AwkValue], _result: &mut AwkValue, _unused: &AwkExtFunc) -> AwkValue {
    match waitpid(None, None) {
        Ok(WaitStatus::Exited(pid, _)) => AwkValue::from_number(pid.as_raw() as f64),
        Ok(WaitStatus::Signaled(pid, _, _)) => AwkValue::from_number(pid.as_raw() as f64),
        Ok(WaitStatus::Stopped(pid, _)) => AwkValue::from_number(pid.as_raw() as f64),
        Err(e) => {
            API.update_errno_int(e as i32);
            AwkValue::from_number(-1.0)
        }
        _ => AwkValue::from_number(0.0),
    }
}

static FUNC_TABLE: &[AwkExtFunc] = &[
    AwkExtFunc {
        name: "fork",
        func: do_fork,
        min_args: 0,
        max_args: 0,
        do_lint: false,
        lint_msg: None,
    },
    AwkExtFunc {
        name: "waitpid",
        func: do_waitpid,
        min_args: 1,
        max_args: 1,
        do_lint: false,
        lint_msg: None,
    },
    AwkExtFunc {
        name: "wait",
        func: do_wait,
        min_args: 0,
        max_args: 0,
        do_lint: false,
        lint_msg: None,
    },
];

#[no_mangle]
pub extern "C" fn dl_load(_api: &GawkApi, ext_id: usize) -> bool {
    // Initialize extension
    true
}