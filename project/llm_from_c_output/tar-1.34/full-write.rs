// An interface to write() that writes all it is asked to write.

// Copyright (C) 2002-2003, 2009-2021 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;

/// Write COUNT bytes at BUF to descriptor FD, retrying if interrupted
/// or if partial writes occur. Return the number of bytes successfully
/// written, or an error if that is less than COUNT.
pub fn full_write<W: Write>(fd: &mut W, buf: &[u8]) -> io::Result<usize> {
    let mut total = 0;
    let mut remaining = buf;

    while !remaining.is_empty() {
        match fd.write(remaining) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ));
            }
            Ok(n) => {
                total += n;
                remaining = &remaining[n..];
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }

    Ok(total)
}

/// Read COUNT bytes from descriptor FD into BUF, retrying if interrupted
/// or if partial reads occur. Return the number of bytes successfully
/// read, or an error if that is less than COUNT (with errno == 0 meaning EOF).
pub fn full_read<R: Read>(fd: &mut R, buf: &mut [u8]) -> io::Result<usize> {
    let mut total = 0;
    let mut remaining = buf;

    while !remaining.is_empty() {
        match fd.read(remaining) {
            Ok(0) => break, // EOF
            Ok(n) => {
                total += n;
                remaining = &mut remaining[n..];
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }

    Ok(total)
}