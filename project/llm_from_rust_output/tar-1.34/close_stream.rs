use std::io::{self, Error, ErrorKind};
use std::fs::File;
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
pub struct FileWrapper {
    file: File,
}

impl FileWrapper {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub fn close(self) -> io::Result<()> {
        let pending = self.pending()?;
        let prev_fail = self.error()?;
        let fclose_fail = match self.file.sync_all() {
            Ok(_) => false,
            Err(e) => {
                if e.raw_os_error() != Some(9) { // EBADF
                    true
                } else {
                    false
                }
            }
        };

        if prev_fail || (fclose_fail && (pending || Error::last_os_error().raw_os_error() != Some(9))) {
            if !fclose_fail {
                // Clear errno
                let _ = Error::last_os_error();
            }
            return Err(Error::new(ErrorKind::Other, "stream close failed"));
        }

        Ok(())
    }

    fn pending(&self) -> io::Result<bool> {
        // In Rust, we don't have direct access to __fpending
        // This is an approximation - checking if we're at EOF
        let metadata = self.file.metadata()?;
        let pos = self.file.stream_position()?;
        Ok(pos < metadata.len())
    }

    fn error(&self) -> io::Result<bool> {
        // Check for error state
        match self.file.try_clone() {
            Ok(_) => Ok(false),
            Err(e) => {
                if e.raw_os_error() == Some(9) { // EBADF
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
}

impl AsRawFd for FileWrapper {
    fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
        self.file.as_raw_fd()
    }
}