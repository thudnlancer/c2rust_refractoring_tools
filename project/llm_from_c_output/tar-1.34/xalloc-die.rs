// Report a memory allocation failure and exit.

// Copyright (C) 1997-2000, 2002-2004, 2006, 2009-2021 Free Software
// Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::process;
use gettextrs::gettext;

/// Function to report memory allocation failure and exit
pub fn xalloc_die() -> ! {
    let msg = gettext("memory exhausted");
    eprintln!("{}", msg);
    
    // Exit with failure status code
    // Using abort() here to ensure compiler knows this function never returns
    // and as a safety feature if exit_failure was 0 (which shouldn't happen)
    process::abort();
}