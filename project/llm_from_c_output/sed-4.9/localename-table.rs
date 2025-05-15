// Table that maps a locale object to the names of the locale categories.
// Copyright (C) 2018-2022 Free Software Foundation, Inc.
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
    use std::sync::RwLock;
    use std::collections::HashMap;
    use std::ffi::CString;
    use std::ptr::NonNull;

    pub struct LocaleCategoriesNames {
        // Locale category -> name (allocated with indefinite extent)
        pub category_name: [Option<CString>; 6],
    }

    // A node in a hash bucket collision list
    pub struct LocaleHashNode {
        pub next: Option<Box<LocaleHashNode>>,
        pub locale: NonNull<libc::locale_t>,
        pub names: LocaleCategoriesNames,
    }

    const LOCALE_HASH_TABLE_SIZE: usize = 101;

    lazy_static::lazy_static! {
        // A hash table of fixed size protected by a RwLock
        pub static ref LOCALE_HASH_TABLE: RwLock<HashMap<usize, Option<Box<LocaleHashNode>>>> = {
            let mut map = HashMap::with_capacity(LOCALE_HASH_TABLE_SIZE);
            for _ in 0..LOCALE_HASH_TABLE_SIZE {
                map.insert(0, None);
            }
            RwLock::new(map)
        };
    }

    // A hash function for pointers
    pub fn locale_hash_function(x: NonNull<libc::locale_t>) -> usize {
        let p = x.as_ptr() as usize;
        ((p % 4177) << 12) + ((p % 79) << 6) + (p % 61)
    }
}

#[cfg(not(all(feature = "working_uselocale", feature = "nameless_locales")))]
mod implementation {
    // Dummy implementation when features are not enabled
    pub struct Dummy;
}

// Re-export based on feature flags
#[cfg(all(feature = "working_uselocale", feature = "nameless_locales"))]
pub use implementation::*;

#[cfg(not(all(feature = "working_uselocale", feature = "nameless_locales")))]
pub use implementation::Dummy as dummy;