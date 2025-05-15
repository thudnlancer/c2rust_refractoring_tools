//! Take file names apart into directory and base names.
//!
//! This module provides functionality similar to the C version, but implemented
//! in safe Rust with proper error handling and memory management.

use std::path::{Path, PathBuf};
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use std::borrow::Cow;

/// Returns the directory component of a path.
/// 
/// Similar to `mdir_name` in the C version, but returns a `PathBuf` instead.
/// Handles memory allocation safely using Rust's ownership system.
pub fn mdir_name(file: &Path) -> Option<PathBuf> {
    let parent = file.parent();
    match parent {
        Some(p) if p.as_os_str().is_empty() => Some(PathBuf::from(".")),
        Some(p) => Some(p.to_path_buf()),
        None => Some(PathBuf::from(".")),
    }
}

/// Returns the directory component of a path, exiting on allocation failure.
/// 
/// Similar to `dir_name` in the C version, but panics on error rather than
/// exiting directly, as that's more idiomatic in Rust.
pub fn dir_name(file: &Path) -> PathBuf {
    mdir_name(file).expect("memory exhausted")
}

/// Returns the length of the directory component of a path.
/// 
/// Similar to `dir_len` in the C version, but works with Rust's Path.
pub fn dir_len(file: &Path) -> usize {
    file.parent()
        .map(|p| p.as_os_str().len())
        .unwrap_or(0)
}

/// Strips trailing slashes from a path in place.
/// 
/// Similar to `strip_trailing_slashes` in the C version, but works with
/// Rust's Path and OsStr types.
pub fn strip_trailing_slashes(file: &mut PathBuf) -> bool {
    let mut s = file.as_os_str().as_bytes().to_vec();
    let mut changed = false;
    
    while s.len() > 1 && s.last() == Some(&b'/') {
        s.pop();
        changed = true;
    }
    
    if changed {
        *file = PathBuf::from(OsStrExt::from_bytes(&s));
    }
    
    changed
}

/// Returns the base name component of a path.
/// 
/// Similar to `base_name` in the C version, but returns a Cow<str> to avoid
/// unnecessary allocations.
pub fn base_name(file: &Path) -> Cow<str> {
    file.file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| Cow::Borrowed("."))
}