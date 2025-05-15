// Print --version and bug-reporting information in a consistent format.
// Copyright (C) 1999, 2003, 2005, 2009-2022 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Jim Meyering.

use std::io::{self, Write};
use std::fmt::Write as FmtWrite;

pub const VERSION_ETC_COPYRIGHT: &str = "Copyright (C) {} Free Software Foundation, Inc.";

/// Display version information with authors in an array
pub fn version_etc_arn(
    stream: &mut dyn Write,
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
            gettext!("Packaged by {} ({})"),
            PACKAGE_PACKAGER,
            PACKAGE_PACKAGER_VERSION
        )?;
        #[cfg(not(feature = "packager_version"))]
        writeln!(stream, gettext!("Packaged by {}"), PACKAGE_PACKAGER)?;
    }

    writeln!(
        stream,
        VERSION_ETC_COPYRIGHT,
        gettext!("(C)"),
        COPYRIGHT_YEAR
    )?;

    writeln!(stream)?;

    writeln!(
        stream,
        gettext!(
            "License GPLv3+: GNU GPL version 3 or later <{}>.\n\
             This is free software: you are free to change and redistribute it.\n\
             There is NO WARRANTY, to the extent permitted by law.\n",
        ),
        "https://gnu.org/licenses/gpl.html"
    )?;

    writeln!(stream)?;

    match authors.len() {
        0 => {}
        1 => writeln!(stream, gettext!("Written by {}."), authors[0])?,
        2 => writeln!(
            stream,
            gettext!("Written by {} and {}."),
            authors[0], authors[1]
        )?,
        3 => writeln!(
            stream,
            gettext!("Written by {}, {}, and {}."),
            authors[0], authors[1], authors[2]
        )?,
        4 => writeln!(
            stream,
            gettext!("Written by {}, {}, {},\nand {}."),
            authors[0], authors[1], authors[2], authors[3]
        )?,
        5 => writeln!(
            stream,
            gettext!("Written by {}, {}, {},\n{}, and {}."),
            authors[0], authors[1], authors[2], authors[3], authors[4]
        )?,
        6 => writeln!(
            stream,
            gettext!("Written by {}, {}, {},\n{}, {}, and {}."),
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5]
        )?,
        7 => writeln!(
            stream,
            gettext!("Written by {}, {}, {},\n{}, {}, {}, and {}."),
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6]
        )?,
        8 => writeln!(
            stream,
            gettext!("Written by {}, {}, {},\n{}, {}, {}, {},\nand {}."),
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6],
            authors[7]
        )?,
        9 => writeln!(
            stream,
            gettext!("Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}."),
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6],
            authors[7], authors[8]
        )?,
        _ => writeln!(
            stream,
            gettext!("Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others."),
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6],
            authors[7], authors[8]
        )?,
    }

    Ok(())
}

/// Display version information with NULL-terminated array of authors
pub fn version_etc_ar(
    stream: &mut dyn Write,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    version_etc_arn(stream, command_name, package, version, authors)
}

/// Display version information with authors in a va_list (variadic args)
pub fn version_etc_va(
    stream: &mut dyn Write,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    version_etc_arn(stream, command_name, package, version, authors)
}

/// Display version information with variadic authors
pub fn version_etc(
    stream: &mut dyn Write,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    version_etc_arn(stream, command_name, package, version, authors)
}

/// Display bug reporting information
pub fn emit_bug_reporting_address() -> io::Result<()> {
    writeln!(std::io::stdout())?;
    writeln!(
        std::io::stdout(),
        gettext!("Report bugs to: {}"),
        PACKAGE_BUGREPORT
    )?;

    #[cfg(feature = "packager_bug_reports")]
    writeln!(
        std::io::stdout(),
        gettext!("Report {} bugs to: {}"),
        PACKAGE_PACKAGER,
        PACKAGE_PACKAGER_BUG_REPORTS
    )?;

    #[cfg(feature = "package_url")]
    writeln!(
        std::io::stdout(),
        gettext!("{} home page: <{}>"),
        PACKAGE_NAME,
        PACKAGE_URL
    )?;
    #[cfg(not(feature = "package_url"))]
    writeln!(
        std::io::stdout(),
        gettext!("{} home page: <{}>"),
        PACKAGE_NAME,
        format!("https://www.gnu.org/software/{}/", PACKAGE)
    )?;

    writeln!(
        std::io::stdout(),
        gettext!("General help using GNU software: <{}>"),
        "https://www.gnu.org/gethelp/"
    )
}

const COPYRIGHT_YEAR: i32 = 2022;

// Mock gettext implementation for compilation
#[macro_export]
macro_rules! gettext {
    ($s:expr) => {
        $s
    };
}

// Mock package info for compilation
#[cfg(not(feature = "package"))]
const PACKAGE: &str = "package";
#[cfg(not(feature = "package_bugreport"))]
const PACKAGE_BUGREPORT: &str = "bug-report@example.com";
#[cfg(not(feature = "package_name"))]
const PACKAGE_NAME: &str = "Package Name";