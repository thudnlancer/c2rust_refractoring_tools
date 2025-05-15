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

use libc::posix_spawn_file_actions_t;
use std::ptr;

#[cfg(not(replace_posix_spawn))]
pub fn posix_spawn_file_actions_destroy(file_actions: &mut posix_spawn_file_actions_t) -> i32 {
    unsafe { libc::posix_spawn_file_actions_destroy(file_actions) }
}

#[cfg(replace_posix_spawn)]
pub fn posix_spawn_file_actions_destroy(file_actions: &mut posix_spawn_file_actions_t) -> i32 {
    use libc::{free, spawn_do_chdir, spawn_do_open};
    use std::mem::MaybeUninit;

    // Free the paths in the open actions.
    for i in 0..file_actions._used {
        let sa = &file_actions._actions[i];
        match sa.tag {
            spawn_do_open => {
                if !sa.action.open_action.path.is_null() {
                    unsafe {
                        free(sa.action.open_action.path as *mut _);
                    }
                }
            }
            spawn_do_chdir => {
                if !sa.action.chdir_action.path.is_null() {
                    unsafe {
                        free(sa.action.chdir_action.path as *mut _);
                    }
                }
            }
            _ => {
                // No cleanup required.
            }
        }
    }

    // Free the array of actions.
    if !file_actions._actions.is_null() {
        unsafe {
            free(file_actions._actions as *mut _);
        }
    }

    0
}