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

use std::alloc::{self, Layout};
use std::ptr;
use std::mem;

/// Spawn file actions structure
pub struct PosixSpawnFileActions {
    actions: *mut SpawnAction,
    allocated: usize,
}

/// Spawn action structure
#[repr(C)]
struct SpawnAction {
    // Fields would be defined here based on actual implementation
}

/// Function used to increase the size of the allocated array.
/// This function is called from the 'add'-functions.
fn posix_spawn_file_actions_realloc(file_actions: &mut PosixSpawnFileActions) -> Result<(), i32> {
    let newalloc = file_actions.allocated + 8;
    let new_size = newalloc * mem::size_of::<SpawnAction>();
    
    let layout = Layout::from_size_align(new_size, mem::align_of::<SpawnAction>())
        .map_err(|_| libc::ENOMEM)?;
    
    let new_ptr = unsafe {
        if file_actions.allocated == 0 {
            alloc::alloc(layout)
        } else {
            let old_size = file_actions.allocated * mem::size_of::<SpawnAction>();
            let old_ptr = file_actions.actions as *mut u8;
            alloc::realloc(old_ptr, Layout::from_size_align(old_size, mem::align_of::<SpawnAction>()).unwrap(), new_size)
        }
    };
    
    if new_ptr.is_null() {
        return Err(libc::ENOMEM);
    }
    
    file_actions.actions = new_ptr as *mut SpawnAction;
    file_actions.allocated = newalloc;
    
    Ok(())
}

/// Initialize data structure for file attribute for 'spawn' call.
pub fn posix_spawn_file_actions_init(file_actions: &mut PosixSpawnFileActions) -> Result<(), i32> {
    // Simply clear all the elements
    file_actions.actions = ptr::null_mut();
    file_actions.allocated = 0;
    Ok(())
}