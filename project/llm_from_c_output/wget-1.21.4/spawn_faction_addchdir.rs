/* Copyright (C) 2018-2023 Free Software Foundation, Inc.

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

use libc::{posix_spawn_file_actions_t, ENOMEM};
use std::ffi::CString;
use std::ptr;

#[cfg(not(REPLACE_POSIX_SPAWN))]
pub fn posix_spawn_file_actions_addchdir(
    file_actions: *mut posix_spawn_file_actions_t,
    path: &str,
) -> i32 {
    let c_path = match CString::new(path) {
        Ok(s) => s,
        Err(_) => return ENOMEM,
    };
    unsafe { posix_spawn_file_actions_addchdir_np(file_actions, c_path.as_ptr()) }
}

#[cfg(REPLACE_POSIX_SPAWN)]
pub fn posix_spawn_file_actions_addchdir(
    file_actions: *mut posix_spawn_file_actions_t,
    path: &str,
) -> i32 {
    use spawn_int::{__posix_spawn_file_actions_realloc, __spawn_action, spawn_do_chdir};

    // Copy PATH, because the caller may free it before calling posix_spawn()
    // or posix_spawnp().
    let path_copy = match CString::new(path) {
        Ok(s) => s,
        Err(_) => return ENOMEM,
    };

    // Allocate more memory if needed.
    unsafe {
        if (*file_actions)._used == (*file_actions)._allocated
            && __posix_spawn_file_actions_realloc(file_actions) != 0
        {
            // This can only mean we ran out of memory.
            return ENOMEM;
        }

        // Add the new value.
        let rec = &mut (*file_actions)._actions[(*file_actions)._used];
        rec.tag = spawn_do_chdir;
        rec.action.chdir_action.path = path_copy.into_raw();

        // Account for the new entry.
        (*file_actions)._used += 1;

        0
    }
}