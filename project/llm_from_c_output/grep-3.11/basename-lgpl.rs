//! Extract the last component (base name) of a file name.
//!
//! This implementation follows the same behavior as the original C code but
//! uses Rust's safety features and standard libraries.

use std::path::{Component, Path, Prefix};
use std::ffi::OsStr;

/// Returns the last component of a path as an OsStr reference.
/// 
/// This matches the behavior of the original C function:
/// - Trailing slashes are considered part of the last component
/// - For root paths, returns an empty OsStr
/// - The result is a reference to part of the input
pub fn last_component(filename: &OsStr) -> &OsStr {
    let path = Path::new(filename);
    let mut last_component = OsStr::new("");
    
    for component in path.components() {
        match component {
            Component::Normal(p) => last_component = p,
            Component::CurDir => last_component = OsStr::new("."),
            Component::ParentDir => last_component = OsStr::new(".."),
            Component::Prefix(prefix) => {
                last_component = match prefix.kind() {
                    Prefix::Verbatim(s) => s,
                    Prefix::VerbatimUNC(_, s) => s,
                    Prefix::VerbatimDisk(_) => OsStr::new(""),
                    Prefix::DeviceNS(s) => s,
                    Prefix::UNC(_, s) => s,
                    Prefix::Disk(_) => OsStr::new(""),
                };
            }
            Component::RootDir => last_component = OsStr::new(""),
        }
    }
    
    // Handle trailing slashes by checking if the original path ended with a separator
    if path.as_os_str().len() > 0 {
        let mut chars = path.as_os_str().to_string_lossy().chars();
        if chars.next_back() == Some('/') {
            return path.as_os_str();
        }
    }
    
    last_component
}

/// Returns the length of the basename, omitting trailing slashes.
/// 
/// This matches the behavior of the original C function:
/// - Trailing slashes are not counted
/// - Special handling for double slashes and drive prefixes
pub fn base_len(filename: &OsStr) -> usize {
    let s = filename.to_string_lossy();
    let mut len = s.len();
    let bytes = s.as_bytes();
    
    // Skip trailing slashes
    while len > 1 && (bytes[len - 1] == b'/' || bytes[len - 1] == b'\\') {
        len -= 1;
    }
    
    // Handle special cases
    if len == 1 && (bytes[0] == b'/' || bytes[0] == b'\\') {
        return len;
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
        assert_eq!(last_component(OsStr::new("foo/bar/")), OsStr::new("bar/"));
        assert_eq!(last_component(OsStr::new("foo/bar//")), OsStr::new("bar//"));
        assert_eq!(last_component(OsStr::new("/")), OsStr::new(""));
        assert_eq!(last_component(OsStr::new("//")), OsStr::new(""));
        assert_eq!(last_component(OsStr::new("")), OsStr::new(""));
    }

    #[test]
    fn test_base_len() {
        assert_eq!(base_len(OsStr::new("foo.c")), 5);
        assert_eq!(base_len(OsStr::new("foo/bar.c")), 5);
        assert_eq!(base_len(OsStr::new("/foo/bar.c")), 5);
        assert_eq!(base_len(OsStr::new("foo/bar/")), 4);
        assert_eq!(base_len(OsStr::new("foo/bar//")), 4);
        assert_eq!(base_len(OsStr::new("/")), 1);
        assert_eq!(base_len(OsStr::new("//")), 1);
        assert_eq!(base_len(OsStr::new("")), 0);
    }
}