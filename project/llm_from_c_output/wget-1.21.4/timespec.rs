//! System time interface
//!
//! This module provides functionality for working with timespec structures.
//! Original C code copyright (C) 2000-2023 Free Software Foundation, Inc.
//! Licensed under GNU Lesser General Public License version 3 or later.

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;

/// Inverse resolution of timespec timestamps (in units per second)
pub const TIMESPEC_HZ: u64 = 1_000_000_000;
/// Log base 10 of the inverse resolution
pub const LOG10_TIMESPEC_HZ: u32 = 9;

/// Obsolescent names for backward compatibility.
/// They are misnomers, because TIMESPEC_RESOLUTION is not a resolution.
pub const TIMESPEC_RESOLUTION: u64 = TIMESPEC_HZ;
pub const LOG10_TIMESPEC_RESOLUTION: u32 = LOG10_TIMESPEC_HZ;

/// Represents a timespec with seconds and nanoseconds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    /// Creates a new Timespec with given seconds and nanoseconds
    pub fn new(s: i64, ns: i64) -> Self {
        Self {
            tv_sec: s,
            tv_nsec: ns,
        }
    }

    /// Compares two Timespec values
    pub fn cmp(&self, other: &Self) -> Ordering {
        match self.tv_sec.cmp(&other.tv_sec) {
            Ordering::Equal => self.tv_nsec.cmp(&other.tv_nsec),
            ord => ord,
        }
    }

    /// Returns the sign of the Timespec
    pub fn sign(&self) -> i32 {
        match self.tv_sec.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => {
                if self.tv_nsec > 0 {
                    1
                } else {
                    0
                }
            }
        }
    }

    /// Converts Timespec to f64 seconds
    pub fn to_seconds(&self) -> f64 {
        self.tv_sec as f64 + (self.tv_nsec as f64 / 1e9)
    }

    /// Adds two Timespec values
    pub fn add(&self, other: &Self) -> Self {
        let mut sec = self.tv_sec + other.tv_sec;
        let mut nsec = self.tv_nsec + other.tv_nsec;

        if nsec >= TIMESPEC_HZ as i64 {
            nsec -= TIMESPEC_HZ as i64;
            sec += 1;
        }

        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }

    /// Subtracts two Timespec values
    pub fn sub(&self, other: &Self) -> Self {
        let mut sec = self.tv_sec - other.tv_sec;
        let mut nsec = self.tv_nsec - other.tv_nsec;

        if nsec < 0 {
            nsec += TIMESPEC_HZ as i64;
            sec -= 1;
        }

        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }

    /// Converts from f64 seconds to Timespec
    pub fn from_seconds(seconds: f64) -> Self {
        let sec = seconds.trunc() as i64;
        let nsec = (seconds.fract() * 1e9) as i64;
        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }
}

/// Gets the current time resolution
pub fn gettime_res() -> i64 {
    // In Rust, we typically use Duration's resolution which is nanoseconds
    // This is a placeholder as Rust doesn't expose system clock resolution directly
    1
}

/// Gets the current time as Timespec
pub fn current_timespec() -> Timespec {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => Timespec {
            tv_sec: dur.as_secs() as i64,
            tv_nsec: dur.subsec_nanos() as i64,
        },
        Err(e) => {
            // Handle system time earlier than UNIX_EPOCH
            let duration = e.duration();
            Timespec {
                tv_sec: -(duration.as_secs() as i64),
                tv_nsec: -(duration.subsec_nanos() as i64),
            }
        }
    }
}

/// Gets the current time and stores it in the provided Timespec
pub fn gettime(ts: &mut Timespec) {
    *ts = current_timespec();
}

/// Sets the system time (not implemented in Rust due to permissions requirements)
pub fn settime(_ts: &Timespec) -> Result<(), String> {
    Err("Setting system time is not supported in this implementation".to_string())
}