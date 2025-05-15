// stripslash.rs -- remove redundant trailing slashes from a file name

// Original Copyright (C) 1990, 2001, 2003-2006, 2009-2022 Free Software Foundation,
// Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

/// Remove trailing slashes from `file`. Returns `true` if a trailing slash
/// was removed. This is useful when using file name completion from a
/// shell that adds a "/" after directory names (such as tcsh and
/// bash), because on symlinks to directories, several system calls
/// have different semantics according to whether a trailing slash is
/// present.
pub fn strip_trailing_slashes(file: &mut String) -> bool {
    // Find the last component of the path
    let base = last_component(file);
    
    // Handle file system roots (like "///" which should become "/")
    let base = if base.is_empty() {
        file.as_str()
    } else {
        base
    };
    
    let base_len = base_len(base);
    let base_end = base_len + (file.len() - base.len());
    let had_slash = file.len() > base_end;
    
    // Truncate the string at the end of the base component
    file.truncate(base_end);
    
    had_slash
}

/// Returns the last component of the path
fn last_component(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or("")
}

/// Returns the length of the base component (without trailing slashes)
fn base_len(base: &str) -> usize {
    base.trim_end_matches('/').len()
}