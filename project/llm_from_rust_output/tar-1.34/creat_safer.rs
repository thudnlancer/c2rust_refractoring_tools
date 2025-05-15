use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use libc::mode_t;

pub fn creat_safer(file: &std::ffi::CStr, mode: mode_t) -> std::io::Result<File> {
    let file_path = file.to_str().map_err(|_| std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid file path"
    ))?;
    
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(mode)
        .open(file_path)?;
    
    Ok(file)
}