use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{OpenOptionsExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::ptr;
use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use libc::{access, F_OK, O_CLOEXEC, O_DIRECTORY, O_NOCTTY, O_NONBLOCK};

const OPENAT_BUFFER_SIZE: usize = 1024; // Adjust based on actual needs
const PROC_SELF_FD_FORMAT: &str = "/proc/self/fd/%d/";

/// Creates a path for the subfile of the directory identified by `fd`.
/// Returns the path in the provided buffer if it fits, otherwise allocates a new string.
/// Returns None on error (with errno set).
pub fn openat_proc_name(buf: &mut [u8; OPENAT_BUFFER_SIZE], fd: RawFd, file: &str) -> Option<PathBuf> {
    if file.is_empty() {
        buf[0] = b'\0';
        return Some(PathBuf::from(unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) }.to_str().ok()?));
    }

    #[cfg(not(target_os = "os2"))]
    {
        static PROC_STATUS: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
        let proc_status = PROC_STATUS.get_or_init(|| {
            let proc_self_fd = OpenOptions::new()
                .read(true)
                .directory(true)
                .custom_flags(O_DIRECTORY | O_NOCTTY | O_NONBLOCK | O_CLOEXEC)
                .open("/proc/self/fd")
                .map(|f| f.as_raw_fd())
                .unwrap_or(-1);

            if proc_self_fd < 0 {
                return -1;
            }

            let dotdot_buf = format!("{}{}", format!(PROC_SELF_FD_FORMAT, proc_self_fd), "../fd");
            let status = unsafe { access(dotdot_buf.as_ptr() as *const i8, F_OK) };
            unsafe { libc::close(proc_self_fd) };
            if status != 0 { -1 } else { 1 }
        });

        if *proc_status < 0 {
            return None;
        }

        let path_str = format!("{}{}", format!(PROC_SELF_FD_FORMAT, fd), file);
        if path_str.len() >= OPENAT_BUFFER_SIZE {
            return Some(PathBuf::from(path_str));
        }

        unsafe {
            ptr::copy_nonoverlapping(
                path_str.as_ptr(),
                buf.as_mut_ptr(),
                path_str.len().min(OPENAT_BUFFER_SIZE),
            );
            *buf.get_unchecked_mut(path_str.len()) = b'\0';
        }
        Some(PathBuf::from(unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) }.to_str().ok()?))
    }

    #[cfg(target_os = "os2")]
    {
        let mut dir = [0u8; libc::_MAX_PATH as usize];
        if unsafe { __libc_Back_ioFHToPath(fd, dir.as_mut_ptr() as *mut i8, dir.len() as u32) } != 0 {
            return None;
        }

        let dir_len = unsafe { libc::strlen(dir.as_ptr() as *const i8) };
        let path_str = format!(
            "{}/{}",
            unsafe { std::ffi::CStr::from_ptr(dir.as_ptr() as *const i8) }.to_str().ok()?,
            file
        );

        if path_str.len() >= OPENAT_BUFFER_SIZE {
            return Some(PathBuf::from(path_str));
        }

        unsafe {
            ptr::copy_nonoverlapping(
                path_str.as_ptr(),
                buf.as_mut_ptr(),
                path_str.len().min(OPENAT_BUFFER_SIZE),
            );
            *buf.get_unchecked_mut(path_str.len()) = b'\0';
        }
        Some(PathBuf::from(unsafe { std::ffi::CStr::from_ptr(buf.as_ptr() as *const i8) }.to_str().ok()?))
    }
}