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
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::sync::RwLock;
    use std::ptr;
    use std::ffi::CStr;

    pub struct LocaleCategoriesNames {
        pub category_name: [Option<&'static CStr>; 6],
    }

    pub struct LocaleHashNode {
        pub next: Option<Box<LocaleHashNode>>,
        pub locale: *mut libc::locale_t,
        pub names: LocaleCategoriesNames,
    }

    pub const LOCALE_HASH_TABLE_SIZE: usize = 101;
    pub static LOCALE_HASH_TABLE: [Option<Box<LocaleHashNode>>; LOCALE_HASH_TABLE_SIZE] = [None; LOCALE_HASH_TABLE_SIZE];

    lazy_static! {
        pub static ref LOCALE_LOCK: RwLock<()> = RwLock::new(());
    }

    pub fn locale_hash_function(x: *mut libc::locale_t) -> usize {
        let mut hasher = DefaultHasher::new();
        (x as usize).hash(&mut hasher);
        hasher.finish() as usize % LOCALE_HASH_TABLE_SIZE
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