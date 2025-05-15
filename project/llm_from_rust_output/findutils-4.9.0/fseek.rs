use std::io::{Seek, SeekFrom};
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct FileWrapper {
    file: std::fs::File,
}

impl FileWrapper {
    pub fn new(file: std::fs::File) -> Self {
        Self { file }
    }

    pub fn seek(&mut self, offset: i64, whence: i32) -> std::io::Result<()> {
        let seek_from = match whence {
            0 => SeekFrom::Start(offset as u64),
            1 => SeekFrom::Current(offset),
            2 => SeekFrom::End(offset),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid whence")),
        };
        self.file.seek(seek_from).map(|_| ())
    }
}

pub fn rpl_fseek(fp: &mut FileWrapper, offset: i64, whence: i32) -> i32 {
    match fp.seek(offset, whence) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}