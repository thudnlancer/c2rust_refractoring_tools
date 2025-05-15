use std::fs::File;
use std::io::{Error, ErrorKind, Result, Write};
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct FileWrapper {
    file: File,
    pending_data: bool,
}

impl FileWrapper {
    pub fn new(file: File) -> Self {
        FileWrapper {
            file,
            pending_data: false,
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.pending_data = !buf.is_empty();
        self.file.write_all(buf)
    }

    pub fn close(self) -> Result<()> {
        let prev_fail = self.file.sync_all().is_err();
        let fclose_fail = self.file.sync_all().is_err();

        if prev_fail || (fclose_fail && (self.pending_data || Error::last_os_error().raw_os_error() != Some(9))) {
            if !fclose_fail {
                // Clear the error if close didn't fail
                let _ = Error::last_os_error();
            }
            return Err(Error::new(ErrorKind::Other, "Stream close failed"));
        }

        Ok(())
    }
}

impl AsRawFd for FileWrapper {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.file.as_raw_fd()
    }
}