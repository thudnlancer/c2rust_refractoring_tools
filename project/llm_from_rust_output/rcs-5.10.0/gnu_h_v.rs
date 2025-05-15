use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::process::exit;
use std::ptr;

#[derive(Debug, Clone)]
pub struct Program {
    pub invoke: String,
    pub name: String,
    pub desc: String,
    pub help: String,
    pub tyag: c_int,
}

#[derive(Debug, Clone)]
pub struct Option {
    pub name: String,
    pub has_arg: c_int,
    pub flag: Option<Box<c_int>>,
    pub val: c_int,
}

#[derive(Debug, PartialEq)]
pub enum HvOptionValues {
    Help = 0,
    Version = 1,
}

pub fn nice_getopt(argc: c_int, argv: &[String], longopts: &[Option]) -> c_int {
    unsafe {
        libc::optind = 0;
        libc::opterr = 0;

        let c_argv: Vec<*mut c_char> = argv
            .iter()
            .map(|arg| CString::new(arg.as_str()).unwrap().into_raw())
            .collect();

        let c_longopts: Vec<libc::option> = longopts
            .iter()
            .map(|opt| libc::option {
                name: CString::new(opt.name.as_str()).unwrap().into_raw(),
                has_arg: opt.has_arg,
                flag: match &opt.flag {
                    Some(f) => &**f as *const c_int as *mut c_int,
                    None => ptr::null_mut(),
                },
                val: opt.val,
            })
            .collect();

        let result = libc::getopt_long(
            argc,
            c_argv.as_ptr(),
            b"+\0".as_ptr() as *const c_char,
            c_longopts.as_ptr(),
            ptr::null_mut(),
        );

        for ptr in c_argv {
            drop(CString::from_raw(ptr));
        }
        for opt in c_longopts {
            drop(CString::from_raw(opt.name as *mut c_char));
        }

        result
    }
}

pub fn display_version(prog: &Program, flags: c_int) {
    if flags & 1 != 0 {
        eprintln!("-V is obsolete; instead, use --version");
    }

    println!(
        "{}{}",
        prog.name,
        " (GNU RCS) 5.10.0\nCopyright (C) 2010-2020 Thien-Thi Nguyen\n\
         Copyright (C) 1990-1995 Paul Eggert\n\
         Copyright (C) 1982,1988,1989 Walter F. Tichy, Purdue CS\n\
         License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\n\
         This is free software: you are free to change and redistribute it.\n\
         There is NO WARRANTY, to the extent permitted by law.\n"
    );

    if flags & 2 != 0 {
        exit(0);
    }
}

pub fn check_hv(argc: c_int, argv: &[String], prog: &Program) {
    if argc <= 1 {
        return;
    }

    let options = [
        Option {
            name: "help".to_string(),
            has_arg: 0,
            flag: None,
            val: HvOptionValues::Help as c_int,
        },
        Option {
            name: "version".to_string(),
            has_arg: 0,
            flag: None,
            val: HvOptionValues::Version as c_int,
        },
        Option {
            name: "".to_string(),
            has_arg: 0,
            flag: None,
            val: 0,
        },
    ];

    match nice_getopt(argc, argv, &options) {
        0 => {
            let mut usage = prog.help.clone();
            if let Some(nl) = usage.find('\n') {
                usage.truncate(nl);
            }

            println!(
                "Usage: {} {}\n\n{}\n{}\nReport bugs to: <bug-rcs@gnu.org>\n\
                 RCS home page: <http://www.gnu.org/software/rcs/>\n\
                 General help using GNU software: <http://www.gnu.org/gethelp/>\n",
                prog.name,
                usage,
                prog.desc,
                prog.help.split_once('\n').unwrap_or(("", "")).1
            );
            exit(0);
        }
        1 => {
            display_version(prog, 2);
        }
        _ => {}
    }
}