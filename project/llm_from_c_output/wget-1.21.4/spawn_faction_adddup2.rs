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

use libc::{EBADF, ENOMEM};
use std::os::unix::io::RawFd;

#[cfg(not(REPLACE_POSIX_SPAWN))]
use libc::posix_spawn_file_actions_adddup2;

#[cfg(REPLACE_POSIX_SPAWN)]
use crate::spawn_int::{
    __posix_spawn_file_actions_realloc, posix_spawn_file_actions_t, spawn_action, spawn_do_dup2,
};

/// Add an action to FILE-ACTIONS which tells the implementation to call
/// 'dup2' for the given file descriptors during the 'spawn' call.
pub fn posix_spawn_file_actions_adddup2(
    file_actions: &mut posix_spawn_file_actions_t,
    fd: RawFd,
    newfd: RawFd,
) -> Result<(), i32> {
    let maxfd = unsafe { libc::sysconf(libc::_SC_OPEN_MAX) } as RawFd;

    // Test for the validity of the file descriptor.
    if fd < 0 || newfd < 0 || fd >= maxfd || newfd >= maxfd {
        return Err(EBADF);
    }

    #[cfg(not(REPLACE_POSIX_SPAWN))]
    {
        let ret = unsafe { posix_spawn_file_actions_adddup2(file_actions, fd, newfd) };
        if ret != 0 {
            Err(ret)
        } else {
            Ok(())
        }
    }

    #[cfg(REPLACE_POSIX_SPAWN)]
    {
        // Allocate more memory if needed.
        if file_actions._used == file_actions._allocated
            && unsafe { __posix_spawn_file_actions_realloc(file_actions) } != 0
        {
            // This can only mean we ran out of memory.
            return Err(ENOMEM);
        }

        // Add the new value.
        let rec = &mut file_actions._actions[file_actions._used];
        rec.tag = spawn_do_dup2;
        rec.action.dup2_action.fd = fd;
        rec.action.dup2_action.newfd = newfd;

        // Account for the new entry.
        file_actions._used += 1;

        Ok(())
    }
}