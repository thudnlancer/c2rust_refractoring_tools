//! Stat-related time functions.
//!
//! This module provides functions to access and manipulate file time information
//! from `std::fs::Metadata` in a cross-platform way.

use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

/// Returns the nanosecond component of the access time.
pub fn get_stat_atime_ns(metadata: &Metadata) -> i64 {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        metadata.atime_nsec() as i64
    }
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        metadata.atime_nsec() as i64
    }
    #[cfg(target_os = "windows")]
    {
        0
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
        target_os = "windows"
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the status change time.
pub fn get_stat_ctime_ns(metadata: &Metadata) -> i64 {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        metadata.ctime_nsec() as i64
    }
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        metadata.ctime_nsec() as i64
    }
    #[cfg(target_os = "windows")]
    {
        0
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
        target_os = "windows"
    )))]
    {
        0
    }
}

/// Returns the nanosecond component of the modification time.
pub fn get_stat_mtime_ns(metadata: &Metadata) -> i64 {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        metadata.mtime_nsec() as i64
    }
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd",
        target_os = "dragonfly"
    ))]
    {
        metadata.mtime_nsec() as i64
    }
    #[cfg(target_os = "windows")]
    {
        0
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
        target_os = "windows"
    )))]
    {
        0
    }
}

/// Returns the access time as a `SystemTime`.
pub fn get_stat_atime(metadata: &Metadata) -> SystemTime {
    let sec = metadata.atime() as i64;
    let nsec = get_stat_atime_ns(metadata);
    UNIX_EPOCH + Duration::new(sec as u64, nsec as u32)
}

/// Returns the status change time as a `SystemTime`.
pub fn get_stat_ctime(metadata: &Metadata) -> SystemTime {
    let sec = metadata.ctime() as i64;
    let nsec = get_stat_ctime_ns(metadata);
    UNIX_EPOCH + Duration::new(sec as u64, nsec as u32)
}

/// Returns the modification time as a `SystemTime`.
pub fn get_stat_mtime(metadata: &Metadata) -> SystemTime {
    let sec = metadata.mtime() as i64;
    let nsec = get_stat_mtime_ns(metadata);
    UNIX_EPOCH + Duration::new(sec as u64, nsec as u32)
}

/// Returns the birth time as a `SystemTime`, if available.
pub fn get_stat_birthtime(metadata: &Metadata) -> Option<SystemTime> {
    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    {
        let sec = metadata.birthtime() as i64;
        let nsec = metadata.birthtime_nsec() as i64;
        
        if sec == 0 && nsec == 0 {
            None
        } else {
            Some(UNIX_EPOCH + Duration::new(sec as u64, nsec as u32))
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, creation time is stored in the file attributes
        let sec = metadata.creation_time() as i64;
        Some(UNIX_EPOCH + Duration::new(sec as u64, 0))
    }
    
    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "windows"
    )))]
    {
        None
    }
}