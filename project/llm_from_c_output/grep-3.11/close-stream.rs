// Close a stream, with nicer error checking than fclose's.

// Copyright (C) 2006-2023 Free Software Foundation, Inc.

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
/// A failure with error kind BrokenPipe may or may not indicate an error
/// situation worth signaling to the user.
///
/// If a program writes *anything* to STREAM, that program should close
/// STREAM and make sure that it succeeds before exiting. Otherwise,
/// suppose that you go to the extreme of checking the return status
/// of every function that does an explicit write to STREAM. The last
/// write can succeed in writing to the internal stream buffer, and yet
/// the close could still fail (due e.g., to a disk full error)
/// when it tries to write out that buffered data. Thus, you would be
/// left with an incomplete output file and the offending program would
/// exit successfully.
pub fn close_stream(stream: &mut File) -> io::Result<()> {
    let some_pending = stream.buffer().len() != 0;
    let prev_fail = stream.sync_all().is_err();
    
    match stream.sync_all() {
        Ok(_) => {
            if prev_fail {
                Err(io::Error::new(ErrorKind::Other, "previous write error"))
            } else {
                Ok(())
            }
        }
        Err(e) => {
            if prev_fail || (some_pending || e.kind() != ErrorKind::InvalidInput) {
                Err(e)
            } else {
                Ok(())
            }
        }
    }
}