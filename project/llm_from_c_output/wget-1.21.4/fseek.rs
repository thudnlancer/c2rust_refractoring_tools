// An fseek() function that, together with fflush(), is POSIX compliant.
// Copyright (C) 2007, 2009-2023 Free Software Foundation, Inc.
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

use std::io::{Seek, SeekFrom};
use std::io::Result as IoResult;

pub fn fseek<F: Seek>(fp: &mut F, offset: i64, whence: i32) -> IoResult<()> {
    let seek_from = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid whence value"
        )),
    };
    
    fp.seek(seek_from).map(|_| ())
}