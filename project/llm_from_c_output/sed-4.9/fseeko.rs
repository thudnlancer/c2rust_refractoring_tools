use std::fs::File;
use std::io::{Seek, SeekFrom, Error, ErrorKind};
use std::os::unix::io::AsRawFd;

pub fn fseeko(file: &mut File, offset: i64, whence: SeekFrom) -> Result<(), Error> {
    // Check if the file is seekable (Unix-specific)
    unsafe {
        let current_pos = libc::lseek(file.as_raw_fd(), 0, libc::SEEK_CUR);
        if current_pos == -1 {
            return Err(Error::last_os_error());
        }
    }

    // Simplified version without all the platform-specific checks
    // since Rust's File handles buffering internally
    match file.seek(whence) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == ErrorKind::InvalidInput {
                // Handle special cases where seek might fail on certain platforms
                Err(Error::new(ErrorKind::Other, "seek failed on non-seekable file"))
            } else {
                Err(e)
            }
        }
    }
}