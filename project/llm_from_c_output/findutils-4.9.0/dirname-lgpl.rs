use std::path::{Component, Path, PathBuf};
use std::ffi::OsStr;

/// Return the length of the prefix of FILE that will be used by dir_name.
/// If FILE is in the working directory, this returns zero even though
/// 'dir_name(FILE)' will return ".". Works properly even if there are
/// trailing slashes (by effectively ignoring them).
fn dir_len(file: &Path) -> usize {
    let mut components = file.components();
    let mut prefix_length = 0;

    // Handle prefix components (like drive letters or UNC paths)
    if let Some(Component::Prefix(prefix)) = components.next() {
        prefix_length = prefix.as_os_str().len();
    }

    // Handle root components
    if let Some(Component::RootDir) = components.next() {
        prefix_length += 1;
        // Handle double slash for distinct root if needed
        if prefix_length == 1 && file.as_os_str().len() > 1 {
            let bytes = file.as_os_str().as_bytes();
            if bytes.len() > 1 && bytes[1] == b'/' {
                prefix_length += 1;
            }
        }
    }

    // Find the last component and calculate length
    let mut last_component_start = prefix_length;
    for component in components {
        last_component_start += component.as_os_str().len();
    }

    // Strip trailing slashes
    let mut length = last_component_start;
    while length > prefix_length {
        if !file.as_os_str().as_bytes()[length - 1] == b'/' {
            break;
        }
        length -= 1;
    }

    length
}

/// Return the leading directories part of FILE, allocated in a PathBuf.
/// Works properly even if there are trailing slashes (by effectively
/// ignoring them). Returns None on allocation failure.
///
/// If lstat(FILE) would succeed, then { chdir(dir_name(FILE));
/// lstat(base_name(FILE)); } will access the same file. Likewise,
/// if the sequence { chdir(dir_name(FILE));
/// rename(base_name(FILE), "foo"); } succeeds, you have renamed FILE
/// to "foo" in the same directory FILE was in.
fn mdir_name(file: &Path) -> Option<PathBuf> {
    let length = dir_len(file);
    let append_dot = length == 0 || (length == file.components().next()
        .and_then(|c| match c {
            Component::Prefix(p) => Some(p.as_os_str().len()),
            _ => None,
        })
        .unwrap_or(0) && file.as_os_str().len() > 2 
        && file.as_os_str().as_bytes()[2] != b'/');

    let mut dir = PathBuf::new();
    if let Some(parent) = file.parent() {
        dir.push(parent);
    }
    if append_dot {
        dir.push(".");
    }
    if dir.as_os_str().is_empty() {
        dir.push(".");
    }
    Some(dir)
}