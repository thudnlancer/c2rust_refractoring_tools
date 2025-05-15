use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::io::{Result, Error, ErrorKind};

/// Open a directory relative to another directory.
///
/// Relative to `dir_fd`, open the directory `dir`, passing `extra_flags` to
/// the underlying openat call. On success, store into `pnew_fd` the
/// underlying file descriptor of the newly opened directory and return
/// the directory stream. On failure, return an error.
///
/// On success, `pnew_fd` is at least 3, so this is a "safer" function.
pub fn opendirat(dir_fd: RawFd, dir: &Path, extra_flags: i32, pnew_fd: &mut RawFd) -> Result<std::fs::ReadDir> {
    let open_flags = libc::O_RDONLY | libc::O_CLOEXEC | libc::O_DIRECTORY | 
                     libc::O_NOCTTY | libc::O_NONBLOCK | extra_flags;
    
    // SAFETY: We immediately convert the raw fd to a managed File
    let new_fd = unsafe {
        let path_cstr = std::ffi::CString::new(dir.to_str().ok_or_else(|| Error::new(ErrorKind::InvalidInput, "invalid path"))?)?;
        libc::openat(dir_fd, path_cstr.as_ptr(), open_flags)
    };

    if new_fd < 0 {
        return Err(Error::last_os_error());
    }

    // SAFETY: We just created this fd and know it's valid
    let file = unsafe { File::from_raw_fd(new_fd) };
    
    match std::fs::ReadDir::from_file(file) {
        Ok(read_dir) => {
            *pnew_fd = new_fd;
            Ok(read_dir)
        }
        Err(e) => {
            // Close the fd if fdopendir failed
            unsafe { libc::close(new_fd) };
            Err(e)
        }
    }
}