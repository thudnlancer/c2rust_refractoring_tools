use std::ffi::CString;
use std::io::{self, Write};
use gettextrs::{dcgettext, TextDomain};

const COPYRIGHT_YEAR: u32 = 2023;

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

    let copyright = dcgettext(
        None,
        "(C)",
        gettextrs::LocaleCategory::LC_MESSAGES,
    ).to_string_lossy();
    writeln!(stream, "{} {}", copyright, COPYRIGHT_YEAR)?;

    let license = dcgettext(
        None,
        "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\n\
         This is free software: you are free to change and redistribute it.\n\
         There is NO WARRANTY, to the extent permitted by law.",
        gettextrs::LocaleCategory::LC_MESSAGES,
    ).to_string_lossy();
    writeln!(stream, "\n{}\n", license)?;

    match authors.len() {
        0 => {}
        1 => {
            let msg = dcgettext(
                None,
                "Written by {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0])?;
        }
        2 => {
            let msg = dcgettext(
                None,
                "Written by {} and {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1])?;
        }
        3 => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, and {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2])?;
        }
        4 => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, {},\nand {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2], authors[3])?;
        }
        5 => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, {},\n{}, and {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2], authors[3], authors[4])?;
        }
        6 => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, {},\n{}, {}, and {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2], authors[3], authors[4], authors[5])?;
        }
        7 => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, {},\n{}, {}, {}, and {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6])?;
        }
        8 => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7])?;
        }
        9 => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7], authors[8])?;
        }
        _ => {
            let msg = dcgettext(
                None,
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.",
                gettextrs::LocaleCategory::LC_MESSAGES,
            ).to_string_lossy();
            writeln!(stream, msg, authors[0], authors[1], authors[2], authors[3], authors[4], authors[5], authors[6], authors[7], authors[8])?;
        }
    }

    Ok(())
}

pub fn emit_bug_reporting_address() -> io::Result<()> {
    let mut stdout = io::stdout();
    
    writeln!(stdout)?;
    
    let report_msg = dcgettext(
        None,
        "Report bugs to: {}",
        gettextrs::LocaleCategory::LC_MESSAGES,
    ).to_string_lossy();
    writeln!(stdout, report_msg, "bug-cpio@gnu.org")?;
    
    let home_msg = dcgettext(
        None,
        "{} home page: <{}>",
        gettextrs::LocaleCategory::LC_MESSAGES,
    ).to_string_lossy();
    writeln!(stdout, home_msg, "GNU cpio", "http://www.gnu.org/software/cpio")?;
    
    let help_msg = dcgettext(
        None,
        "General help using GNU software: <{}>",
        gettextrs::LocaleCategory::LC_MESSAGES,
    ).to_string_lossy();
    writeln!(stdout, help_msg, "https://www.gnu.org/gethelp/")?;
    
    Ok(())
}