use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// Return the length of the prefix of FILE that will be used by dir_name.
/// If FILE is in the working directory, this returns zero even though
/// 'dir_name (FILE)' will return ".". Works properly even if there are
/// trailing slashes (by effectively ignoring them).
fn dir_len(file: &Path) -> usize {
    let file_str = file.as_os_str().as_bytes();
    let mut prefix_length = file_str
        .iter()
        .position(|&c| c != b'/')
        .unwrap_or(file_str.len());

    // Advance prefix_length beyond important leading slashes
    if prefix_length > 0 {
        if prefix_length == 1 {
            // Single leading slash
            prefix_length = 1;
        } else if prefix_length >= 2 && file_str[1] == b'/' && (prefix_length < 3 || file_str[2] != b'/') {
            // Double slash distinct root case
            prefix_length = 2;
        }
    }

    // Find the last component
    let last_comp_pos = file_str
        .iter()
        .rposition(|&c| c != b'/')
        .map_or(0, |pos| {
            file_str[..=pos]
                .iter()
                .rposition(|&c| c == b'/')
                .map_or(0, |slash_pos| slash_pos + 1)
        });

    // Strip the basename and any redundant slashes before it
    let mut length = last_comp_pos;
    while prefix_length < length {
        if file_str[length - 1] != b'/' {
            break;
        }
        length -= 1;
    }

    length
}

/// Return the leading directories part of FILE, allocated with malloc.
/// Works properly even if there are trailing slashes (by effectively
/// ignoring them). Return None on failure.
pub fn mdir_name(file: &Path) -> Option<PathBuf> {
    let length = dir_len(file);
    let file_str = file.as_os_str().as_bytes();
    
    let append_dot = length == 0 || (length == 2 && file_str.len() > 2 && file_str[2] != b'/');
    
    let mut result = if length > 0 {
        PathBuf::from(OsStrExt::from_bytes(&file_str[..length]))
    } else {
        PathBuf::new()
    };
    
    if append_dot {
        result.push(".");
    }
    
    Some(result)
}