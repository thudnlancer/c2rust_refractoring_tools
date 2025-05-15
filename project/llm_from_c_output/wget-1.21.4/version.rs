//! Extern declarations for printing version information
//! Copyright (C) 2013, 2015, 2018-2023 Free Software Foundation, Inc.
//!
//! This file is part of GNU Wget.
//!
//! GNU Wget is free software; you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation; either version 3 of the License, or
//! (at your option) any later version.
//!
//! GNU Wget is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with Wget.  If not, see <http://www.gnu.org/licenses/>.
//!
//! Additional permission under GNU GPL version 3 section 7
//!
//! If you modify this program, or any covered work, by linking or
//! combining it with the OpenSSL project's OpenSSL library (or a
//! modified version of that library), containing parts covered by the
//! terms of the OpenSSL or SSLeay licenses, the Free Software Foundation
//! grants you additional permission to convey the resulting work.
//! Corresponding Source for a non-source form of such a combination
//! shall include the source code for the parts of OpenSSL used as well
//! as that of the covered work.

pub const VERSION_STRING: &str = "1.21.4";
pub const COMPILATION_STRING: &str = "gcc -DHAVE_CONFIG_H -DSYSTEM_WGETRC=\"/usr/local/etc/wgetrc\" -DLOCALEDIR=\"/usr/local/share/locale\" -I. -I../lib -I../lib -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2";
pub const LINK_STRING: &str = "gcc -I/usr/include/p11-kit-1 -DHAVE_LIBGNUTLS -DNDEBUG -g -O2 -lpcre -lidn2 -lnettle -lgnutls -lz ../lib/libgnu.a ";

// Note: In Rust, we don't need separate declarations and definitions for constants.
// The compiled_features array would be defined in the build_info module.