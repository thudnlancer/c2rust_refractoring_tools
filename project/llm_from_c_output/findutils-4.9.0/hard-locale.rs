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
    // SAFETY: We're using libc's setlocale which is thread-safe in this context
    let locale = unsafe { libc::setlocale(category, std::ptr::null()) };
    if locale.is_null() {
        return false;
    }
    
    // SAFETY: The pointer is valid and points to a null-terminated string
    let locale_str = unsafe { CStr::from_ptr(locale) };
    
    locale_str != "C" && locale_str != "POSIX"
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{LC_CTYPE, LC_NUMERIC, LC_TIME};

    #[test]
    fn test_hard_locale() {
        // These tests assume the test environment has standard locales available
        let _ = unsafe { libc::setlocale(LC_CTYPE, b"C\0".as_ptr() as *const libc::c_char) };
        assert!(!hard_locale(LC_CTYPE));
        
        let _ = unsafe { libc::setlocale(LC_CTYPE, b"POSIX\0".as_ptr() as *const libc::c_char) };
        assert!(!hard_locale(LC_CTYPE));
        
        // This test may fail if the system doesn't have en_US.UTF-8 locale
        if let Ok(_) = std::env::var("CI") {
            // Skip in CI as locale availability isn't guaranteed
            return;
        }
        
        let _ = unsafe { libc::setlocale(LC_CTYPE, b"en_US.UTF-8\0".as_ptr() as *const libc::c_char) };
        if unsafe { !libc::setlocale(LC_CTYPE, std::ptr::null()).is_null() } {
            assert!(hard_locale(LC_CTYPE));
        }
    }
}