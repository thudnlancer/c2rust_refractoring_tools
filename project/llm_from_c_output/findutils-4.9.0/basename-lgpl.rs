//! Extract the last component (base name) of a file name.
//!
//! This implementation follows the behavior of the GNU LGPL version of basename,
//! where trailing slashes are considered part of the last component.

use std::path::{Component, Path, Prefix};
use std::ffi::CStr;

/// Returns the last component of a path as a string slice.
/// 
/// This function mimics the behavior of the GNU LGPL `last_component` function.
/// Trailing slashes are considered part of the last component.
/// 
/// # Examples
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
    let mut last_component = "";
    let mut has_components = false;

    for component in path.components() {
        match component {
            Component::Normal(s) => {
                last_component = s.to_str().unwrap_or("");
                has_components = true;
            }
            Component::Prefix(prefix) => {
                last_component = match prefix.kind() {
                    Prefix::Verbatim(s) | Prefix::VerbatimUNC(_, s) | Prefix::VerbatimDisk(_) => {
                        s.to_str().unwrap_or("")
                    }
                    Prefix::DeviceNS(_) | Prefix::UNC(_, _) | Prefix::Disk(_) => "",
                };
            }
            Component::RootDir => {
                last_component = "";
                has_components = true;
            }
            Component::CurDir | Component::ParentDir => {
                last_component = component.as_os_str().to_str().unwrap_or("");
                has_components = true;
            }
        }
    }

    if !has_components {
        return "";
    }

    // Handle trailing slashes
    if filename.ends_with('/') || filename.ends_with('\\') {
        let base = path.iter().last().unwrap_or_default();
        let base_str = base.to_str().unwrap_or("");
        let trailing_slashes = &filename[filename.rfind(base_str).unwrap_or(0) + base_str.len()..];
        return &filename[filename.rfind(base_str).unwrap_or(0)..];
    }

    last_component
}

/// Returns the length of the basename, omitting trailing slashes.
/// 
/// # Examples
/// ```
/// assert_eq!(base_len("foo.c"), 5);
/// assert_eq!(base_len("foo/bar.c"), 5);
/// assert_eq!(base_len("/foo/bar.c"), 5);
/// assert_eq!(base_len("foo/bar/"), 4);
/// assert_eq!(base_len("foo/bar//"), 4);
/// assert_eq!(base_len("/"), 0);
/// assert_eq!(base_len("//"), 0);
/// assert_eq!(base_len(""), 0);
/// ```
pub fn base_len(filename: &str) -> usize {
    let mut len = filename.len();
    
    // Remove trailing slashes
    while len > 1 && (filename.as_bytes()[len - 1] == b'/' || filename.as_bytes()[len - 1] == b'\\') {
        len -= 1;
    }

    // Handle special cases for root paths
    if len == 1 && (filename.as_bytes()[0] == b'/' || filename.as_bytes()[0] == b'\\') {
        return 0;
    }

    // Handle double slash root case
    if len == 2 
        && (filename.as_bytes()[0] == b'/' || filename.as_bytes()[0] == b'\\')
        && (filename.as_bytes()[1] == b'/' || filename.as_bytes()[1] == b'\\')
        && filename.as_bytes().get(2).is_none()
    {
        return 0;
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
        assert_eq!(base_len("foo.c"), 5);
        assert_eq!(base_len("foo/bar.c"), 5);
        assert_eq!(base_len("/foo/bar.c"), 5);
        assert_eq!(base_len("foo/bar/"), 4);
        assert_eq!(base_len("foo/bar//"), 4);
        assert_eq!(base_len("/"), 0);
        assert_eq!(base_len("//"), 0);
        assert_eq!(base_len(""), 0);
    }
}