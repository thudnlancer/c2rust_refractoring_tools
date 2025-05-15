// Test whether ACLs are well supported on this system.
//
// Copyright 2013-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// Written by Paul Eggert.

use std::io::Error;

/// Return true if errno value indicates that ACLs are well
/// supported on this system. The error should be obtained
/// after an ACL-related system call fails.
pub fn acl_errno_valid(err: Error) -> bool {
    // Recognize some common errors such as from an NFS mount that does
    // not support ACLs, even when local drives do.
    match err.raw_os_error() {
        Some(libc::EBUSY) => false,
        Some(libc::EINVAL) => false,
        #[cfg(target_os = "macos")]
        Some(libc::ENOENT) => false,
        Some(libc::ENOSYS) => false,
        #[cfg(all(target_os = "windows", not(target_env = "gnu")))]
        Some(libc::ENOTSUP) => false,
        Some(libc::EOPNOTSUPP) => false,
        _ => true,
    }
}