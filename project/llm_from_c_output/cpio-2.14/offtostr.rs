// Convert 'off_t' integer to printable string.
//
// Copyright (C) 2004-2023 Free Software Foundation, Inc.
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

use std::io::{self, Write};

/// Converts an off_t integer to a printable string
pub fn offtostr(value: i64) -> String {
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offtostr() {
        assert_eq!(offtostr(0), "0");
        assert_eq!(offtostr(123), "123");
        assert_eq!(offtostr(-456), "-456");
        assert_eq!(offtostr(i64::MAX), i64::MAX.to_string());
        assert_eq!(offtostr(i64::MIN), i64::MIN.to_string());
    }
}