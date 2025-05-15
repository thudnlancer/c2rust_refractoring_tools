//! Extract the last component (base name) of a file name.
//!
//! This implementation follows the same behavior as the original C code:
//! - Returns the last component of a path, including any trailing slashes
//! - Returns an empty string for filesystem roots
//! - Handles platform-specific path separators
//! - Preserves the original behavior regarding trailing slashes

use std::path::{Component, Path, Prefix};
use std::ffi::OsStr;

/// Return the last component of a path as an OsStr reference.
/// 
/// This matches the behavior of the C last_component() function:
/// - Trailing slashes are considered part of the last component
/// - Filesystem roots return an empty string
/// - The result is a reference to the original path
pub fn last_component(path: &OsStr) -> &OsStr {
    let path = Path::new(path);
    let mut last_component = OsStr::new("");
    let mut has_components = false;

    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                last_component = component.as_os_str();
                has_components = false;
            }
            Component::CurDir | Component::ParentDir | Component::Normal(_) => {
                last_component = component.as_os_str();
                has_components = true;
            }
        }
    }

    if !has_components && path.components().count() > 0 {
        // This is a root path, return empty string
        OsStr::new("")
    } else {
        last_component
    }
}

/// Return the length of the basename, omitting trailing slashes.
/// 
/// This matches the behavior of the C base_len() function:
/// - Trailing slashes are not counted
/// - Special handling for filesystem roots
pub fn base_len(path: &OsStr) -> usize {
    let path = Path::new(path);
    let last = last_component(path.as_os_str());
    
    let mut len = last.len();
    let bytes = if let Some(s) = last.to_str() {
        s.as_bytes()
    } else {
        // For non-UTF8 paths, we'll treat as bytes
        last.to_bytes()
    };

    // Remove trailing slashes
    while len > 0 && matches!(bytes.get(len - 1), Some(b'/' | b'\\')) {
        len -= 1;
    }

    // Handle special cases for roots
    if len == 0 {
        if let Some(Component::Prefix(prefix)) = path.components().next() {
            match prefix.kind() {
                Prefix::VerbatimDisk(_) | Prefix::Disk(_) => return 2,
                Prefix::VerbatimUNC(_, _) | Prefix::UNC(_, _) => return 0,
                _ => {}
            }
        }
        
        // Check for root directory case
        if path.components().any(|c| matches!(c, Component::RootDir)) {
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
        assert_eq!(last_component(OsStr::new("foo.c")), "foo.c");
        assert_eq!(last_component(OsStr::new("foo/bar.c")), "bar.c");
        assert_eq!(last_component(OsStr::new("/foo/bar.c")), "bar.c");
        assert_eq!(last_component(OsStr::new("foo/bar/")), "bar/");
        assert_eq!(last_component(OsStr::new("foo/bar//")), "bar//");
        assert_eq!(last_component(OsStr::new("/")), "");
        assert_eq!(last_component(OsStr::new("//")), "");
        assert_eq!(last_component(OsStr::new("")), "");
    }

    #[test]
    fn test_base_len() {
        assert_eq!(base_len(OsStr::new("foo.c")), 5);
        assert_eq!(base_len(OsStr::new("foo/bar.c")), 5);
        assert_eq!(base_len(OsStr::new("/foo/bar.c")), 5);
        assert_eq!(base_len(OsStr::new("foo/bar/")), 4);
        assert_eq!(base_len(OsStr::new("foo/bar//")), 4);
        assert_eq!(base_len(OsStr::new("/")), 0);
        assert_eq!(base_len(OsStr::new("//")), 0);
        assert_eq!(base_len(OsStr::new("")), 0);
    }
}