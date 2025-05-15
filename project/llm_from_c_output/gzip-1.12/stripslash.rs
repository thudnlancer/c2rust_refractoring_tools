// stripslash.rs -- remove redundant trailing slashes from a file name

// Original Copyright (C) 1990, 2001, 2003-2006, 2009-2022 Free Software Foundation,
// Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

/// Remove trailing slashes from FILE. Returns true if a trailing slash
/// was removed. This is useful when using file name completion from a
/// shell that adds a "/" after directory names (such as tcsh and
/// bash), because on symlinks to directories, several system calls
/// have different semantics according to whether a trailing slash is
/// present.
pub fn strip_trailing_slashes(file: &mut String) -> bool {
    let base = last_component(file);
    let base_len = base.len();
    
    // last_component returns "" for file system roots, but we need to turn
    // "///" into "/".
    let (base_start, base_end) = if base.is_empty() {
        (0, file.len())
    } else {
        let base_start = base.as_ptr() as usize - file.as_ptr() as usize;
        (base_start, base_start + base_len)
    };

    let had_slash = file.len() > base_end;
    file.truncate(base_end);
    had_slash
}

/// Helper function to find the last component of a path
fn last_component(path: &str) -> &str {
    let mut end = path.len();
    
    // Skip trailing slashes
    while end > 0 && path.as_bytes()[end - 1] == b'/' {
        end -= 1;
    }
    
    if end == 0 {
        return "";
    }
    
    // Find the start of the last component
    let start = path[..end].rfind('/').map(|i| i + 1).unwrap_or(0);
    &path[start..end]
}

/// Helper function to get the length of the base component
fn base_len(base: &str) -> usize {
    base.len()
}