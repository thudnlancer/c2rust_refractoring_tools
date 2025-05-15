use std::fs::File;
use std::io::{Error, Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

#[derive(Debug)]
pub enum CloseError {
    FlushError(Error),
    CloseError(Error),
}

pub fn rpl_fclose(file: File) -> Result<(), CloseError> {
    let fd = file.as_raw_fd();
    if fd < 0 {
        return Err(CloseError::CloseError(Error::last_os_error()));
    }

    let mut saved_errno = None;
    {
        let mut file_ref = &file;
        let is_reading = file_ref.seek(SeekFrom::Current(0)).is_ok();
        
        if (!is_reading || file_ref.seek(SeekFrom::Current(0)).is_err()) && file_ref.flush().is_err() {
            saved_errno = Some(Error::last_os_error());
        }
    }

    match file.into_raw_fd() {
        fd if fd >= 0 => {
            unsafe {
                let _ = File::from_raw_fd(fd);
            }
            Ok(())
        }
        _ => {
            if let Some(err) = saved_errno {
                Err(CloseError::CloseError(err))
            } else {
                Err(CloseError::CloseError(Error::last_os_error()))
            }
        }
    }
}