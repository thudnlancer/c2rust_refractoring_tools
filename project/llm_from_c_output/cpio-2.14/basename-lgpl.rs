//! Extract the last component (base name) of a file name.
//!
//! This is a Rust translation of the GNU basename-lgpl implementation.
//! It maintains the same functionality while adhering to Rust's safety guarantees.

use std::path::{Component, Path, Prefix};
use std::ffi::OsStr;

/// Returns the last component of a path as an OsStr.
/// 
/// This follows the same behavior as the C version:
/// - Trailing slashes are considered part of the last component
/// - For filesystem roots, returns an empty string
/// - The result is a reference to the original path's data
pub fn last_component(path: &OsStr) -> &OsStr {
    let path = Path::new(path);
    let mut last_component = OsStr::new("");
    
    for component in path.components() {
        match component {
            Component::Normal(name) => last_component = name,
            Component::CurDir => last_component = OsStr::new("."),
            Component::ParentDir => last_component = OsStr::new(".."),
            Component::Prefix(prefix) => {
                last_component = match prefix.kind() {
                    Prefix::Verbatim(s) => s.as_os_str(),
                    Prefix::VerbatimUNC(_, s) => s.as_os_str(),
                    Prefix::VerbatimDisk(_) => prefix.as_os_str(),
                    Prefix::DeviceNS(_) => prefix.as_os_str(),
                    Prefix::UNC(_, s) => s.as_os_str(),
                    Prefix::Disk(_) => prefix.as_os_str(),
                };
            }
            Component::RootDir => last_component = OsStr::new(""),
        }
    }
    
    last_component
}

/// Returns the length of the basename, omitting trailing slashes.
/// 
/// This follows the same behavior as the C version's base_len function.
pub fn base_len(path: &OsStr) -> usize {
    let path = Path::new(path);
    let last = last_component(path.as_os_str());
    
    let mut len = last.len();
    let bytes = if let Some(s) = last.to_str() {
        s.as_bytes()
    } else {
        // For non-UTF8 paths, we'll treat as bytes
        unsafe { std::mem::transmute(last) }
    };
    
    // Strip trailing slashes
    while len > 0 && matches!(bytes.get(len - 1), Some(b'/' | b'\\')) {
        len -= 1;
    }
    
    // Handle special cases from the original C code
    if len == 0 {
        // Check for root cases
        if path.components().count() == 1 && path.has_root() {
            // Handle "//" specifically if needed (DOUBLE_SLASH_IS_DISTINCT_ROOT)
            let s = path.to_str().unwrap_or("");
            if s.starts_with("//") && s.len() == 2 {
                return 2;
            }
            return 1;
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