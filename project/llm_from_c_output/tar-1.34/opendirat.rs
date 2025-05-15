use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, FileExt};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::path::Path;
use std::io;
use libc::{O_RDONLY, O_CLOEXEC, O_DIRECTORY, O_NOCTTY, O_NONBLOCK};

/// Open a directory relative to another directory.
///
/// Relative to `dir_fd`, open the directory `dir`, passing `extra_flags` to
/// the underlying openat call. On success, store into `pnew_fd` the
/// underlying file descriptor of the newly opened directory and return
/// the directory stream. On failure, return an error.
///
/// On success, `pnew_fd` is at least 3, so this is a "safer" function.
pub fn opendirat(dir_fd: RawFd, dir: &Path, extra_flags: i32, pnew_fd: &mut i32) -> io::Result<std::fs::File> {
    let open_flags = O_RDONLY | O_CLOEXEC | O_DIRECTORY | O_NOCTTY | O_NONBLOCK | extra_flags;
    
    let new_fd = unsafe {
        let path_cstr = std::ffi::CString::new(dir.to_str().ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid path string",
        ))?)?;
        libc::openat(dir_fd, path_cstr.as_ptr(), open_flags)
    };
    
    if new_fd < 0 {
        return Err(io::Error::last_os_error());
    }
    
    let file = unsafe { File::from_raw_fd(new_fd) };
    
    // Verify it's actually a directory
    if file.metadata()?.is_dir() {
        *pnew_fd = new_fd;
        Ok(file)
    } else {
        drop(file); // This will close the file descriptor
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a directory",
        ))
    }
}