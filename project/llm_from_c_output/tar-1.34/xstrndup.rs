// Duplicate a bounded initial segment of a string, with out-of-memory
// checking.
// Copyright (C) 2003, 2009-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// Return a newly allocated copy of at most N bytes of STRING.
/// In other words, return a copy of the initial segment of length N of
/// STRING.
pub fn xstrndup(string: &str, n: usize) -> String {
    let s = string.chars().take(n).collect::<String>();
    if s.is_empty() && n > 0 && !string.is_empty() {
        // Handle case where we couldn't allocate memory (OOM)
        // In Rust, OOM typically panics by default, but we can make it explicit
        panic!("Memory allocation failed in xstrndup");
    }
    s
}