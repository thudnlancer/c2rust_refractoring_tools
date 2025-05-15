/* -*- buffer-read-only: t -*- vi: set ro: */
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Default definition for ARGP_PROGRAM_VERSION_HOOK.
   Copyright (C) 1996, 1997, 1999, 2004 Free Software Foundation, Inc.
   This file is part of the GNU C Library.
   Written by Miles Bader <miles@gnu.ai.mit.edu>.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

use std::io::{self, Write};
use std::sync::Arc;

/// Represents the state of argument parsing.
pub struct ArgpState {
    // Implementation details of argp state
}

/// A function pointer type for version hook.
/// If set by the user program to a non-None value, then a default option
/// --version is added (unless the ARGP_NO_HELP flag is used), which calls
/// this function with a stream to print the version to and a pointer to the
/// current parsing state, and then exits (unless the ARGP_NO_EXIT flag is
/// used). This variable takes precedence over ARGP_PROGRAM_VERSION.
pub type ArgpProgramVersionHook = Option<Arc<dyn Fn(&mut dyn Write, &ArgpState) -> io::Result<()>>>;

/// Global variable holding the version hook function.
pub static mut ARGP_PROGRAM_VERSION_HOOK: ArgpProgramVersionHook = None;