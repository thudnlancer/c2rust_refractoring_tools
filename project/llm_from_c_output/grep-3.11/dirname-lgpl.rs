use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, Component};
use std::ffi::CString;

/// Return the length of the prefix of FILE that will be used by dir_name.
/// If FILE is in the working directory, this returns zero even though
/// 'dir_name (FILE)' will return ".". Works properly even if there are
/// trailing slashes (by effectively ignoring them).
fn dir_len(file: &Path) -> usize {
    let mut components = file.components();
    let mut prefix_length = 0;
    
    // Handle prefix components (like drive letters or UNC paths on Windows)
    if let Some(Component::Prefix(prefix)) = components.next() {
        prefix_length = prefix.as_os_str().len();
    }
    
    // Handle root components
    if let Some(Component::RootDir) = components.next() {
        prefix_length += 1;
    }
    
    // Find the position of the last component
    let mut last_component_pos = 0;
    for (i, component) in file.components().enumerate() {
        match component {
            Component::Normal(_) => last_component_pos = i,
            _ => (),
        }
    }
    
    // Calculate the length up to the last component
    let mut length = 0;
    for (i, component) in file.components().enumerate() {
        if i >= last_component_pos {
            break;
        }
        length += component.as_os_str().len();
    }
    
    // Return the maximum between prefix length and component length
    std::cmp::max(prefix_length, length)
}

/// Return the leading directories part of FILE, allocated on heap.
/// Works properly even if there are trailing slashes (by effectively ignoring them).
/// Returns None on failure.
fn mdir_name(file: &str) -> Option<CString> {
    let path = Path::new(file);
    let length = dir_len(path);
    
    let append_dot = length == 0 || 
        (cfg!(windows) && 
         length == file.len() && 
         file.chars().nth(2).map_or(false, |c| c != '\\' && c != '/'));
    
    let mut result = if length > 0 {
        file[..length].to_string()
    } else {
        String::new()
    };
    
    if append_dot {
        result.push('.');
    }
    
    CString::new(result).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_dir_len() {
        assert_eq!(dir_len(Path::new("foo/bar")), 4);
        assert_eq!(dir_len(Path::new("foo/")), 4);
        assert_eq!(dir_len(Path::new("foo")), 0);
        assert_eq!(dir_len(Path::new("/foo")), 1);
        assert_eq!(dir_len(Path::new("/foo/bar")), 5);
    }
    
    #[test]
    fn test_mdir_name() {
        assert_eq!(mdir_name("foo/bar").unwrap().to_str().unwrap(), "foo");
        assert_eq!(mdir_name("foo/").unwrap().to_str().unwrap(), "foo");
        assert_eq!(mdir_name("foo").unwrap().to_str().unwrap(), ".");
        assert_eq!(mdir_name("/foo").unwrap().to_str().unwrap(), "/");
        assert_eq!(mdir_name("/foo/bar").unwrap().to_str().unwrap(), "/foo");
    }
}