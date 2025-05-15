/* listfile.rs -- run a function in a specific directory
   Copyright (C) 2007-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::os::unix::io::RawFd;
use std::io::{Error, Result};
use nix::fcntl::{openat, FcntlArg};
use nix::unistd::{fchdir, chdir};
use nix::errno::Errno;

pub struct SavedCwd {
    // This would contain the actual saved directory state
    // Implementation depends on the actual save_cwd/restore_cwd functionality
}

impl SavedCwd {
    pub fn save() -> Result<Self> {
        // Implementation of directory saving
        Err(Error::last_os_error())
    }

    pub fn restore(&self) -> Result<()> {
        // Implementation of directory restoration
        Err(Error::last_os_error())
    }

    pub fn free(self) {
        // Cleanup implementation
    }
}

pub fn run_in_dir<F>(there: &SavedCwd, callback: F, usercontext: &mut dyn std::any::Any) -> Result<i32> 
where
    F: FnOnce(&mut dyn std::any::Any) -> Result<i32>,
{
    let here = match SavedCwd::save() {
        Ok(cwd) => cwd,
        Err(e) => {
            openat_save_fail(e.raw_os_error().unwrap_or(0));
            return Err(e);
        }
    };

    let result = match there.restore() {
        Ok(_) => {
            let res = callback(usercontext);
            match res {
                Ok(val) => Ok(val),
                Err(e) => {
                    let saved_errno = e.raw_os_error().unwrap_or(0);
                    if saved_errno != 0 {
                        Err(Error::from_raw_os_error(saved_errno))
                    } else {
                        Ok(-1)
                    }
                }
            }
        }
        Err(e) => {
            openat_restore_fail(e.raw_os_error().unwrap_or(0));
            Err(e)
        }
    };

    if let Err(e) = here.restore() {
        openat_restore_fail(e.raw_os_error().unwrap_or(0));
    }

    here.free();
    result
}

pub fn run_in_dirfd<F>(dir_fd: RawFd, callback: F, usercontext: &mut dyn std::any::Any) -> Result<i32>
where
    F: FnOnce(&mut dyn std::any::Any) -> Result<i32>,
{
    if dir_fd == nix::fcntl::AT_FDCWD {
        callback(usercontext)
    } else {
        let saved_cwd = match SavedCwd::save() {
            Ok(cwd) => cwd,
            Err(e) => {
                openat_save_fail(e.raw_os_error().unwrap_or(0));
                return Err(e);
            }
        };

        if let Err(e) = fchdir(dir_fd) {
            let saved_errno = e as i32;
            saved_cwd.free();
            return Err(Error::from_raw_os_error(saved_errno));
        }

        let result = callback(usercontext);
        let saved_errno = match &result {
            Ok(_) => 0,
            Err(e) => e.raw_os_error().unwrap_or(0),
        };

        if let Err(e) = saved_cwd.restore() {
            openat_restore_fail(e.raw_os_error().unwrap_or(0));
        }

        saved_cwd.free();

        if saved_errno != 0 {
            Err(Error::from_raw_os_error(saved_errno))
        } else {
            result
        }
    }
}

fn openat_save_fail(errno: i32) {
    // Handle save failure
}

fn openat_restore_fail(errno: i32) {
    // Handle restore failure
}