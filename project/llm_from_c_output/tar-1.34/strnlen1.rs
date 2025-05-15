// Find the length of STRING + 1, but scan at most MAXLEN bytes.
// Copyright (C) 2005, 2009-2021 Free Software Foundation, Inc.
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

/// Find the length of STRING + 1, but scan at most MAXLEN bytes.
/// If no '\0' terminator is found in that many characters, return MAXLEN.
/// This is the same as strnlen(string, maxlen - 1) + 1.
pub fn strnlen1(string: &str, maxlen: usize) -> usize {
    if let Some(pos) = string.as_bytes().iter().take(maxlen).position(|&c| c == b'\0') {
        pos + 1
    } else {
        maxlen
    }
}