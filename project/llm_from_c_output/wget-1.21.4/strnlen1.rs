// Find the length of STRING + 1, but scan at most MAXLEN bytes.
// Copyright (C) 2005, 2009-2023 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// Find the length of STRING + 1, but scan at most MAXLEN bytes.
/// If no '\0' terminator is found in that many characters, return MAXLEN.
/// This is the same as strnlen(string, maxlen - 1) + 1.
#[must_use]
pub fn strnlen1(string: &[u8], maxlen: usize) -> usize {
    if maxlen == 0 {
        return 0;
    }
    
    let search_len = maxlen.min(string.len());
    match string[..search_len].iter().position(|&c| c == b'\0') {
        Some(pos) => pos + 1,
        None => maxlen,
    }
}