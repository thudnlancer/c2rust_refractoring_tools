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

pub fn strtoll(s: &str, radix: u32) -> Result<i64, ParseIntError> {
    i64::from_str_radix(s, radix)
}

pub fn strtoq(s: &str, radix: u32) -> Result<i64, ParseIntError> {
    strtoll(s, radix)
}