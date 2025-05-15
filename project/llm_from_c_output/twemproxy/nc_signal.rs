/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use libc::{c_int, SIGUSR1, SIGUSR2, SIGTTIN, SIGTTOU, SIGHUP, SIGINT, SIGSEGV, SIGPIPE};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr;
use std::process;

#[derive(Debug)]
struct SignalConfig {
    signo: c_int,
    signame: &'static str,
    flags: SaFlags,
    handler: SigHandler,
}

impl SignalConfig {
    const fn new(signo: c_int, signame: &'static str, flags: SaFlags, handler: SigHandler) -> Self {
        SignalConfig {
            signo,
            signame,
            flags,
            handler,
        }
    }
}

const SIGNALS: &[SignalConfig] = &[
    SignalConfig::new(SIGUSR1, "SIGUSR1", SaFlags::empty(), SigHandler::Handler(signal_handler)),
    SignalConfig::new(SIGUSR2, "SIGUSR2", SaFlags::empty(), SigHandler::Handler(signal_handler)),
    SignalConfig::new(SIGTTIN, "SIGTTIN", SaFlags::empty(), SigHandler::Handler(signal_handler)),
    SignalConfig::new(SIGTTOU, "SIGTTOU", SaFlags::empty(), SigHandler::Handler(signal_handler)),
    SignalConfig::new(SIGHUP, "SIGHUP", SaFlags::empty(), SigHandler::Handler(signal_handler)),
    SignalConfig::new(SIGINT, "SIGINT", SaFlags::empty(), SigHandler::Handler(signal_handler)),
    SignalConfig::new(SIGSEGV, "SIGSEGV", SaFlags::SA_RESETHAND, SigHandler::Handler(signal_handler)),
    SignalConfig::new(SIGPIPE, "SIGPIPE", SaFlags::empty(), SigHandler::SigIgn),
];

pub fn signal_init() -> Result<(), String> {
    for sig in SIGNALS {
        let sa = SigAction::new(
            sig.handler,
            sig.flags,
            SigSet::empty(),
        );

        if let Err(e) = sigaction(Signal::from_c_int(sig.signo).unwrap(), &sa) {
            return Err(format!("sigaction({}) failed: {}", sig.signame, e));
        }
    }

    Ok(())
}

pub fn signal_deinit() {
    // No cleanup needed in Rust
}

extern "C" fn signal_handler(signo: c_int) {
    let sig = match SIGNALS.iter().find(|s| s.signo == signo) {
        Some(s) => s,
        None => {
            log_safe("Unknown signal received: {}", signo);
            process::exit(1);
        }
    };

    let mut actionstr = "";
    let mut action: Option<fn()> = None;
    let mut done = false;

    match signo {
        SIGUSR1 | SIGUSR2 => (),
        SIGTTIN => {
            actionstr = ", up logging level";
            action = Some(log_level_up);
        }
        SIGTTOU => {
            actionstr = ", down logging level";
            action = Some(log_level_down);
        }
        SIGHUP => {
            actionstr = ", reopening log file";
            action = Some(log_reopen);
        }
        SIGINT => {
            done = true;
            actionstr = ", exiting";
        }
        SIGSEGV => {
            log_stacktrace();
            actionstr = ", core dumping";
            unsafe { libc::raise(SIGSEGV) };
        }
        _ => unreachable!(),
    };

    log_safe("signal {} ({}) received{}", signo, sig.signame, actionstr);

    if let Some(act) = action {
        act();
    }

    if done {
        process::exit(1);
    }
}

// Placeholder functions - these should be implemented elsewhere
fn log_safe(_msg: &str, _args: std::fmt::Arguments) {}
fn log_level_up() {}
fn log_level_down() {}
fn log_reopen() {}
fn log_stacktrace() {}