//! stat-related time functions.
//!
//! This module provides functions to work with file timestamps from `stat` in a portable way.

use std::os::raw::{c_int, c_long};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io;
use libc::{stat, timespec};

/// Returns the nanosecond component of the access time from a `stat` structure.
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

    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        st.st_atimensec
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "solaris",
        target_os = "illumos"
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the status change time from a `stat` structure.
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

    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        st.st_ctimensec
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "solaris",
        target_os = "illumos"
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the modification time from a `stat` structure.
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

    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    {
        st.st_mtimensec
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "solaris",
        target_os = "illumos"
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the birth time from a `stat` structure.
pub fn get_stat_birthtime_ns(st: &stat) -> c_long {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        st.st_birthtim.tv_nsec
    }

    #[cfg(any(target_os = "netbsd"))]
    {
        st.st_birthtimensec
    }

    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd"
    )))]
    {
        0
    }
}

/// Returns the access time as a `timespec` from a `stat` structure.
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

/// Returns the status change time as a `timespec` from a `stat` structure.
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

/// Returns the modification time as a `timespec` from a `stat` structure.
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

/// Returns the birth time as a `timespec` from a `stat` structure.
pub fn get_stat_birthtime(st: &stat) -> timespec {
    let mut t = timespec {
        tv_sec: -1,
        tv_nsec: -1,
    };

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        t = st.st_birthtim;
    }

    #[cfg(any(target_os = "netbsd"))]
    {
        t.tv_sec = st.st_birthtime;
        t.tv_nsec = st.st_birthtimensec;
    }

    #[cfg(target_os = "windows")]
    {
        t.tv_sec = st.st_ctime;
        t.tv_nsec = 0;
    }

    // Validate the birth time if available
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd"
    ))]
    {
        if !(t.tv_sec > 0 && t.tv_nsec >= 0 && t.tv_nsec < 1_000_000_000) {
            t.tv_sec = -1;
            t.tv_nsec = -1;
        }
    }

    t
}

/// Normalizes timestamps in a `stat` structure to handle potential negative nanoseconds.
/// Returns the original result or an error if normalization overflowed.
pub fn stat_time_normalize(result: c_int, st: &mut stat) -> io::Result<c_int> {
    #[cfg(target_os = "solaris")]
    {
        if result == 0 {
            const TIMESPEC_HZ: c_long = 1_000_000_000;
            let offsets = [
                unsafe { &st.st_atim as *const _ as usize - st as *const _ as usize },
                unsafe { &st.st_mtim as *const _ as usize - st as *const _ as usize },
                unsafe { &st.st_ctim as *const _ as usize - st as *const _ as usize },
            ];

            for &offset in &offsets {
                let ts = unsafe { &mut *((st as *mut stat as *mut u8).add(offset) as *mut timespec };
                unsafe {
                    let mut q = (*ts).tv_nsec / TIMESPEC_HZ;
                    let mut r = (*ts).tv_nsec % TIMESPEC_HZ;
                    
                    if r < 0 {
                        r += TIMESPEC_HZ;
                        q -= 1;
                    }
                    
                    (*ts).tv_nsec = r;
                    
                    if let Some(new_sec) = (*ts).tv_sec.checked_add(q) {
                        (*ts).tv_sec = new_sec;
                    } else {
                        return Err(io::Error::new(io::ErrorKind::Other, "timestamp overflow"));
                    }
                }
            }
        }
    }

    Ok(result)
}