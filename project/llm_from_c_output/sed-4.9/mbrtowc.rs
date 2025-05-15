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
use std::str;

#[cfg(target_os = "windows")]
use winapi::um::winnls::*;

#[cfg(all(not(target_os = "windows"), feature = "pthread"))]
use libc::{pthread_mutex_lock, pthread_mutex_unlock};

#[cfg(feature = "threads")]
use std::thread;

static mut INTERNAL_STATE: [u8; 4] = [0; 4];

pub fn mbrtowc(pwc: Option<&mut char>, s: Option<&[u8]>, n: usize, ps: Option<&mut [u8; 4]>) -> Result<usize, io::Error> {
    let s = match s {
        Some(s) => s,
        None => {
            let pwc = match pwc {
                Some(_) => None,
                None => None,
            };
            return mbrtowc(pwc, Some(&[]), 1, ps);
        }
    };

    if n == 0 {
        return Ok(usize::MAX - 1); // -2 in C terms
    }

    let pwc = match pwc {
        Some(p) => p,
        None => {
            let mut wc = '\0';
            return mbrtowc(Some(&mut wc), s, n, ps);
        }
    };

    let ps = match ps {
        Some(p) => p,
        None => unsafe { &mut INTERNAL_STATE },
    };

    if !mbsinit(ps) {
        let mut count = 0;
        for byte in s.iter().take(n) {
            let mut wc = '\0';
            let ret = mbrtowc(Some(&mut wc), Some(&[*byte]), 1, Some(ps))?;
            
            if ret == usize::MAX { // -1 in C terms
                return Ok(usize::MAX);
            }
            count += 1;
            if ret != usize::MAX - 1 { // not -2
                *pwc = wc;
                return Ok(if wc == '\0' { 0 } else { count });
            }
        }
        return Ok(usize::MAX - 1); // -2
    }

    let (ret, wc) = match str::from_utf8(&s[..n]) {
        Ok(s) => {
            let mut chars = s.chars();
            if let Some(c) = chars.next() {
                (chars.as_str().len(), c)
            } else {
                (0, '\0')
            }
        }
        Err(e) => {
            if e.valid_up_to() == 0 {
                return Ok(usize::MAX); // -1
            } else {
                return Ok(usize::MAX - 1); // -2
            }
        }
    };

    *pwc = wc;
    if wc == '\0' {
        Ok(0)
    } else {
        Ok(ret + 1)
    }
}

fn mbsinit(ps: &[u8; 4]) -> bool {
    ps.iter().all(|&b| b == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbrtowc() {
        let mut wc = '\0';
        let s = "A".as_bytes();
        let mut state = [0; 4];
        
        assert_eq!(mbrtowc(Some(&mut wc), Some(s), 1, Some(&mut state)), Ok(0));
        assert_eq!(wc, 'A');
    }
}