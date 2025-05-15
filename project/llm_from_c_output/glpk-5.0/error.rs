/* error.rs (error handling) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000-2015 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::fmt;
use std::process;
use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};

type ErrorHook = Box<dyn Fn(&mut ErrorInfo) + Send + Sync>;

struct Env {
    err_st: bool,
    term_out: bool,
    err_file: Option<String>,
    err_line: i32,
    err_hook: Option<ErrorHook>,
    err_info: Option<Box<dyn std::any::Any>>,
    err_buf: String,
}

#[derive(Default)]
struct ErrorInfo {
    msg: String,
}

lazy_static::lazy_static! {
    static ref ENV: Arc<Mutex<Env>> = Arc::new(Mutex::new(Env {
        err_st: false,
        term_out: false,
        err_file: None,
        err_line: 0,
        err_hook: None,
        err_info: None,
        err_buf: String::new(),
    }));
}

fn get_env() -> Arc<Mutex<Env>> {
    ENV.clone()
}

fn errfunc(fmt: &str, args: fmt::Arguments) -> ! {
    let mut env = get_env().lock().unwrap();
    env.err_st = true;
    env.term_out = true;
    
    println!("{}", args);
    println!(
        "Error detected in file {} at line {}",
        env.err_file.as_deref().unwrap_or("unknown"),
        env.err_line
    );
    
    if let Some(hook) = &env.err_hook {
        if let Some(info) = &mut env.err_info {
            hook(info);
        }
    }
    
    process::abort();
}

pub fn glp_error(file: &str, line: i32) -> Box<dyn Fn(&str, fmt::Arguments) -> !> {
    let mut env = get_env().lock().unwrap();
    env.err_file = Some(file.to_string());
    env.err_line = line;
    Box::new(errfunc)
}

pub fn glp_at_error() -> bool {
    let env = get_env().lock().unwrap();
    env.err_st
}

pub fn glp_assert(expr: &str, file: &str, line: i32) -> ! {
    glp_error(file, line)("Assertion failed: {}", format_args!("{}", expr));
}

pub fn glp_error_hook<F>(func: Option<F>, info: Option<Box<dyn std::any::Any>>)
where
    F: Fn(&mut dyn std::any::Any) + 'static + Send + Sync,
{
    let mut env = get_env().lock().unwrap();
    match func {
        Some(f) => {
            env.err_hook = Some(Box::new(move |info: &mut dyn std::any::Any| {
                f(info);
            }));
            env.err_info = info;
        }
        None => {
            env.err_hook = None;
            env.err_info = None;
        }
    }
}

pub fn put_err_msg(msg: &str) {
    let mut env = get_env().lock().unwrap();
    let mut msg = msg.to_string();
    if msg.len() > 0 && msg.ends_with('\n') {
        msg.pop();
    }
    env.err_buf = msg;
}

pub fn get_err_msg() -> String {
    let env = get_env().lock().unwrap();
    env.err_buf.clone()
}