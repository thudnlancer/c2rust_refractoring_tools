// Copyright (C) 2000, 2009-2023 Free Software Foundation, Inc.
// This file is part of the GNU C Library.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::{CString, CStr};
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::ptr;
use libc::{pid_t, c_char};

use crate::spawn_int::__spawni;

/// Spawn a new process executing FILE with the attributes describes in *ATTRP.
/// Before running the process perform the actions described in FILE-ACTIONS.
pub fn posix_spawnp(
    pid: &mut pid_t,
    file: &CStr,
    file_actions: Option<&posix_spawn_file_actions_t>,
    attrp: Option<&posix_spawnattr_t>,
    argv: &[&CStr],
    envp: &[&CStr],
) -> Result<(), std::io::Error> {
    let argv_ptr = argv.as_ptr() as *const *const c_char;
    let envp_ptr = envp.as_ptr() as *const *const c_char;
    
    let result = unsafe {
        __spawni(
            pid,
            file.as_ptr(),
            file_actions.map_or(ptr::null(), |p| p),
            attrp.map_or(ptr::null(), |p| p),
            argv_ptr,
            envp_ptr,
            1,
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(std::io::Error::from_raw_os_error(result))
    }
}