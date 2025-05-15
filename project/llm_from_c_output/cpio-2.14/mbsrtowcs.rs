// Convert string to wide string.
// Copyright (C) 2008-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2008.
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

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::io::{Error, ErrorKind};
use std::slice;

use libc::{wchar_t, mbstate_t, mbrtowc};

static mut _gl_mbsrtowcs_state: mbstate_t = mbstate_t { __count: 0, __value: [0; 1] };

pub fn mbsrtowcs(dest: Option<&mut [wchar_t]>, src: &mut *const c_char, n: usize) -> Result<usize, Error> {
    let mut state = unsafe { _gl_mbsrtowcs_state };
    let mut count = 0;
    
    if let Some(dest_slice) = dest {
        if dest_slice.len() < n {
            return Err(Error::new(ErrorKind::InvalidInput, "Destination buffer too small"));
        }
    }
    
    let mut remaining = n;
    let mut dst_ptr = dest.map(|d| d.as_mut_ptr()).unwrap_or(ptr::null_mut());
    
    while remaining > 0 {
        if src.is_null() || unsafe { **src } == 0 {
            break;
        }
        
        let mut wc: wchar_t = 0;
        let bytes_processed = unsafe {
            mbrtowc(
                &mut wc as *mut wchar_t,
                *src,
                remaining,
                &mut state as *mut mbstate_t
            )
        };
        
        match bytes_processed {
            0 => {
                // Null terminator encountered
                if !dst_ptr.is_null() {
                    unsafe { *dst_ptr = 0 };
                }
                unsafe { *src = ptr::null() };
                break;
            },
            len if len > 0 => {
                if !dst_ptr.is_null() {
                    unsafe {
                        *dst_ptr = wc;
                        dst_ptr = dst_ptr.add(1);
                    }
                }
                unsafe { *src = src.add(len) };
                count += 1;
                remaining -= 1;
            },
            _ => {
                // Error occurred
                return Err(Error::new(ErrorKind::InvalidData, "Invalid multibyte sequence"));
            }
        }
    }
    
    unsafe { _gl_mbsrtowcs_state = state };
    Ok(count)
}