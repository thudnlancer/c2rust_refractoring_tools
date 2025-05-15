// Convert double to timespec.
//
// Copyright (C) 2011-2022 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// written by Paul Eggert

// Convert the double value SEC to a struct timespec.  Round toward
// positive infinity.  On overflow, return an extremal value.

use libc::{time_t, timespec};
use std::f64;

const TIMESPEC_HZ: i64 = 1_000_000_000;

fn make_timespec(sec: time_t, nsec: i64) -> timespec {
    timespec {
        tv_sec: sec,
        tv_nsec: nsec,
    }
}

pub fn dtotimespec(sec: f64) -> timespec {
    if !(i64::MIN as f64 < sec) {
        make_timespec(i64::MIN as time_t, 0)
    } else if !(sec < 1.0 + i64::MAX as f64) {
        make_timespec(i64::MAX as time_t, TIMESPEC_HZ - 1)
    } else {
        let s = sec.trunc() as time_t;
        let frac = TIMESPEC_HZ as f64 * (sec - sec.trunc());
        let mut ns = frac.trunc() as i64;
        if ns < frac {
            ns += 1;
        }
        let s = s.wrapping_add((ns / TIMESPEC_HZ) as time_t);
        let ns = ns % TIMESPEC_HZ;

        if ns < 0 {
            make_timespec(s.wrapping_sub(1), ns + TIMESPEC_HZ)
        } else {
            make_timespec(s, ns)
        }
    }
}