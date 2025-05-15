// Real definitions for extern inline functions in argp-fmtstream.h
// Copyright (C) 1997-2021 Free Software Foundation, Inc.
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

#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(feature = "config")]
use config::Config;

#[cfg(feature = "libc")]
const ARGP_FS_EI: &str = "";

#[cfg(not(feature = "libc"))]
const ARGP_FS_EI: &str = "_GL_EXTERN_INLINE";

// Note: In Rust, we don't need to undefine __OPTIMIZE__ as Rust has different optimization controls
// The original header inclusion is replaced by Rust module system
// use argp_fmtstream; // Assuming this would be a Rust module equivalent

// The weak alias section is commented out in original and not relevant in Rust
// as Rust has different linking mechanisms