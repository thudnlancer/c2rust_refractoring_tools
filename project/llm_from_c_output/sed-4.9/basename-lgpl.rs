//! Extract the last component (base name) of a file name.
//!
//! This implementation follows the same behavior as the original C code,
//! but uses Rust's safety features and standard libraries.

/// Returns the last component of a file path.
/// 
/// This function behaves like the C `last_component` function:
/// - Trailing slashes are considered part of the last component
/// - Returns empty string for filesystem roots
/// - The result is a substring of the input (no allocation)
pub fn last_component(filename: &str) -> &str {
    let bytes = filename.as_bytes();
    let mut base = skip_prefix_and_leading_slashes(bytes);
    let mut last_was_slash = false;
    let mut last_component_start = base;

    for (i, &c) in bytes[base..].iter().enumerate() {
        if is_slash(c) {
            last_was_slash = true;
        } else if last_was_slash {
            last_component_start = base + i;
            last_was_slash = false;
        }
    }

    &filename[last_component_start..]
}

/// Returns the length of the base name, omitting trailing slashes.
/// 
/// This behaves like the C `base_len` function:
/// - Removes all trailing slashes
/// - Special handling for root paths and drive prefixes
pub fn base_len(filename: &str) -> usize {
    let bytes = filename.as_bytes();
    let mut len = bytes.len();
    
    // Strip trailing slashes
    while len > 1 && is_slash(bytes[len - 1]) {
        len -= 1;
    }

    // Handle special cases
    if DOUBLE_SLASH_IS_DISTINCT_ROOT && len == 1 
        && bytes.len() >= 2 && is_slash(bytes[0]) && is_slash(bytes[1]) 
        && (bytes.len() == 2 || bytes[2] == 0) {
        return 2;
    }

    let prefix_len = file_system_prefix_len(filename);
    if FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE && prefix_len > 0 
        && len == prefix_len && bytes.len() > prefix_len 
        && is_slash(bytes[prefix_len]) {
        return prefix_len + 1;
    }

    len
}

// Helper functions

#[inline]
fn is_slash(c: u8) -> bool {
    c == b'/' || c == b'\\'
}

fn skip_prefix_and_leading_slashes(bytes: &[u8]) -> usize {
    let mut start = file_system_prefix_len_bytes(bytes);
    while start < bytes.len() && is_slash(bytes[start]) {
        start += 1;
    }
    start
}

// These would be implemented based on the platform-specific behavior
const DOUBLE_SLASH_IS_DISTINCT_ROOT: bool = false;
const FILE_SYSTEM_DRIVE_PREFIX_CAN_BE_RELATIVE: bool = false;

fn file_system_prefix_len(_path: &str) -> usize {
    // Platform-specific implementation would go here
    0
}

fn file_system_prefix_len_bytes(_bytes: &[u8]) -> usize {
    // Platform-specific implementation would go here
    0
}