use std::fs::File;
use std::io::{self, Error, ErrorKind, Write, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct FileWrapper {
    file: File,
    pending: bool,
}

impl FileWrapper {
    pub fn new(file: File) -> Self {
        FileWrapper {
            file,
            pending: false,
        }
    }

    pub fn pending(&mut self) -> io::Result<bool> {
        let pos = self.file.stream_position()?;
        self.pending = pos > 0;
        Ok(self.pending)
    }

    pub fn close(self) -> io::Result<()> {
        let prev_fail = self.file.sync_all().is_err();
        let fclose_fail = self.file.sync_all().is_err(); // Simulate fclose

        if prev_fail || (fclose_fail && (self.pending || Error::last_os_error().raw_os_error() != Some(9))) {
            if !fclose_fail {
                // Reset errno to 0
                let _ = Error::last_os_error();
            }
            return Err(Error::new(ErrorKind::Other, "close failed"));
        }
        Ok(())
    }
}

pub fn close_stream(file: File) -> io::Result<()> {
    let mut wrapper = FileWrapper::new(file);
    wrapper.pending()?;
    wrapper.close()
}