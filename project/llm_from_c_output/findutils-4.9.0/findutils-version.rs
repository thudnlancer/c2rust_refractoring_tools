// findutils-version.rs -- show version information for findutils
// Copyright (C) 2007-2022 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{self, Write};

/// Official name of the program, for example "find" or "xargs" (as opposed to "/bin/find" or gfind).
pub fn display_findutils_version(official_name: &str) -> io::Result<()> {
    // We use official_name rather than program name in the version
    // information. This is deliberate, it is specified by the
    // GNU coding standard.
    io::stdout().flush()?;
    
    version_etc(
        official_name,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        &["Eric B. Decker", "James Youngman", "Kevin Dalley"],
    )
}

fn version_etc(
    program_name: &str,
    package_name: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    writeln!(handle, "{} (GNU {}) {}", program_name, package_name, version)?;
    writeln!(handle, "Copyright (C) Free Software Foundation, Inc.")?;
    writeln!(handle, "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>")?;
    writeln!(handle, "This is free software: you are free to change and redistribute it.")?;
    writeln!(handle, "There is NO WARRANTY, to the extent permitted by law.")?;
    
    writeln!(handle, "\nWritten by {}", authors.join(", "))?;
    
    Ok(())
}