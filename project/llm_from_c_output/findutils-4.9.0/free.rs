// Make free() preserve errno.

// Copyright (C) 2003, 2006, 2009-2022 Free Software Foundation, Inc.

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

// written by Paul Eggert

use std::io;

/// A safe wrapper around free that preserves errno
pub fn rpl_free<T>(p: Option<Box<T>>) {
    // Save the current errno value
    let saved_errno = io::Error::last_os_error();
    
    // Drop the boxed value (equivalent to free() in C)
    drop(p);
    
    // Restore the original errno value
    if saved_errno.raw_os_error().is_some() {
        io::Error::set_last_os_error(saved_errno);
    }
}