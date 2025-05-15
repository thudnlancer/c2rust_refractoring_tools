//! GNUish --help and --version handling

use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::process::exit;
use std::io::{stdout, Write};
use std::ptr;
use libc::{c_int, c_char, EXIT_SUCCESS};
use getopts::Options;

const DV_ONLY: i32 = 0;
const DV_WARN: i32 = 1;
const DV_EXIT: i32 = 2;

pub struct Program {
    pub name: String,
    pub help: String,
    pub desc: String,
    pub tyag: String,
    pub invoke: String,
}

/// Clear 'optind' and 'opterr' then call 'getopt_long', arranging
/// to do not permute 'argv'. Return what 'getopt_long' returns.
pub fn nice_getopt(argc: c_int, argv: *mut *mut c_char, longopts: *const libc::option) -> c_int {
    unsafe {
        libc::optind = 0;
        libc::opterr = 0;
        libc::getopt_long(argc, argv, CString::new("+").unwrap().as_ptr(), longopts, ptr::null_mut())
    }
}

/// Display the version blurb to stdout, starting with:
/// | NAME (GNU RCS) PACKAGE_VERSION
/// | ...
/// and ending with newline. NAME is the value of 'prog.name'.
/// FLAGS is the logical-OR of:
/// | DV_ONLY -- don't do anything special
/// | DV_WARN -- warn that this usage is obsolete (for '-V');
/// |            suggest using --version, instead
/// | DV_EXIT -- finish w/ 'exit(EXIT_SUCCESS)'
/// The default is 0.
pub fn display_version(prog: &Program, flags: i32) {
    if flags & DV_WARN != 0 {
        eprintln!("-V is obsolete; instead, use --version");
    }

    println!("{} {}", prog.name, COMMAND_VERSION);

    if flags & DV_EXIT != 0 {
        exit(EXIT_SUCCESS);
    }
}

const COMMAND_VERSION: &str = concat!(
    " (", env!("PACKAGE_NAME"), ") ", env!("PACKAGE_VERSION"), "\n",
    "Copyright (C) 2010-2020 Thien-Thi Nguyen\n",
    "Copyright (C) 1990-1995 Paul Eggert\n",
    "Copyright (C) 1982,1988,1989 Walter F. Tichy, Purdue CS\n",
    "License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\n",
    "This is free software: you are free to change and redistribute it.\n",
    "There is NO WARRANTY, to the extent permitted by law.\n"
);

const BUGME: &str = concat!(
    "\nReport bugs to: <", env!("PACKAGE_BUGREPORT"), ">\n",
    "RCS home page: <http://www.gnu.org/software/rcs/>\n",
    "General help using GNU software: <http://www.gnu.org/gethelp/>\n"
);

enum HvOptionValues {
    Help,
    Version,
}

/// If ARGC is less than 2, do nothing.
/// If ARGV[1] is "--version", use 'display_version' and exit successfully.
/// If ARGV[1] is "--help", display the help blurb, starting with:
/// | NAME HELP
/// and exit successfully. NAME is the value of 'prog.name',
/// while HELP is the value of 'prog.help'.
pub fn check_hv(args: &[String], prog: &Program) {
    if args.len() <= 1 {
        return;
    }

    let mut opts = Options::new();
    opts.optflag("", "help", "display this help and exit");
    opts.optflag("", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => return,
    };

    if matches.opt_present("help") {
        let help = prog.help.split_once('\n').unwrap_or((&prog.help, ""));
        println!("Usage: {} {}\n\n{}\n{}{}",
                 prog.name, help.0,
                 prog.desc,
                 help.1,
                 BUGME);
        exit(EXIT_SUCCESS);
    } else if matches.opt_present("version") {
        display_version(prog, DV_EXIT);
    }
}

/// Idioms
pub fn nice_opt(name: &str, value: i32) -> libc::option {
    libc::option {
        name: CString::new(name).unwrap().into_raw(),
        has_arg: libc::no_argument,
        flag: ptr::null_mut(),
        val: value,
    }
}

pub const NO_MORE_OPTIONS: libc::option = libc::option {
    name: ptr::null(),
    has_arg: 0,
    flag: ptr::null_mut(),
    val: 0,
};

#[macro_export]
macro_rules! check_hv {
    ($cmd:expr, $argc:expr, $argv:expr, $program:expr) => {
        {
            $program.invoke = $argv[0].clone();
            $program.name = $cmd.to_string();
            check_hv(&$argv, &$program);
        }
    };
}

#[macro_export]
macro_rules! declare_program {
    ($prog:ident, $tyag:expr) => {
        static mut PROGRAM: Program = Program {
            desc: concat!($prog, "_blurb"),
            help: concat!($prog, "_help"),
            tyag: $tyag,
            name: String::new(),
            invoke: String::new(),
        };
    };
}