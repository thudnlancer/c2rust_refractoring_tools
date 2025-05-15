// stripslash.rs -- remove redundant trailing slashes from a file name

// Original Copyright (C) 1990, 2001, 2003-2006, 2009-2023 Free Software Foundation,
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
    let path = std::path::Path::new(file);
    let components: Vec<_> = path.components().collect();
    
    // Handle root paths specially to turn "///" into "/"
    let base = if components.is_empty() {
        path
    } else {
        components.last().unwrap().as_os_str()
    };
    
    let base_str = base.to_string_lossy();
    let base_len = base_str.len();
    let file_len = file.len();
    
    // Calculate the position where the base ends in the original string
    let base_end = file_len - (file.trim_end_matches('/').len() - base_len);
    
    let had_slash = file_len > base_end;
    file.truncate(base_end);
    had_slash
}