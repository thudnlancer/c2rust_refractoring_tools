// Function to parse a 'long long int' from text.
// Copyright (C) 1995-1997, 1999, 2001, 2009-2023 Free Software Foundation,
// Inc.
// This file is part of the GNU C Library.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::str::FromStr;
use std::num::ParseIntError;

pub fn strtoll(s: &str, base: u32) -> Result<i64, ParseIntError> {
    i64::from_str_radix(s.trim(), base)
}

pub fn strtoq(s: &str, base: u32) -> Result<i64, ParseIntError> {
    strtoll(s, base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtoll() {
        assert_eq!(strtoll("123", 10).unwrap(), 123);
        assert_eq!(strtoll("-456", 10).unwrap(), -456);
        assert_eq!(strtoll("FF", 16).unwrap(), 255);
    }

    #[test]
    fn test_strtoq() {
        assert_eq!(strtoq("123", 10).unwrap(), 123);
        assert_eq!(strtoq("-456", 10).unwrap(), -456);
        assert_eq!(strtoq("FF", 16).unwrap(), 255);
    }
}