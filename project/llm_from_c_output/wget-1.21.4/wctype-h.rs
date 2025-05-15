// Inline functions for <wctype.h>.
//
// Copyright (C) 2012-2023 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Normally this would be wctype.c, but that name's already taken.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Use Rust's built-in unicode support instead of C's wctype
use std::char;

// Re-export the necessary unicode functionality
pub use char::*;