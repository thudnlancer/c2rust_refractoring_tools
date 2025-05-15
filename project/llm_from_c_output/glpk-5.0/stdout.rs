/* stdout.rs (terminal output) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000-2013 Free Software Foundation, Inc.
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
use std::io::{self, Write};
use std::ffi::CString;
use std::os::raw::c_char;

pub struct Env {
    term_out: bool,
    term_hook: Option<Box<dyn FnMut(*mut std::ffi::c_void, &str) -> i32>>,
    term_info: *mut std::ffi::c_void,
    term_buf: String,
    tee_file: Option<Box<dyn Write>>,
}

impl Env {
    const TBUF_SIZE: usize = 4096;

    pub fn get_env_ptr() -> &'static mut Env {
        // Implementation depends on how the environment is managed
        // This is a placeholder for the actual implementation
        unsafe {
            static mut ENV: Env = Env {
                term_out: true,
                term_hook: None,
                term_info: std::ptr::null_mut(),
                term_buf: String::with_capacity(Self::TBUF_SIZE),
                tee_file: None,
            };
            &mut ENV
        }
    }
}

/***********************************************************************
*  NAME
*
*  glp_puts - write string on terminal
*
*  SYNOPSIS
*
*  void glp_puts(const char *s);
*
*  The routine glp_puts writes the string s on the terminal. */

pub fn glp_puts(s: &str) {
    let env = Env::get_env_ptr();
    // if terminal output is disabled, do nothing
    if !env.term_out {
        return;
    }
    // pass the string to the hook routine, if defined
    if let Some(ref mut hook) = env.term_hook {
        if hook(env.term_info, s) != 0 {
            return;
        }
    }
    // write the string on the terminal
    print!("{}", s);
    io::stdout().flush().unwrap();
    // write the string on the tee file, if required
    if let Some(ref mut file) = env.tee_file {
        write!(file, "{}", s).unwrap();
        file.flush().unwrap();
    }
}

/***********************************************************************
*  NAME
*
*  glp_printf - write formatted output on terminal
*
*  SYNOPSIS
*
*  void glp_printf(const char *fmt, ...);
*
*  DESCRIPTION
*
*  The routine glp_printf uses the format control string fmt to format
*  its parameters and writes the formatted output on the terminal. */

#[macro_export]
macro_rules! glp_printf {
    ($($arg:tt)*) => {{
        let env = $crate::Env::get_env_ptr();
        // if terminal output is disabled, do nothing
        if !env.term_out {
            return;
        }
        // format the output
        env.term_buf.clear();
        fmt::write(&mut env.term_buf, format_args!($($arg)*)).unwrap();
        assert!(env.term_buf.len() < Env::TBUF_SIZE);
        // write the formatted output on the terminal
        glp_puts(&env.term_buf);
    }};
}

/***********************************************************************
*  NAME
*
*  glp_vprintf - write formatted output on terminal
*
*  SYNOPSIS
*
*  void glp_vprintf(const char *fmt, va_list arg);
*
*  DESCRIPTION
*
*  The routine glp_vprintf uses the format control string fmt to format
*  its parameters specified by the list arg and writes the formatted
*  output on the terminal. */

pub fn glp_vprintf(fmt: &str, args: fmt::Arguments) {
    let env = Env::get_env_ptr();
    // if terminal output is disabled, do nothing
    if !env.term_out {
        return;
    }
    // format the output
    env.term_buf.clear();
    fmt::write(&mut env.term_buf, args).unwrap();
    assert!(env.term_buf.len() < Env::TBUF_SIZE);
    // write the formatted output on the terminal
    glp_puts(&env.term_buf);
}

/***********************************************************************
*  NAME
*
*  glp_term_out - enable/disable terminal output
*
*  SYNOPSIS
*
*  int glp_term_out(int flag);
*
*  DESCRIPTION
*
*  Depending on the parameter flag the routine glp_term_out enables or
*  disables terminal output performed by glpk routines:
*
*  GLP_ON  - enable terminal output;
*  GLP_OFF - disable terminal output.
*
*  RETURNS
*
*  The routine glp_term_out returns the previous value of the terminal
*  output flag. */

pub const GLP_ON: i32 = 1;
pub const GLP_OFF: i32 = 0;

pub fn glp_term_out(flag: i32) -> i32 {
    let env = Env::get_env_ptr();
    let old = if env.term_out { GLP_ON } else { GLP_OFF };
    if !(flag == GLP_ON || flag == GLP_OFF) {
        panic!("glp_term_out: flag = {}; invalid parameter", flag);
    }
    env.term_out = flag == GLP_ON;
    old
}

/***********************************************************************
*  NAME
*
*  glp_term_hook - install hook to intercept terminal output
*
*  SYNOPSIS
*
*  void glp_term_hook(int (*func)(void *info, const char *s),
*     void *info);
*
*  DESCRIPTION
*
*  The routine glp_term_hook installs a user-defined hook routine to
*  intercept all terminal output performed by glpk routines. */

pub fn glp_term_hook<F>(func: Option<F>, info: *mut std::ffi::c_void)
where
    F: FnMut(*mut std::ffi::c_void, &str) -> i32 + 'static,
{
    let env = Env::get_env_ptr();
    if let Some(f) = func {
        env.term_hook = Some(Box::new(f));
        env.term_info = info;
    } else {
        env.term_hook = None;
        env.term_info = std::ptr::null_mut();
    }
}

/***********************************************************************
*  NAME
*
*  glp_open_tee - start copying terminal output to text file
*
*  SYNOPSIS
*
*  int glp_open_tee(const char *name);
*
*  DESCRIPTION
*
*  The routine glp_open_tee starts copying all the terminal output to
*  an output text file, whose name is specified by the character string
*  name.
*
*  RETURNS
*
*  0 - operation successful
*  1 - copying terminal output is already active
*  2 - unable to create output file */

pub fn glp_open_tee(name: &str) -> i32 {
    let env = Env::get_env_ptr();
    if env.tee_file.is_some() {
        // copying terminal output is already active
        return 1;
    }
    match std::fs::File::create(name) {
        Ok(file) => {
            env.tee_file = Some(Box::new(file));
            0
        }
        Err(_) => {
            // unable to create output file
            2
        }
    }
}

/***********************************************************************
*  NAME
*
*  glp_close_tee - stop copying terminal output to text file
*
*  SYNOPSIS
*
*  int glp_close_tee(void);
*
*  DESCRIPTION
*
*  The routine glp_close_tee stops copying the terminal output to the
*  output text file previously open by the routine glp_open_tee closing
*  that file.
*
*  RETURNS
*
*  0 - operation successful
*  1 - copying terminal output was not started */

pub fn glp_close_tee() -> i32 {
    let env = Env::get_env_ptr();
    if env.tee_file.is_none() {
        // copying terminal output was not started
        return 1;
    }
    env.tee_file = None;
    0
}