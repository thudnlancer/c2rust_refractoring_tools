// Default definition for ARGP_PROGRAM_VERSION.
// Copyright (C) 1996-2021 Free Software Foundation, Inc.
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

// If set by the user program to a non-zero value, then a default option
// --version is added (unless the ARGP_NO_HELP flag is used), which will
// print this string followed by a newline and exit (unless the
// ARGP_NO_EXIT flag is used).  Overridden by ARGP_PROGRAM_VERSION_HOOK.
//
// This variable should be zero-initialized.  On most systems, putting it into
// BSS is sufficient.  Not so on Mac OS X 10.3 and 10.4, see
// <https://lists.gnu.org/r/bug-gnulib/2009-01/msg00329.html>
// <https://lists.gnu.org/r/bug-gnulib/2009-08/msg00096.html>.
#[cfg(not(target_env = "gnu"))]
static ARGP_PROGRAM_VERSION: Option<&'static str> = None;

#[cfg(target_env = "gnu")]
// On ELF systems, variables in BSS behave well.
static ARGP_PROGRAM_VERSION: Option<&'static str> = None;