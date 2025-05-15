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
use std::mem;
use std::os::raw::c_int;

/// Return true if the specified CATEGORY of the current locale is hard, i.e.
/// different from the C or POSIX locale that has a fixed behavior.
/// CATEGORY must be one of the LC_* values, but not LC_ALL.
/// Note: This function uses the current global locale; it ignores the
/// per-thread locale.
pub fn hard_locale(category: c_int) -> bool {
    // Get the current locale for the specified category
    let locale = unsafe { libc::setlocale(category, std::ptr::null()) };
    if locale.is_null() {
        return false;
    }

    // Convert to Rust string and compare with "C" and "POSIX"
    let c_str = unsafe { CStr::from_ptr(locale) };
    if let Ok(locale_str) = c_str.to_str() {
        if locale_str != "C" && locale_str != "POSIX" {
            return true;
        }
    }

    // Android-specific check
    #[cfg(target_os = "android")]
    {
        if unsafe { libc::MB_CUR_MAX } > 1 {
            return true;
        }
    }

    false
}