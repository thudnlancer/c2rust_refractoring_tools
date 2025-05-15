// Duplicate a bounded initial segment of a string, with out-of-memory
// checking.
// Copyright (C) 2003, 2009-2023 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::CString;
use std::ptr;
use std::os::raw::c_char;

/// Return a newly allocated copy of at most N bytes of STRING.
/// In other words, return a copy of the initial segment of length N of
/// STRING.
pub fn xstrndup(string: &str, n: usize) -> CString {
    let truncated = if string.len() > n {
        &string[..n]
    } else {
        string
    };
    
    CString::new(truncated).expect("Failed to create CString")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_xstrndup() {
        let s = "hello world";
        let dup = xstrndup(s, 5);
        assert_eq!(dup.to_str().unwrap(), "hello");
        
        let dup_full = xstrndup(s, 20);
        assert_eq!(dup_full.to_str().unwrap(), "hello world");
    }
    
    #[test]
    fn test_empty_string() {
        let s = "";
        let dup = xstrndup(s, 5);
        assert_eq!(dup.to_str().unwrap(), "");
    }
}