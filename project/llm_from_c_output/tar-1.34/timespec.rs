//! System time interface
//!
//! This module provides functionality for working with timespec structures.
//! Original C code copyright (C) 2000-2021 Free Software Foundation, Inc.
//! Licensed under GNU General Public License version 3 or later.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Inverse resolution of timespec timestamps (in units per second)
pub const TIMESPEC_HZ: u64 = 1_000_000_000;
/// Log base 10 of the inverse resolution
pub const LOG10_TIMESPEC_HZ: u32 = 9;

/// Obsolescent names for backward compatibility.
/// They are misnomers, because TIMESPEC_RESOLUTION is not a resolution.
pub const TIMESPEC_RESOLUTION: u64 = TIMESPEC_HZ;
pub const LOG10_TIMESPEC_RESOLUTION: u32 = LOG10_TIMESPEC_HZ;

/// Timespec structure equivalent to C's struct timespec
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    /// Creates a new Timespec with seconds and nanoseconds
    pub fn new(s: i64, ns: i64) -> Self {
        Self { tv_sec: s, tv_nsec: ns }
    }

    /// Compares two Timespec values
    pub fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.tv_sec.cmp(&other.tv_sec) {
            std::cmp::Ordering::Equal => self.tv_nsec.cmp(&other.tv_nsec),
            ord => ord,
        }
    }

    /// Returns the sign of the Timespec
    pub fn sign(&self) -> i32 {
        match self.tv_sec.cmp(&0) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => {
                if self.tv_nsec > 0 { 1 } else { 0 }
            }
        }
    }

    /// Adds two Timespec values
    pub fn add(&self, other: Self) -> Self {
        let mut sec = self.tv_sec + other.tv_sec;
        let mut nsec = self.tv_nsec + other.tv_nsec;
        
        if nsec >= TIMESPEC_HZ as i64 {
            nsec -= TIMESPEC_HZ as i64;
            sec += 1;
        }
        
        Self::new(sec, nsec)
    }

    /// Subtracts two Timespec values
    pub fn sub(&self, other: Self) -> Self {
        let mut sec = self.tv_sec - other.tv_sec;
        let mut nsec = self.tv_nsec - other.tv_nsec;
        
        if nsec < 0 {
            nsec += TIMESPEC_HZ as i64;
            sec -= 1;
        }
        
        Self::new(sec, nsec)
    }

    /// Converts the Timespec to f64 seconds
    pub fn to_seconds(&self) -> f64 {
        self.tv_sec as f64 + (self.tv_nsec as f64 / 1e9)
    }

    /// Creates a Timespec from f64 seconds
    pub fn from_seconds(seconds: f64) -> Self {
        let sec = seconds.trunc() as i64;
        let nsec = (seconds.fract() * 1e9).round() as i64;
        Self::new(sec, nsec)
    }

    /// Gets the current system time as Timespec
    pub fn now() -> Result<Self, std::time::SystemTimeError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur| Self::new(dur.as_secs() as i64, dur.subsec_nanos() as i64))
    }
}

/// Gets the current system time and stores it in the provided Timespec
pub fn gettime(ts: &mut Timespec) -> Result<(), std::time::SystemTimeError> {
    *ts = Timespec::now()?;
    Ok(())
}

/// Sets the system time (not implemented as it requires platform-specific unsafe code)
pub fn settime(_ts: &Timespec) -> Result<(), &'static str> {
    Err("Setting system time is not supported in safe Rust")
}