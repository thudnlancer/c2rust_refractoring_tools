//! Charset conversion with out-of-memory checking.
//! Copyright (C) 2001-2004, 2006-2007, 2009-2023 Free Software Foundation, Inc.
//! Written by Bruno Haible and Simon Josefsson.
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::error::Error;
use std::ffi::{CString, NulError};
use std::ptr;
use std::io::{Error as IoError, ErrorKind};
use libc::{iconv_t, size_t};
use crate::striconv::{mem_cd_iconv, str_cd_iconv, str_iconv};
use crate::xalloc::xalloc_die;

#[cfg(feature = "iconv")]
pub fn xmem_cd_iconv(
    src: &[u8],
    cd: iconv_t,
    resultp: &mut Option<Vec<u8>>,
    lengthp: &mut usize,
) -> Result<(), Box<dyn Error>> {
    let mut temp_result = resultp.take().unwrap_or_default();
    let mut temp_length = *lengthp;

    match mem_cd_iconv(src, cd, &mut temp_result, &mut temp_length) {
        Ok(()) => {
            *resultp = Some(temp_result);
            *lengthp = temp_length;
            Ok(())
        }
        Err(e) if e.kind() == ErrorKind::OutOfMemory => {
            xalloc_die();
            Err(Box::new(e))
        }
        Err(e) => Err(Box::new(e)),
    }
}

#[cfg(feature = "iconv")]
pub fn xstr_cd_iconv(src: &str, cd: iconv_t) -> Result<String, Box<dyn Error>> {
    match str_cd_iconv(src, cd) {
        Ok(s) => Ok(s),
        Err(e) if e.kind() == ErrorKind::OutOfMemory => {
            xalloc_die();
            Err(Box::new(e))
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn xstr_iconv(
    src: &str,
    from_codeset: &str,
    to_codeset: &str,
) -> Result<String, Box<dyn Error>> {
    match str_iconv(src, from_codeset, to_codeset) {
        Ok(s) => Ok(s),
        Err(e) if e.kind() == ErrorKind::OutOfMemory => {
            xalloc_die();
            Err(Box::new(e))
        }
        Err(e) => Err(Box::new(e)),
    }
}