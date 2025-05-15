// b-complain.rs --- various ways of writing to standard error

// Copyright (C) 2010-2020 Thien-Thi Nguyen
//
// This file is part of GNU RCS.
//
// GNU RCS is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// GNU RCS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::io::{self, Write, stderr};
use std::fmt;
use std::error::Error;
use std::process;

static mut UNBUFFERED: bool = false;
static mut QUIET: bool = false;
static mut ERRONEOUS: bool = false;

pub fn unbuffer_standard_error() {
    unsafe {
        UNBUFFERED = true;
    }
}

pub fn vcomplain(fmt: &str, args: fmt::Arguments) {
    let stderr = stderr();
    let mut handle = stderr.lock();
    
    // TODO: Handle MANI(standard_output) and top condition
    write!(handle, "{}", args).unwrap();
    
    unsafe {
        if !UNBUFFERED {
            handle.flush().unwrap();
        }
    }
}

pub fn complain(fmt: &str) {
    vcomplain(fmt, format_args!("{}", fmt));
}

macro_rules! complain_plus_newline {
    ($fmt:expr, $($arg:tt)*) => {
        {
            vcomplain($fmt, format_args!($($arg)*));
            complain("\n");
        }
    }
}

pub fn diagnose(fmt: &str) {
    unsafe {
        if !QUIET {
            complain_plus_newline!(fmt);
        }
    }
}

fn whoami(who: Option<&str>) {
    // TODO: Handle PROGRAM(name)
    complain("program: ");
    if let Some(w) = who {
        complain(&format!("{}: ", w));
    }
}

fn erroneous_x() {
    unsafe {
        ERRONEOUS = true;
    }
}

pub fn syserror(e: i32, who: &str) {
    whoami(None);
    erroneous_x();
    let err = io::Error::from_raw_os_error(e);
    complain(&format!("{}: {}", who, err));
}

pub fn generic_warn(who: Option<&str>, fmt: &str) {
    unsafe {
        if !QUIET {
            whoami(who);
            complain("warning: ");
            complain_plus_newline!(fmt);
        }
    }
}

pub fn generic_error(who: Option<&str>, fmt: &str) {
    erroneous_x();
    whoami(who);
    complain_plus_newline!(fmt);
}

fn die() {
    // TODO: Handle PROGRAM(name)
    complain("program aborted\n");
    process::exit(1);
}

pub fn generic_fatal(who: Option<&str>, fmt: &str) {
    erroneous_x();
    whoami(who);
    complain_plus_newline!(fmt);
    die();
}

pub fn fatal_syntax(lno: usize, fmt: &str) {
    // TODO: Handle PROGRAM(name) and REPO(filename)
    complain("program: filename:");
    if lno != 0 {
        complain(&format!("{}:", lno));
    }
    complain(" ");
    complain_plus_newline!(fmt);
    die();
}

pub fn fatal_sys(who: &str) {
    syserror(io::Error::last_os_error().raw_os_error().unwrap(), who);
    die();
}

// Idioms
pub fn syserror_errno(who: &str) {
    syserror(io::Error::last_os_error().raw_os_error().unwrap(), who);
}

macro_rules! pwarn {
    ($($arg:tt)*) => {
        generic_warn(None, &format!($($arg)*))
    }
}

macro_rules! mwarn {
    ($($arg:tt)*) => {
        // TODO: Handle MANI(filename)
        generic_warn(Some("filename"), &format!($($arg)*))
    }
}

macro_rules! rwarn {
    ($($arg:tt)*) => {
        // TODO: Handle REPO(filename)
        generic_warn(Some("filename"), &format!($($arg)*))
    }
}

macro_rules! perr {
    ($($arg:tt)*) => {
        generic_error(None, &format!($($arg)*))
    }
}

macro_rules! merr {
    ($($arg:tt)*) => {
        // TODO: Handle MANI(filename)
        generic_error(Some("filename"), &format!($($arg)*))
    }
}

macro_rules! rerr {
    ($($arg:tt)*) => {
        // TODO: Handle REPO(filename)
        generic_error(Some("filename"), &format!($($arg)*))
    }
}

macro_rules! pfatal {
    ($($arg:tt)*) => {
        generic_fatal(None, &format!($($arg)*))
    }
}

macro_rules! rfatal {
    ($($arg:tt)*) => {
        // TODO: Handle REPO(filename)
        generic_fatal(Some("filename"), &format!($($arg)*))
    }
}

macro_rules! syntax_error {
    ($($arg:tt)*) => {
        fatal_syntax(0, &format!($($arg)*))
    }
}