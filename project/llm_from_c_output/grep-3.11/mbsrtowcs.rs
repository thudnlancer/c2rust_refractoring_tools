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
use std::slice;
use std::str;
use std::mem;
use std::io::{Error, ErrorKind};

use libc::{wchar_t, mbstate_t, mbrtowc};

static mut _gl_mbsrtowcs_state: mbstate_t = mbstate_t { __count: 0, __value: [0; 1] };

pub fn mbsrtowcs(dest: Option<&mut [wchar_t]>, src: &mut *const c_char, n: usize) -> Result<usize, Error> {
    let mut state = unsafe { _gl_mbsrtowcs_state };
    let mut count = 0;
    
    if src.is_null() || (*src).is_null() {
        return Err(Error::new(ErrorKind::InvalidInput, "Null pointer"));
    }

    let c_str = unsafe { CStr::from_ptr(*src) };
    let bytes = c_str.to_bytes();
    
    if let Some(dest_slice) = dest {
        let mut dest_iter = dest_slice.iter_mut();
        
        for &b in bytes {
            if count >= n {
                break;
            }
            
            let mut wc: wchar_t = 0;
            let result = unsafe {
                mbrtowc(
                    &mut wc,
                    &b as *const _ as *const c_char,
                    1,
                    &mut state
                )
            };
            
            match result {
                0 => {
                    if let Some(d) = dest_iter.next() {
                        *d = wc;
                        count += 1;
                    }
                    break;
                },
                1..=isize::MAX as usize => {
                    if let Some(d) = dest_iter.next() {
                        *d = wc;
                        count += 1;
                    }
                },
                _ => return Err(Error::last_os_error()),
            }
        }
        
        if count < dest_slice.len() {
            if let Some(d) = dest_iter.next() {
                *d = 0;
            }
        }
    } else {
        // Just count the characters
        for &b in bytes {
            let mut wc: wchar_t = 0;
            let result = unsafe {
                mbrtowc(
                    &mut wc,
                    &b as *const _ as *const c_char,
                    1,
                    &mut state
                )
            };
            
            match result {
                0 => {
                    count += 1;
                    break;
                },
                1..=isize::MAX as usize => count += 1,
                _ => return Err(Error::last_os_error()),
            }
        }
    }
    
    unsafe {
        _gl_mbsrtowcs_state = state;
    }
    
    if count < n {
        *src = ptr::null();
    }
    
    Ok(count)
}