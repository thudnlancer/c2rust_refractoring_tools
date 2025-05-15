use std::path::{Path, Component};
use std::ffi::OsStr;
use std::borrow::Cow;

/// Return the length of the prefix of `file` that will be used by `dir_name`.
/// If `file` is in the working directory, this returns zero even though
/// `dir_name(file)` will return ".". Works properly even if there are trailing
/// slashes (by effectively ignoring them).
fn dir_len(file: &str) -> usize {
    let path = Path::new(file);
    let mut prefix_len = 0;
    
    // Handle prefix (like "C:" or "/")
    if let Some(prefix) = path.prefix() {
        prefix_len = prefix.as_os_str().len();
    }
    
    // Handle leading slashes after prefix
    let mut components = path.components();
    if prefix_len == 0 {
        if let Some(Component::RootDir) = components.next() {
            prefix_len = 1;
            // Check for double slash (distinct root case)
            if let Some(Component::RootDir) = components.next() {
                if components.next().is_none() {
                    prefix_len = 2;
                }
            }
        }
    }
    
    // Find the position of the last component
    let last_component_pos = path.components()
        .map(|c| c.as_os_str().len())
        .sum::<usize>();
    
    // Strip the basename and any redundant slashes before it
    let mut length = last_component_pos;
    while length > prefix_len {
        if !file[..length].ends_with('/') {
            break;
        }
        length -= 1;
    }
    
    length
}

/// Return the leading directories part of `file`, allocated in a new String.
/// Works properly even if there are trailing slashes (by effectively ignoring them).
/// Returns None on allocation failure.
/// 
/// If lstat(file) would succeed, then { chdir(dir_name(file)); lstat(base_name(file)); }
/// will access the same file. Likewise, if the sequence { chdir(dir_name(file));
/// rename(base_name(file), "foo"); } succeeds, you have renamed file to "foo" in
/// the same directory file was in.
fn mdir_name(file: &str) -> Option<String> {
    let length = dir_len(file);
    let path = Path::new(file);
    
    let append_dot = length == 0 || (path.prefix().is_some() 
        && length == path.prefix().unwrap().as_os_str().len()
        && file.len() > 2 
        && !file[2..].starts_with('/'));
    
    let mut result = String::with_capacity(length + append_dot as usize + 1);
    result.push_str(&file[..length]);
    
    if append_dot {
        result.push('.');
    }
    
    Some(result)
}