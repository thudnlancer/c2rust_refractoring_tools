// Table that maps a locale object to the names of the locale categories.
// Copyright (C) 2018-2023 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 2.1 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Bruno Haible <bruno@clisp.org>, 2018.

#[cfg(all(feature = "working_uselocale", feature = "nameless_locales"))]
mod implementation {
    use std::collections::HashMap;
    use std::sync::RwLock;
    use std::ffi::CString;
    use std::ptr;

    pub struct LocaleCategoriesNames {
        pub category_name: [Option<CString>; 6],
    }

    pub struct LocaleHashNode {
        pub next: Option<Box<LocaleHashNode>>,
        pub locale: *mut libc::locale_t,
        pub names: LocaleCategoriesNames,
    }

    pub const LOCALE_HASH_TABLE_SIZE: usize = 101;
    pub static LOCALE_HASH_TABLE: RwLock<HashMap<usize, LocaleHashNode>> = RwLock::new(HashMap::with_capacity(LOCALE_HASH_TABLE_SIZE));

    pub fn locale_hash_function(x: *mut libc::locale_t) -> usize {
        let p = x as usize;
        ((p % 4177) << 12) + ((p % 79) << 6) + (p % 61)
    }
}

#[cfg(not(all(feature = "working_uselocale", feature = "nameless_locales")))]
mod implementation {
    pub type Dummy = i32;
}

#[cfg(all(feature = "working_uselocale", feature = "nameless_locales"))]
pub use implementation::*;

#[cfg(not(all(feature = "working_uselocale", feature = "nameless_locales")))]
pub use implementation::Dummy;