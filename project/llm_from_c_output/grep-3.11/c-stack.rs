// Stack overflow handling.
//
// Copyright (C) 2002, 2004, 2008-2023 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use libc::{c_int, c_void, SIGSEGV, STDERR_FILENO};
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::process;
use std::io::{self, Write};

type SegvHandler = extern fn(*mut c_void) -> c_int;
type OverflowHandler = extern fn(c_int, *mut c_void) -> ();

extern {
    fn stackoverflow_install_handler(
        handler: OverflowHandler,
        stack: *mut c_void,
        size: usize
    ) -> c_int;
    fn sigsegv_install_handler(handler: SegvHandler) -> c_int;
}

static mut SEGV_ACTION: Option<extern fn(c_int)> = None;
static mut PROGRAM_ERROR_MESSAGE: Option<CString> = None;
static mut STACK_OVERFLOW_MESSAGE: Option<CString> = None;
static mut PROGNAME: Option<CString> = None;
static mut SEGV_HANDLER_MISSING: c_int = 0;

const ALTERNATE_STACK_SIZE: usize = 64 * 1024;
static mut ALTERNATE_SIGNAL_STACK: [u8; ALTERNATE_STACK_SIZE] = [0; ALTERNATE_STACK_SIZE];

extern fn null_action(_: c_int) {}

#[no_mangle]
pub extern fn die(signo: c_int) -> ! {
    unsafe {
        if let Some(action) = SEGV_ACTION {
            action(signo);
        }

        let message = if signo != 0 {
            PROGRAM_ERROR_MESSAGE.as_ref().unwrap()
        } else {
            STACK_OVERFLOW_MESSAGE.as_ref().unwrap()
        };

        let progname = PROGNAME.as_ref().unwrap();
        let separator = b": ";

        let mut stderr = io::stderr();
        let _ = stderr.write_all(progname.as_bytes());
        let _ = stderr.write_all(separator);
        let _ = stderr.write_all(message.as_bytes());
        let _ = stderr.write_all(b"\n");

        if signo == 0 {
            process::exit(1);
        } else {
            libc::raise(SIGSEGV);
            process::abort();
        }
    }
}

extern fn segv_handler(_: *mut c_void, serious: c_int) -> c_int {
    if serious == 0 {
        return 0;
    }
    die(SIGSEGV);
}

extern fn overflow_handler(emergency: c_int, _: *mut c_void) {
    unsafe {
        die(if emergency == 0 || SEGV_HANDLER_MISSING != 0 {
            0
        } else {
            SIGSEGV
        });
    }
}

#[no_mangle]
pub extern fn c_stack_action(action: Option<extern fn(c_int)>) -> c_int {
    unsafe {
        SEGV_ACTION = Some(action.unwrap_or(null_action));
        
        PROGRAM_ERROR_MESSAGE = Some(CString::new("program error").unwrap());
        STACK_OVERFLOW_MESSAGE = Some(CString::new("stack overflow").unwrap());
        
        let progname = std::env::current_exe()
            .ok()
            .and_then(|p| p.file_name().map(|s| s.to_owned()))
            .unwrap_or_else(|| std::ffi::OsString::from(""));
        PROGNAME = Some(CString::new(progname.to_string_lossy().into_owned()).unwrap());

        if stackoverflow_install_handler(
            overflow_handler,
            ALTERNATE_SIGNAL_STACK.as_mut_ptr() as *mut c_void,
            ALTERNATE_STACK_SIZE
        ) != 0 {
            return -1;
        }

        SEGV_HANDLER_MISSING = sigsegv_install_handler(segv_handler);
        0
    }
}