// stripslash.rs -- remove redundant trailing slashes from a file name

// Original Copyright (C) 1990, 2001, 2003-2006, 2009-2023 Free Software Foundation,
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

use std::path::{Component, Path, PathBuf};

pub fn strip_trailing_slashes(file: &mut PathBuf) -> bool {
    let mut had_slash = false;
    
    // Normalize the path first to handle redundant slashes
    let normalized = file.clone().into_os_string().into_string().unwrap();
    let mut components = Path::new(&normalized).components().peekable();
    
    // Handle root cases
    let is_root = match components.peek() {
        Some(Component::RootDir) => {
            components.next();
            true
        }
        _ => false,
    };
    
    let mut new_path = PathBuf::new();
    if is_root {
        new_path.push("/");
    }
    
    let mut last_component = None;
    for component in components {
        last_component = Some(component);
        new_path.push(component);
    }
    
    // Check if the last component ends with a slash
    if let Some(comp) = last_component {
        if let Component::Normal(s) = comp {
            let s = s.to_str().unwrap();
            had_slash = s.ends_with('/');
        }
    }
    
    // Remove trailing slashes
    let mut s = new_path.into_os_string().into_string().unwrap();
    while s.ends_with('/') && s.len() > 1 {
        s.pop();
        had_slash = true;
    }
    
    *file = PathBuf::from(s);
    had_slash
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_strip_trailing_slashes() {
        let mut p = PathBuf::from("/some/path///");
        assert!(strip_trailing_slashes(&mut p));
        assert_eq!(p, PathBuf::from("/some/path"));
        
        let mut p = PathBuf::from("/some/path");
        assert!(!strip_trailing_slashes(&mut p));
        assert_eq!(p, PathBuf::from("/some/path"));
        
        let mut p = PathBuf::from("///");
        assert!(strip_trailing_slashes(&mut p));
        assert_eq!(p, PathBuf::from("/"));
        
        let mut p = PathBuf::from("/");
        assert!(!strip_trailing_slashes(&mut p));
        assert_eq!(p, PathBuf::from("/"));
    }
}