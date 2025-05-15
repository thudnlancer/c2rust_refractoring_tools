// Query the name of the current global locale.
// Copyright (C) 2019-2023 Free Software Foundation, Inc.
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

// Written by Bruno Haible <bruno@clisp.org>, 2019.

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;
use std::sync::Mutex;
use std::thread_local;

// Recommended buffer sizes
pub const SETLOCALE_NULL_MAX: usize = 256 + 1;
pub const SETLOCALE_NULL_ALL_MAX: usize = 148 + 12 * 256 + 1;

thread_local! {
    static LC_RESULT_BUFFERS: [Mutex<Option<Box<[u8; SETLOCALE_NULL_MAX]>>; LC_INDICES_COUNT] = 
        Default::default();
}

const LC_INDICES_COUNT: usize = 12; // Adjust based on actual LC categories supported

#[derive(Debug)]
pub enum SetLocaleError {
    InvalidCategory,
    BufferTooSmall,
}

fn setlocale_null_androidfix(category: c_int) -> Option<&'static str> {
    unsafe {
        let result = libc::setlocale(category, ptr::null());
        if result.is_null() {
            #[cfg(target_os = "android")]
            {
                match category {
                    libc::LC_CTYPE
                    | libc::LC_NUMERIC
                    | libc::LC_TIME
                    | libc::LC_COLLATE
                    | libc::LC_MONETARY
                    | libc::LC_MESSAGES
                    | libc::LC_ALL
                    | libc::LC_PAPER
                    | libc::LC_NAME
                    | libc::LC_ADDRESS
                    | libc::LC_TELEPHONE
                    | libc::LC_MEASUREMENT => Some("C"),
                    _ => None,
                }
            }
            #[cfg(not(target_os = "android"))]
            None
        } else {
            CStr::from_ptr(result).to_str().ok()
        }
    }
}

pub fn setlocale_null_r(
    category: c_int,
    buf: &mut [u8],
) -> Result<(), SetLocaleError> {
    if buf.is_empty() {
        return Err(SetLocaleError::BufferTooSmall);
    }

    let result = match setlocale_null_androidfix(category) {
        Some(s) => s,
        None => {
            buf[0] = 0;
            return Err(SetLocaleError::InvalidCategory);
        }
    };

    let bytes = result.as_bytes();
    if bytes.len() < buf.len() {
        buf[..bytes.len()].copy_from_slice(bytes);
        buf[bytes.len()] = 0;
        Ok(())
    } else {
        let copy_len = buf.len() - 1;
        buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
        buf[copy_len] = 0;
        Err(SetLocaleError::BufferTooSmall)
    }
}

pub fn setlocale_null(category: c_int) -> Option<&'static str> {
    if category == libc::LC_ALL {
        setlocale_null_androidfix(libc::LC_ALL)
    } else {
        LC_RESULT_BUFFERS.with(|buffers| {
            let index = match category {
                libc::LC_CTYPE => 0,
                libc::LC_NUMERIC => 1,
                libc::LC_TIME => 2,
                libc::LC_COLLATE => 3,
                libc::LC_MONETARY => 4,
                libc::LC_MESSAGES => 5,
                _ => return setlocale_null_androidfix(category),
            };

            let mut buffer = buffers[index].lock().unwrap();
            if buffer.is_none() {
                *buffer = Some(Box::new([0u8; SETLOCALE_NULL_MAX]));
            }

            let mut local_buf = [0u8; SETLOCALE_NULL_MAX];
            match setlocale_null_r(category, &mut local_buf) {
                Ok(()) => {
                    let len = local_buf.iter().position(|&b| b == 0).unwrap_or(0);
                    buffer.as_mut().unwrap()[..len].copy_from_slice(&local_buf[..len]);
                    Some(unsafe {
                        std::str::from_utf8_unchecked(
                            &buffer.as_ref().unwrap()[..len]
                        )
                    })
                }
                Err(SetLocaleError::InvalidCategory) => None,
                Err(SetLocaleError::BufferTooSmall) => Some("C"),
            }
        })
    }
}