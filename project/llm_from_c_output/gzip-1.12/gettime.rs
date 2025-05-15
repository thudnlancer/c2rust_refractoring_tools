// gettime -- get the system clock

// Copyright (C) 2002, 2004-2007, 2009-2022 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert.

use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

/// Get the system time into `Timespec`.
pub fn gettime() -> Result<Timespec, std::time::SystemTimeError> {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH)?;
    
    Ok(Timespec {
        tv_sec: duration.as_secs().try_into().unwrap(),
        tv_nsec: duration.subsec_nanos().try_into().unwrap(),
    })
}

/// Timespec structure similar to C's timespec
#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

/// Return the current system time as a Timespec.
pub fn current_timespec() -> Result<Timespec, std::time::SystemTimeError> {
    gettime()
}