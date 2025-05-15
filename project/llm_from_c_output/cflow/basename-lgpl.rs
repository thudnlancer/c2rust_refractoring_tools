//! Return the last element in a file name
//!
//! This code is translated from C to Rust, maintaining the same functionality
//! while adhering to Rust's safety practices.

use std::path::{Component, Path, Prefix};
use std::ffi::OsStr;

/// Return the last component of a path as an OsStr reference.
/// If the path has no relative components (i.e., it's a filesystem root),
/// returns an empty OsStr.
pub fn last_component(path: &OsStr) -> &OsStr {
    let path = Path::new(path);
    let mut last_component = OsStr::new("");
    
    for component in path.components() {
        match component {
            Component::Normal(name) => {
                last_component = name;
            }
            Component::RootDir | Component::Prefix(_) => {
                last_component = OsStr::new("");
            }
            _ => {}
        }
    }
    
    last_component
}

/// Return the length of the basename, omitting all trailing slashes.
pub fn base_len(path: &OsStr) -> usize {
    let path = Path::new(path);
    let mut len = 0;
    
    if let Some(name) = path.file_name() {
        len = name.len();
    } else {
        // Handle root and prefix cases
        for component in path.components() {
            match component {
                Component::RootDir => len = 1,
                Component::Prefix(prefix) => {
                    len = match prefix.kind() {
                        Prefix::Verbatim(s) => s.len(),
                        Prefix::VerbatimUNC(server, share) => server.len() + share.len() + 2,
                        Prefix::VerbatimDisk(d) => 1 + d.len_utf8(),
                        Prefix::DeviceNS(device) => device.len(),
                        Prefix::UNC(server, share) => server.len() + share.len() + 2,
                        Prefix::Disk(d) => 1 + d.len_utf8(),
                    };
                }
                _ => {}
            }
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
        assert_eq!(last_component(OsStr::new("")), OsStr::new(""));
        assert_eq!(last_component(OsStr::new("/")), OsStr::new(""));
        assert_eq!(last_component(OsStr::new("///")), OsStr::new(""));
        assert_eq!(last_component(OsStr::new("/foo/")), OsStr::new("foo"));
        assert_eq!(last_component(OsStr::new("/foo/bar")), OsStr::new("bar"));
        assert_eq!(last_component(OsStr::new("foo/bar")), OsStr::new("bar"));
        assert_eq!(last_component(OsStr::new("foo")), OsStr::new("foo"));
    }

    #[test]
    fn test_base_len() {
        assert_eq!(base_len(OsStr::new("")), 0);
        assert_eq!(base_len(OsStr::new("/")), 1);
        assert_eq!(base_len(OsStr::new("///")), 1);
        assert_eq!(base_len(OsStr::new("/foo/")), 3);
        assert_eq!(base_len(OsStr::new("/foo/bar")), 3);
        assert_eq!(base_len(OsStr::new("foo/bar")), 3);
        assert_eq!(base_len(OsStr::new("foo")), 3);
    }
}