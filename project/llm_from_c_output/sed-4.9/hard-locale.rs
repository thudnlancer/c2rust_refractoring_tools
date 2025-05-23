// Determine whether a locale is hard.

// Copyright (C) 1999, 2003-2004, 2009-2022 Free Software Foundation, Inc.

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
pub fn hard_locale(category: c_int) -> bool {
    let locale = unsafe { std::ffi::CStr::from_ptr(libc::setlocale(category, std::ptr::null())) };
    
    match locale.to_str() {
        Ok(locale_str) => {
            locale_str != "C" && locale_str != "POSIX"
        },
        Err(_) => false
    }
}