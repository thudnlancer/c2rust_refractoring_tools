use std::ffi::CStr;
use std::io::{self, Write};
use std::ptr;

const COPYRIGHT_YEAR: u32 = 2009;

pub fn version_etc_arn(
    stream: &mut dyn Write,
    command_name: Option<&CStr>,
    package: &CStr,
    version: &CStr,
    authors: &[&CStr],
) -> io::Result<()> {
    if let Some(cmd) = command_name {
        writeln!(
            stream,
            "{} ({}) {}",
            cmd.to_str().unwrap(),
            package.to_str().unwrap(),
            version.to_str().unwrap()
        )?;
    } else {
        writeln!(
            stream,
            "{} {}",
            package.to_str().unwrap(),
            version.to_str().unwrap()
        )?;
    }

    writeln!(
        stream,
        "Copyright (C) {} Free Software Foundation, Inc.",
        COPYRIGHT_YEAR
    )?;
    writeln!(
        stream,
        "License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>."
    )?;
    writeln!(
        stream,
        "This is free software: you are free to change and redistribute it."
    )?;
    writeln!(
        stream,
        "There is NO WARRANTY, to the extent permitted by law.\n"
    )?;

    match authors.len() {
        0 => panic!("No authors specified"),
        1 => writeln!(
            stream,
            "Written by {}.",
            authors[0].to_str().unwrap()
        )?,
        2 => writeln!(
            stream,
            "Written by {} and {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap()
        )?,
        3 => writeln!(
            stream,
            "Written by {}, {}, and {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap()
        )?,
        4 => writeln!(
            stream,
            "Written by {}, {}, {},\nand {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap(),
            authors[3].to_str().unwrap()
        )?,
        5 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, and {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap(),
            authors[3].to_str().unwrap(),
            authors[4].to_str().unwrap()
        )?,
        6 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, and {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap(),
            authors[3].to_str().unwrap(),
            authors[4].to_str().unwrap(),
            authors[5].to_str().unwrap()
        )?,
        7 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, and {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap(),
            authors[3].to_str().unwrap(),
            authors[4].to_str().unwrap(),
            authors[5].to_str().unwrap(),
            authors[6].to_str().unwrap()
        )?,
        8 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap(),
            authors[3].to_str().unwrap(),
            authors[4].to_str().unwrap(),
            authors[5].to_str().unwrap(),
            authors[6].to_str().unwrap(),
            authors[7].to_str().unwrap()
        )?,
        9 => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap(),
            authors[3].to_str().unwrap(),
            authors[4].to_str().unwrap(),
            authors[5].to_str().unwrap(),
            authors[6].to_str().unwrap(),
            authors[7].to_str().unwrap(),
            authors[8].to_str().unwrap()
        )?,
        _ => writeln!(
            stream,
            "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.",
            authors[0].to_str().unwrap(),
            authors[1].to_str().unwrap(),
            authors[2].to_str().unwrap(),
            authors[3].to_str().unwrap(),
            authors[4].to_str().unwrap(),
            authors[5].to_str().unwrap(),
            authors[6].to_str().unwrap(),
            authors[7].to_str().unwrap(),
            authors[8].to_str().unwrap()
        )?,
    }

    Ok(())
}

pub fn emit_bug_reporting_address() -> io::Result<()> {
    println!("\nReport bugs to <bug-cflow@gnu.org>.");
    println!("GNU cflow home page: <http://www.gnu.org/software/cflow/>.");
    println!("General help using GNU software: <http://www.gnu.org/gethelp/>.");
    Ok(())
}