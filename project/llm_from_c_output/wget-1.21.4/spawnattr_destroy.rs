// Copyright (C) 2000, 2009-2023 Free Software Foundation, Inc.
// This file is part of the GNU C Library.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Implementation of posix_spawnattr_destroy function.

use std::os::raw::c_int;

/// Initialize data structure for file attribute for 'spawn' call.
#[no_mangle]
pub extern "C" fn posix_spawnattr_destroy(_attr: *mut libc::posix_spawnattr_t) -> c_int {
    // Nothing to do in the moment.
    0
}