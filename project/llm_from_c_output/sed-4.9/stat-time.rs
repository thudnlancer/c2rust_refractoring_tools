//! Stat-related time functions.
//!
//! This module provides functions to access and manipulate file time information
//! from `std::fs::Metadata` in a cross-platform way.

use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

/// Returns the nanosecond component of the access time from metadata.
pub fn get_stat_atime_ns(metadata: &Metadata) -> i64 {
    metadata.atime_nsec() as i64
}

/// Returns the nanosecond component of the status change time from metadata.
pub fn get_stat_ctime_ns(metadata: &Metadata) -> i64 {
    metadata.ctime_nsec() as i64
}

/// Returns the nanosecond component of the modification time from metadata.
pub fn get_stat_mtime_ns(metadata: &Metadata) -> i64 {
    metadata.mtime_nsec() as i64
}

/// Returns the access time as a `SystemTime`.
pub fn get_stat_atime(metadata: &Metadata) -> SystemTime {
    UNIX_EPOCH + Duration::new(metadata.atime() as u64, metadata.atime_nsec() as u32)
}

/// Returns the status change time as a `SystemTime`.
pub fn get_stat_ctime(metadata: &Metadata) -> SystemTime {
    UNIX_EPOCH + Duration::new(metadata.ctime() as u64, metadata.ctime_nsec() as u32)
}

/// Returns the modification time as a `SystemTime`.
pub fn get_stat_mtime(metadata: &Metadata) -> SystemTime {
    UNIX_EPOCH + Duration::new(metadata.mtime() as u64, metadata.mtime_nsec() as u32)
}

/// Returns the birth time as a `SystemTime`, if available.
/// Returns `None` if birth time is not supported or unavailable.
pub fn get_stat_birthtime(metadata: &Metadata) -> Option<SystemTime> {
    #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "netbsd"))]
    {
        let birthtime = metadata.birthtime() as u64;
        let birthtime_nsec = metadata.birthtime_nsec() as u32;
        
        if birthtime == 0 && birthtime_nsec == 0 {
            None
        } else {
            Some(UNIX_EPOCH + Duration::new(birthtime, birthtime_nsec))
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // On Windows, creation time is stored in the standard creation time field
        let creation_time = metadata.creation_time();
        Some(UNIX_EPOCH + Duration::from_nanos(creation_time as u64 * 100))
    }
    
    #[cfg(not(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "windows"
    )))]
    {
        None
    }
}

/// Normalizes timestamps in metadata to handle potential platform-specific issues.
/// Currently a no-op in Rust implementation as the standard library handles this.
pub fn stat_time_normalize(result: std::io::Result<()>) -> std::io::Result<()> {
    result
}