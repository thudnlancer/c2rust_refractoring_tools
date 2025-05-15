use std::path::{Path, Component};
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;

/// Return the length of the prefix of FILE that will be used by dir_name.
/// If FILE is in the working directory, this returns zero even though
/// 'dir_name(FILE)' will return ".". Works properly even if there are
/// trailing slashes (by effectively ignoring them).
fn dir_len(file: &Path) -> usize {
    let mut components = file.components();
    let mut prefix_length = 0;
    
    // Handle prefix components (like drive letters or UNC paths on Windows)
    if let Some(Component::Prefix(prefix)) = components.next() {
        prefix_length = prefix.as_os_str().as_bytes().len();
    }
    
    // Handle root components
    if let Some(Component::RootDir) = components.next() {
        prefix_length += 1;
    }
    
    // Find the position of the last component
    let last_component_pos = file.components().last()
        .map(|c| {
            let s = file.as_os_str().as_bytes();
            let c_str = c.as_os_str().as_bytes();
            let c_len = c_str.len();
            let c_pos = s.len() - c_len;
            c_pos
        })
        .unwrap_or(0);
    
    // Strip the basename and any redundant slashes before it
    let mut length = last_component_pos;
    while prefix_length < length {
        length -= 1;
        if !file.as_os_str().as_bytes()[length].is_ascii_whitespace() {
            break;
        }
    }
    
    length
}

/// Return the leading directories part of FILE, allocated with malloc.
/// Works properly even if there are trailing slashes (by effectively
/// ignoring them). Return None on failure.
fn mdir_name(file: &Path) -> Option<Box<Path>> {
    let length = dir_len(file);
    let append_dot = length == 0 || 
        (file.as_os_str().as_bytes().len() > 2 && 
         !file.as_os_str().as_bytes()[2].is_ascii_whitespace());
    
    let mut dir = if append_dot {
        let mut v = Vec::with_capacity(length + 2);
        v.extend_from_slice(&file.as_os_str().as_bytes()[..length]);
        v.push(b'.');
        v
    } else {
        file.as_os_str().as_bytes()[..length].to_vec()
    };
    
    dir.push(b'\0');
    
    let c_str = unsafe { CStr::from_bytes_with_nul_unchecked(&dir) };
    Some(Box::from(Path::new(c_str)))
}