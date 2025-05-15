// Full and short program names for argp module
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

#[cfg(not(feature = "have_program_invocation_short_name"))]
pub static mut PROGRAM_INVOCATION_SHORT_NAME: Option<String> = None;

#[cfg(not(feature = "have_program_invocation_name"))]
pub static mut PROGRAM_INVOCATION_NAME: Option<String> = None;

#[cfg(all(
    feature = "have_program_invocation_short_name",
    feature = "have_program_invocation_name"
))]
/// This declaration is solely to ensure that after preprocessing
/// this file is never empty.
type Dummy = i32;