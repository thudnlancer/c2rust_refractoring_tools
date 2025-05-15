use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::ptr;

const OPENAT_BUFFER_SIZE: usize = 1024;

/// Create /proc/self/fd-related names for subfiles of open directories.
///
/// Returns a buffer containing the path if successful.
/// The buffer may be either the provided buffer or a newly allocated one.
/// Returns an error if the operation fails.
pub fn openat_proc_name(
    buf: &mut [u8; OPENAT_BUFFER_SIZE],
    fd: i32,
    file: &str,
) -> Result<PathBuf, io::Error> {
    // Make sure the caller gets ENOENT when appropriate
    if file.is_empty() {
        buf[0] = b'\0';
        return Ok(PathBuf::from(unsafe { CString::from_vec_unchecked(buf[..1].to_vec()) }));
    }

    #[cfg(not(any(target_os = "os2", target_os = "zos")))]
    {
        const PROC_SELF_FD_FORMAT: &str = "/proc/self/fd/";
        static PROC_STATUS: std::sync::OnceLock<i32> = std::sync::OnceLock::new();

        let proc_status = PROC_STATUS.get_or_init(|| {
            // Check if /proc/self/fd is reliable
            match OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_DIRECTORY | libc::O_NOCTTY | libc::O_NONBLOCK | libc::O_CLOEXEC)
                .open("/proc/self/fd")
            {
                Ok(proc_self_fd) => {
                    let dotdot_path = format!("{}{}/../fd", PROC_SELF_FD_FORMAT, proc_self_fd.as_raw_fd());
                    if Path::new(&dotdot_path).exists() {
                        1
                    } else {
                        -1
                    }
                }
                Err(_) => -1,
            }
        });

        if *proc_status < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "/proc/self/fd not reliable"));
        }

        let proc_path = format!("{}{}/{}", PROC_SELF_FD_FORMAT, fd, file);
        if proc_path.len() < OPENAT_BUFFER_SIZE {
            buf[..proc_path.len()].copy_from_slice(proc_path.as_bytes());
            buf[proc_path.len()] = b'\0';
            Ok(PathBuf::from(unsafe { CString::from_vec_unchecked(buf[..=proc_path.len()].to_vec()) }))
        } else {
            Ok(PathBuf::from(proc_path))
        }
    }

    #[cfg(any(target_os = "os2", target_os = "zos"))]
    {
        let dir = if cfg!(target_os = "os2") {
            // OS/2 kLIBC implementation would go here
            return Err(io::Error::new(io::ErrorKind::Unsupported, "OS/2 not supported"));
        } else {
            // z/OS implementation would go here
            return Err(io::Error::new(io::ErrorKind::Unsupported, "z/OS not supported"));
        };

        let full_path = Path::new(&dir).join(file);
        let path_str = full_path.to_string_lossy();
        if path_str.len() < OPENAT_BUFFER_SIZE {
            buf[..path_str.len()].copy_from_slice(path_str.as_bytes());
            buf[path_str.len()] = b'\0';
            Ok(PathBuf::from(unsafe { CString::from_vec_unchecked(buf[..=path_str.len()].to_vec()) }))
        } else {
            Ok(full_path)
        }
    }
}