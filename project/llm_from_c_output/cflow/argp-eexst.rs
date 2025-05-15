/* -*- buffer-read-only: t -*- vi: set ro: */
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Default definition for ARGP_ERR_EXIT_STATUS
   Copyright (C) 1997 Free Software Foundation, Inc.
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

#[cfg(feature = "config")]
use config;

use std::process;

/// The exit status that argp will use when exiting due to a parsing error.
/// If not defined or set by the user program, this defaults to EX_USAGE (64).
pub static mut ARGP_ERR_EXIT_STATUS: i32 = 64; // EX_USAGE is typically 64 in sysexits.h

// Note: In Rust, we'd typically avoid global mutable state where possible.
// A safer alternative would be to use interior mutability patterns like Mutex or AtomicI32,
// but since this is meant to match the C behavior exactly, we keep it as mutable static.
// In real Rust code, you'd want to consider thread safety and prefer safer alternatives.