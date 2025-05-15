// Determine whether a locale is hard.

// Copyright (C) 1999, 2003-2004, 2009-2023 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::CStr;
use std::os::raw::c_int;

/// Return true if the specified CATEGORY of the current locale is hard, i.e.
/// different from the C or POSIX locale that has a fixed behavior.
/// CATEGORY must be one of the LC_* values, but not LC_ALL.
/// Note: This function uses the current global locale; it ignores the
/// per-thread locale.
pub fn hard_locale(category: c_int) -> bool {
    // SAFETY: setlocale returns a pointer to static data or null
    let locale = unsafe { std::ffi::CStr::from_ptr(std::locale::setlocale(category, std::ptr::null())) };
    
    if locale.is_err() {
        return false;
    }
    
    let locale = locale.unwrap();
    
    if locale != CStr::from_bytes_with_nul(b"C\0").unwrap() && 
       locale != CStr::from_bytes_with_nul(b"POSIX\0").unwrap() {
        return true;
    }

    #[cfg(target_os = "android")]
    {
        // On Android 5.0 or newer, it is possible to set a locale that has the same
        // name as the "C" locale but in fact uses UTF-8 encoding.
        if std::char::decode_utf8(std::iter::repeat(b'a')).count() > 1 {
            return true;
        }
    }

    false
}