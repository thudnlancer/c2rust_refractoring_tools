use std::ffi::CString;
use std::io::{self, Write};
use std::ptr;

pub fn explain_how_to_report_bugs<W: Write>(
    mut writer: W,
    program_name: &str,
) -> io::Result<()> {
    let message = format!(
        "Please see also the documentation at {}.\n\
         You can report (and track progress on fixing) bugs in the \"{}\"\n\
         program via the {} bug-reporting page at\n\
         {} or, if\n\
         you have no web access, by sending email to <{}>.\n",
        "http://www.gnu.org/software/findutils/",
        program_name,
        "GNU findutils",
        "https://savannah.gnu.org/bugs/?group=findutils",
        "bug-findutils@gnu.org"
    );

    writer.write_all(message.as_bytes())?;
    Ok(())
}