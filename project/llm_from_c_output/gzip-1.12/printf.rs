// Formatted output to a stream.
// Copyright (C) 2007, 2010-2022 Free Software Foundation, Inc.
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

use std::io::{self, Write};

/// Print formatted output to standard output.
/// Returns the string length of formatted string on success, or an IO error on failure.
pub fn printf(format: &str, args: std::fmt::Arguments<'_>) -> io::Result<usize> {
    let mut stdout = io::stdout();
    stdout.write_fmt(args)
}