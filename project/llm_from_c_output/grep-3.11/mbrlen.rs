// Recognize multibyte character.
// Copyright (C) 1999-2000, 2008-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2008.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io;
use std::str;

static mut INTERNAL_STATE: std::mem::MaybeUninit<std::ffi::c_void> = std::mem::MaybeUninit::uninit();

pub fn mbrlen(s: &[u8], ps: Option<&mut std::ffi::c_void>) -> Result<usize, io::Error> {
    let state = match ps {
        Some(ps) => ps,
        None => unsafe {
            // Safety: We're using MaybeUninit to handle the static mut safely
            if INTERNAL_STATE.as_mut_ptr().is_null() {
                INTERNAL_STATE = std::mem::MaybeUninit::new(std::ffi::c_void {});
            }
            &mut *INTERNAL_STATE.as_mut_ptr()
        }
    };

    let result = std::str::from_utf8(s)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?;
    
    if result.is_empty() {
        Ok(0)
    } else {
        Ok(result.chars().next().unwrap().len_utf8())
    }
}