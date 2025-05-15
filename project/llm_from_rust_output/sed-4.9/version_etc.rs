use std::ffi::CStr;
use std::fmt::Write;
use std::io::{self, Write as IoWrite};

const COPYRIGHT_YEAR: u32 = 2022;

pub fn version_etc_arn(
    mut stream: impl io::Write,
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
        "(C) {} Free Software Foundation, Inc.",
        COPYRIGHT_YEAR
    )?;
    writeln!(stream)?;
    writeln!(
        stream,
        "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\n\
         This is free software: you are free to change and redistribute it.\n\
         There is NO WARRANTY, to the extent permitted by law."
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

    Ok(())
}

pub fn emit_bug_reporting_address(mut stream: impl io::Write) -> io::Result<()> {
    writeln!(stream)?;
    writeln!(stream, "Report bugs to: bug-sed@gnu.org")?;
    writeln!(
        stream,
        "GNU sed home page: <https://www.gnu.org/software/sed/>"
    )?;
    writeln!(
        stream,
        "General help using GNU software: <https://www.gnu.org/gethelp/>"
    )?;
    Ok(())
}

// Safe wrappers for FFI functions
pub fn version_etc_arn_ffi(
    stream: *mut libc::FILE,
    command_name: *const libc::c_char,
    package: *const libc::c_char,
    version: *const libc::c_char,
    authors: *const *const libc::c_char,
    n_authors: libc::size_t,
) {
    unsafe {
        let cmd_name = if command_name.is_null() {
            None
        } else {
            Some(CStr::from_ptr(command_name).to_str().unwrap_or_default())
        };
        
        let pkg = CStr::from_ptr(package).to_str().unwrap_or_default();
        let ver = CStr::from_ptr(version).to_str().unwrap_or_default();
        
        let authors_slice = std::slice::from_raw_parts(authors, n_authors)
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_str().unwrap_or_default())
            .collect::<Vec<_>>();
        
        let mut stream = io::stdout();
        version_etc_arn(&mut stream, cmd_name, pkg, ver, &authors_slice).unwrap();
    }
}

pub fn emit_bug_reporting_address_ffi() {
    let mut stream = io::stdout();
    emit_bug_reporting_address(&mut stream).unwrap();
}