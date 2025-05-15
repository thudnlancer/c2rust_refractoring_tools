// Removes leading and/or trailing whitespaces
// Copyright (C) 2006, 2009-2023 Free Software Foundation, Inc.
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

// Written by Davide Angelocola <davide.angelocola@gmail.com>

use std::iter::FromIterator;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrimMode {
    Trailing,
    Leading,
    Both,
}

/// Removes trailing and leading whitespaces.
pub fn trim(s: &str) -> String {
    trim2(s, TrimMode::Both)
}

/// Removes trailing whitespaces.
pub fn trim_trailing(s: &str) -> String {
    trim2(s, TrimMode::Trailing)
}

/// Removes leading whitespaces.
pub fn trim_leading(s: &str) -> String {
    trim2(s, TrimMode::Leading)
}

/// Removes leading and/or trailing whitespaces
pub fn trim2(s: &str, how: TrimMode) -> String {
    let mut d = s.to_string();

    if how != TrimMode::Trailing {
        // Trim leading whitespaces
        d = d.chars()
            .skip_while(|c| c.is_whitespace())
            .collect::<String>();
    }

    if how != TrimMode::Leading {
        // Trim trailing whitespaces
        d = d.chars()
            .rev()
            .skip_while(|c| c.is_whitespace())
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect::<String>();
    }

    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_both() {
        assert_eq!(trim("  hello  "), "hello");
        assert_eq!(trim("hello  "), "hello");
        assert_eq!(trim("  hello"), "hello");
        assert_eq!(trim("hello"), "hello");
        assert_eq!(trim("   "), "");
    }

    #[test]
    fn test_trim_trailing() {
        assert_eq!(trim_trailing("  hello  "), "  hello");
        assert_eq!(trim_trailing("hello  "), "hello");
        assert_eq!(trim_trailing("  hello"), "  hello");
        assert_eq!(trim_trailing("hello"), "hello");
        assert_eq!(trim_trailing("   "), "");
    }

    #[test]
    fn test_trim_leading() {
        assert_eq!(trim_leading("  hello  "), "hello  ");
        assert_eq!(trim_leading("hello  "), "hello  ");
        assert_eq!(trim_leading("  hello"), "hello");
        assert_eq!(trim_leading("hello"), "hello");
        assert_eq!(trim_leading("   "), "");
    }
}