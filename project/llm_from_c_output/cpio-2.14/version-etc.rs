//! Print --version and bug-reporting information in a consistent format.
//! Copyright (C) 1999-2023 Free Software Foundation, Inc.
//!
//! This file is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version.

use std::fmt::Write;
use std::io::{self, Write as IoWrite};

pub const VERSION_ETC_COPYRIGHT: &str = "Copyright (C) {} Free Software Foundation, Inc.\n\
    License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\n\
    This is free software: you are free to change and redistribute it.\n\
    There is NO WARRANTY, to the extent permitted by law.\n";

const COPYRIGHT_YEAR: u32 = 2023;

/// Display version information with authors in an array.
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

    #[cfg(feature = "packager_info")]
    {
        if let Some(packager) = option_env!("PACKAGE_PACKAGER") {
            if let Some(ver) = option_env!("PACKAGE_PACKAGER_VERSION") {
                writeln!(stream, "Packaged by {} ({})", packager, ver)?;
            } else {
                writeln!(stream, "Packaged by {}", packager)?;
            }
        }
    }

    writeln!(stream, VERSION_ETC_COPYRIGHT, COPYRIGHT_YEAR)?;
    writeln!(stream)?;

    match authors.len() {
        0 => {}
        1 => writeln!(stream, "Written by {}.", authors[0])?,
        2 => writeln!(stream, "Written by {} and {}.", authors[0], authors[1])?,
        3 => writeln!(stream, "Written by {}, {}, and {}.", authors[0], authors[1], authors[2])?,
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

/// Display version information with NULL-terminated array of authors.
pub fn version_etc_ar(
    stream: &mut impl IoWrite,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    let n_authors = authors.iter().take_while(|&&a| !a.is_empty()).count();
    version_etc_arn(stream, command_name, package, version, &authors[..n_authors])
}

/// Display version information with authors passed as separate arguments.
pub fn version_etc(
    stream: &mut impl IoWrite,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: &[&str],
) -> io::Result<()> {
    version_etc_ar(stream, command_name, package, version, authors)
}

/// Display bug reporting information.
pub fn emit_bug_reporting_address() -> io::Result<()> {
    let mut stdout = io::stdout();
    writeln!(stdout)?;

    if let Some(bugreport) = option_env!("PACKAGE_BUGREPORT") {
        writeln!(stdout, "Report bugs to: {}", bugreport)?;
    }

    #[cfg(feature = "packager_info")]
    {
        if let (Some(packager), Some(reports)) = (
            option_env!("PACKAGE_PACKAGER"),
            option_env!("PACKAGE_PACKAGER_BUG_REPORTS"),
        ) {
            writeln!(stdout, "Report {} bugs to: {}", packager, reports)?;
        }
    }

    if let Some(url) = option_env!("PACKAGE_URL") {
        if let Some(name) = option_env!("PACKAGE_NAME") {
            writeln!(stdout, "{} home page: <{}>", name, url)?;
        }
    } else if let Some(name) = option_env!("PACKAGE_NAME") {
        writeln!(
            stdout,
            "{} home page: <https://www.gnu.org/software/{}/>",
            name, name.to_lowercase()
        )?;
    }

    writeln!(
        stdout,
        "General help using GNU software: <https://www.gnu.org/gethelp/>"
    )?;

    Ok(())
}