//! stat-related time functions.
//!
//! This module provides functions for working with file stat timestamps in Rust,
//! equivalent to the original C code.

use std::os::raw::{c_int, c_long};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::io;
use libc::{stat, timespec};

/// Returns the nanosecond component of the access time from a stat structure.
pub fn get_stat_atime_ns(st: &stat) -> c_long {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        st.st_atim.tv_nsec
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the status change time from a stat structure.
pub fn get_stat_ctime_ns(st: &stat) -> c_long {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        st.st_ctim.tv_nsec
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the modification time from a stat structure.
pub fn get_stat_mtime_ns(st: &stat) -> c_long {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        st.st_mtim.tv_nsec
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    )))]
    {
        0
    }
}

/// Returns the access time as a timespec from a stat structure.
pub fn get_stat_atime(st: &stat) -> timespec {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        st.st_atim
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    )))]
    {
        timespec {
            tv_sec: st.st_atime,
            tv_nsec: get_stat_atime_ns(st),
        }
    }
}

/// Returns the status change time as a timespec from a stat structure.
pub fn get_stat_ctime(st: &stat) -> timespec {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        st.st_ctim
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    )))]
    {
        timespec {
            tv_sec: st.st_ctime,
            tv_nsec: get_stat_ctime_ns(st),
        }
    }
}

/// Returns the modification time as a timespec from a stat structure.
pub fn get_stat_mtime(st: &stat) -> timespec {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        st.st_mtim
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    )))]
    {
        timespec {
            tv_sec: st.st_mtime,
            tv_nsec: get_stat_mtime_ns(st),
        }
    }
}

/// Returns the birth time as a timespec from a stat structure if available.
/// Returns a timespec with tv_sec and tv_nsec set to -1 if not available.
pub fn get_stat_birthtime(st: &stat) -> timespec {
    let mut t = timespec {
        tv_sec: -1,
        tv_nsec: -1,
    };

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        t.tv_sec = st.st_birthtime;
        t.tv_nsec = st.st_birthtimensec;
    }

    #[cfg(target_os = "windows")]
    {
        t.tv_sec = st.st_ctime;
        t.tv_nsec = 0;
    }

    // Check for invalid timespec values
    if t.tv_sec > 0 && (0 <= t.tv_nsec && t.tv_nsec < 1_000_000_000) {
        t
    } else {
        timespec {
            tv_sec: -1,
            tv_nsec: -1,
        }
    }
}

/// Normalizes the timestamps in a stat structure to handle potential negative nanoseconds.
/// Returns the original result or -1 with errno set to EOVERFLOW if normalization fails.
pub fn stat_time_normalize(result: c_int, st: &mut stat) -> c_int {
    if result != 0 {
        return result;
    }

    #[cfg(target_os = "solaris")]
    {
        const TIMESPEC_HZ: c_long = 1_000_000_000;
        let fields = [
            &mut st.st_atim,
            &mut st.st_mtim,
            &mut st.st_ctim,
        ];

        for ts in fields.iter_mut() {
            let mut q = ts.tv_nsec / TIMESPEC_HZ;
            let mut r = ts.tv_nsec % TIMESPEC_HZ;

            if r < 0 {
                r += TIMESPEC_HZ;
                q -= 1;
            }

            ts.tv_nsec = r;
            if let Some(new_sec) = ts.tv_sec.checked_add(q) {
                ts.tv_sec = new_sec;
            } else {
                return -1;
            }
        }
    }

    result
}