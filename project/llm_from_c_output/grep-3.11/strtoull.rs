// Function to parse an 'unsigned long long int' from text.
// Copyright (C) 1995-1997, 1999, 2009-2023 Free Software Foundation, Inc.
// NOTE: The canonical source of this file is maintained with the GNU C
// Library.  Bugs can be reported to bug-glibc@gnu.org.
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

pub fn strtoull(s: &str, base: u32) -> Result<u64, ParseIntError> {
    u64::from_str_radix(s, base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtoull() {
        assert_eq!(strtoull("123", 10), Ok(123));
        assert_eq!(strtoull("FF", 16), Ok(255));
        assert!(strtoull("invalid", 10).is_err());
    }
}