/*  GNU SED, a batch stream editor.
    Copyright (C) 2003-2022 Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3, or (at your option)
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; If not, see <https://www.gnu.org/licenses/>. */

use std::ffi::CString;
use std::mem;
use std::str;
use std::char;
use std::io::{self, Error, ErrorKind};
use encoding_rs::Encoding;
use encoding_rs::mem::decode_latin1;

pub static mut MB_CUR_MAX: usize = 1;
pub static mut IS_UTF8: bool = false;

/// Return non-zero if CH is part of a valid multibyte sequence:
/// Either incomplete yet valid sequence (in case of a leading byte),
/// or the last byte of a valid multibyte sequence.
///
/// Return zero in all other cases:
///  CH is a valid single-byte character (e.g. 0x01-0x7F in UTF-8 locales);
///  CH is an invalid byte in a multibyte sequence for the current locale,
///  CH is the NUL byte.
///
/// Reset CUR_STAT in the case of an invalid byte.
pub fn is_mb_char(ch: u8, cur_stat: &mut std::char::DecodeUtf16State) -> Result<bool, io::Error> {
    let c = ch;
    let mb_pending = !matches!(cur_stat, std::char::DecodeUtf16State::Empty);
    let result = char::decode_utf16(std::slice::from_ref(&c as &u8 as *const u8 as *const u16), cur_stat);

    match result {
        Ok(_) => Ok(mb_pending),
        Err(e) if e.unexpected_eof() => Ok(true),
        Err(_) => {
            *cur_stat = std::char::DecodeUtf16State::Empty;
            Ok(false)
        }
    }
}

pub fn initialize_mbcs() -> Result<(), io::Error> {
    let codeset_name = get_locale_charset()?;
    unsafe {
        IS_UTF8 = codeset_name == "UTF-8";
        MB_CUR_MAX = if IS_UTF8 { 4 } else { 1 };
    }
    Ok(())
}

fn get_locale_charset() -> Result<String, io::Error> {
    // This is a simplified version - in real code you'd use a proper locale detection crate
    match std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_CTYPE"))
        .or_else(|_| std::env::var("LANG"))
    {
        Ok(locale) => {
            if locale.to_uppercase().contains("UTF-8") {
                Ok("UTF-8".to_string())
            } else {
                Ok("ANSI_X3.4-1968".to_string()) // ASCII as fallback
            }
        }
        Err(_) => Ok("ANSI_X3.4-1968".to_string()),
    }
}