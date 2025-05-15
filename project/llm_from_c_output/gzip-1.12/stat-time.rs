//! Stat-related time functions.
//!
//! This module provides functions to access and manipulate file time metadata
//! in a cross-platform manner, similar to the original C code.

use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn new(sec: i64, nsec: i64) -> Self {
        Timespec { tv_sec: sec, tv_nsec: nsec }
    }
}

/// Returns the nanosecond component of the access time from metadata.
pub fn get_stat_atime_ns(metadata: &std::fs::Metadata) -> i64 {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        metadata.atime_nsec()
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd")))]
    {
        metadata.atime_nsec().try_into().unwrap_or(0)
    }
}

/// Returns the nanosecond component of the status change time from metadata.
pub fn get_stat_ctime_ns(metadata: &std::fs::Metadata) -> i64 {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        metadata.ctime_nsec()
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd")))]
    {
        metadata.ctime_nsec().try_into().unwrap_or(0)
    }
}

/// Returns the nanosecond component of the modification time from metadata.
pub fn get_stat_mtime_ns(metadata: &std::fs::Metadata) -> i64 {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        metadata.mtime_nsec()
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "freebsd")))]
    {
        metadata.mtime_nsec().try_into().unwrap_or(0)
    }
}

/// Returns the access time as a Timespec.
pub fn get_stat_atime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec {
        tv_sec: metadata.atime(),
        tv_nsec: get_stat_atime_ns(metadata),
    }
}

/// Returns the status change time as a Timespec.
pub fn get_stat_ctime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec {
        tv_sec: metadata.ctime(),
        tv_nsec: get_stat_ctime_ns(metadata),
    }
}

/// Returns the modification time as a Timespec.
pub fn get_stat_mtime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec {
        tv_sec: metadata.mtime(),
        tv_nsec: get_stat_mtime_ns(metadata),
    }
}

/// Returns the birth time as a Timespec if available, otherwise returns (-1, -1).
pub fn get_stat_birthtime(metadata: &std::fs::Metadata) -> Timespec {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    {
        let birthtime = metadata.created().ok();
        if let Some(time) = birthtime {
            if let Ok(duration) = time.duration_since(UNIX_EPOCH) {
                return Timespec {
                    tv_sec: duration.as_secs() as i64,
                    tv_nsec: duration.subsec_nanos() as i64,
                };
            }
        }
    }
    
    Timespec::new(-1, -1)
}

/// Normalizes the timestamps in the metadata to handle potential negative nanoseconds.
/// Returns Ok(()) on success or Err(io::Error) on overflow.
pub fn stat_time_normalize(metadata: &mut std::fs::Metadata) -> io::Result<()> {
    // In Rust, the MetadataExt methods already provide normalized timestamps,
    // so this is mostly a no-op. We keep the function for compatibility.
    Ok(())
}