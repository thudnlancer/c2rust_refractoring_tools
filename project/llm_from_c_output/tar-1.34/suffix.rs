// This file is part of GNU tar.
// Copyright 2007-2021 Free Software Foundation, Inc.
//
// Written by Sergey Poznyakoff.
//
// GNU tar is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option) any later
// version.
//
// GNU tar is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General
// Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with GNU tar.  If not, see <http://www.gnu.org/licenses/>.

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::str;

#[derive(Debug)]
struct CompressionSuffix {
    suffix: &'static str,
    program: Option<&'static str>,
}

impl CompressionSuffix {
    fn new(suffix: &'static str, program: Option<&'static str>) -> Self {
        CompressionSuffix { suffix, program }
    }
}

static COMPRESSION_SUFFIXES: &[CompressionSuffix] = &[
    CompressionSuffix::new("tar", None),
    CompressionSuffix::new("gz", Some("GZIP_PROGRAM")),
    CompressionSuffix::new("tgz", Some("GZIP_PROGRAM")),
    CompressionSuffix::new("taz", Some("GZIP_PROGRAM")),
    CompressionSuffix::new("Z", Some("COMPRESS_PROGRAM")),
    CompressionSuffix::new("taZ", Some("COMPRESS_PROGRAM")),
    CompressionSuffix::new("bz2", Some("BZIP2_PROGRAM")),
    CompressionSuffix::new("tbz", Some("BZIP2_PROGRAM")),
    CompressionSuffix::new("tbz2", Some("BZIP2_PROGRAM")),
    CompressionSuffix::new("tz2", Some("BZIP2_PROGRAM")),
    CompressionSuffix::new("lz", Some("LZIP_PROGRAM")),
    CompressionSuffix::new("lzma", Some("LZMA_PROGRAM")),
    CompressionSuffix::new("tlz", Some("LZMA_PROGRAM")),
    CompressionSuffix::new("lzo", Some("LZOP_PROGRAM")),
    CompressionSuffix::new("xz", Some("XZ_PROGRAM")),
    CompressionSuffix::new("txz", Some("XZ_PROGRAM")),
    CompressionSuffix::new("zst", Some("ZSTD_PROGRAM")),
    CompressionSuffix::new("tzst", Some("ZSTD_PROGRAM")),
];

fn find_compression_suffix(name: &str) -> Option<(&CompressionSuffix, usize)> {
    if let Some(dot_pos) = name.rfind('.') {
        let suffix = &name[dot_pos + 1..];
        for comp_suffix in COMPRESSION_SUFFIXES {
            if comp_suffix.suffix == suffix {
                let len = name.len() - suffix.len() - 1;
                return Some((comp_suffix, len));
            }
        }
    }
    None
}

fn find_compression_program(name: &str, defprog: Option<&str>) -> Option<&str> {
    find_compression_suffix(name)
        .and_then(|(suffix, _)| suffix.program)
        .or(defprog)
}

fn set_compression_program_by_suffix(name: &str, defprog: Option<&str>) {
    if let Some(program) = find_compression_program(name, defprog) {
        // Assuming use_compress_program_option is a mutable static or similar
        // In Rust, we'd typically avoid mutable statics and use proper sharing
        unsafe {
            use_compress_program_option = CString::new(program).unwrap().into_raw();
        }
    }
}

fn strip_compression_suffix(name: &str) -> Option<String> {
    if let Some((suffix, len)) = find_compression_suffix(name) {
        let mut new_len = len;
        // Strip additional ".tar" suffix if conditions met
        if len > 4 && name[len - 4..].starts_with(".tar") && !suffix.suffix.starts_with('t') {
            new_len -= 4;
        }
        if new_len == 0 {
            return None;
        }
        Some(name[..new_len].to_string())
    } else {
        None
    }
}

// Assuming this is defined elsewhere as needed
static mut use_compress_program_option: *mut c_char = ptr::null_mut();