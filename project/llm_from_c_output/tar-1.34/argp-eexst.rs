// Default definition for ARGP_ERR_EXIT_STATUS
// Copyright (C) 1997, 2009-2021 Free Software Foundation, Inc.
// This file is part of the GNU C Library.
// Written by Miles Bader <miles@gnu.ai.mit.edu>.
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

// The exit status that argp will use when exiting due to a parsing error.
// If not defined or set by the user program, this defaults to EX_USAGE from
// <sysexits.h>.
pub static mut ARGP_ERR_EXIT_STATUS: i32 = 64; // EX_USAGE is typically 64 in sysexits.h

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_value() {
        unsafe {
            assert_eq!(ARGP_ERR_EXIT_STATUS, 64);
        }
    }
}