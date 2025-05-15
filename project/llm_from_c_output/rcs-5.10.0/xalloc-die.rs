// Report a memory allocation failure and exit.

// Copyright (C) 1997-2000, 2002-2004, 2006, 2009-2020 Free Software
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

const EXIT_FAILURE: i32 = 1;

pub fn xalloc_die() -> ! {
    eprintln!("{}", gettext("memory exhausted"));
    
    // Since we want to ensure this function never returns,
    // we call process::abort() which is guaranteed to terminate
    // the program immediately.
    process::abort();
}