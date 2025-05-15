//! Stat-related time functions.
//!
//! This module provides functions to access and manipulate timestamps from file metadata.
//! It's a safe Rust translation of the original C code, using Rust's standard library
//! and following Rust's safety practices.

use std::os::unix::fs::MetadataExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::io;
use std::path::Path;

/// Represents a timestamp with second and nanosecond components
#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    /// Creates a new Timespec with the given seconds and nanoseconds
    pub fn new(sec: i64, nsec: i64) -> Self {
        Timespec { tv_sec: sec, tv_nsec: nsec }
    }

    /// Creates a Timespec representing "not available"
    pub fn not_available() -> Self {
        Timespec { tv_sec: -1, tv_nsec: -1 }
    }
}

/// Returns the nanosecond component of the access time from metadata
pub fn get_stat_atime_ns(metadata: &std::fs::Metadata) -> i64 {
    metadata.atime_nsec() as i64
}

/// Returns the nanosecond component of the status change time from metadata
pub fn get_stat_ctime_ns(metadata: &std::fs::Metadata) -> i64 {
    metadata.ctime_nsec() as i64
}

/// Returns the nanosecond component of the modification time from metadata
pub fn get_stat_mtime_ns(metadata: &std::fs::Metadata) -> i64 {
    metadata.mtime_nsec() as i64
}

/// Returns the access time as Timespec from metadata
pub fn get_stat_atime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec::new(metadata.atime() as i64, get_stat_atime_ns(metadata))
}

/// Returns the status change time as Timespec from metadata
pub fn get_stat_ctime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec::new(metadata.ctime() as i64, get_stat_ctime_ns(metadata))
}

/// Returns the modification time as Timespec from metadata
pub fn get_stat_mtime(metadata: &std::fs::Metadata) -> Timespec {
    Timespec::new(metadata.mtime() as i64, get_stat_mtime_ns(metadata))
}

/// Returns the birth time as Timespec from metadata if available
pub fn get_stat_birthtime(metadata: &std::fs::Metadata) -> Timespec {
    #[cfg(target_os = "macos")]
    {
        // On macOS, birthtime is available
        let sec = metadata.created().unwrap_or_else(|_| UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::new(0, 0))
            .as_secs() as i64;
        Timespec::new(sec, 0)
    }
    #[cfg(not(target_os = "macos"))]
    {
        // On other platforms, birth time may not be available
        Timespec::not_available()
    }
}

/// Normalizes timestamps in case of negative nanoseconds (Solaris 11 bug workaround)
pub fn stat_time_normalize(result: io::Result<()>, _metadata: &std::fs::Metadata) -> io::Result<()> {
    // In Rust, the standard library already handles timestamp normalization,
    // so we just return the original result
    result
}

/// Helper function to get metadata for a path
pub fn get_metadata<P: AsRef<Path>>(path: P) -> io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_timestamps() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file");
        File::create(&file_path).unwrap();

        let metadata = get_metadata(&file_path).unwrap();

        let atime = get_stat_atime(&metadata);
        let ctime = get_stat_ctime(&metadata);
        let mtime = get_stat_mtime(&metadata);
        let birthtime = get_stat_birthtime(&metadata);

        assert!(atime.tv_sec > 0);
        assert!(ctime.tv_sec > 0);
        assert!(mtime.tv_sec > 0);

        #[cfg(target_os = "macos")]
        assert!(birthtime.tv_sec > 0);
        #[cfg(not(target_os = "macos"))]
        assert_eq!(birthtime.tv_sec, -1);
    }
}