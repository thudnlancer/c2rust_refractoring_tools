use std::io::{Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::fs::File;

#[derive(Debug)]
pub enum SeekError {
    IoError(std::io::Error),
    UnsupportedOperation,
}

pub fn rpl_fseek(file: &mut File, offset: i64, whence: i32) -> Result<(), SeekError> {
    let seek_from = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return Err(SeekError::UnsupportedOperation),
    };

    file.seek(seek_from)
        .map(|_| ())
        .map_err(SeekError::IoError)
}