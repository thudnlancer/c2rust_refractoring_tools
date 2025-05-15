use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct FileWrapper {
    file: File,
    offset: i64,
    flags: i32,
}

impl FileWrapper {
    pub fn new(file: File) -> Self {
        FileWrapper {
            file,
            offset: 0,
            flags: 0,
        }
    }

    pub fn rpl_fseeko(&mut self, offset: i64, whence: i32) -> std::io::Result<()> {
        let seek_from = match whence {
            0 => SeekFrom::Start(offset as u64),
            1 => SeekFrom::Current(offset),
            2 => SeekFrom::End(offset),
            _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid whence")),
        };

        let pos = self.file.seek(seek_from)?;
        self.offset = pos as i64;
        self.flags &= !0x10;
        Ok(())
    }

    pub fn as_raw_fd(&self) -> i32 {
        self.file.as_raw_fd()
    }
}