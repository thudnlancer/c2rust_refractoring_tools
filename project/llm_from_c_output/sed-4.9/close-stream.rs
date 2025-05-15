// Close a stream, with nicer error checking than fclose's.

// Copyright (C) 2006-2022 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{self, Write, ErrorKind};
use std::fs::File;

/// Close a stream, with nicer error checking than fclose's.
///
/// Returns Ok(()) if successful, Err(io::Error) otherwise.
///
/// A failure might set the error kind to Other with no underlying error if the
/// error number cannot be determined.
///
/// A failure with error kind BrokenPipe may or may not indicate an error
/// situation worth signaling to the user.
///
/// If a program writes *anything* to STREAM, that program should close
/// STREAM and make sure that it succeeds before exiting.
pub fn close_stream(stream: &mut File) -> io::Result<()> {
    let some_pending = stream.buffer().map(|b| !b.is_empty()).unwrap_or(false);
    let prev_fail = stream.sync_all().is_err();
    let fclose_fail = stream.sync_all().is_err();

    // Return an error indication if there was a previous failure or if
    // fclose failed, with one exception: ignore an fclose failure if
    // there was no previous error, no data remains to be flushed, and
    // fclose failed with EBADF.
    if prev_fail || (fclose_fail && (some_pending || io::Error::last_os_error().kind() != ErrorKind::InvalidInput)) {
        if !fclose_fail {
            return Err(io::Error::new(ErrorKind::Other, "stream error"));
        }
        return Err(io::Error::last_os_error());
    }

    Ok(())
}