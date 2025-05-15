// A more useful interface to str::parse for intmax_t equivalent (i64/i128 depending on platform)

// Copyright (C) 2001-2023 Free Software Foundation, Inc.

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

use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;

type IntMax = i64; // Use i128 for 128-bit platforms

pub fn xstrtoimax(s: &str) -> Result<IntMax, ParseIntError> {
    s.trim().parse::<IntMax>()
}