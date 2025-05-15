use std::ffi::CStr;
use std::io::{self, Write};
use std::fmt;

const COPYRIGHT_YEAR: u32 = 2021;

struct VersionInfo<'a> {
    command_name: Option<&'a str>,
    package: &'a str,
    version: &'a str,
    authors: Vec<&'a str>,
}

impl<'a> VersionInfo<'a> {
    fn new(
        command_name: Option<&'a str>,
        package: &'a str,
        version: &'a str,
        authors: Vec<&'a str>,
    ) -> Self {
        Self {
            command_name,
            package,
            version,
            authors,
        }
    }

    fn display(&self, writer: &mut impl Write) -> io::Result<()> {
        if let Some(cmd) = self.command_name {
            writeln!(writer, "{} ({}) {}", cmd, self.package, self.version)?;
        } else {
            writeln!(writer, "{} {}", self.package, self.version)?;
        }

        writeln!(writer, "(C) {}", COPYRIGHT_YEAR)?;
        writeln!(writer)?;

        writeln!(
            writer,
            "License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.\n\
             This is free software: you are free to change and redistribute it.\n\
             There is NO WARRANTY, to the extent permitted by law.\n"
        )?;

        match self.authors.len() {
            0 => {}
            1 => writeln!(writer, "Written by {}.\n", self.authors[0])?,
            2 => writeln!(
                writer,
                "Written by {} and {}.\n",
                self.authors[0], self.authors[1]
            )?,
            3 => writeln!(
                writer,
                "Written by {}, {}, and {}.\n",
                self.authors[0], self.authors[1], self.authors[2]
            )?,
            4 => writeln!(
                writer,
                "Written by {}, {}, {},\nand {}.\n",
                self.authors[0], self.authors[1], self.authors[2], self.authors[3]
            )?,
            5 => writeln!(
                writer,
                "Written by {}, {}, {},\n{}, and {}.\n",
                self.authors[0], self.authors[1], self.authors[2], self.authors[3], self.authors[4]
            )?,
            6 => writeln!(
                writer,
                "Written by {}, {}, {},\n{}, {}, and {}.\n",
                self.authors[0], self.authors[1], self.authors[2], self.authors[3], self.authors[4],
                self.authors[5]
            )?,
            7 => writeln!(
                writer,
                "Written by {}, {}, {},\n{}, {}, {}, and {}.\n",
                self.authors[0], self.authors[1], self.authors[2], self.authors[3], self.authors[4],
                self.authors[5], self.authors[6]
            )?,
            8 => writeln!(
                writer,
                "Written by {}, {}, {},\n{}, {}, {}, {},\nand {}.\n",
                self.authors[0], self.authors[1], self.authors[2], self.authors[3], self.authors[4],
                self.authors[5], self.authors[6], self.authors[7]
            )?,
            9 => writeln!(
                writer,
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, and {}.\n",
                self.authors[0], self.authors[1], self.authors[2], self.authors[3], self.authors[4],
                self.authors[5], self.authors[6], self.authors[7], self.authors[8]
            )?,
            _ => writeln!(
                writer,
                "Written by {}, {}, {},\n{}, {}, {}, {},\n{}, {}, and others.\n",
                self.authors[0], self.authors[1], self.authors[2], self.authors[3], self.authors[4],
                self.authors[5], self.authors[6], self.authors[7], self.authors[8]
            )?,
        }

        Ok(())
    }
}

fn emit_bug_reporting_address(writer: &mut impl Write) -> io::Result<()> {
    writeln!(writer)?;
    writeln!(writer, "Report bugs to: bug-tar@gnu.org")?;
    writeln!(writer, "GNU tar home page: <http://www.gnu.org/software/tar/>")?;
    writeln!(
        writer,
        "General help using GNU software: <https://www.gnu.org/gethelp/>"
    )?;
    Ok(())
}

// Helper function to convert C strings to Rust &str safely
unsafe fn c_str_to_str<'a>(s: *const libc::c_char) -> Option<&'a str> {
    if s.is_null() {
        None
    } else {
        Some(CStr::from_ptr(s).to_str().unwrap_or(""))
    }
}

// Safe wrapper for the version functions
pub fn version_etc(
    writer: &mut impl Write,
    command_name: Option<&str>,
    package: &str,
    version: &str,
    authors: Vec<&str>,
) -> io::Result<()> {
    let info = VersionInfo::new(command_name, package, version, authors);
    info.display(writer)
}

// For FFI compatibility
#[no_mangle]
pub unsafe extern "C" fn version_etc_arn(
    stream: *mut libc::FILE,
    command_name: *const libc::c_char,
    package: *const libc::c_char,
    version: *const libc::c_char,
    authors: *const *const libc::c_char,
    n_authors: libc::size_t,
) {
    let mut writer = io::stdout();
    let cmd_name = c_str_to_str(command_name);
    let pkg = c_str_to_str(package).unwrap_or("");
    let ver = c_str_to_str(version).unwrap_or("");
    
    let authors_slice = std::slice::from_raw_parts(authors, n_authors);
    let authors_vec: Vec<&str> = authors_slice
        .iter()
        .filter_map(|&ptr| c_str_to_str(ptr))
        .collect();

    version_etc(&mut writer, cmd_name, pkg, ver, authors_vec).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn emit_bug_reporting_address() {
    emit_bug_reporting_address(&mut io::stdout()).unwrap();
}