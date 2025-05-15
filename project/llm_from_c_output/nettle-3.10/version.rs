// version.rs

// Information about library version.
//
// Copyright (C) 2015 Red Hat, Inc.
// Copyright (C) 2015 Niels Möller
//
// This file is part of GNU Nettle.
//
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
//
//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.
//
// or
//
//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.
//
// or both in parallel, as here.
//
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

/// Individual version numbers in decimal
pub const NETTLE_VERSION_MAJOR: i32 = 3;
pub const NETTLE_VERSION_MINOR: i32 = 10;

pub const NETTLE_USE_MINI_GMP: bool = false;

// We need a preprocessor constant for GMP_NUMB_BITS, simply using
// sizeof(mp_limb_t) * CHAR_BIT is not good enough.
#[cfg(NETTLE_USE_MINI_GMP)]
pub const GMP_NUMB_BITS: &str = "n/a";

/// Returns the major version number of the library
pub fn nettle_version_major() -> i32 {
    NETTLE_VERSION_MAJOR
}

/// Returns the minor version number of the library
pub fn nettle_version_minor() -> i32 {
    NETTLE_VERSION_MINOR
}