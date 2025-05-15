/* Localization of proper names.  -*- coding: utf-8 -*-
   Copyright (C) 2006, 2008-2023 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2006.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::str;
use std::ffi::CStr;
use std::cmp::Ordering;
use std::borrow::Cow;
use encoding_rs::{Encoding, UTF_8};
use gettextrs::{gettext, LocaleCategory};
use libc::{setlocale, LC_ALL};

#[no_mangle]
pub extern "C" fn proper_name(name: *const c_char) -> *const c_char {
    let name_str = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    let translation = gettext(name_str);
    
    if translation != name_str {
        if mbsstr_trimmed_wordbounded(translation, name_str) {
            return CString::new(translation).unwrap().into_raw();
        } else {
            let result = format!("{} ({})", translation, name_str);
            return CString::new(result).unwrap().into_raw();
        }
    } else {
        return CString::new(name_str).unwrap().into_raw();
    }
}

#[no_mangle]
pub extern "C" fn proper_name_utf8(name_ascii: *const c_char, name_utf8: *const c_char) -> *const c_char {
    let name_ascii_str = unsafe { CStr::from_ptr(name_ascii) }.to_str().unwrap();
    let name_utf8_str = unsafe { CStr::from_ptr(name_utf8) }.to_str().unwrap();
    
    let translation = gettext(name_ascii_str);
    let locale_code = locale_charset();
    
    let (name_converted, name_converted_translit) = if locale_code.to_lowercase() != "utf-8" {
        let encoding = Encoding::for_label(locale_code.as_bytes()).unwrap_or(UTF_8);
        let (cow, _encoding, had_errors) = encoding.decode(name_utf8_str.as_bytes());
        let name_converted = if !had_errors {
            Some(cow.into_owned())
        } else {
            None
        };
        
        let translit_encoding = format!("{}//TRANSLIT", locale_code);
        let encoding_translit = Encoding::for_label(translit_encoding.as_bytes()).unwrap_or(UTF_8);
        let (cow_translit, _encoding, had_errors) = encoding_translit.decode(name_utf8_str.as_bytes());
        let name_converted_translit = if !had_errors && !cow_translit.contains('?') {
            Some(cow_translit.into_owned())
        } else {
            None
        };
        
        (name_converted, name_converted_translit)
    } else {
        (Some(name_utf8_str.to_string()), Some(name_utf8_str.to_string()))
    };
    
    let name = name_converted.as_ref()
        .or(name_converted_translit.as_ref())
        .unwrap_or(&name_ascii_str);
    
    if translation != name_ascii_str {
        if mbsstr_trimmed_wordbounded(translation, name_ascii_str) || 
           name_converted.as_ref().map_or(false, |nc| mbsstr_trimmed_wordbounded(translation, nc)) ||
           name_converted_translit.as_ref().map_or(false, |nct| mbsstr_trimmed_wordbounded(translation, nct)) 
        {
            return CString::new(translation).unwrap().into_raw();
        } else {
            let result = format!("{} ({})", translation, name);
            return CString::new(result).unwrap().into_raw();
        }
    } else {
        return CString::new(name).unwrap().into_raw();
    }
}

fn mbsstr_trimmed_wordbounded(string: &str, sub: &str) -> bool {
    let tsub = sub.trim();
    let mut found = false;
    let mut pos = 0;
    
    while let Some(tsub_pos) = string[pos..].find(tsub) {
        let tsub_start = pos + tsub_pos;
        let tsub_end = tsub_start + tsub.len();
        
        let word_boundary_before = if tsub_start > 0 {
            !string[..tsub_start].chars().last().unwrap().is_alphanumeric()
        } else {
            true
        };
        
        let word_boundary_after = if tsub_end < string.len() {
            !string[tsub_end..].chars().next().unwrap().is_alphanumeric()
        } else {
            true
        };
        
        if word_boundary_before && word_boundary_after {
            found = true;
            break;
        }
        
        pos = tsub_start + 1;
        if pos >= string.len() {
            break;
        }
    }
    
    found
}

fn locale_charset() -> String {
    unsafe {
        let charset = libc::nl_langinfo(libc::CODESET);
        CStr::from_ptr(charset).to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    
    #[test]
    fn test_proper_name() {
        let name = CString::new("Test Name").unwrap();
        let result = proper_name(name.as_ptr());
        let result_str = unsafe { CStr::from_ptr(result) }.to_str().unwrap();
        assert_eq!(result_str, "Test Name");
        unsafe { CString::from_raw(result as *mut c_char) };
    }
    
    #[test]
    fn test_proper_name_utf8() {
        let name_ascii = CString::new("Test Name").unwrap();
        let name_utf8 = CString::new("Test Name UTF-8").unwrap();
        let result = proper_name_utf8(name_ascii.as_ptr(), name_utf8.as_ptr());
        let result_str = unsafe { CStr::from_ptr(result) }.to_str().unwrap();
        assert!(result_str.contains("Test Name"));
        unsafe { CString::from_raw(result as *mut c_char) };
    }
}