/* Convert multibyte character to wide character.
   Copyright (C) 1999-2002, 2005-2021 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2008.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::char;
use std::io;
use std::mem;
use std::ptr;

#[cfg(target_os = "windows")]
use winapi::um::winnls::*;

#[cfg(all(feature = "threads", not(target_os = "windows")))]
use std::sync::Mutex;

#[cfg(feature = "threads")]
lazy_static::lazy_static! {
    static ref MBRTOWC_LOCK: Mutex<()> = Mutex::new(());
}

const WCHAR_MAX: u32 = 0x10FFFF;

fn fits_in_char_type(wc: u32) -> bool {
    wc <= WCHAR_MAX
}

#[cfg(not(feature = "gnulib_defined_mbstate_t"))]
pub fn mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut std::mbstate_t>) -> Result<usize, io::Error> {
    let s = match s {
        Some(s) => s,
        None => {
            let empty = &b""[..];
            return mbrtowc(None, Some(empty), 1, ps);
        }
    };

    if n == 0 {
        return Ok(usize::MAX - 1); // -2 in C terms
    }

    let mut wc = char::REPLACEMENT_CHARACTER;
    let pwc = pwc.unwrap_or(&mut wc);

    let mut internal_state = std::mbstate_t::new();
    let ps = ps.unwrap_or(&mut internal_state);

    if !ps.is_init() {
        let mut count = 0;
        for &byte in s.iter().take(n) {
            let ret = mbrtowc(Some(pwc), Some(&[byte]), 1, Some(ps))?;
            
            if ret == usize::MAX { // -1 in C terms
                return Ok(usize::MAX);
            }
            count += 1;
            if ret != usize::MAX - 1 { // not -2
                return Ok(if *pwc == '\0' { 0 } else { count });
            }
        }
        return Ok(usize::MAX - 1);
    }

    let ret = std::str::from_utf8(&s[..n])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?
        .chars()
        .next();

    match ret {
        Some(c) => {
            *pwc = c;
            Ok(if c == '\0' { 0 } else { c.len_utf8() })
        }
        None => Ok(usize::MAX - 1),
    }
}

#[cfg(feature = "gnulib_defined_mbstate_t")]
pub fn mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut std::mbstate_t>) -> Result<usize, io::Error> {
    // Implementation for GNULIB defined mbstate_t would go here
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbrtowc() {
        let mut wc = '\0';
        let s = "A".as_bytes();
        let result = mbrtowc(Some(&mut wc), Some(s), s.len(), None);
        assert_eq!(result.unwrap(), 1);
        assert_eq!(wc, 'A');
    }
}