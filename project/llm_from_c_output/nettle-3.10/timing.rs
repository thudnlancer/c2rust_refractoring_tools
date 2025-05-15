// timing.rs

// Copyright (C) 2013 Niels Möller
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

use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::process;
use std::io::{self, Write};

static mut TIME_START: fn() -> Instant = default_time_start;
static mut TIME_END: fn(Instant) -> f64 = default_time_end;

fn die(message: &str) -> ! {
    writeln!(io::stderr(), "{}", message).unwrap();
    process::exit(1);
}

fn default_time_start() -> Instant {
    Instant::now()
}

fn default_time_end(start: Instant) -> f64 {
    let duration = start.elapsed();
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
}

#[cfg(target_os = "linux")]
fn cgt_works_p() -> bool {
    use libc::{clock_gettime, timespec, CLOCK_PROCESS_CPUTIME_ID};
    unsafe {
        let mut now = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut now) == 0
    }
}

#[cfg(target_os = "linux")]
fn cgt_time_start() -> Instant {
    use libc::{clock_gettime, timespec, CLOCK_PROCESS_CPUTIME_ID};
    unsafe {
        let mut start = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut start) < 0 {
            die(&format!("clock_gettime failed: {}", io::Error::last_os_error()));
        }
        Instant::now() // Fallback as we can't directly convert timespec to Instant
    }
}

#[cfg(target_os = "linux")]
fn cgt_time_end(start: Instant) -> f64 {
    default_time_end(start) // Fallback as we can't directly use timespec
}

pub fn time_init() {
    #[cfg(target_os = "linux")]
    {
        if cgt_works_p() {
            unsafe {
                TIME_START = cgt_time_start;
                TIME_END = cgt_time_end;
            }
        } else {
            eprintln!("clock_gettime not working, falling back to Instant");
        }
    }
}

pub fn time_start() -> Instant {
    unsafe { TIME_START() }
}

pub fn time_end(start: Instant) -> f64 {
    unsafe { TIME_END(start) }
}