/*
 * Copyright 1991 Free Software Foundation, Inc.
 * Copyright 1997,1999-2002,2007-2009,2022 Alain Knaff.
 * This file is part of mtools.
 *
 * Mtools is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Mtools is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::process;
use std::io::{self, Write};
use std::path::Path;
use std::str;

pub fn strdup(s: &str) -> Option<String> {
    Some(s.to_string())
}

pub fn strndup(s: &str, n: usize) -> Option<String> {
    let end = s.char_indices().take(n).last().map(|(i, _)| i).unwrap_or(0);
    Some(s[..end].to_string())
}

#[cfg(feature = "wchar")]
pub fn wcsdup(s: &wstr) -> Option<Box<wstr>> {
    Some(s.to_owned().into_boxed_wstr())
}

pub fn memcpy(dest: &mut [u8], src: &[u8]) -> &mut [u8] {
    dest.copy_from_slice(src);
    dest
}

pub fn memset(s: &mut [u8], c: u8) -> &mut [u8] {
    for byte in s.iter_mut() {
        *byte = c;
    }
    s
}

pub fn strchr(s: &str, c: char) -> Option<&str> {
    s.find(c).map(|i| &s[i..])
}

pub fn strrchr(s: &str, c: char) -> Option<&str> {
    s.rfind(c).map(|i| &s[i..])
}

pub fn strstr(haystack: &str, needle: &str) -> Option<&str> {
    haystack.find(needle).map(|i| &haystack[i..])
}

pub fn mkdir(path: &str, _mode: u32) -> io::Result<()> {
    std::fs::create_dir(path)
}

pub fn strpbrk(s: &str, accept: &str) -> Option<&str> {
    s.find(|c| accept.contains(c)).map(|i| &s[i..])
}

pub fn strtoul(s: &str, radix: u32) -> Result<u64, &'static str> {
    u64::from_str_radix(s, radix).map_err(|_| "invalid number")
}

pub fn strtol(s: &str, radix: u32) -> Result<i64, &'static str> {
    i64::from_str_radix(s, radix).map_err(|_| "invalid number")
}

pub fn strspn(s: &str, accept: &str) -> usize {
    s.chars().take_while(|c| accept.contains(*c)).count()
}

pub fn strcspn(s: &str, reject: &str) -> usize {
    s.chars().take_while(|c| !reject.contains(*c)).count()
}

pub fn strerror(errno: i32) -> &'static str {
    std::error::Error::description(&std::io::Error::from_raw_os_error(errno))
}

pub fn strcasecmp(s1: &str, s2: &str) -> i32 {
    let s1_lower = s1.to_lowercase();
    let s2_lower = s2.to_lowercase();
    s1_lower.cmp(&s2_lower) as i32
}

#[cfg(feature = "wchar")]
pub fn wcscasecmp(s1: &wstr, s2: &wstr) -> i32 {
    let s1_lower: Vec<_> = s1.chars().map(|c| c.to_lowercase()).collect();
    let s2_lower: Vec<_> = s2.chars().map(|c| c.to_lowercase()).collect();
    s1_lower.cmp(&s2_lower) as i32
}

pub fn strncasecmp(s1: &str, s2: &str, n: usize) -> i32 {
    let s1_lower = s1[..n.min(s1.len())].to_lowercase();
    let s2_lower = s2[..n.min(s2.len())].to_lowercase();
    s1_lower.cmp(&s2_lower) as i32
}

pub fn getpass(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn atexit<F: FnOnce() + 'static>(callback: F) -> Result<(), Box<dyn std::any::Any + Send>> {
    std::panic::catch_unwind(|| {
        let _ = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        callback();
    })
}

pub fn mt_basename(path: &str) -> &str {
    Path::new(path).file_name().unwrap().to_str().unwrap()
}

pub fn mt_stripexe(filename: &mut String) {
    if filename.to_lowercase().ends_with(".exe") {
        filename.truncate(filename.len() - 4);
    }
}

pub fn strnlen(s: &str, max_len: usize) -> usize {
    s.char_indices().take(max_len).count()
}

#[cfg(feature = "wchar")]
pub fn wcsnlen(s: &wstr, max_len: usize) -> usize {
    s.chars().take(max_len).count()
}