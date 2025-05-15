use std::env;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::process;
use std::ptr;
use std::str;

mod ffi {
    use std::os::raw::{c_char, c_int, c_void};

    extern "C" {
        pub fn printf(format: *const c_char, ...) -> c_int;
        pub fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
        pub fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
        pub fn strlen(s: *const c_char) -> usize;
        pub fn access(path: *const c_char, mode: c_int) -> c_int;
    }
}

struct Program {
    invoke: *const c_char,
    name: *const c_char,
    desc: *const c_char,
    help: *const c_char,
    tyag: c_int,
}

static mut PROGRAM: Program = Program {
    invoke: ptr::null(),
    name: ptr::null(),
    desc: b"Dispatch an RCS command.\0".as_ptr() as *const c_char,
    help: b"[options] command [command-arg ...]\nOptions:\n  --commands       Display available commands and exit.\n  --aliases        Display command aliases and exit.\n  --help COMMAND   Display help for COMMAND.\n\nTo display help for the legacy interface, use:\n  --help frob\n\0".as_ptr() as *const c_char,
    tyag: 0,
};

fn main() {
    let args: Vec<CString> = env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    
    let mut c_args: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
    c_args.push(ptr::null());

    unsafe {
        let argc = c_args.len() as c_int - 1;
        let argv = c_args.as_ptr() as *mut *mut c_char;
        process::exit(main_0(argc, argv) as i32);
    }
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    if argc == 3 && ffi::strcmp(b"--help\0".as_ptr() as *const c_char, *argv.offset(1)) == 0 {
        let tmp = *argv.offset(2);
        *argv.offset(2) = *argv.offset(1);
        *argv.offset(1) = tmp;
    }

    PROGRAM.invoke = *argv;
    // TODO: Initialize peer_super properly
    // PROGRAM.name = peer_super.meaningful;

    // TODO: Implement check_hv
    // check_hv(argc, argv, &mut PROGRAM);

    // TODO: Implement gnurcs_init
    // gnurcs_init(&PROGRAM);

    if argc < 2 || is_option_short(argv) {
        // TODO: Implement rcs command handling
        // let cmd = b"rcs\0".as_ptr() as *const c_char;
        // let sub = recognize(cmd);
        // dispatch(&mut exitval, sub, cmd, argc, argv);
        0
    } else {
        // TODO: Implement option handling and command dispatch
        0
    }
}

unsafe fn is_option_short(argv: *mut *mut c_char) -> bool {
    let mut i = 1;
    while !(*argv.offset(i)).is_null() {
        let arg = CStr::from_ptr(*argv.offset(i));
        let bytes = arg.to_bytes();
        if bytes.is_empty() || bytes[0] != b'-' {
            break;
        }
        if bytes.len() > 1 && bytes[1] == b'-' {
            return false;
        }
        i += 1;
    }
    true
}

// TODO: Implement remaining functions:
// - recognize
// - dispatch
// - display_commands
// - display_aliases
// - check_hv
// - gnurcs_init
// - gnurcs_goodbye