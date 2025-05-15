// Report a memory allocation failure and exit.

// Copyright (C) 1997-2000, 2002-2004, 2006, 2009-2023 Free Software
// Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::process;

pub fn xalloc_die() -> ! {
    eprintln!("memory exhausted");
    process::exit(1);
    
    // The original C code had a comment about _Noreturn and abort().
    // In Rust, the return type ! (never type) ensures this function never returns,
    // and process::exit() is guaranteed not to return, so no abort() is needed.
}