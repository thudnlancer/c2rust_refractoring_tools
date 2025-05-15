// stripslash.rs -- remove redundant trailing slashes from a file name

// Original Copyright:
// Copyright (C) 1990, 2001, 2003-2006, 2009-2023 Free Software Foundation,
// Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// Remove trailing slashes from FILE. Return true if a trailing slash
// was removed. This is useful when using file name completion from a
// shell that adds a "/" after directory names (such as tcsh and
// bash), because on symlinks to directories, several system calls
// have different semantics according to whether a trailing slash is
// present.

use std::path::{Path, Component};
use std::ffi::OsStr;

/// Returns the length of the base component of the path
fn base_len(path: &OsStr) -> usize {
    Path::new(path)
        .components()
        .last()
        .map(|c| c.as_os_str().len())
        .unwrap_or(0)
}

/// Remove trailing slashes from a file path. Returns true if any were removed.
pub fn strip_trailing_slashes(file: &mut String) -> bool {
    let path = Path::new(file);
    let mut components = path.components();
    
    // Handle root paths specially
    let base = if components.clone().count() == 1 && path.has_root() {
        path
    } else {
        match components.next_back() {
            Some(Component::Normal(p)) => Path::new(p),
            _ => path,
        }
    };
    
    let base_len = base_len(base.as_os_str());
    let base_end = file.len() - (path.as_os_str().len() - base.as_os_str().len()) + base_len;
    
    let had_slash = base_end < file.len();
    file.truncate(base_end);
    had_slash
}