//! Extract the last component (base name) of a file name.
//!
//! This is a Rust translation of the GNU LGPL basename implementation.
//! It follows the same behavior as the original C code while adhering to Rust's safety guarantees.

/// Returns the last component of a file path.
///
/// This behaves similarly to the C `last_component` function:
/// - Trailing slashes are considered part of the last component
/// - For root paths, returns an empty string
/// - The result is a slice of the input string
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
    let mut bytes = filename.as_bytes();
    let mut base = bytes;
    
    // Skip filesystem prefix
    let prefix_len = file_system_prefix_len(filename);
    base = &base[prefix_len..];
    
    // Skip leading slashes
    while !base.is_empty() && is_slash(base[0]) {
        base = &base[1..];
    }
    
    let mut last_was_slash = false;
    let mut last_base = base;
    
    for (i, &c) in base.iter().enumerate() {
        if is_slash(c) {
            last_was_slash = true;
        } else if last_was_slash {
            last_base = &base[i..];
            last_was_slash = false;
        }
    }
    
    std::str::from_utf8(last_base).unwrap_or("")
}

/// Returns the length of the basename, omitting trailing slashes.
///
/// # Examples
/// ```
/// assert_eq!(base_len("foo.c"), 5);
/// assert_eq!(base_len("foo/bar/"), 3);
/// assert_eq!(base_len("/"), 0);
/// assert_eq!(base_len("//"), 0);
/// ```
pub fn base_len(name: &str) -> usize {
    let bytes = name.as_bytes();
    let mut len = bytes.len();
    let prefix_len = file_system_prefix_len(name);
    
    // Remove trailing slashes
    while len > 1 && is_slash(bytes[len - 1]) {
        len -= 1;
    }
    
    // Handle distinct double slash root case
    if DOUBLE_SLASH_IS_DISTINCT_ROOT && len == 1 
        && is_slash(bytes[0]) && is_slash(bytes[1]) && bytes.get(2).is_none() {
        return 2;
    }
    
    // Handle filesystem drive prefix case
    if FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE && prefix_len > 0
        && len == prefix_len && is_slash(bytes[prefix_len]) {
        return prefix_len + 1;
    }
    
    len
}

// Helper functions

#[cfg(windows)]
const DOUBLE_SLASH_IS_DISTINCT_ROOT: bool = true;
#[cfg(not(windows))]
const DOUBLE_SLASH_IS_DISTINCT_ROOT: bool = false;

#[cfg(windows)]
const FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE: bool = true;
#[cfg(not(windows))]
const FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE: bool = false;

fn is_slash(c: u8) -> bool {
    c == b'/' || (cfg!(windows) && c == b'\\')
}

fn file_system_prefix_len(filename: &str) -> usize {
    if cfg!(windows) {
        // On Windows, handle drive letters and UNC paths
        let bytes = filename.as_bytes();
        if bytes.len() >= 2 && bytes[1] == b':' {
            // Drive letter path (e.g., "C:")
            return 2;
        } else if bytes.len() >= 2 && is_slash(bytes[0]) && is_slash(bytes[1]) {
            // UNC path (e.g., "\\server\share")
            if let Some(pos) = bytes[2..].iter().position(|&c| !is_slash(c)) {
                let server_start = 2 + pos;
                if let Some(pos) = bytes[server_start..].iter().position(|&c| is_slash(c)) {
                    let share_start = server_start + pos;
                    if let Some(pos) = bytes[share_start..].iter().position(|&c| !is_slash(c)) {
                        return share_start + pos;
                    }
                }
            }
        }
    }
    0
}