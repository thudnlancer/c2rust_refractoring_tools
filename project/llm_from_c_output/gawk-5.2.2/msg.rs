/*
 * msg.rs - routines for error messages.
 */

/*
 * Copyright (C) 1986, 1988, 1989, 1991-2001, 2003, 2010-2013, 2017-2019,
 * 2021, 2022, 2023,
 * the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3, or (at your option)
 * any later version.
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

use std::env;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io::{self, Write};
use std::os::raw::c_int;
use std::panic;
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Once;

static SOURCE_LINE: AtomicI32 = AtomicI32::new(0);
static SOURCE: Option<&'static str> = None;
static SRC_FILE: Option<&'static str> = None;
static SRC_LINE: AtomicI32 = AtomicI32::new(0);

static FATAL_TAG_VALID: AtomicBool = AtomicBool::new(false);
static mut FATAL_TAG: Option<*mut std::ffi::c_void> = None;

static FIRST: Once = Once::new();
static ADD_SRC_INFO: AtomicBool = AtomicBool::new(false);
static LINENO_VAL: AtomicI32 = AtomicI32::new(0);

pub extern "C" fn gawk_exit(status: c_int) {
    if FATAL_TAG_VALID.load(Ordering::SeqCst) {
        unsafe {
            if let Some(tag) = FATAL_TAG {
                panic::resume_unwind(Box::new(status));
            }
        }
    }
    final_exit(status);
}

pub extern "C" fn final_exit(status: c_int) {
    // run any extension exit handlers
    run_ext_exit_handlers(status);

    // we could close_io() here
    close_extensions();

    std::process::exit(status);
}

pub fn err(isfatal: bool, s: &str, emsg: &str, args: fmt::Arguments) {
    FIRST.call_once(|| {
        ADD_SRC_INFO.store(env::var("GAWK_MSG_SRC").is_ok(), Ordering::SeqCst);
        if !do_traditional() {
            if let Some(n) = lookup("LINENO") {
                if n.node_type() == NodeType::Var {
                    LINENO_VAL.store(get_number_d(n.var_value()) as i32, Ordering::SeqCst);
                }
            }
        }
    });

    let _ = io::stdout().flush();
    let me = myname();
    eprint!("{}: ", me);

    if let Some(srcfile) = SRC_FILE.as_ref() && ADD_SRC_INFO.load(Ordering::SeqCst) {
        eprint!("{}:{}:", srcfile, SRC_LINE.load(Ordering::SeqCst));
        unsafe { SRC_FILE = None; }
    }

    let sourceline = SOURCE_LINE.load(Ordering::SeqCst);
    if sourceline > 0 {
        if let Some(source) = SOURCE.as_ref() {
            eprint!("{}:", source);
        } else {
            eprint!("cmd. line:");
        }
        eprint!("{}: ", sourceline + LINENO_VAL.load(Ordering::SeqCst));
    }

    if FNR > 0 {
        if let Some(filename_node) = FILENAME_NODE {
            let file = filename_node.var_value().stptr();
            let len = filename_node.var_value().stlen();
            eprint!("(");
            if !file.is_null() {
                eprint!("FILENAME={} ", unsafe { CStr::from_ptr(file) }.to_string_lossy());
            }
            eprint!("FNR={}) ", FNR);
        }
    }

    eprint!("{}", s);
    eprint!("{}", emsg);
    eprintln!();
    let _ = io::stderr().flush();

    if isfatal {
        gawk_exit(EXIT_FATAL);
    }
}

pub fn msg(mesg: &str) {
    err(false, "", mesg, format_args!(""));
}

pub fn r_warning(mesg: &str) {
    err(false, "warning: ", mesg, format_args!(""));
}

pub fn error(mesg: &str) {
    err(false, "error: ", mesg, format_args!(""));
}

pub fn set_loc(file: &str, line: i32) {
    unsafe {
        SRC_FILE = Some(file);
        SRC_LINE.store(line, Ordering::SeqCst);
    }
}

pub fn r_fatal(mesg: &str) {
    err(true, "fatal: ", mesg, format_args!(""));
}

// Note: The following are placeholder implementations for the FFI and other required functions.
// In a real implementation, these would need to be properly defined.

extern "C" {
    fn do_traditional() -> bool;
    fn lookup(name: *const i8) -> Option<&'static Node>;
    fn get_number_d(node: *const Node) -> i64;
    fn run_ext_exit_handlers(status: c_int);
    fn close_extensions();
    fn myname() -> &'static str;
}

const EXIT_FATAL: c_int = 1;

struct Node {
    // Node implementation details
}

impl Node {
    fn node_type(&self) -> NodeType {
        // Implementation
        NodeType::Var
    }

    fn var_value(&self) -> *const Node {
        // Implementation
        ptr::null()
    }
}

enum NodeType {
    Var,
    // Other variants
}

static FILENAME_NODE: Option<&'static Node> = None;
static mut FNR: i64 = 0;