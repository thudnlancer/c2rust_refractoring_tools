//! Stat-related time functions.
//!
//! This module provides functions to access and manipulate timestamps from file metadata.

use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io;
use std::path::Path;

/// Represents a timestamp with nanosecond precision.
#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    /// Creates a new Timespec with the given seconds and nanoseconds.
    pub fn new(sec: i64, nsec: i64) -> Self {
        Timespec { tv_sec: sec, tv_nsec: nsec }
    }

    /// Creates a Timespec representing an invalid timestamp.
    pub fn invalid() -> Self {
        Timespec::new(-1, -1)
    }

    /// Checks if this timestamp is valid.
    pub fn is_valid(&self) -> bool {
        self.tv_sec >= 0 && self.tv_nsec >= 0 && self.tv_nsec < 1_000_000_000
    }
}

/// Returns the nanosecond component of the access time from metadata.
pub fn get_stat_atime_ns(metadata: &std::fs::Metadata) -> i64 {
    metadata.atime_nsec() as i64
}

/// Returns the nanosecond component of the status change time from metadata.
pub fn get_stat_ctime_ns(metadata: &std::fs::Metadata) -> i64 {
    metadata.ctime_nsec() as i64
}

/// Returns the nanosecond component of the modification time from metadata.
pub fn get_stat_mtime_ns(metadata: &std::fs::Metadata) -> i64 {
    metadata.mtime_nsec() as i64
}

/// Returns the access time as a Timespec from metadata.
pub fn get_stat_atime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec::new(metadata.atime() as i64, get_stat_atime_ns(metadata))
}

/// Returns the status change time as a Timespec from metadata.
pub fn get_stat_ctime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec::new(metadata.ctime() as i64, get_stat_ctime_ns(metadata))
}

/// Returns the modification time as a Timespec from metadata.
pub fn get_stat_mtime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec::new(metadata.mtime() as i64, get_stat_mtime_ns(metadata))
}

/// Returns the birth time as a Timespec from metadata if available.
/// Returns an invalid Timespec if birth time is not supported.
pub fn get_stat_birthtime(metadata: &std::fs::Metadata) -> Timespec {
    #[cfg(target_os = "macos")]
    {
        let birthtime = metadata.created().ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok());
        
        if let Some(duration) = birthtime {
            Timespec::new(duration.as_secs() as i64, duration.subsec_nanos() as i64)
        } else {
            Timespec::invalid()
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Timespec::invalid()
    }
}

/// Normalizes timestamps in metadata to handle potential negative nanoseconds.
/// Returns Ok(()) if successful, or Err(io::Error) if overflow occurred.
pub fn stat_time_normalize(metadata: &mut std::fs::Metadata) -> io::Result<()> {
    // On Unix systems, the metadata should already be normalized
    Ok(())
}

/// Helper function to get metadata for a path.
pub fn get_metadata<P: AsRef<Path>>(path: P) -> io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}