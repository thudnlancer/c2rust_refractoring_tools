// Convert string to 'unsigned long', with error checking.

// Copyright (C) 1994-2022 Free Software Foundation, Inc.

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

pub fn xstrtoul(s: &str) -> Result<u64, ParseIntError> {
    let result = u64::from_str(s)?;
    if result < 0 {
        return Err(ParseIntError { kind: std::num::IntErrorKind::NegOverflow });
    }
    Ok(result)
}