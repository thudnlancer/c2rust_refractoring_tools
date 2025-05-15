// -*- buffer-read-only: t -*- vi: set ro: */
// DO NOT EDIT! GENERATED AUTOMATICALLY! */
// Print --version and bug-reporting information in a consistent format.
// Copyright (C) 1999-2009 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

// Written by Jim Meyering.

use std::fmt::Write;
use std::io::{self, Write as IoWrite};

const COPYRIGHT_YEAR: u32 = 2009;

pub static version_etc_copyright: &str = "Copyright (C) {} Free Software Foundation, Inc.";

/// Display the --version information the standard way.
///
/// Author names are given in the slice `authors`. `n_authors` is the
/// number of elements in the slice.
pub fn version_etc_arn(
    stream: &mut impl IoWrite,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    if let Some(cmd) = command_name {
        writeln!(stream, "{} ({}) {}", cmd, package, version)?;
    } else {
        writeln!(stream, "{} {}", package, version)?;
    }

    #[cfg(feature = "packager")]
    {
        #[cfg(feature = "packager_version")]
        writeln!(
            stream,
            "Packaged by {} ({})",
            env!("PACKAGE_PACKAGER"),
            env!("PACKAGE_PACKAGER_VERSION")
        )?;
        #[cfg(not(feature = "packager_version"))]
        writeln!(stream, "Packaged by {}", env!("PACKAGE_PACKAGER"))?;
    }

    writeln!(
        stream,
        version_etc_copyright,
        "(C)", // Translation placeholder
        COPYRIGHT_YEAR
    )?;

    writeln!(
        stream,
        "\n\
        License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\n\
        This is free software: you are free to change and redistribute it.\n\
        There is NO WARRANTY, to the extent permitted by law.\n"
    )?;

    match authors.len() {
        0 => panic!("At least one author must be provided"),
        1 => writeln!(stream, "Written by {}.", authors[0])?,
        2 => writeln!(stream, "Written by {} and {}.", authors[0], authors[1])?,
        3 => writeln!(
            stream,
            "Written by {}, {}, and {}.",
            authors[0], authors[1], authors[2]
        )?,
        4 => writeln!(
            stream,
            "Written by {}, {}, {},\nand {}.",
            authors[0], authors[1], authors[2], authors[3]
        )?,
        5 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, and {}.",
            authors[0], authors[1], authors[2], authors[3], authors[4]
        )?,
        6 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, and {}.",
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5]
        )?,
        7 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, and {}.",
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6]
        )?,
        8 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.",
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7]
        )?,
        9 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.",
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7], authors[8]
        )?,
        _ => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.",
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7], authors[8]
        )?,
    }
}

/// Display the --version information the standard way.
///
/// Author names are given in the slice `authors`.
pub fn version_etc_ar(
    stream: &mut impl IoWrite,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    version_etc_arn(stream, command_name, package, version, authors)
}

/// Display the --version information the standard way.
///
/// Author names are given as variable arguments.
pub fn version_etc(
    stream: &mut impl IoWrite,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    version_etc_arn(stream, command_name, package, version, authors)
}

pub fn emit_bug_reporting_address() -> io::Result<()> {
    writeln!(
        io::stdout(),
        "\nReport bugs to <{}>.",
        env!("PACKAGE_BUGREPORT")
    )?;

    #[cfg(feature = "packager_bug_reports")]
    writeln!(
        io::stdout(),
        "Report {} bugs to <{}>.",
        env!("PACKAGE_PACKAGER"),
        env!("PACKAGE_PACKAGER_BUG_REPORTS")
    )?;

    writeln!(
        io::stdout(),
        "{} home page: <http://www.gnu.org/software/{}/>.",
        env!("PACKAGE_NAME"),
        env!("PACKAGE")
    )?;

    writeln!(
        io::stdout(),
        "General help using GNU software: <http://www.gnu.org/gethelp/>."
    )
}