/* Template for the remote job exportation interface to GNU Make.
Copyright (C) 1988-2023 Free Software Foundation, Inc.
This file is part of GNU Make.

GNU Make is free software; you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation; either version 3 of the License, or (at your option) any later
version.

GNU Make is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::CString;
use std::os::unix::process::ExitStatusExt;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

static REMOTE_DESCRIPTION: Option<&str> = None;

/// Call once at startup even if no commands are run.
pub fn remote_setup() {
}

/// Called before exit.
pub fn remote_cleanup() {
}

/// Return true if the next job should be done remotely.
pub fn start_remote_job_p(_first_p: bool) -> bool {
    false
}

/// Start a remote job running the command in `argv`,
/// with environment from `envp`. It gets standard input from `stdin_fd`.
/// On failure, return Err. On success, return Ok containing:
/// - `used_stdin`: true if it will actually use stdin_fd, false if not
/// - `id`: a unique identification
/// - `is_remote`: false if the job is local, true if it is remote
pub fn start_remote_job(
    _argv: &[CString],
    _envp: &[CString],
    _stdin_fd: i32,
) -> Result<(bool, u32, bool), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "not implemented"))
}

/// Get the status of a dead remote child.
/// Block waiting for one to die if `block` is true.
/// Returns:
/// - Ok(Some((id, exit_status, signal, coredump))) if a child died
/// - Ok(None) if we would have to block and !block
/// - Err if there were none
pub fn remote_status(block: bool) -> Result<Option<(u32, Option<i32>, Option<i32>, bool)>, std::io::Error> {
    if block {
        Err(std::io::Error::from_raw_os_error(libc::ECHILD))
    } else {
        Ok(None)
    }
}

/// Block asynchronous notification of remote child death.
/// If this notification is done by raising the child termination
/// signal, do not block that signal.
pub fn block_remote_children() {
}

/// Restore asynchronous notification of remote child death.
/// If this is done by raising the child termination signal,
/// do not unblock that signal.
pub fn unblock_remote_children() {
}

/// Send signal `sig` to child `id`.
/// Return Ok(()) if successful, Err if not.
pub fn remote_kill(id: u32, sig: i32) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "not implemented"))
}