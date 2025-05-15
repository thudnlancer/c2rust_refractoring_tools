use std::ffi::CStr;
use std::io::{self, Write};
use gettextrs::gettext;

const COPYRIGHT_YEAR: u32 = 2022;

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

    writeln!(
        stream,
        "{} {}",
        gettext("(C)"),
        COPYRIGHT_YEAR
    )?;

    writeln!(stream)?;
    writeln!(
        stream,
        "{}",
        gettext("License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.")
    )?;
    writeln!(stream)?;

    match authors.len() {
        0 => {}
        1 => writeln!(stream, "{}", gettext(&format!("Written by {}.", authors[0])))?,
        2 => writeln!(
            stream,
            "{}",
            gettext(&format!("Written by {} and {}.", authors[0], authors[1]))
        )?,
        3 => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, and {}.",
                authors[0], authors[1], authors[2]
            ))
        )?,
        4 => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, {},\nand {}.",
                authors[0], authors[1], authors[2], authors[3]
            ))
        )?,
        5 => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, {},\n{}, and {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4]
            ))
        )?,
        6 => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, {},\n{}, {}, and {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5]
            ))
        )?,
        7 => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, {},\n{}, {}, {}, and {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6]
            ))
        )?,
        8 => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7]
            ))
        )?,
        9 => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7], authors[8]
            ))
        )?,
        _ => writeln!(
            stream,
            "{}",
            gettext(&format!(
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.",
                authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7], authors[8]
            ))
        )?,
    }

    Ok(())
}

pub fn emit_bug_reporting_address() -> io::Result<()> {
    let mut stdout = io::stdout();
    writeln!(stdout)?;
    writeln!(
        stdout,
        "{}",
        gettext("Report bugs to: bug-findutils@gnu.org")
    )?;
    writeln!(
        stdout,
        "{}",
        gettext("GNU findutils home page: <http://www.gnu.org/software/findutils/>")
    )?;
    writeln!(
        stdout,
        "{}",
        gettext("General help using GNU software: <https://www.gnu.org/gethelp/>")
    )?;

    Ok(())
}