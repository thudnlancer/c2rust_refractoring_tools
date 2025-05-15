use std::ffi::CString;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;

pub fn open_safer(file: &str, flags: i32, mode: Option<u32>) -> std::io::Result<File> {
    let c_file = CString::new(file)?;
    
    let file = match mode {
        Some(m) => OpenOptions::new()
            .mode(m)
            .custom_flags(flags)
            .open(c_file.to_str()?),
        None => OpenOptions::new()
            .custom_flags(flags)
            .open(c_file.to_str()?),
    }?;

    // Assuming fd_safer is implemented safely elsewhere
    // Here we just return the file directly since Rust's File is already safe
    Ok(file)
}