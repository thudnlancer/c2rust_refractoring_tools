// A more useful interface to strtoumax.

// Copyright (C) 1999-2022 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::num::ParseIntError;
use std::str::FromStr;
use std::u128; // Using u128 as equivalent to uintmax_t

pub fn xstrtoumax(s: &str, base: u32) -> Result<u128, ParseIntError> {
    u128::from_str_radix(s, base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xstrtoumax() {
        assert_eq!(xstrtoumax("42", 10), Ok(42));
        assert_eq!(xstrtoumax("2a", 16), Ok(42));
        assert!(xstrtoumax("invalid", 10).is_err());
    }
}