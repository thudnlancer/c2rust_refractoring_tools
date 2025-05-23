// Convert string to wide string.
// Copyright (C) 2008-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2008.
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

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

// Internal state used by the functions mbsrtowcs() and mbsnrtowcs().
// The state must initially be in the "initial state"; so, zero-initialize it.
// On most systems, putting it into BSS is sufficient. Not so on Mac OS X 10.3,
// see <https://lists.gnu.org/r/bug-gnulib/2009-01/msg00329.html>.
// When it needs an initializer, use 0 or {0} as initializer? 0 only works
// when mbstate_t is a scalar type (such as when gnulib defines it, or on
// AIX, IRIX, mingw). {0} works as an initializer in all cases: for a struct
// or union type, but also for a scalar type (ISO C 99, 6.7.8.(11)).
#[cfg(not(target_env = "gnu"))]
static mut _gl_mbsrtowcs_state: std::ffi::c_void = std::ffi::c_void {};

#[cfg(target_env = "gnu")]
static mut _gl_mbsrtowcs_state: std::ffi::c_void = std::mem::zeroed();