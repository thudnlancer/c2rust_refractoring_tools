// Add two struct timespec values.
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
//
// Written by Paul Eggert.
//
// Return the sum of two timespec values A and B.  On overflow, return
// an extremal value.  This assumes 0 <= tv_nsec < TIMESPEC_HZ.

use std::time::Duration;

const TIMESPEC_HZ: i32 = 1_000_000_000; // nanoseconds per second
type TimeT = i64; // Assuming time_t is i64, adjust if needed

#[derive(Debug, Clone, Copy)]
struct Timespec {
    tv_sec: TimeT,
    tv_nsec: i32,
}

impl Timespec {
    fn new(sec: TimeT, nsec: i32) -> Self {
        Timespec { tv_sec: sec, tv_nsec: nsec }
    }
}

fn timespec_add(a: Timespec, b: Timespec) -> Timespec {
    let mut rs = a.tv_sec;
    let mut bs = b.tv_sec;
    let ns = a.tv_nsec + b.tv_nsec;
    let nsd = ns - TIMESPEC_HZ;
    let mut rns = ns;

    if 0 <= nsd {
        rns = nsd;
        match bs.checked_add(1) {
            Some(bs1) => bs = bs1,
            None => {
                if rs < 0 {
                    rs = rs.saturating_add(1);
                } else {
                    return Timespec::new(TimeT::MAX, TIMESPEC_HZ - 1);
                }
            }
        }
    }

    match rs.checked_add(bs) {
        Some(sum) => rs = sum,
        None => {
            if bs < 0 {
                return Timespec::new(TimeT::MIN, 0);
            } else {
                return Timespec::new(TimeT::MAX, TIMESPEC_HZ - 1);
            }
        }
    }

    Timespec::new(rs, rns)
}