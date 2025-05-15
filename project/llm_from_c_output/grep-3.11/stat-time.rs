//! Stat-related time functions.
//!
//! This module provides functions for working with file stat timestamps in Rust,
//! equivalent to the C stat-time.h functionality.

use std::os::raw::{c_int, c_long};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io;
use libc::{stat, timespec};

/// Returns the nanosecond component of the access time from stat structure.
pub fn get_stat_atime_ns(st: &stat) -> c_long {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
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
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the status change time from stat structure.
pub fn get_stat_ctime_ns(st: &stat) -> c_long {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
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
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the modification time from stat structure.
pub fn get_stat_mtime_ns(st: &stat) -> c_long {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
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
    )))]
    {
        0
    }
}

/// Returns the access time as a timespec from stat structure.
pub fn get_stat_atime(st: &stat) -> timespec {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
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
    )))]
    {
        timespec {
            tv_sec: st.st_atime,
            tv_nsec: get_stat_atime_ns(st),
        }
    }
}

/// Returns the status change time as a timespec from stat structure.
pub fn get_stat_ctime(st: &stat) -> timespec {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
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
    )))]
    {
        timespec {
            tv_sec: st.st_ctime,
            tv_nsec: get_stat_ctime_ns(st),
        }
    }
}

/// Returns the modification time as a timespec from stat structure.
pub fn get_stat_mtime(st: &stat) -> timespec {
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
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
    )))]
    {
        timespec {
            tv_sec: st.st_mtime,
            tv_nsec: get_stat_mtime_ns(st),
        }
    }
}

/// Returns the birth time as a timespec from stat structure if available.
pub fn get_stat_birthtime(st: &stat) -> timespec {
    let mut t = timespec {
        tv_sec: -1,
        tv_nsec: -1,
    };

    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
    ))]
    {
        t = st.st_birthtim;
        if !(t.tv_sec > 0 && (0..1_000_000_000).contains(&t.tv_nsec)) {
            t.tv_sec = -1;
            t.tv_nsec = -1;
        }
    }

    #[cfg(target_os = "netbsd")]
    {
        t.tv_sec = st.st_birthtime;
        t.tv_nsec = st.st_birthtimensec;
        if !(t.tv_sec > 0 && (0..1_000_000_000).contains(&t.tv_nsec)) {
            t.tv_sec = -1;
            t.tv_nsec = -1;
        }
    }

    #[cfg(target_os = "windows")]
    {
        t.tv_sec = st.st_ctime;
        t.tv_nsec = 0;
    }

    t
}

/// Normalizes timestamps in stat structure to handle potential negative nanoseconds.
/// Returns Ok(adjusted_result) or Err(io::Error) if overflow occurred.
pub fn stat_time_normalize(result: c_int, st: &mut stat) -> io::Result<c_int> {
    #[cfg(target_os = "solaris")]
    {
        if result == 0 {
            const TIMESPEC_HZ: i64 = 1_000_000_000;
            let offsets = [
                std::memoffset::offset_of!(stat, st_atim),
                std::memoffset::offset_of!(stat, st_mtim),
                std::memoffset::offset_of!(stat, st_ctim),
            ];

            for &offset in &offsets {
                let ts = unsafe { &mut *((st as *mut stat as *mut u8).add(offset) as *mut timespec) };
                let mut q = ts.tv_nsec / TIMESPEC_HZ;
                let mut r = ts.tv_nsec % TIMESPEC_HZ;
                
                if r < 0 {
                    r += TIMESPEC_HZ;
                    q -= 1;
                }
                
                ts.tv_nsec = r;
                ts.tv_sec = ts.tv_sec.checked_add(q)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "timestamp overflow"))?;
            }
        }
    }
    Ok(result)
}