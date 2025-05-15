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
use std::locale::{Locale, Category};

/// Return true if the specified CATEGORY of the current locale is hard, i.e.
/// different from the C or POSIX locale that has a fixed behavior.
/// CATEGORY must be one of the LC_* values, but not LC_ALL.
/// Note: This function uses the current global locale; it ignores the
/// per-thread locale.
pub fn hard_locale(category: c_int) -> bool {
    let category = match category {
        libc::LC_COLLATE => Category::Collate,
        libc::LC_CTYPE => Category::Ctype,
        libc::LC_MONETARY => Category::Monetary,
        libc::LC_NUMERIC => Category::Numeric,
        libc::LC_TIME => Category::Time,
        libc::LC_MESSAGES => Category::Messages,
        _ => return false,
    };

    let locale = Locale::new(category);
    let name = locale.name().to_string_lossy();

    if name != "C" && name != "POSIX" {
        return true;
    }

    #[cfg(target_os = "android")]
    {
        if std::char::MAX_UTF8_SIZE > 1 {
            return true;
        }
    }

    false
}