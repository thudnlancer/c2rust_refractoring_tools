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
    
    // Handle root directory components
    if let Some(Component::RootDir) = components.next() {
        prefix_length += 1;
    }
    
    // Find the position of the last component
    let mut last_pos = prefix_length;
    for component in components {
        match component {
            Component::Normal(_) => {
                last_pos = component.as_os_str().as_bytes().len() + 
                    file.as_os_str().as_bytes()[..last_pos].iter()
                        .rposition(|&c| c == b'/')
                        .unwrap_or(0) + 1;
            },
            _ => continue,
        }
    }
    
    // Strip trailing slashes
    while last_pos > prefix_length {
        if file.as_os_str().as_bytes()[last_pos - 1] != b'/' {
            break;
        }
        last_pos -= 1;
    }
    
    last_pos
}

/// Return the leading directories part of FILE, allocated in a new String.
/// Works properly even if there are trailing slashes (by effectively ignoring them).
/// Returns None on failure (allocation error).
/// 
/// If lstat(FILE) would succeed, then { chdir(dir_name(FILE)); lstat(base_name(FILE)); }
/// will access the same file. Likewise, if the sequence { chdir(dir_name(FILE));
/// rename(base_name(FILE), "foo"); } succeeds, you have renamed FILE to "foo" in
/// the same directory FILE was in.
fn mdir_name(file: &str) -> Option<String> {
    let path = Path::new(file);
    let length = dir_len(path);
    
    let append_dot = length == 0 || (length == file.len() && !file.ends_with('/'));
    let mut result = String::with_capacity(length + if append_dot { 1 } else { 0 });
    
    result.push_str(&file[..length]);
    if append_dot {
        result.push('.');
    }
    
    Some(result)
}