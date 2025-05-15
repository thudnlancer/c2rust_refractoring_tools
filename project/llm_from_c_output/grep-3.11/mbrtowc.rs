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
use std::io;
use std::mem;
use std::ptr;
use std::slice;
use std::sync::OnceLock;

#[cfg(target_os = "windows")]
mod windows {
    pub use winapi::um::winnt::*;
}

static INTERNAL_STATE: OnceLock<[u8; 4]> = OnceLock::new();

#[cfg(not(feature = "gnulib_defined_mbstate_t"))]
pub fn rpl_mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut [u8; 4]>) -> Result<usize, io::Error> {
    let s = match s {
        Some(s) => s,
        None => {
            let empty = &b""[..];
            let n = 1;
            if n == 0 {
                return Ok(usize::MAX - 1); // -2 in C
            }
            empty
        }
    };

    let mut wc = char::REPLACEMENT_CHARACTER;
    let pwc = match pwc {
        Some(p) => p,
        None => &mut wc,
    };

    let ps = match ps {
        Some(p) => p,
        None => INTERNAL_STATE.get_or_init(|| [0; 4]),
    };

    if !is_mbsinit(ps) {
        let mut count = 0;
        for byte in s.iter().take(n) {
            let ret = mbrtowc(Some(&mut wc), Some(&[*byte]), 1, Some(ps))?;
            count += 1;
            if ret != usize::MAX - 1 { // -2 in C
                *pwc = wc;
                return Ok(if wc == '\0' { 0 } else { count });
            }
        }
        return Ok(usize::MAX - 1); // -2 in C
    }

    let ret = mbrtowc(Some(pwc), Some(s), n, Some(ps))?;

    if ret < usize::MAX - 2 && *pwc == '\0' {
        Ok(0)
    } else {
        Ok(ret)
    }
}

fn is_mbsinit(ps: &[u8; 4]) -> bool {
    ps.iter().all(|&b| b == 0)
}

#[cfg(feature = "gnulib_defined_mbstate_t")]
pub fn mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut [u8; 4]>) -> Result<usize, io::Error> {
    let s = match s {
        Some(s) => s,
        None => {
            let empty = &b""[..];
            let n = 1;
            if n == 0 {
                return Ok(usize::MAX - 1); // -2 in C
            }
            empty
        }
    };

    let mut wc = char::REPLACEMENT_CHARACTER;
    let pwc = match pwc {
        Some(p) => p,
        None => &mut wc,
    };

    let ps = match ps {
        Some(p) => p,
        None => INTERNAL_STATE.get_or_init(|| [0; 4]),
    };

    if s.is_empty() {
        *pwc = '\0';
        return Ok(0);
    }

    let mut iter = s.iter();
    let first_byte = *iter.next().unwrap();
    let remaining = iter.as_slice();

    if first_byte < 0x80 {
        *pwc = first_byte as char;
        Ok(1)
    } else {
        let mut buf = [0u8; 4];
        buf[0] = first_byte;
        let len = if first_byte & 0b11100000 == 0b11000000 {
            2
        } else if first_byte & 0b11110000 == 0b11100000 {
            3
        } else if first_byte & 0b11111000 == 0b11110000 {
            4
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"));
        };

        if remaining.len() + 1 < len {
            return Ok(usize::MAX - 1); // -2 in C
        }

        for i in 1..len {
            buf[i] = *remaining.get(i - 1).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Incomplete UTF-8 sequence")
            })?;
        }

        match std::str::from_utf8(&buf[..len]) {
            Ok(s) => {
                *pwc = s.chars().next().unwrap();
                Ok(len)
            }
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence")),
        }
    }
}