/* Miscellaneous error functions

   Copyright (C) 2005-2007, 2023 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 3, or (at your option) any later
   version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General
   Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program; if not, write to the Free Software Foundation, Inc.,
   51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.  */

use std::process;

static mut EXIT_STATUS: i32 = 0;

pub fn set_exit_status(status: i32) {
    unsafe {
        EXIT_STATUS = status;
    }
}

pub fn pax_exit() -> ! {
    unsafe {
        process::exit(EXIT_STATUS);
    }
}