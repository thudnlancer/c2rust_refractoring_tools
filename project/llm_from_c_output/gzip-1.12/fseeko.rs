use std::fs::File;
use std::io::{Seek, SeekFrom, Error, ErrorKind};
use std::os::unix::io::AsRawFd;

pub fn fseeko(file: &mut File, offset: i64, whence: i32) -> Result<(), Error> {
    let seek_from = match whence {
        libc::SEEK_SET => SeekFrom::Start(offset as u64),
        libc::SEEK_CUR => SeekFrom::Current(offset),
        libc::SEEK_END => SeekFrom::End(offset),
        _ => return Err(Error::new(ErrorKind::InvalidInput, "invalid whence")),
    };

    // Check if the file is seekable
    let fd = file.as_raw_fd();
    unsafe {
        if libc::lseek(fd, 0, libc::SEEK_CUR) == -1 {
            return Err(Error::last_os_error());
        }
    }

    // Attempt to seek directly if buffers are empty
    file.seek(seek_from)?;

    Ok(())
}