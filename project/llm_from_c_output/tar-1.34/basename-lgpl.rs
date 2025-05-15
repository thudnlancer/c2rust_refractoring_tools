//! Extract the last component (base name) of a file name.
//!
//! This is a Rust translation of the GNU basename-lgpl implementation.
//! It maintains the same functionality while following Rust's safety practices.

use std::path::{Component, Path, Prefix};
use std::ffi::OsStr;

/// Returns the last component of a path as an OsStr reference.
/// 
/// This follows the same behavior as the C version:
/// - Trailing slashes are considered part of the last component
/// - Returns empty string for filesystem roots
/// - The result is a reference to part of the input
pub fn last_component(path: &OsStr) -> &OsStr {
    let path = Path::new(path);
    let mut last_component = OsStr::new("");
    
    for component in path.components() {
        match component {
            Component::Prefix(p) => {
                // Handle Windows prefixes specially
                last_component = match p.kind() {
                    Prefix::Verbatim(s) | Prefix::VerbatimUNC(_, s) | Prefix::VerbatimDisk(s) => s,
                    Prefix::DeviceNS(s) | Prefix::UNC(_, s) | Prefix::Disk(s) => s,
                };
            }
            Component::RootDir => last_component = OsStr::new("/"),
            Component::CurDir => last_component = OsStr::new("."),
            Component::ParentDir => last_component = OsStr::new(".."),
            Component::Normal(c) => last_component = c,
        }
    }
    
    last_component
}

/// Returns the length of the basename, omitting trailing slashes.
/// 
/// Follows the same behavior as the C version:
/// - Counts all characters except trailing slashes
/// - Special handling for root paths and drive prefixes
pub fn base_len(path: &OsStr) -> usize {
    let path = Path::new(path);
    let mut len = 0;
    
    if let Some(name) = path.file_name() {
        len = name.len();
    } else {
        // Handle root paths and paths ending with slashes
        let s = path.to_string_lossy();
        let mut chars = s.chars().rev();
        
        // Skip trailing slashes
        while let Some('/') = chars.next() {
            continue;
        }
        
        // Count remaining characters
        len = chars.count();
        
        // Special case for root paths
        if len == 0 && s.starts_with('/') {
            return 0;
        }
    }
    
    len
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_last_component() {
        assert_eq!(last_component(OsStr::new("foo.c")), OsStr::new("foo.c"));
        assert_eq!(last_component(OsStr::new("foo/bar.c")), OsStr::new("bar.c"));
        assert_eq!(last_component(OsStr::new("/foo/bar.c")), OsStr::new("bar.c"));
        assert_eq!(last_component(OsStr::new("foo/bar/")), OsStr::new("bar"));
        assert_eq!(last_component(OsStr::new("foo/bar//")), OsStr::new("bar"));
        assert_eq!(last_component(OsStr::new("/")), OsStr::new(""));
        assert_eq!(last_component(OsStr::new("//")), OsStr::new(""));
        assert_eq!(last_component(OsStr::new("")), OsStr::new(""));
    }

    #[test]
    fn test_base_len() {
        assert_eq!(base_len(OsStr::new("foo.c")), 5);
        assert_eq!(base_len(OsStr::new("foo/bar.c")), 5);
        assert_eq!(base_len(OsStr::new("/foo/bar.c")), 5);
        assert_eq!(base_len(OsStr::new("foo/bar/")), 3);
        assert_eq!(base_len(OsStr::new("foo/bar//")), 3);
        assert_eq!(base_len(OsStr::new("/")), 0);
        assert_eq!(base_len(OsStr::new("//")), 0);
        assert_eq!(base_len(OsStr::new("")), 0);
    }
}