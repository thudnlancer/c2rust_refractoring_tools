use std::env;
use std::ffi::CString;
use std::io::{self, Write};

pub fn print_end_colorize(sgr_end: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(sgr_end.as_bytes())?;
    Ok(())
}

pub fn print_start_colorize(sgr_start: &str, sgr_seq: &str) -> io::Result<()> {
    print!("{}", sgr_start.replace("{}", sgr_seq));
    Ok(())
}

pub fn init_colorize() {}

pub fn should_colorize() -> bool {
    match env::var("TERM") {
        Ok(term) => term != "dumb",
        Err(_) => false,
    }
}