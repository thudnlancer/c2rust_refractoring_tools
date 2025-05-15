// Copyright (C) 2000, 2004, 2009-2023 Free Software Foundation, Inc.
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

use libc::{c_short, EINVAL};
use std::os::raw::c_int;

const ALL_FLAGS: c_short = libc::POSIX_SPAWN_RESETIDS
    | libc::POSIX_SPAWN_SETPGROUP
    | libc::POSIX_SPAWN_SETSIGDEF
    | libc::POSIX_SPAWN_SETSIGMASK
    | libc::POSIX_SPAWN_SETSCHEDPARAM
    | libc::POSIX_SPAWN_SETSCHEDULER
    | libc::POSIX_SPAWN_USEVFORK;

/// Store flags in the attribute structure.
pub fn posix_spawnattr_setflags(attr: &mut libc::posix_spawnattr_t, flags: c_short) -> c_int {
    // Check no invalid bits are set.
    if (flags & !ALL_FLAGS) != 0 {
        return EINVAL;
    }

    // Store the flag word.
    unsafe {
        (*attr).__flags = flags;
    }

    0
}