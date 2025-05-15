// An interface to write() that retries after interrupts.
// Copyright (C) 2002, 2009-2021 Free Software Foundation, Inc.
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

// Some system calls may be interrupted and fail with errno = EINTR in the
// following situations:
//   - The process is stopped and restarted (signal SIGSTOP and SIGCONT, user
//     types Ctrl-Z) on some platforms: Mac OS X.
//   - The process receives a signal for which a signal handler was installed
//     with sigaction() with an sa_flags field that does not contain
//     SA_RESTART.
//   - The process receives a signal for which a signal handler was installed
//     with signal() and for which no call to siginterrupt(sig,0) was done,
//     on some platforms: AIX, HP-UX, IRIX, OSF/1, Solaris.
//
// This module provides a wrapper around write() that handles EINTR.

use std::io::{self, Write};
use std::os::unix::io::RawFd;

const SAFE_WRITE_ERROR: usize = usize::MAX;

/// Write up to `count` bytes at `buf` to descriptor `fd`, retrying if interrupted.
/// Return the actual number of bytes written, zero for EOF, or SAFE_WRITE_ERROR
/// upon error.
pub fn safe_write(fd: RawFd, buf: &[u8], count: usize) -> usize {
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
    let result = safe_write_impl(&mut file, &buf[..count.min(buf.len())]);
    std::mem::forget(file); // Prevent closing the file descriptor
    result
}

fn safe_write_impl(file: &mut std::fs::File, buf: &[u8]) -> usize {
    let mut total_written = 0;
    let mut remaining = buf;

    while !remaining.is_empty() {
        match file.write(remaining) {
            Ok(0) => break, // EOF
            Ok(n) => {
                total_written += n;
                remaining = &remaining[n..];
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => return SAFE_WRITE_ERROR,
        }
    }

    total_written
}