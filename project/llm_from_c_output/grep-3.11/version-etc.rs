/* Print --version and bug-reporting information in a consistent format.
   Copyright (C) 1999, 2003, 2005, 2009-2023 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Jim Meyering. */

use std::fmt::Write;
use std::io::{self, Write as IoWrite};

pub const VERSION_ETC_COPYRIGHT: &str = "Copyright (C) {} Free Software Foundation, Inc.";

/// Display version information with authors in an array
pub fn version_etc_arn(
    stream: &mut dyn IoWrite,
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
        VERSION_ETC_COPYRIGHT,
        // TRANSLATORS: Translate "(C)" to the copyright symbol
        if cfg!(feature = "gettext") {
            gettext("(C)")
        } else {
            "(C)"
        },
        2023
    )?;

    writeln!(stream)?;

    // TRANSLATORS: The %s placeholder is the web address of the GPL license.
    writeln!(
        stream,
        "License GPLv3+: GNU GPL version 3 or later <{}>.\n\
         This is free software: you are free to change and redistribute it.\n\
         There is NO WARRANTY, to the extent permitted by law.\n",
        "https://gnu.org/licenses/gpl.html"
    )?;

    writeln!(stream)?;

    match authors.len() {
        0 => {}
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
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6],
            authors[7]
        )?,
        9 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.",
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6],
            authors[7], authors[8]
        )?,
        _ => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.",
            authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6],
            authors[7], authors[8]
        )?,
    }

    Ok(())
}

/// Display version information with NULL-terminated array of authors
pub fn version_etc_ar(
    stream: &mut dyn IoWrite,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    let n_authors = authors.iter().take_while(|&&a| !a.is_empty()).count();
    version_etc_arn(stream, command_name, package, version, &authors[..n_authors])
}

/// Display version information with authors as separate arguments
pub fn version_etc(
    stream: &mut dyn IoWrite,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    version_etc_ar(stream, command_name, package, version, authors)
}

/// Display bug reporting information
pub fn emit_bug_reporting_address() -> io::Result<()> {
    let mut stdout = io::stdout();
    writeln!(stdout)?;

    writeln!(
        stdout,
        "Report bugs to: {}",
        env!("PACKAGE_BUGREPORT")
    )?;

    #[cfg(feature = "packager_bug_reports")]
    writeln!(
        stdout,
        "Report {} bugs to: {}",
        env!("PACKAGE_PACKAGER"),
        env!("PACKAGE_PACKAGER_BUG_REPORTS")
    )?;

    #[cfg(feature = "package_url")]
    writeln!(
        stdout,
        "{} home page: <{}>",
        env!("PACKAGE_NAME"),
        env!("PACKAGE_URL")
    )?;
    #[cfg(not(feature = "package_url"))]
    writeln!(
        stdout,
        "{} home page: <{}>",
        env!("PACKAGE_NAME"),
        format!("https://www.gnu.org/software/{}/", env!("PACKAGE"))
    )?;

    writeln!(
        stdout,
        "General help using GNU software: <{}>",
        "https://www.gnu.org/gethelp/"
    )?;

    Ok(())
}