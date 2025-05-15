//! Take file names apart into directory and base names.
//!
//! This module provides functionality similar to the C version, but implemented
//! in safe Rust with proper error handling and memory management.

use std::path::{Path, PathBuf};
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use std::io;

pub const DIRECTORY_SEPARATOR: char = '/';

/// Returns the base name of the given file path.
/// The caller is responsible for freeing the returned string.
pub fn base_name(file: &Path) -> Option<String> {
    file.file_name()
        .and_then(|f| f.to_str())
        .map(|s| s.to_string())
}

/// Returns the directory name of the given file path.
/// Returns None if memory allocation fails.
pub fn dir_name(file: &Path) -> Option<PathBuf> {
    file.parent().map(|p| p.to_path_buf())
}

/// Returns the directory name of the given file path.
/// Exits the process if memory allocation fails.
pub fn dir_name_or_exit(file: &Path) -> PathBuf {
    dir_name(file).unwrap_or_else(|| {
        eprintln!("memory exhausted");
        std::process::exit(1);
    })
}

/// Returns the length of the directory portion of the path.
pub fn dir_len(file: &Path) -> usize {
    file.parent()
        .and_then(|p| p.to_str())
        .map(|s| s.len())
        .unwrap_or(0)
}

/// Strips trailing slashes from the given path in place.
/// Returns true if any slashes were stripped.
pub fn strip_trailing_slashes(file: &mut PathBuf) -> bool {
    let mut s = file.to_string_lossy().into_owned();
    let original_len = s.len();
    
    while s.ends_with(DIRECTORY_SEPARATOR) {
        s.pop();
    }
    
    if s.len() != original_len {
        *file = PathBuf::from(s);
        true
    } else {
        false
    }
}

/// Memory-safe version of dir_name that returns Result instead of exiting.
pub fn mdir_name(file: &Path) -> io::Result<PathBuf> {
    dir_name(file).ok_or_else(|| io::Error::new(io::ErrorKind::Other, "invalid path"))
}