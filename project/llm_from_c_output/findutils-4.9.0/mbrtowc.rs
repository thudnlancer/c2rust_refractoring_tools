/* Convert multibyte character to wide character.
   Copyright (C) 1999-2002, 2005-2022 Free Software Foundation, Inc.
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
use std::ptr;
use std::sync::OnceLock;

static INTERNAL_STATE: OnceLock<Vec<u8>> = OnceLock::new();

fn fits_in_char_type(wc: char) -> bool {
    wc as u32 <= char::MAX as u32
}

#[cfg(not(feature = "gnulib_defined_mbstate_t"))]
pub fn mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut Vec<u8>>) -> Result<usize, io::Error> {
    let s = match s {
        Some(s) => s,
        None => {
            let pwc = match pwc {
                Some(_) => None,
                None => Some(&mut 0 as *mut _),
            };
            return mbrtowc(pwc, Some(&[]), 1, ps);
        }
    };

    if n == 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "empty input"));
    }

    let mut wc = 0 as char;
    let pwc = match pwc {
        Some(p) => p,
        None => &mut wc,
    };

    let mut internal_state = Vec::new();
    let ps = match ps {
        Some(ps) => ps,
        None => INTERNAL_STATE.get_or_init(|| Vec::new()),
    };

    if !ps.is_empty() {
        let mut count = 0;
        for byte in s.iter().take(n) {
            let ret = mbrtowc(Some(&mut wc), Some(&[*byte]), 1, Some(ps))?;
            count += 1;
            if ret != usize::MAX - 1 {
                *pwc = wc;
                return Ok(if wc == '\0' { 0 } else { count });
            }
        }
        return Err(io::Error::new(io::ErrorKind::InvalidData, "incomplete multibyte sequence"));
    }

    let ret = match std::str::from_utf8(&s[..n.min(s.len())]) {
        Ok(valid) => {
            if let Some(c) = valid.chars().next() {
                *pwc = c;
                Ok(c.len_utf8())
            } else {
                Ok(0)
            }
        }
        Err(e) => {
            if e.error_len().is_none() {
                Err(io::Error::new(io::ErrorKind::InvalidData, "incomplete utf-8 sequence"))
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "invalid utf-8 sequence"))
            }
        }
    };

    if let Ok(len) = ret {
        if len == 0 && pwc == &'\0' {
            return Ok(0);
        }
    }

    #[cfg(feature = "c_locale_eilseq")]
    {
        if ret == Err(io::ErrorKind::InvalidData.into()) && n != 0 && !hard_locale() {
            let uc = s[0];
            *pwc = uc as char;
            return Ok(1);
        }
    }

    ret
}

#[cfg(feature = "gnulib_defined_mbstate_t")]
pub fn mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut Vec<u8>>) -> Result<usize, io::Error> {
    // Implementation for GNULIB defined mbstate_t would go here
    unimplemented!("GNULIB defined mbstate_t not implemented")
}

#[cfg(feature = "c_locale_eilseq")]
fn hard_locale() -> bool {
    // Implementation to check if in hard locale
    unimplemented!("hard_locale check not implemented")
}