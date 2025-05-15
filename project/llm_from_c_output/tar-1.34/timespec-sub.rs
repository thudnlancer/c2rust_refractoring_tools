//! Subtract two struct timespec values.
//!
//! This code is translated from C to Rust while maintaining the same functionality.
//! It follows Rust's safety practices and uses standard Rust libraries.

use std::time::Duration;
use std::cmp::{min, max};
use std::i64;

/// Represents a timespec similar to the C struct timespec
#[derive(Debug, Copy, Clone)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}

const TIMESPEC_HZ: i32 = 1_000_000_000;

/// Subtracts two Timespec values, handling overflow by returning extremal values.
/// Assumes 0 <= tv_nsec < TIMESPEC_HZ for both inputs.
pub fn timespec_sub(a: Timespec, b: Timespec) -> Timespec {
    let mut rs = a.tv_sec;
    let mut bs = b.tv_sec;
    let ns = a.tv_nsec - b.tv_nsec;
    let mut rns = ns;

    if ns < 0 {
        rns = ns + TIMESPEC_HZ;
        if let Some(bs1) = bs.checked_add(1) {
            bs = bs1;
        } else if rs > i64::MIN {
            rs -= 1;
        } else {
            // low_overflow
            return Timespec {
                tv_sec: i64::MIN,
                tv_nsec: 0,
            };
        }
    }

    if let Some(sub_result) = rs.checked_sub(bs) {
        rs = sub_result;
    } else {
        if bs > 0 {
            // low_overflow
            return Timespec {
                tv_sec: i64::MIN,
                tv_nsec: 0,
            };
        } else {
            return Timespec {
                tv_sec: i64::MAX,
                tv_nsec: TIMESPEC_HZ - 1,
            };
        }
    }

    Timespec {
        tv_sec: rs,
        tv_nsec: rns,
    }
}

/// Helper function to create a Timespec (equivalent to make_timespec in C)
pub fn make_timespec(sec: i64, nsec: i32) -> Timespec {
    Timespec {
        tv_sec: sec,
        tv_nsec: nsec,
    }
}