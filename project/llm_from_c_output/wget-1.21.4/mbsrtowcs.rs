/* Convert string to wide string.
   Copyright (C) 2008-2023 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2008.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::io::{Error, ErrorKind};
use std::convert::TryInto;

extern "C" {
    static mut _gl_mbsrtowcs_state: std::ffi::c_void;
}

pub fn mbsrtowcs(
    dest: Option<&mut [u32]>,
    src: &mut *const c_char,
    n: usize,
    state: &mut std::ffi::c_void,
) -> Result<usize, Error> {
    if src.is_null() {
        return Err(Error::new(ErrorKind::InvalidInput, "Null pointer"));
    }

    let s = unsafe { CStr::from_ptr(*src) };
    let bytes = s.to_bytes();
    
    if bytes.is_empty() {
        *src = ptr::null();
        return Ok(0);
    }

    let mut count = 0;
    let mut remaining = n;
    let mut iter = bytes.iter().peekable();
    
    while remaining > 0 && iter.peek().is_some() {
        let mut mb_char = [0u8; 4];
        let mut len = 0;
        
        for &byte in iter.by_ref().take(4) {
            mb_char[len] = byte;
            len += 1;
            if len >= mb_char.len() || (byte & 0xC0) != 0x80 {
                break;
            }
        }

        let wide_char = match std::char::decode_utf8(mb_char[..len].iter().cloned()) {
            Ok(c) => c as u32,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Invalid UTF-8 sequence")),
        };

        if let Some(ref mut dest_slice) = dest {
            if count < dest_slice.len() {
                dest_slice[count] = wide_char;
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "Destination buffer too small"));
            }
        }

        count += 1;
        remaining -= 1;
    }

    if iter.peek().is_none() {
        *src = ptr::null();
    } else {
        *src = unsafe { (*src).add(bytes.len() - iter.count()) };
    }

    Ok(count)
}