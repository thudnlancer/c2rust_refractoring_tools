/* -*- buffer-read-only: t -*- vi: set ro: */
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Report a memory allocation failure and exit.

   Copyright (C) 1997, 1998, 1999, 2000, 2002, 2003, 2004, 2006 Free
   Software Foundation, Inc.

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

use std::process;
use gettextrs::gettext;

/// Reports a memory allocation failure and exits.
///
/// This function is marked as `#[noreturn]` since it never returns.
#[noreturn]
pub fn xalloc_die() {
    eprintln!("{}", gettext("memory exhausted"));
    
    // The `noreturn` cannot be given to error, since it may return if
    // its first argument is 0. To help compilers understand the
    // xalloc_die does not return, call abort. Also, the abort is a
    // safety feature if exit_failure is 0 (which shouldn't happen).
    process::abort();
}