use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub enum SeekError {
    InvalidWhence,
    SeekFailed,
}

pub fn rpl_fseeko(
    file: &mut File,
    offset: i64,
    whence: i32,
) -> Result<(), SeekError> {
    let seek_from = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return Err(SeekError::InvalidWhence),
    };

    file.seek(seek_from)
        .map(|_| ())
        .map_err(|_| SeekError::SeekFailed)
}

// Note: The original C code had special handling for certain FILE* conditions,
// but in Rust we work with File directly which doesn't expose those low-level
// buffer details. The Rust version provides equivalent seeking functionality
// while being completely safe.