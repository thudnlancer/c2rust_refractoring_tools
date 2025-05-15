//! Extract the last component (base name) of a file name.
//!
//! This is a Rust implementation of the GNU basename-lgpl functionality,
//! following the same logic but using Rust's safety features.

use std::path::{Component, Path, Prefix};
use std::ffi::OsStr;

/// Return the last file name component of a path.
/// 
/// If the path has trailing slashes, they are considered part of the last component.
/// If the path is a filesystem root, returns an empty string.
/// 
/// Examples:
/// ```
/// assert_eq!(last_component("foo.c"), "foo.c");
/// assert_eq!(last_component("foo/bar.c"), "bar.c");
/// assert_eq!(last_component("/foo/bar.c"), "bar.c");
/// assert_eq!(last_component("foo/bar/"), "bar/");
/// assert_eq!(last_component("foo/bar//"), "bar//");
/// assert_eq!(last_component("/"), "");
/// assert_eq!(last_component("//"), "");
/// assert_eq!(last_component(""), "");
/// ```
pub fn last_component(filename: &str) -> &str {
    let path = Path::new(filename);
    let mut last_component = OsStr::new("");
    let mut has_trailing_slash = false;
    
    for component in path.components() {
        match component {
            Component::Normal(s) => {
                last_component = s;
                has_trailing_slash = false;
            }
            Component::Prefix(_) | Component::RootDir => {
                last_component = OsStr::new("");
                has_trailing_slash = false;
            }
            Component::CurDir | Component::ParentDir => {
                last_component = component.as_os_str();
                has_trailing_slash = false;
            }
        }
    }
    
    // Handle trailing slashes
    if filename.ends_with('/') || filename.ends_with('\\') {
        let base = path.file_name().unwrap_or(OsStr::new(""));
        let base_str = base.to_str().unwrap_or("");
        let trailing = &filename[filename.rfind(base_str).unwrap_or(0) + base_str.len()..];
        return &filename[filename.len() - trailing.len() - base_str.len()..];
    }
    
    last_component.to_str().unwrap_or("")
}

/// Return the length of the basename, omitting trailing slashes.
/// 
/// Examples:
/// ```
/// assert_eq!(base_len("foo/bar/"), 3);  // "bar"
/// assert_eq!(base_len("foo/bar//"), 3); // "bar"
/// assert_eq!(base_len("/"), 0);
/// assert_eq!(base_len("//"), 0);
/// assert_eq!(base_len(""), 0);
/// ```
pub fn base_len(filename: &str) -> usize {
    let path = Path::new(filename);
    let mut len = filename.len();
    
    // Remove trailing slashes
    while len > 1 && (filename.chars().nth(len - 1) == Some('/') || 
                      filename.chars().nth(len - 1) == Some('\\')) {
        len -= 1;
    }
    
    // Handle special cases
    if filename.len() >= 2 && filename.chars().nth(0) == Some('/') && 
       filename.chars().nth(1) == Some('/') && filename.len() == 2 {
        return 2;
    }
    
    // Handle Windows drive prefixes
    if cfg!(windows) {
        if let Some(Component::Prefix(prefix)) = path.components().next() {
            let prefix_len = match prefix.kind() {
                Prefix::Verbatim(s) | 
                Prefix::VerbatimUNC(_, s) | 
                Prefix::VerbatimDisk(s) => s.len(),
                Prefix::DeviceNS(s) | 
                Prefix::UNC(_, s) | 
                Prefix::Disk(s) => s.len(),
            };
            
            if len == prefix_len && (filename.chars().nth(prefix_len) == Some('/') || 
                                    filename.chars().nth(prefix_len) == Some('\\')) {
                return prefix_len + 1;
            }
        }
    }
    
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_component() {
        assert_eq!(last_component("foo.c"), "foo.c");
        assert_eq!(last_component("foo/bar.c"), "bar.c");
        assert_eq!(last_component("/foo/bar.c"), "bar.c");
        assert_eq!(last_component("foo/bar/"), "bar/");
        assert_eq!(last_component("foo/bar//"), "bar//");
        assert_eq!(last_component("/"), "");
        assert_eq!(last_component("//"), "");
        assert_eq!(last_component(""), "");
    }

    #[test]
    fn test_base_len() {
        assert_eq!(base_len("foo/bar/"), 3);
        assert_eq!(base_len("foo/bar//"), 3);
        assert_eq!(base_len("/"), 0);
        assert_eq!(base_len("//"), if cfg!(windows) { 2 } else { 0 });
        assert_eq!(base_len(""), 0);
    }
}