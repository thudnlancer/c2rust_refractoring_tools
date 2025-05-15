//! Take file names apart into directory and base names.
//!
//! This module provides functionality similar to the C version, but implemented
//! in safe Rust following Rust's ownership and error handling principles.

use std::path::{Path, PathBuf};
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use std::io;

const DIRECTORY_SEPARATOR: char = '/';

/// Returns the base name of the given file path.
/// Similar to base_name in C, but returns a PathBuf for better safety.
pub fn base_name(file: &Path) -> PathBuf {
    PathBuf::from(file.file_name().unwrap_or_else(|| file.as_os_str()))
}

/// Returns the directory name of the given file path.
/// Similar to dir_name in C, but returns a PathBuf and handles errors properly.
pub fn dir_name(file: &Path) -> io::Result<PathBuf> {
    let parent = file.parent();
    match parent {
        Some(p) if p.as_os_str().is_empty() => Ok(PathBuf::from(".")),
        Some(p) => Ok(p.to_path_buf()),
        None => Ok(PathBuf::from(".")),
    }
}

/// Returns the directory name of the given file path, exiting on allocation failure.
/// Similar to the C dir_name function that calls xalloc_die on failure.
pub fn dir_name_or_exit(file: &Path) -> PathBuf {
    dir_name(file).unwrap_or_else(|_| {
        eprintln!("memory exhausted");
        std::process::exit(1);
    })
}

/// Returns the length of the directory portion of the path.
pub fn dir_len(file: &Path) -> usize {
    file.parent().map_or(0, |p| p.as_os_str().len())
}

/// Strips trailing slashes from the given path (in-place).
/// Returns true if any slashes were stripped.
pub fn strip_trailing_slashes(file: &mut PathBuf) -> bool {
    let mut s = file.to_string_lossy().into_owned();
    let len = s.len();
    let mut changed = false;
    
    while len > 1 && s.ends_with(DIRECTORY_SEPARATOR) {
        s.pop();
        changed = true;
    }
    
    if changed {
        *file = PathBuf::from(s);
    }
    changed
}

/// Similar to mdir_name in C, but returns a Result for proper error handling.
pub fn mdir_name(file: &Path) -> io::Result<PathBuf> {
    dir_name(file)
}