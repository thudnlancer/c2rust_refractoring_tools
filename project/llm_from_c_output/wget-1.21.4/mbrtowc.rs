/* Convert multibyte character to wide character.
   Copyright (C) 1999-2002, 2005-2023 Free Software Foundation, Inc.
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

use std::char;
use std::mem;
use std::ptr;
use std::slice;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::io::{Error, ErrorKind};

#[cfg(target_os = "windows")]
use winapi::um::winnls::{
    MultiByteToWideChar, CP_ACP, MB_ERR_INVALID_CHARS
};

#[cfg(not(target_os = "windows"))]
use libc::{mbrtowc as libc_mbrtowc, mbsinit, mbstate_t};

pub struct MbState {
    #[cfg(not(target_os = "windows"))]
    state: mbstate_t,
    #[cfg(target_os = "windows")]
    state: [u8; 4],
}

impl Default for MbState {
    fn default() -> Self {
        MbState {
            #[cfg(not(target_os = "windows"))]
            state: unsafe { mem::zeroed() },
            #[cfg(target_os = "windows")]
            state: [0; 4],
        }
    }
}

impl MbState {
    pub fn is_init(&self) -> bool {
        #[cfg(not(target_os = "windows"))]
        unsafe { mbsinit(&self.state as *const _) != 0 }
        #[cfg(target_os = "windows")]
        true
    }
}

pub fn mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut MbState>) -> Result<usize, Error> {
    let s = match s {
        Some(s) => s,
        None => {
            let empty = &b""[..];
            return mbrtowc(None, Some(empty), 1, ps);
        }
    };

    if n == 0 {
        return Err(Error::new(ErrorKind::Interrupted, "Incomplete multibyte sequence"));
    }

    let mut wc = 0 as char;
    let pwc = match pwc {
        Some(p) => p,
        None => &mut wc,
    };

    #[cfg(target_os = "windows")]
    {
        let mut state = MbState::default();
        let ps = ps.unwrap_or(&mut state);
        
        let mut wide_char = [0u16; 2];
        let result = unsafe {
            MultiByteToWideChar(
                CP_ACP,
                MB_ERR_INVALID_CHARS,
                s.as_ptr() as *const i8,
                n as c_int,
                wide_char.as_mut_ptr(),
                2,
            )
        };

        if result == 0 {
            return Err(Error::last_os_error());
        }

        let ch = char::decode_utf16(wide_char.iter().cloned())
            .next()
            .unwrap()
            .unwrap();
        *pwc = ch;
        Ok(ch.len_utf8())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut state = MbState::default();
        let ps = match ps {
            Some(ps) => ps,
            None => &mut state,
        };

        if !ps.is_init() {
            let mut count = 0;
            for byte in s.iter().take(n) {
                let ch = *byte as char;
                *pwc = ch;
                count += 1;
                if ch != '\0' {
                    return Ok(count);
                }
            }
            return Err(Error::new(ErrorKind::Interrupted, "Incomplete multibyte sequence"));
        }

        let mut bytes_processed = 0;
        let result = unsafe {
            libc_mbrtowc(
                pwc as *mut _ as *mut _,
                s.as_ptr() as *const c_char,
                n,
                &mut ps.state as *mut _,
            )
        };

        match result {
            libc::size_t::MAX => Err(Error::new(ErrorKind::InvalidData, "Invalid multibyte sequence")),
            0 => Ok(0),
            n if n <= n => Ok(n),
            _ => Err(Error::new(ErrorKind::Interrupted, "Incomplete multibyte sequence")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let mut wc = ' ';
        let s = "A".as_bytes();
        let result = mbrtowc(Some(&mut wc), Some(s), s.len(), None);
        assert_eq!(result.unwrap(), 1);
        assert_eq!(wc, 'A');
    }

    #[test]
    fn test_null_input() {
        let result = mbrtowc(None, None, 1, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_input() {
        let result = mbrtowc(None, Some(&[]), 0, None);
        assert!(result.is_err());
    }
}