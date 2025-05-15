/* Report a memory allocation failure and exit.

   Copyright (C) 1997-2000, 2002-2004, 2006, 2009-2022 Free Software
   Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::process;
use gettextrs::gettext;

/// Reports a memory allocation failure and exits.
/// This function never returns as it either exits the process or aborts.
pub fn xalloc_die() -> ! {
    eprintln!("{}", gettext("memory exhausted"));
    
    // Exit with failure status code. If exit_failure is 0 (which shouldn't happen),
    // we abort as a safety measure.
    process::exit(exit_failure());
}

/// Returns the exit failure status code.
/// In Rust, we typically use std::process::ExitCode, but for compatibility
/// with the original C code's behavior, we use a function that returns i32.
fn exit_failure() -> i32 {
    // Standard Unix exit code for failure
    1
}