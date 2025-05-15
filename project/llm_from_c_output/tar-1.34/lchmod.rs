/* Implement lchmod on platforms where it does not work correctly.

   Copyright 2020-2021 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* written by Paul Eggert */

use std::fs;
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use std::path::Path;
use std::io;

/// Work like chmod, except when FILE is a symbolic link.
/// In that case, on systems where permissions on symbolic links are unsupported
/// (such as Linux), set errno to EOPNOTSUPP and return -1.
pub fn lchmod(file: &Path, mode: u32) -> io::Result<()> {
    #[cfg(all(target_os = "linux", feature = "procfs"))]
    {
        use std::os::fd::{AsRawFd, FromRawFd};
        use std::ffi::CString;
        use std::fs::File;

        // Open a file descriptor with O_NOFOLLOW, to make sure we don't
        // follow symbolic links, if /proc is mounted. O_PATH is used to
        // avoid a failure if the file is not readable.
        let fd = fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_PATH | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(file)?;

        // Check if it's a symlink
        let metadata = fd.metadata()?;
        if metadata.file_type().is_symlink() {
            return Err(io::Error::from_raw_os_error(libc::EOPNOTSUPP));
        }

        // Use /proc/self/fd/ approach on Linux
        let proc_path = format!("/proc/self/fd/{}", fd.as_raw_fd());
        match fs::set_permissions(proc_path, fs::Permissions::from_mode(mode)) {
            Ok(_) => Ok(()),
            Err(e) if e.raw_os_error() == Some(libc::ENOENT) => {
                // Fall through to regular chmod if /proc doesn't exist
                fs::set_permissions(file, fs::Permissions::from_mode(mode))
            }
            Err(e) => Err(e),
        }
    }

    #[cfg(not(all(target_os = "linux", feature = "procfs")))]
    {
        // Fallback implementation using lstat
        let metadata = fs::symlink_metadata(file)?;
        if metadata.file_type().is_symlink() {
            return Err(io::Error::from_raw_os_error(libc::EOPNOTSUPP));
        }

        // Regular chmod
        fs::set_permissions(file, fs::Permissions::from_mode(mode))
    }
}