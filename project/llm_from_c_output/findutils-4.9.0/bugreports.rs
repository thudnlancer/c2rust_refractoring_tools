/*
 * bugreports.rs -- explain how to report bugs
 * Copyright (C) 2016-2022 Free Software Foundation, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
/* Written by James Youngman <jay@gnu.org>.
 */

use std::io::{self, Write};

/// Explains how to report bugs, writing the explanation to the given writer.
/// Returns the number of bytes written or an I/O error.
pub fn explain_how_to_report_bugs<W: Write>(
    mut writer: W,
    program_name: &str,
) -> io::Result<usize> {
    let message = format!(
        "Please see also the documentation at {}.\n\
         You can report (and track progress on fixing) bugs in the \"{}\"\n\
         program via the {} bug-reporting page at\n\
         {} or, if\n\
         you have no web access, by sending email to <{}>.\n",
        env!("PACKAGE_URL"),
        program_name,
        env!("PACKAGE_NAME"),
        env!("PACKAGE_BUGREPORT_URL"),
        env!("PACKAGE_BUGREPORT")
    );
    writer.write(message.as_bytes())
}