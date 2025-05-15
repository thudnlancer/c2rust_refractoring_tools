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
pub fn gettime() -> Result<libc::timespec, String> {
    if cfg!(target_os = "linux") || cfg!(target_os = "android") {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            if libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts) == 0 {
                return Ok(ts);
            }
        }
    }

    if cfg!(feature = "timespec_get") {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        unsafe {
            if libc::timespec_get(&mut ts, libc::TIME_UTC) != 0 {
                return Ok(ts);
            }
        }
    }

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Ok(libc::timespec {
            tv_sec: duration.as_secs() as i64,
            tv_nsec: duration.subsec_nanos() as i64,
        }),
        Err(e) => Err(format!("SystemTime before UNIX EPOCH: {}", e)),
    }
}

/// Return the current system time as a `Timespec`.
pub fn current_timespec() -> Result<libc::timespec, String> {
    gettime()
}