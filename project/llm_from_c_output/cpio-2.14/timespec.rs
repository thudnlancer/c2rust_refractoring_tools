//! System time interface

#![allow(non_upper_case_globals)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::cmp::Ordering;

/// Inverse resolution of timespec timestamps (in units per second),
/// and log base 10 of the inverse resolution.
pub const TIMESPEC_HZ: u64 = 1_000_000_000;
pub const LOG10_TIMESPEC_HZ: u32 = 9;

/// Obsolescent names for backward compatibility.
/// They are misnomers, because TIMESPEC_RESOLUTION is not a resolution.
pub const TIMESPEC_RESOLUTION: u64 = TIMESPEC_HZ;
pub const LOG10_TIMESPEC_RESOLUTION: u32 = LOG10_TIMESPEC_HZ;

/// A structure representing a timestamp with seconds and nanoseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl TimeSpec {
    /// Creates a new TimeSpec with seconds and nanoseconds.
    pub fn new(s: i64, ns: i64) -> Self {
        Self {
            tv_sec: s,
            tv_nsec: ns,
        }
    }

    /// Compares two TimeSpec values.
    pub fn cmp(&self, other: &Self) -> Ordering {
        match self.tv_sec.cmp(&other.tv_sec) {
            Ordering::Equal => self.tv_nsec.cmp(&other.tv_nsec),
            ord => ord,
        }
    }

    /// Returns the sign of the TimeSpec.
    pub fn sign(&self) -> i32 {
        match self.tv_sec.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => {
                if self.tv_nsec > 0 {
                    1
                } else {
                    0
                }
            }
        }
    }

    /// Converts the TimeSpec to a floating-point seconds value.
    pub fn to_seconds(&self) -> f64 {
        self.tv_sec as f64 + (self.tv_nsec as f64) / 1e9
    }

    /// Adds two TimeSpec values.
    pub fn add(&self, other: &Self) -> Self {
        let mut sec = self.tv_sec + other.tv_sec;
        let mut nsec = self.tv_nsec + other.tv_nsec;
        
        if nsec >= TIMESPEC_HZ as i64 {
            nsec -= TIMESPEC_HZ as i64;
            sec += 1;
        }
        
        Self::new(sec, nsec)
    }

    /// Subtracts two TimeSpec values.
    pub fn sub(&self, other: &Self) -> Self {
        let mut sec = self.tv_sec - other.tv_sec;
        let mut nsec = self.tv_nsec - other.tv_nsec;
        
        if nsec < 0 {
            nsec += TIMESPEC_HZ as i64;
            sec -= 1;
        }
        
        Self::new(sec, nsec)
    }

    /// Converts a duration in seconds to TimeSpec.
    pub fn from_seconds(seconds: f64) -> Self {
        let sec = seconds.trunc() as i64;
        let nsec = (seconds.fract() * 1e9).round() as i64;
        Self::new(sec, nsec)
    }
}

/// Gets the current time resolution.
pub fn gettime_res() -> i64 {
    // In Rust, we can't directly get the clock resolution,
    // so we return the nanosecond resolution as in the C version
    TIMESPEC_HZ as i64
}

/// Gets the current time as TimeSpec.
pub fn current_timespec() -> TimeSpec {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => TimeSpec::new(dur.as_secs() as i64, dur.subsec_nanos() as i64),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

/// Gets the current time and stores it in the provided TimeSpec.
pub fn gettime(ts: &mut TimeSpec) {
    *ts = current_timespec();
}

/// Sets the system time (not implemented in Rust due to platform-specific requirements and permissions).
pub fn settime(_ts: &TimeSpec) -> Result<(), String> {
    Err("Setting system time is not implemented in this Rust version".to_string())
}