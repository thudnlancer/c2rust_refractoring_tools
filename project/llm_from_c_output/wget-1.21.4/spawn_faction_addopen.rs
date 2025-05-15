/* Copyright (C) 2000, 2009-2023 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use libc::{mode_t, c_int};
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::ptr;

#[cfg(not(REPLACE_POSIX_SPAWN))]
extern "C" {
    fn posix_spawn_file_actions_addopen(
        file_actions: *mut posix_spawn_file_actions_t,
        fd: c_int,
        path: *const libc::c_char,
        oflag: c_int,
        mode: mode_t,
    ) -> c_int;
}

#[cfg(REPLACE_POSIX_SPAWN)]
mod spawn_int {
    pub const spawn_do_open: c_int = 0;
    pub struct __spawn_action {
        pub tag: c_int,
        pub action: SpawnAction,
    }
    pub union SpawnAction {
        pub open_action: OpenAction,
    }
    pub struct OpenAction {
        pub fd: c_int,
        pub path: *mut libc::c_char,
        pub oflag: c_int,
        pub mode: mode_t,
    }
    pub type posix_spawn_file_actions_t = FileActions;
    pub struct FileActions {
        pub _actions: *mut __spawn_action,
        pub _used: usize,
        pub _allocated: usize,
    }
    pub fn __posix_spawn_file_actions_realloc(
        file_actions: *mut posix_spawn_file_actions_t,
    ) -> c_int {
        // Implementation omitted for brevity
        0
    }
}

#[cfg(REPLACE_POSIX_SPAWN)]
use spawn_int::*;

pub fn posix_spawn_file_actions_addopen(
    file_actions: &mut posix_spawn_file_actions_t,
    fd: RawFd,
    path: &str,
    oflag: c_int,
    mode: mode_t,
) -> Result<(), c_int> {
    let maxfd = unsafe { libc::sysconf(libc::_SC_OPEN_MAX) };
    if maxfd == -1 {
        return Err(libc::EBADF);
    }

    if fd < 0 || fd >= maxfd as RawFd {
        return Err(libc::EBADF);
    }

    #[cfg(not(REPLACE_POSIX_SPAWN))]
    {
        let path_cstr = CString::new(path).map_err(|_| libc::ENOMEM)?;
        let ret = unsafe {
            posix_spawn_file_actions_addopen(
                file_actions,
                fd,
                path_cstr.as_ptr(),
                oflag,
                mode,
            )
        };
        if ret != 0 {
            Err(ret)
        } else {
            Ok(())
        }
    }

    #[cfg(REPLACE_POSIX_SPAWN)]
    {
        let path_cstr = CString::new(path).map_err(|_| libc::ENOMEM)?;
        let path_copy = path_cstr.into_raw();

        if file_actions._used == file_actions._allocated {
            if unsafe { __posix_spawn_file_actions_realloc(file_actions) } != 0 {
                unsafe { libc::free(path_copy as *mut libc::c_void) };
                return Err(libc::ENOMEM);
            }
        }

        unsafe {
            let rec = &mut *file_actions._actions.add(file_actions._used);
            rec.tag = spawn_do_open;
            rec.action.open_action.fd = fd;
            rec.action.open_action.path = path_copy;
            rec.action.open_action.oflag = oflag;
            rec.action.open_action.mode = mode;

            file_actions._used += 1;
        }

        Ok(())
    }
}