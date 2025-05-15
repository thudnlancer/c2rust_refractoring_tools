use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::ffi::CString;

/// Return the length of the prefix of FILE that will be used by dir_name.
/// If FILE is in the working directory, this returns zero even though
/// 'dir_name (FILE)' will return ".". Works properly even if there are
/// trailing slashes (by effectively ignoring them).
fn dir_len(file: &CStr) -> usize {
    let file_str = file.to_str().unwrap_or("");
    let path = Path::new(file_str);
    
    let mut prefix_length = if cfg!(windows) {
        // On Windows, check for drive prefix (e.g., "C:\")
        if file_str.len() >= 3 && file_str.chars().nth(1) == Some(':') {
            2
        } else {
            0
        }
    } else {
        0
    };

    // Advance prefix_length beyond important leading slashes
    if prefix_length != 0 {
        if cfg!(windows) && file_str.chars().nth(prefix_length).map_or(false, |c| c == '/') {
            prefix_length += 1;
        }
    } else if file_str.starts_with('/') {
        prefix_length = if cfg!(unix) && file_str.starts_with("//") && !file_str.starts_with("///") {
            2
        } else {
            1
        };
    }

    // Strip the basename and any redundant slashes before it
    let components: Vec<_> = path.components().collect();
    let last_component_pos = if components.is_empty() {
        0
    } else {
        let last = components.last().unwrap();
        file_str.len() - last.as_os_str().to_str().unwrap_or("").len()
    };

    let mut length = last_component_pos;
    while prefix_length < length {
        if !file_str[..length].ends_with('/') {
            break;
        }
        length -= 1;
    }

    length
}

/// Return the leading directories part of FILE, allocated with malloc.
/// Works properly even if there are trailing slashes (by effectively
/// ignoring them). Return None on failure.
fn mdir_name(file: &CStr) -> Option<CString> {
    let length = dir_len(file);
    let file_str = file.to_str().unwrap_or("");
    
    let append_dot = length == 0 || (cfg!(windows) 
        && length == 2 
        && file_str.len() > 2 
        && !file_str.chars().nth(2).map_or(false, |c| c == '/'));

    let mut result = if append_dot {
        let mut s = file_str[..length].to_string();
        s.push('.');
        s
    } else {
        file_str[..length].to_string()
    };

    // Handle special case when result is empty - return "."
    if result.is_empty() {
        result = ".".to_string();
    }

    CString::new(result).ok()
}