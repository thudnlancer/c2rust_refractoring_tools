/* Record version and build host architecture for GNU Make.
Copyright (C) 1988-2023 Free Software Foundation, Inc.
This file is part of GNU Make.

GNU Make is free software; you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation; either version 3 of the License, or (at your option) any later
version.

GNU Make is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.  */

// We use build-time configuration from built crate metadata
// instead of config.h in C version

lazy_static::lazy_static! {
    pub static ref VERSION_STRING: String = env!("CARGO_PKG_VERSION").to_string();
    pub static ref MAKE_HOST: String = option_env!("MAKE_HOST").unwrap_or("unknown").to_string();
}

/*
  Local variables:
  version-control: never
  End:
 */