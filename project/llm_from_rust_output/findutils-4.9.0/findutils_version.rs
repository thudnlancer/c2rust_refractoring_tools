use std::ffi::CString;
use std::io::{self, Write};

const VERSION_STRING: &str = "4.9.0";
const PACKAGE_NAME: &str = "GNU findutils";
const AUTHORS: &[&str] = &["Eric B. Decker", "James Youngman", "Kevin Dalley"];

pub fn display_findutils_version(official_name: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    
    writeln!(stdout_handle, "{} {}", official_name, VERSION_STRING)?;
    writeln!(stdout_handle, "{}", PACKAGE_NAME)?;
    
    for author in AUTHORS {
        writeln!(stdout_handle, "{}", author)?;
    }
    
    stdout_handle.flush()?;
    Ok(())
}